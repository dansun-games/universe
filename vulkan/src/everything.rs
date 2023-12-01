use std::ffi::c_void;
use bitflags::bitflags;

pub type Bool32 = u32;

pub type SampleMask = u32;

pub type DeviceAddress = u64;

pub type DeviceSize = u64;

pub type Flags = u32;

pub type Flags64 = u64;

const QUEUE_FAMILY_FOREIGN_EXT: u32 = !2;

const MAX_DESCRIPTION_SIZE: usize = 256;

const MAX_DRIVER_NAME_SIZE: usize = 256;

const MAX_GLOBAL_PRIORITY_SIZE_KHR: usize = 16;

const MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: usize = 32;

const MAX_MEMORY_HEAPS: usize = 16;

const FALSE: Bool32 = 0;

const MAX_DEVICE_GROUP_SIZE: usize = 32;

const REMAINING_MIP_LEVELS: u32 = !0;

const SHADER_INDEX_UNUSED_AMDX: u32 = !0;

const LUID_SIZE: usize = 8;

const SUBPASS_EXTERNAL: u32 = !0;

const TRUE: Bool32 = 1;

const QUEUE_FAMILY_EXTERNAL: u32 = !1;

const LOD_CLAMP_NONE: f32 = 1000.0;

const REMAINING_ARRAY_LAYERS: u32 = !0;

const REMAINING_3D_SLICES_EXT: u32 = !0;

const SHADER_UNUSED_KHR: u32 = !0;

const MAX_EXTENSION_NAME_SIZE: usize = 256;

const MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;

const MAX_DRIVER_INFO_SIZE: usize = 256;

const QUEUE_FAMILY_IGNORED: u32 = !0;

const UUID_SIZE: usize = 16;

const ATTACHMENT_UNUSED: u32 = !0;

const MAX_MEMORY_TYPES: usize = 32;

const WHOLE_SIZE: u64 = !0;

//const MAX_GLOBAL_PRIORITY_SIZE_EXT: TODO = MAX_GLOBAL_PRIORITY_SIZE_KHR;

//const QUEUE_FAMILY_EXTERNAL_KHR: TODO = QUEUE_FAMILY_EXTERNAL;

//const MAX_DEVICE_GROUP_SIZE_KHR: TODO = MAX_DEVICE_GROUP_SIZE;

//const LUID_SIZE_KHR: TODO = LUID_SIZE;

//const MAX_DRIVER_NAME_SIZE_KHR: TODO = MAX_DRIVER_NAME_SIZE;

//const MAX_DRIVER_INFO_SIZE_KHR: TODO = MAX_DRIVER_INFO_SIZE;

//const SHADER_UNUSED_NV: TODO = SHADER_UNUSED_KHR;

#[repr(transparent)]
pub struct ShaderModule(u64);

#[repr(transparent)]
pub struct DescriptorPool(u64);

#[repr(transparent)]
pub struct DeferredOperationKHR(u64);

#[repr(transparent)]
pub struct BufferView(u64);

#[repr(transparent)]
pub struct Sampler(u64);

#[repr(transparent)]
pub struct Event(u64);

#[repr(transparent)]
pub struct CuModuleNVX(u64);

#[repr(transparent)]
pub struct CuFunctionNVX(u64);

#[repr(transparent)]
pub struct Semaphore(u64);

#[repr(transparent)]
pub struct ValidationCacheEXT(u64);

#[repr(transparent)]
pub struct Pipeline(u64);

#[repr(transparent)]
pub struct BufferCollectionFUCHSIA(u64);

#[repr(transparent)]
pub struct MicromapEXT(u64);

#[repr(transparent)]
pub struct Framebuffer(u64);

#[repr(transparent)]
pub struct Queue(usize);

#[repr(transparent)]
pub struct DebugReportCallbackEXT(u64);

#[repr(transparent)]
pub struct SwapchainKHR(u64);

#[repr(transparent)]
pub struct Device(usize);

#[repr(transparent)]
pub struct IndirectCommandsLayoutNV(u64);

#[repr(transparent)]
pub struct Buffer(u64);

#[repr(transparent)]
pub struct PrivateDataSlot(u64);

#[repr(transparent)]
pub struct VideoSessionKHR(u64);

#[repr(transparent)]
pub struct DisplayModeKHR(u64);

#[repr(transparent)]
pub struct ShaderEXT(u64);

#[repr(transparent)]
pub struct Instance(usize);

#[repr(transparent)]
pub struct RenderPass(u64);

#[repr(transparent)]
pub struct AccelerationStructureNV(u64);

#[repr(transparent)]
pub struct PipelineCache(u64);

#[repr(transparent)]
pub struct DescriptorUpdateTemplate(u64);

#[repr(transparent)]
pub struct VideoSessionParametersKHR(u64);

#[repr(transparent)]
pub struct CommandBuffer(usize);

#[repr(transparent)]
pub struct CommandPool(u64);

#[repr(transparent)]
pub struct DeviceMemory(u64);

#[repr(transparent)]
pub struct PhysicalDevice(usize);

#[repr(transparent)]
pub struct DescriptorSet(u64);

#[repr(transparent)]
pub struct PipelineLayout(u64);

#[repr(transparent)]
pub struct Image(u64);

#[repr(transparent)]
pub struct ImageView(u64);

#[repr(transparent)]
pub struct DescriptorSetLayout(u64);

#[repr(transparent)]
pub struct SamplerYcbcrConversion(u64);

#[repr(transparent)]
pub struct AccelerationStructureKHR(u64);

#[repr(transparent)]
pub struct PerformanceConfigurationINTEL(u64);

#[repr(transparent)]
pub struct OpticalFlowSessionNV(u64);

#[repr(transparent)]
pub struct DisplayKHR(u64);

#[repr(transparent)]
pub struct QueryPool(u64);

#[repr(transparent)]
pub struct SurfaceKHR(u64);

#[repr(transparent)]
pub struct Fence(u64);

#[repr(transparent)]
pub struct DebugUtilsMessengerEXT(u64);

#[repr(transparent)]
pub struct CudaModuleNV(u64);

#[repr(transparent)]
pub struct CudaFunctionNV(u64);

#[repr(transparent)]
pub struct SemaphoreSciSyncPoolNV(u64);

#[repr(C)]
pub enum ProvokingVertexModeEXT {
	ProvokingVertexModeFirstVertexExt = 0,
	ProvokingVertexModeLastVertexExt = 1,
}

#[repr(C)]
pub enum ShaderGroupShaderKHR {
	ShaderGroupShaderGeneralKhr = 0,
	ShaderGroupShaderClosestHitKhr = 1,
	ShaderGroupShaderAnyHitKhr = 2,
	ShaderGroupShaderIntersectionKhr = 3,
}

#[repr(C)]
pub enum DirectDriverLoadingModeLUNARG {
	DirectDriverLoadingModeExclusiveLunarg = 0,
	DirectDriverLoadingModeInclusiveLunarg = 1,
}

#[repr(C)]
pub enum ValidationFeatureEnableEXT {
	ValidationFeatureEnableGpuAssistedExt = 0,
	ValidationFeatureEnableGpuAssistedReserveBindingSlotExt = 1,
	ValidationFeatureEnableBestPracticesExt = 2,
	ValidationFeatureEnableDebugPrintfExt = 3,
	ValidationFeatureEnableSynchronizationValidationExt = 4,
}

#[repr(C)]
pub enum RayTracingInvocationReorderModeNV {
	RayTracingInvocationReorderModeNoneNv = 0,
	RayTracingInvocationReorderModeReorderNv = 1,
}

#[repr(C)]
pub enum SystemAllocationScope {
	SystemAllocationScopeCommand = 0,
	SystemAllocationScopeObject = 1,
	SystemAllocationScopeCache = 2,
	SystemAllocationScopeDevice = 3,
	SystemAllocationScopeInstance = 4,
}

#[repr(C)]
pub enum BlendFactor {
	BlendFactorZero = 0,
	BlendFactorOne = 1,
	BlendFactorSrcColor = 2,
	BlendFactorOneMinusSrcColor = 3,
	BlendFactorDstColor = 4,
	BlendFactorOneMinusDstColor = 5,
	BlendFactorSrcAlpha = 6,
	BlendFactorOneMinusSrcAlpha = 7,
	BlendFactorDstAlpha = 8,
	BlendFactorOneMinusDstAlpha = 9,
	BlendFactorConstantColor = 10,
	BlendFactorOneMinusConstantColor = 11,
	BlendFactorConstantAlpha = 12,
	BlendFactorOneMinusConstantAlpha = 13,
	BlendFactorSrcAlphaSaturate = 14,
	BlendFactorSrc1Color = 15,
	BlendFactorOneMinusSrc1Color = 16,
	BlendFactorSrc1Alpha = 17,
	BlendFactorOneMinusSrc1Alpha = 18,
}

#[repr(C)]
pub enum ComponentTypeKHR {
	ComponentTypeFloat16Khr = 0,
	ComponentTypeFloat32Khr = 1,
	ComponentTypeFloat64Khr = 2,
	ComponentTypeSint8Khr = 3,
	ComponentTypeSint16Khr = 4,
	ComponentTypeSint32Khr = 5,
	ComponentTypeSint64Khr = 6,
	ComponentTypeUint8Khr = 7,
	ComponentTypeUint16Khr = 8,
	ComponentTypeUint32Khr = 9,
	ComponentTypeUint64Khr = 10,
}

#[repr(C)]
pub enum OpacityMicromapFormatEXT {
	OpacityMicromapFormat2StateExt = 1,
	OpacityMicromapFormat4StateExt = 2,
}

#[repr(C)]
pub enum BlockMatchWindowCompareModeQCOM {
	BlockMatchWindowCompareModeMinQcom = 0,
	BlockMatchWindowCompareModeMaxQcom = 1,
}

#[repr(C)]
pub enum ValidationCacheHeaderVersionEXT {
	ValidationCacheHeaderVersionOneExt = 1,
}

#[repr(C)]
pub enum LatencyMarkerNV {
	LatencyMarkerSimulationStartNv = 0,
	LatencyMarkerSimulationEndNv = 1,
	LatencyMarkerRendersubmitStartNv = 2,
	LatencyMarkerRendersubmitEndNv = 3,
	LatencyMarkerPresentStartNv = 4,
	LatencyMarkerPresentEndNv = 5,
	LatencyMarkerInputSampleNv = 6,
	LatencyMarkerTriggerFlashNv = 7,
	LatencyMarkerOutOfBandRendersubmitStartNv = 8,
	LatencyMarkerOutOfBandRendersubmitEndNv = 9,
	LatencyMarkerOutOfBandPresentStartNv = 10,
	LatencyMarkerOutOfBandPresentEndNv = 11,
}

#[repr(C)]
pub enum FragmentShadingRateCombinerOpKHR {
	FragmentShadingRateCombinerOpKeepKhr = 0,
	FragmentShadingRateCombinerOpReplaceKhr = 1,
	FragmentShadingRateCombinerOpMinKhr = 2,
	FragmentShadingRateCombinerOpMaxKhr = 3,
	FragmentShadingRateCombinerOpMulKhr = 4,
}

#[repr(C)]
pub enum OpticalFlowSessionBindingPointNV {
	OpticalFlowSessionBindingPointUnknownNv = 0,
	OpticalFlowSessionBindingPointInputNv = 1,
	OpticalFlowSessionBindingPointReferenceNv = 2,
	OpticalFlowSessionBindingPointHintNv = 3,
	OpticalFlowSessionBindingPointFlowVectorNv = 4,
	OpticalFlowSessionBindingPointBackwardFlowVectorNv = 5,
	OpticalFlowSessionBindingPointCostNv = 6,
	OpticalFlowSessionBindingPointBackwardCostNv = 7,
	OpticalFlowSessionBindingPointGlobalFlowNv = 8,
}

#[repr(C)]
pub enum SamplerAddressMode {
	SamplerAddressModeRepeat = 0,
	SamplerAddressModeMirroredRepeat = 1,
	SamplerAddressModeClampToEdge = 2,
	SamplerAddressModeClampToBorder = 3,
}

#[repr(C)]
pub enum TessellationDomainOrigin {
	TessellationDomainOriginUpperLeft = 0,
	TessellationDomainOriginLowerLeft = 1,
}

#[repr(C)]
pub enum CoverageReductionModeNV {
	CoverageReductionModeMergeNv = 0,
	CoverageReductionModeTruncateNv = 1,
}

#[repr(C)]
pub enum PerformanceCounterStorageKHR {
	PerformanceCounterStorageInt32Khr = 0,
	PerformanceCounterStorageInt64Khr = 1,
	PerformanceCounterStorageUint32Khr = 2,
	PerformanceCounterStorageUint64Khr = 3,
	PerformanceCounterStorageFloat32Khr = 4,
	PerformanceCounterStorageFloat64Khr = 5,
}

#[repr(C)]
pub enum DeviceFaultVendorBinaryHeaderVersionEXT {
	DeviceFaultVendorBinaryHeaderVersionOneExt = 1,
}

#[repr(C)]
pub enum DeviceEventTypeEXT {
	DeviceEventTypeDisplayHotplugExt = 0,
}

#[repr(C)]
pub enum SubpassContents {
	SubpassContentsInline = 0,
	SubpassContentsSecondaryCommandBuffers = 1,
}

#[repr(C)]
pub enum ViewportCoordinateSwizzleNV {
	ViewportCoordinateSwizzlePositiveXNv = 0,
	ViewportCoordinateSwizzleNegativeXNv = 1,
	ViewportCoordinateSwizzlePositiveYNv = 2,
	ViewportCoordinateSwizzleNegativeYNv = 3,
	ViewportCoordinateSwizzlePositiveZNv = 4,
	ViewportCoordinateSwizzleNegativeZNv = 5,
	ViewportCoordinateSwizzlePositiveWNv = 6,
	ViewportCoordinateSwizzleNegativeWNv = 7,
}

#[repr(C)]
pub enum AccelerationStructureTypeKHR {
	AccelerationStructureTypeTopLevelKhr = 0,
	AccelerationStructureTypeBottomLevelKhr = 1,
	AccelerationStructureTypeGenericKhr = 2,
}

#[repr(C)]
pub enum FullScreenExclusiveEXT {
	FullScreenExclusiveDefaultExt = 0,
	FullScreenExclusiveAllowedExt = 1,
	FullScreenExclusiveDisallowedExt = 2,
	FullScreenExclusiveApplicationControlledExt = 3,
}

#[repr(C)]
pub enum LineRasterizationModeEXT {
	LineRasterizationModeDefaultExt = 0,
	LineRasterizationModeRectangularExt = 1,
	LineRasterizationModeBresenhamExt = 2,
	LineRasterizationModeRectangularSmoothExt = 3,
}

#[repr(C)]
pub enum SciSyncPrimitiveTypeNV {
	SciSyncPrimitiveTypeFenceNv = 0,
	SciSyncPrimitiveTypeSemaphoreNv = 1,
}

#[repr(C)]
pub enum StencilOp {
	StencilOpKeep = 0,
	StencilOpZero = 1,
	StencilOpReplace = 2,
	StencilOpIncrementAndClamp = 3,
	StencilOpDecrementAndClamp = 4,
	StencilOpInvert = 5,
	StencilOpIncrementAndWrap = 6,
	StencilOpDecrementAndWrap = 7,
}

#[repr(C)]
pub enum ColorSpaceKHR {
	ColorSpaceSrgbNonlinearKhr = 0,
}

#[repr(C)]
pub enum ImageType {
	ImageType1d = 0,
	ImageType2d = 1,
	ImageType3d = 2,
}

#[repr(C)]
pub enum SamplerYcbcrRange {
	SamplerYcbcrRangeItuFull = 0,
	SamplerYcbcrRangeItuNarrow = 1,
}

#[repr(C)]
pub enum PipelineExecutableStatisticFormatKHR {
	PipelineExecutableStatisticFormatBool32Khr = 0,
	PipelineExecutableStatisticFormatInt64Khr = 1,
	PipelineExecutableStatisticFormatUint64Khr = 2,
	PipelineExecutableStatisticFormatFloat64Khr = 3,
}

#[repr(C)]
pub enum SubpassMergeStatusEXT {
	SubpassMergeStatusMergedExt = 0,
	SubpassMergeStatusDisallowedExt = 1,
	SubpassMergeStatusNotMergedSideEffectsExt = 2,
	SubpassMergeStatusNotMergedSamplesMismatchExt = 3,
	SubpassMergeStatusNotMergedViewsMismatchExt = 4,
	SubpassMergeStatusNotMergedAliasingExt = 5,
	SubpassMergeStatusNotMergedDependenciesExt = 6,
	SubpassMergeStatusNotMergedIncompatibleInputAttachmentExt = 7,
	SubpassMergeStatusNotMergedTooManyAttachmentsExt = 8,
	SubpassMergeStatusNotMergedInsufficientStorageExt = 9,
	SubpassMergeStatusNotMergedDepthStencilCountExt = 10,
	SubpassMergeStatusNotMergedResolveAttachmentReuseExt = 11,
	SubpassMergeStatusNotMergedSingleSubpassExt = 12,
	SubpassMergeStatusNotMergedUnspecifiedExt = 13,
}

#[repr(C)]
pub enum PerformanceValueTypeINTEL {
	PerformanceValueTypeUint32Intel = 0,
	PerformanceValueTypeUint64Intel = 1,
	PerformanceValueTypeFloatIntel = 2,
	PerformanceValueTypeBoolIntel = 3,
	PerformanceValueTypeStringIntel = 4,
}

#[repr(C)]
pub enum PipelineRobustnessBufferBehaviorEXT {
	PipelineRobustnessBufferBehaviorDeviceDefaultExt = 0,
	PipelineRobustnessBufferBehaviorDisabledExt = 1,
	PipelineRobustnessBufferBehaviorRobustBufferAccessExt = 2,
	PipelineRobustnessBufferBehaviorRobustBufferAccess2Ext = 3,
}

#[repr(C)]
pub enum PipelineCacheValidationVersion {
	PipelineCacheValidationVersionSafetyCriticalOne = 1,
}

#[repr(C)]
pub enum ChromaLocation {
	ChromaLocationCositedEven = 0,
	ChromaLocationMidpoint = 1,
}

#[repr(C)]
pub enum AccelerationStructureMotionInstanceTypeNV {
	AccelerationStructureMotionInstanceTypeStaticNv = 0,
	AccelerationStructureMotionInstanceTypeMatrixMotionNv = 1,
	AccelerationStructureMotionInstanceTypeSrtMotionNv = 2,
}

#[repr(C)]
pub enum QueryPoolSamplingModeINTEL {
	QueryPoolSamplingModeManualIntel = 0,
}

#[repr(C)]
pub enum PipelineCacheHeaderVersion {
	PipelineCacheHeaderVersionOne = 1,
}

#[repr(C)]
pub enum SamplerMipmapMode {
	SamplerMipmapModeNearest = 0,
	SamplerMipmapModeLinear = 1,
}

#[repr(C)]
pub enum PhysicalDeviceType {
	PhysicalDeviceTypeOther = 0,
	PhysicalDeviceTypeIntegratedGpu = 1,
	PhysicalDeviceTypeDiscreteGpu = 2,
	PhysicalDeviceTypeVirtualGpu = 3,
	PhysicalDeviceTypeCpu = 4,
}

#[repr(C)]
pub enum FrontFace {
	FrontFaceCounterClockwise = 0,
	FrontFaceClockwise = 1,
}

#[repr(C)]
pub enum IndirectCommandsTokenTypeNV {
	IndirectCommandsTokenTypeShaderGroupNv = 0,
	IndirectCommandsTokenTypeStateFlagsNv = 1,
	IndirectCommandsTokenTypeIndexBufferNv = 2,
	IndirectCommandsTokenTypeVertexBufferNv = 3,
	IndirectCommandsTokenTypePushConstantNv = 4,
	IndirectCommandsTokenTypeDrawIndexedNv = 5,
	IndirectCommandsTokenTypeDrawNv = 6,
	IndirectCommandsTokenTypeDrawTasksNv = 7,
}

#[repr(C)]
pub enum QueueGlobalPriorityKHR {
	QueueGlobalPriorityLowKhr = 128,
	QueueGlobalPriorityMediumKhr = 256,
	QueueGlobalPriorityHighKhr = 512,
	QueueGlobalPriorityRealtimeKhr = 1024,
}

#[repr(C)]
pub enum DisplacementMicromapFormatNV {
	DisplacementMicromapFormat64Triangles64BytesNv = 1,
	DisplacementMicromapFormat256Triangles128BytesNv = 2,
	DisplacementMicromapFormat1024Triangles128BytesNv = 3,
}

#[repr(C)]
pub enum QueryType {
	QueryTypeOcclusion = 0,
	QueryTypePipelineStatistics = 1,
	QueryTypeTimestamp = 2,
}

#[repr(C)]
pub enum Filter {
	FilterNearest = 0,
	FilterLinear = 1,
}

#[repr(C)]
pub enum StructureType {
	StructureTypeApplicationInfo = 0,
	StructureTypeInstanceCreateInfo = 1,
	StructureTypeDeviceQueueCreateInfo = 2,
	StructureTypeDeviceCreateInfo = 3,
	StructureTypeSubmitInfo = 4,
	StructureTypeMemoryAllocateInfo = 5,
	StructureTypeMappedMemoryRange = 6,
	StructureTypeBindSparseInfo = 7,
	StructureTypeFenceCreateInfo = 8,
	StructureTypeSemaphoreCreateInfo = 9,
	StructureTypeEventCreateInfo = 10,
	StructureTypeQueryPoolCreateInfo = 11,
	StructureTypeBufferCreateInfo = 12,
	StructureTypeBufferViewCreateInfo = 13,
	StructureTypeImageCreateInfo = 14,
	StructureTypeImageViewCreateInfo = 15,
	StructureTypeShaderModuleCreateInfo = 16,
	StructureTypePipelineCacheCreateInfo = 17,
	StructureTypePipelineShaderStageCreateInfo = 18,
	StructureTypePipelineVertexInputStateCreateInfo = 19,
	StructureTypePipelineInputAssemblyStateCreateInfo = 20,
	StructureTypePipelineTessellationStateCreateInfo = 21,
	StructureTypePipelineViewportStateCreateInfo = 22,
	StructureTypePipelineRasterizationStateCreateInfo = 23,
	StructureTypePipelineMultisampleStateCreateInfo = 24,
	StructureTypePipelineDepthStencilStateCreateInfo = 25,
	StructureTypePipelineColorBlendStateCreateInfo = 26,
	StructureTypePipelineDynamicStateCreateInfo = 27,
	StructureTypeGraphicsPipelineCreateInfo = 28,
	StructureTypeComputePipelineCreateInfo = 29,
	StructureTypePipelineLayoutCreateInfo = 30,
	StructureTypeSamplerCreateInfo = 31,
	StructureTypeDescriptorSetLayoutCreateInfo = 32,
	StructureTypeDescriptorPoolCreateInfo = 33,
	StructureTypeDescriptorSetAllocateInfo = 34,
	StructureTypeWriteDescriptorSet = 35,
	StructureTypeCopyDescriptorSet = 36,
	StructureTypeFramebufferCreateInfo = 37,
	StructureTypeRenderPassCreateInfo = 38,
	StructureTypeCommandPoolCreateInfo = 39,
	StructureTypeCommandBufferAllocateInfo = 40,
	StructureTypeCommandBufferInheritanceInfo = 41,
	StructureTypeCommandBufferBeginInfo = 42,
	StructureTypeRenderPassBeginInfo = 43,
	StructureTypeBufferMemoryBarrier = 44,
	StructureTypeImageMemoryBarrier = 45,
	StructureTypeMemoryBarrier = 46,
	StructureTypeLoaderInstanceCreateInfo = 47,
	StructureTypeLoaderDeviceCreateInfo = 48,
}

#[repr(C)]
pub enum RasterizationOrderAMD {
	RasterizationOrderStrictAmd = 0,
	RasterizationOrderRelaxedAmd = 1,
}

#[repr(C)]
pub enum PrimitiveTopology {
	PrimitiveTopologyPointList = 0,
	PrimitiveTopologyLineList = 1,
	PrimitiveTopologyLineStrip = 2,
	PrimitiveTopologyTriangleList = 3,
	PrimitiveTopologyTriangleStrip = 4,
	PrimitiveTopologyTriangleFan = 5,
	PrimitiveTopologyLineListWithAdjacency = 6,
	PrimitiveTopologyLineStripWithAdjacency = 7,
	PrimitiveTopologyTriangleListWithAdjacency = 8,
	PrimitiveTopologyTriangleStripWithAdjacency = 9,
	PrimitiveTopologyPatchList = 10,
}

#[repr(C)]
pub enum Format {
	FormatUndefined = 0,
	FormatR4g4UnormPack8 = 1,
	FormatR4g4b4a4UnormPack16 = 2,
	FormatB4g4r4a4UnormPack16 = 3,
	FormatR5g6b5UnormPack16 = 4,
	FormatB5g6r5UnormPack16 = 5,
	FormatR5g5b5a1UnormPack16 = 6,
	FormatB5g5r5a1UnormPack16 = 7,
	FormatA1r5g5b5UnormPack16 = 8,
	FormatR8Unorm = 9,
	FormatR8Snorm = 10,
	FormatR8Uscaled = 11,
	FormatR8Sscaled = 12,
	FormatR8Uint = 13,
	FormatR8Sint = 14,
	FormatR8Srgb = 15,
	FormatR8g8Unorm = 16,
	FormatR8g8Snorm = 17,
	FormatR8g8Uscaled = 18,
	FormatR8g8Sscaled = 19,
	FormatR8g8Uint = 20,
	FormatR8g8Sint = 21,
	FormatR8g8Srgb = 22,
	FormatR8g8b8Unorm = 23,
	FormatR8g8b8Snorm = 24,
	FormatR8g8b8Uscaled = 25,
	FormatR8g8b8Sscaled = 26,
	FormatR8g8b8Uint = 27,
	FormatR8g8b8Sint = 28,
	FormatR8g8b8Srgb = 29,
	FormatB8g8r8Unorm = 30,
	FormatB8g8r8Snorm = 31,
	FormatB8g8r8Uscaled = 32,
	FormatB8g8r8Sscaled = 33,
	FormatB8g8r8Uint = 34,
	FormatB8g8r8Sint = 35,
	FormatB8g8r8Srgb = 36,
	FormatR8g8b8a8Unorm = 37,
	FormatR8g8b8a8Snorm = 38,
	FormatR8g8b8a8Uscaled = 39,
	FormatR8g8b8a8Sscaled = 40,
	FormatR8g8b8a8Uint = 41,
	FormatR8g8b8a8Sint = 42,
	FormatR8g8b8a8Srgb = 43,
	FormatB8g8r8a8Unorm = 44,
	FormatB8g8r8a8Snorm = 45,
	FormatB8g8r8a8Uscaled = 46,
	FormatB8g8r8a8Sscaled = 47,
	FormatB8g8r8a8Uint = 48,
	FormatB8g8r8a8Sint = 49,
	FormatB8g8r8a8Srgb = 50,
	FormatA8b8g8r8UnormPack32 = 51,
	FormatA8b8g8r8SnormPack32 = 52,
	FormatA8b8g8r8UscaledPack32 = 53,
	FormatA8b8g8r8SscaledPack32 = 54,
	FormatA8b8g8r8UintPack32 = 55,
	FormatA8b8g8r8SintPack32 = 56,
	FormatA8b8g8r8SrgbPack32 = 57,
	FormatA2r10g10b10UnormPack32 = 58,
	FormatA2r10g10b10SnormPack32 = 59,
	FormatA2r10g10b10UscaledPack32 = 60,
	FormatA2r10g10b10SscaledPack32 = 61,
	FormatA2r10g10b10UintPack32 = 62,
	FormatA2r10g10b10SintPack32 = 63,
	FormatA2b10g10r10UnormPack32 = 64,
	FormatA2b10g10r10SnormPack32 = 65,
	FormatA2b10g10r10UscaledPack32 = 66,
	FormatA2b10g10r10SscaledPack32 = 67,
	FormatA2b10g10r10UintPack32 = 68,
	FormatA2b10g10r10SintPack32 = 69,
	FormatR16Unorm = 70,
	FormatR16Snorm = 71,
	FormatR16Uscaled = 72,
	FormatR16Sscaled = 73,
	FormatR16Uint = 74,
	FormatR16Sint = 75,
	FormatR16Sfloat = 76,
	FormatR16g16Unorm = 77,
	FormatR16g16Snorm = 78,
	FormatR16g16Uscaled = 79,
	FormatR16g16Sscaled = 80,
	FormatR16g16Uint = 81,
	FormatR16g16Sint = 82,
	FormatR16g16Sfloat = 83,
	FormatR16g16b16Unorm = 84,
	FormatR16g16b16Snorm = 85,
	FormatR16g16b16Uscaled = 86,
	FormatR16g16b16Sscaled = 87,
	FormatR16g16b16Uint = 88,
	FormatR16g16b16Sint = 89,
	FormatR16g16b16Sfloat = 90,
	FormatR16g16b16a16Unorm = 91,
	FormatR16g16b16a16Snorm = 92,
	FormatR16g16b16a16Uscaled = 93,
	FormatR16g16b16a16Sscaled = 94,
	FormatR16g16b16a16Uint = 95,
	FormatR16g16b16a16Sint = 96,
	FormatR16g16b16a16Sfloat = 97,
	FormatR32Uint = 98,
	FormatR32Sint = 99,
	FormatR32Sfloat = 100,
	FormatR32g32Uint = 101,
	FormatR32g32Sint = 102,
	FormatR32g32Sfloat = 103,
	FormatR32g32b32Uint = 104,
	FormatR32g32b32Sint = 105,
	FormatR32g32b32Sfloat = 106,
	FormatR32g32b32a32Uint = 107,
	FormatR32g32b32a32Sint = 108,
	FormatR32g32b32a32Sfloat = 109,
	FormatR64Uint = 110,
	FormatR64Sint = 111,
	FormatR64Sfloat = 112,
	FormatR64g64Uint = 113,
	FormatR64g64Sint = 114,
	FormatR64g64Sfloat = 115,
	FormatR64g64b64Uint = 116,
	FormatR64g64b64Sint = 117,
	FormatR64g64b64Sfloat = 118,
	FormatR64g64b64a64Uint = 119,
	FormatR64g64b64a64Sint = 120,
	FormatR64g64b64a64Sfloat = 121,
	FormatB10g11r11UfloatPack32 = 122,
	FormatE5b9g9r9UfloatPack32 = 123,
	FormatD16Unorm = 124,
	FormatX8D24UnormPack32 = 125,
	FormatD32Sfloat = 126,
	FormatS8Uint = 127,
	FormatD16UnormS8Uint = 128,
	FormatD24UnormS8Uint = 129,
	FormatD32SfloatS8Uint = 130,
	FormatBc1RgbUnormBlock = 131,
	FormatBc1RgbSrgbBlock = 132,
	FormatBc1RgbaUnormBlock = 133,
	FormatBc1RgbaSrgbBlock = 134,
	FormatBc2UnormBlock = 135,
	FormatBc2SrgbBlock = 136,
	FormatBc3UnormBlock = 137,
	FormatBc3SrgbBlock = 138,
	FormatBc4UnormBlock = 139,
	FormatBc4SnormBlock = 140,
	FormatBc5UnormBlock = 141,
	FormatBc5SnormBlock = 142,
	FormatBc6hUfloatBlock = 143,
	FormatBc6hSfloatBlock = 144,
	FormatBc7UnormBlock = 145,
	FormatBc7SrgbBlock = 146,
	FormatEtc2R8g8b8UnormBlock = 147,
	FormatEtc2R8g8b8SrgbBlock = 148,
	FormatEtc2R8g8b8a1UnormBlock = 149,
	FormatEtc2R8g8b8a1SrgbBlock = 150,
	FormatEtc2R8g8b8a8UnormBlock = 151,
	FormatEtc2R8g8b8a8SrgbBlock = 152,
	FormatEacR11UnormBlock = 153,
	FormatEacR11SnormBlock = 154,
	FormatEacR11g11UnormBlock = 155,
	FormatEacR11g11SnormBlock = 156,
	FormatAstc4x4UnormBlock = 157,
	FormatAstc4x4SrgbBlock = 158,
	FormatAstc5x4UnormBlock = 159,
	FormatAstc5x4SrgbBlock = 160,
	FormatAstc5x5UnormBlock = 161,
	FormatAstc5x5SrgbBlock = 162,
	FormatAstc6x5UnormBlock = 163,
	FormatAstc6x5SrgbBlock = 164,
	FormatAstc6x6UnormBlock = 165,
	FormatAstc6x6SrgbBlock = 166,
	FormatAstc8x5UnormBlock = 167,
	FormatAstc8x5SrgbBlock = 168,
	FormatAstc8x6UnormBlock = 169,
	FormatAstc8x6SrgbBlock = 170,
	FormatAstc8x8UnormBlock = 171,
	FormatAstc8x8SrgbBlock = 172,
	FormatAstc10x5UnormBlock = 173,
	FormatAstc10x5SrgbBlock = 174,
	FormatAstc10x6UnormBlock = 175,
	FormatAstc10x6SrgbBlock = 176,
	FormatAstc10x8UnormBlock = 177,
	FormatAstc10x8SrgbBlock = 178,
	FormatAstc10x10UnormBlock = 179,
	FormatAstc10x10SrgbBlock = 180,
	FormatAstc12x10UnormBlock = 181,
	FormatAstc12x10SrgbBlock = 182,
	FormatAstc12x12UnormBlock = 183,
	FormatAstc12x12SrgbBlock = 184,
}

#[repr(C)]
pub enum SemaphoreType {
	SemaphoreTypeBinary = 0,
	SemaphoreTypeTimeline = 1,
}

#[repr(C)]
pub enum VendorId {
	VendorIdViv = 65537,
	VendorIdVsi = 65538,
	VendorIdKazan = 65539,
	VendorIdCodeplay = 65540,
	VendorIdMesa = 65541,
	VendorIdPocl = 65542,
	VendorIdMobileye = 65543,
}

#[repr(C)]
pub enum PerformanceConfigurationTypeINTEL {
	PerformanceConfigurationTypeCommandQueueMetricsDiscoveryActivatedIntel = 0,
}

#[repr(C)]
pub enum BlendOp {
	BlendOpAdd = 0,
	BlendOpSubtract = 1,
	BlendOpReverseSubtract = 2,
	BlendOpMin = 3,
	BlendOpMax = 4,
}

#[repr(C)]
pub enum VertexInputRate {
	VertexInputRateVertex = 0,
	VertexInputRateInstance = 1,
}

#[repr(C)]
pub enum CompareOp {
	CompareOpNever = 0,
	CompareOpLess = 1,
	CompareOpEqual = 2,
	CompareOpLessOrEqual = 3,
	CompareOpGreater = 4,
	CompareOpNotEqual = 5,
	CompareOpGreaterOrEqual = 6,
	CompareOpAlways = 7,
}

#[repr(C)]
pub enum ShaderInfoTypeAMD {
	ShaderInfoTypeStatisticsAmd = 0,
	ShaderInfoTypeBinaryAmd = 1,
	ShaderInfoTypeDisassemblyAmd = 2,
}

#[repr(C)]
pub enum PerformanceCounterScopeKHR {
	PerformanceCounterScopeCommandBufferKhr = 0,
	PerformanceCounterScopeRenderPassKhr = 1,
	PerformanceCounterScopeCommandKhr = 2,
}

#[repr(C)]
pub enum IndexType {
	IndexTypeUint16 = 0,
	IndexTypeUint32 = 1,
}

#[repr(C)]
pub enum FaultLevel {
	FaultLevelUnassigned = 0,
	FaultLevelCritical = 1,
	FaultLevelRecoverable = 2,
	FaultLevelWarning = 3,
}

#[repr(C)]
pub enum FaultType {
	FaultTypeInvalid = 0,
	FaultTypeUnassigned = 1,
	FaultTypeImplementation = 2,
	FaultTypeSystem = 3,
	FaultTypePhysicalDevice = 4,
	FaultTypeCommandBufferFull = 5,
	FaultTypeInvalidApiUsage = 6,
}

#[repr(C)]
pub enum OpacityMicromapSpecialIndexEXT {
	OpacityMicromapSpecialIndexFullyTransparentExt = -1,
	OpacityMicromapSpecialIndexFullyOpaqueExt = -2,
	OpacityMicromapSpecialIndexFullyUnknownTransparentExt = -3,
	OpacityMicromapSpecialIndexFullyUnknownOpaqueExt = -4,
}

#[repr(C)]
pub enum CopyAccelerationStructureModeKHR {
	CopyAccelerationStructureModeCloneKhr = 0,
	CopyAccelerationStructureModeCompactKhr = 1,
	CopyAccelerationStructureModeSerializeKhr = 2,
	CopyAccelerationStructureModeDeserializeKhr = 3,
}

#[repr(C)]
pub enum QueryResultStatusKHR {
	QueryResultStatusErrorKhr = -1,
	QueryResultStatusNotReadyKhr = 0,
	QueryResultStatusCompleteKhr = 1,
}

#[repr(C)]
pub enum DynamicState {
	DynamicStateViewport = 0,
	DynamicStateScissor = 1,
	DynamicStateLineWidth = 2,
	DynamicStateDepthBias = 3,
	DynamicStateBlendConstants = 4,
	DynamicStateDepthBounds = 5,
	DynamicStateStencilCompareMask = 6,
	DynamicStateStencilWriteMask = 7,
	DynamicStateStencilReference = 8,
}

#[repr(C)]
pub enum PipelineRobustnessImageBehaviorEXT {
	PipelineRobustnessImageBehaviorDeviceDefaultExt = 0,
	PipelineRobustnessImageBehaviorDisabledExt = 1,
	PipelineRobustnessImageBehaviorRobustImageAccessExt = 2,
	PipelineRobustnessImageBehaviorRobustImageAccess2Ext = 3,
}

#[repr(C)]
pub enum VideoEncodeTuningModeKHR {
	VideoEncodeTuningModeDefaultKhr = 0,
	VideoEncodeTuningModeHighQualityKhr = 1,
	VideoEncodeTuningModeLowLatencyKhr = 2,
	VideoEncodeTuningModeUltraLowLatencyKhr = 3,
	VideoEncodeTuningModeLosslessKhr = 4,
}

#[repr(C)]
pub enum DriverId {
	DriverIdAmdProprietary = 1,
	DriverIdAmdOpenSource = 2,
	DriverIdMesaRadv = 3,
	DriverIdNvidiaProprietary = 4,
	DriverIdIntelProprietaryWindows = 5,
	DriverIdIntelOpenSourceMesa = 6,
	DriverIdImaginationProprietary = 7,
	DriverIdQualcommProprietary = 8,
	DriverIdArmProprietary = 9,
	DriverIdGoogleSwiftshader = 10,
	DriverIdGgpProprietary = 11,
	DriverIdBroadcomProprietary = 12,
	DriverIdMesaLlvmpipe = 13,
	DriverIdMoltenvk = 14,
	DriverIdCoreaviProprietary = 15,
	DriverIdJuiceProprietary = 16,
	DriverIdVerisiliconProprietary = 17,
	DriverIdMesaTurnip = 18,
	DriverIdMesaV3dv = 19,
	DriverIdMesaPanvk = 20,
	DriverIdSamsungProprietary = 21,
	DriverIdMesaVenus = 22,
	DriverIdMesaDozen = 23,
	DriverIdMesaNvk = 24,
	DriverIdImaginationOpenSourceMesa = 25,
	DriverIdMesaAgxv = 26,
}

#[repr(C)]
pub enum BuildAccelerationStructureModeKHR {
	BuildAccelerationStructureModeBuildKhr = 0,
	BuildAccelerationStructureModeUpdateKhr = 1,
}

#[repr(C)]
pub enum CoarseSampleOrderTypeNV {
	CoarseSampleOrderTypeDefaultNv = 0,
	CoarseSampleOrderTypeCustomNv = 1,
	CoarseSampleOrderTypePixelMajorNv = 2,
	CoarseSampleOrderTypeSampleMajorNv = 3,
}

#[repr(C)]
pub enum MemoryOverallocationBehaviorAMD {
	MemoryOverallocationBehaviorDefaultAmd = 0,
	MemoryOverallocationBehaviorAllowedAmd = 1,
	MemoryOverallocationBehaviorDisallowedAmd = 2,
}

#[repr(C)]
pub enum DisplayEventTypeEXT {
	DisplayEventTypeFirstPixelOutExt = 0,
}

#[repr(C)]
pub enum SamplerYcbcrModelConversion {
	SamplerYcbcrModelConversionRgbIdentity = 0,
	SamplerYcbcrModelConversionYcbcrIdentity = 1,
	SamplerYcbcrModelConversionYcbcr709 = 2,
	SamplerYcbcrModelConversionYcbcr601 = 3,
	SamplerYcbcrModelConversionYcbcr2020 = 4,
}

#[repr(C)]
pub enum SamplerReductionMode {
	SamplerReductionModeWeightedAverage = 0,
	SamplerReductionModeMin = 1,
	SamplerReductionModeMax = 2,
}

#[repr(C)]
pub enum DebugReportObjectTypeEXT {
	DebugReportObjectTypeUnknownExt = 0,
	DebugReportObjectTypeInstanceExt = 1,
	DebugReportObjectTypePhysicalDeviceExt = 2,
	DebugReportObjectTypeDeviceExt = 3,
	DebugReportObjectTypeQueueExt = 4,
	DebugReportObjectTypeSemaphoreExt = 5,
	DebugReportObjectTypeCommandBufferExt = 6,
	DebugReportObjectTypeFenceExt = 7,
	DebugReportObjectTypeDeviceMemoryExt = 8,
	DebugReportObjectTypeBufferExt = 9,
	DebugReportObjectTypeImageExt = 10,
	DebugReportObjectTypeEventExt = 11,
	DebugReportObjectTypeQueryPoolExt = 12,
	DebugReportObjectTypeBufferViewExt = 13,
	DebugReportObjectTypeImageViewExt = 14,
	DebugReportObjectTypeShaderModuleExt = 15,
	DebugReportObjectTypePipelineCacheExt = 16,
	DebugReportObjectTypePipelineLayoutExt = 17,
	DebugReportObjectTypeRenderPassExt = 18,
	DebugReportObjectTypePipelineExt = 19,
	DebugReportObjectTypeDescriptorSetLayoutExt = 20,
	DebugReportObjectTypeSamplerExt = 21,
	DebugReportObjectTypeDescriptorPoolExt = 22,
	DebugReportObjectTypeDescriptorSetExt = 23,
	DebugReportObjectTypeFramebufferExt = 24,
	DebugReportObjectTypeCommandPoolExt = 25,
	DebugReportObjectTypeSurfaceKhrExt = 26,
	DebugReportObjectTypeSwapchainKhrExt = 27,
	DebugReportObjectTypeDebugReportCallbackExtExt = 28,
	DebugReportObjectTypeDisplayKhrExt = 29,
	DebugReportObjectTypeDisplayModeKhrExt = 30,
	DebugReportObjectTypeValidationCacheExtExt = 33,
}

#[repr(C)]
pub enum BuildMicromapModeEXT {
	BuildMicromapModeBuildExt = 0,
}

#[repr(C)]
pub enum TimeDomainEXT {
	TimeDomainDeviceExt = 0,
	TimeDomainClockMonotonicExt = 1,
	TimeDomainClockMonotonicRawExt = 2,
	TimeDomainQueryPerformanceCounterExt = 3,
}

#[repr(C)]
pub enum AttachmentStoreOp {
	AttachmentStoreOpStore = 0,
	AttachmentStoreOpDontCare = 1,
}

#[repr(C)]
pub enum OutOfBandQueueTypeNV {
	OutOfBandQueueTypeRenderNv = 0,
	OutOfBandQueueTypePresentNv = 1,
}

#[repr(C)]
pub enum DiscardRectangleModeEXT {
	DiscardRectangleModeInclusiveExt = 0,
	DiscardRectangleModeExclusiveExt = 1,
}

#[repr(C)]
pub enum PerformanceCounterUnitKHR {
	PerformanceCounterUnitGenericKhr = 0,
	PerformanceCounterUnitPercentageKhr = 1,
	PerformanceCounterUnitNanosecondsKhr = 2,
	PerformanceCounterUnitBytesKhr = 3,
	PerformanceCounterUnitBytesPerSecondKhr = 4,
	PerformanceCounterUnitKelvinKhr = 5,
	PerformanceCounterUnitWattsKhr = 6,
	PerformanceCounterUnitVoltsKhr = 7,
	PerformanceCounterUnitAmpsKhr = 8,
	PerformanceCounterUnitHertzKhr = 9,
	PerformanceCounterUnitCyclesKhr = 10,
}

#[repr(C)]
pub enum BlendOverlapEXT {
	BlendOverlapUncorrelatedExt = 0,
	BlendOverlapDisjointExt = 1,
	BlendOverlapConjointExt = 2,
}

#[repr(C)]
pub enum ImageLayout {
	ImageLayoutUndefined = 0,
	ImageLayoutGeneral = 1,
	ImageLayoutColorAttachmentOptimal = 2,
	ImageLayoutDepthStencilAttachmentOptimal = 3,
	ImageLayoutDepthStencilReadOnlyOptimal = 4,
	ImageLayoutShaderReadOnlyOptimal = 5,
	ImageLayoutTransferSrcOptimal = 6,
	ImageLayoutTransferDstOptimal = 7,
	ImageLayoutPreinitialized = 8,
}

#[repr(C)]
pub enum ImageViewType {
	ImageViewType1d = 0,
	ImageViewType2d = 1,
	ImageViewType3d = 2,
	ImageViewTypeCube = 3,
	ImageViewType1dArray = 4,
	ImageViewType2dArray = 5,
	ImageViewTypeCubeArray = 6,
}

#[repr(C)]
pub enum CoverageModulationModeNV {
	CoverageModulationModeNoneNv = 0,
	CoverageModulationModeRgbNv = 1,
	CoverageModulationModeAlphaNv = 2,
	CoverageModulationModeRgbaNv = 3,
}

#[repr(C)]
pub enum FaultQueryBehavior {
	FaultQueryBehaviorGetAndClearAllFaults = 0,
}

#[repr(C)]
pub enum SharingMode {
	SharingModeExclusive = 0,
	SharingModeConcurrent = 1,
}

#[repr(C)]
pub enum AccelerationStructureCompatibilityKHR {
	AccelerationStructureCompatibilityCompatibleKhr = 0,
	AccelerationStructureCompatibilityIncompatibleKhr = 1,
}

#[repr(C)]
pub enum PresentModeKHR {
	PresentModeImmediateKhr = 0,
	PresentModeMailboxKhr = 1,
	PresentModeFifoKhr = 2,
	PresentModeFifoRelaxedKhr = 3,
}

#[repr(C)]
pub enum SciSyncClientTypeNV {
	SciSyncClientTypeSignalerNv = 0,
	SciSyncClientTypeWaiterNv = 1,
	SciSyncClientTypeSignalerWaiterNv = 2,
}

#[repr(C)]
pub enum FragmentShadingRateNV {
	FragmentShadingRate1InvocationPerPixelNv = 0,
	FragmentShadingRate1InvocationPer1x2PixelsNv = 1,
	FragmentShadingRate1InvocationPer2x1PixelsNv = 4,
	FragmentShadingRate1InvocationPer2x2PixelsNv = 5,
	FragmentShadingRate1InvocationPer2x4PixelsNv = 6,
	FragmentShadingRate1InvocationPer4x2PixelsNv = 9,
	FragmentShadingRate1InvocationPer4x4PixelsNv = 10,
	FragmentShadingRate2InvocationsPerPixelNv = 11,
	FragmentShadingRate4InvocationsPerPixelNv = 12,
	FragmentShadingRate8InvocationsPerPixelNv = 13,
	FragmentShadingRate16InvocationsPerPixelNv = 14,
	FragmentShadingRateNoInvocationsNv = 15,
}

#[repr(C)]
pub enum DescriptorUpdateTemplateType {
	DescriptorUpdateTemplateTypeDescriptorSet = 0,
}

#[repr(C)]
pub enum DisplayPowerStateEXT {
	DisplayPowerStateOffExt = 0,
	DisplayPowerStateSuspendExt = 1,
	DisplayPowerStateOnExt = 2,
}

#[repr(C)]
pub enum MicromapTypeEXT {
	MicromapTypeOpacityMicromapExt = 0,
}

#[repr(C)]
pub enum CubicFilterWeightsQCOM {
	CubicFilterWeightsCatmullRomQcom = 0,
	CubicFilterWeightsZeroTangentCardinalQcom = 1,
	CubicFilterWeightsBSplineQcom = 2,
	CubicFilterWeightsMitchellNetravaliQcom = 3,
}

#[repr(C)]
pub enum AttachmentLoadOp {
	AttachmentLoadOpLoad = 0,
	AttachmentLoadOpClear = 1,
	AttachmentLoadOpDontCare = 2,
}

#[repr(C)]
pub enum ImageTiling {
	ImageTilingOptimal = 0,
	ImageTilingLinear = 1,
}

#[repr(C)]
pub enum ObjectType {
	ObjectTypeUnknown = 0,
	ObjectTypeInstance = 1,
	ObjectTypePhysicalDevice = 2,
	ObjectTypeDevice = 3,
	ObjectTypeQueue = 4,
	ObjectTypeSemaphore = 5,
	ObjectTypeCommandBuffer = 6,
	ObjectTypeFence = 7,
	ObjectTypeDeviceMemory = 8,
	ObjectTypeBuffer = 9,
	ObjectTypeImage = 10,
	ObjectTypeEvent = 11,
	ObjectTypeQueryPool = 12,
	ObjectTypeBufferView = 13,
	ObjectTypeImageView = 14,
	ObjectTypeShaderModule = 15,
	ObjectTypePipelineCache = 16,
	ObjectTypePipelineLayout = 17,
	ObjectTypeRenderPass = 18,
	ObjectTypePipeline = 19,
	ObjectTypeDescriptorSetLayout = 20,
	ObjectTypeSampler = 21,
	ObjectTypeDescriptorPool = 22,
	ObjectTypeDescriptorSet = 23,
	ObjectTypeFramebuffer = 24,
	ObjectTypeCommandPool = 25,
}

#[repr(C)]
pub enum ShaderFloatControlsIndependence {
	ShaderFloatControlsIndependence32BitOnly = 0,
	ShaderFloatControlsIndependenceAll = 1,
	ShaderFloatControlsIndependenceNone = 2,
}

#[repr(C)]
pub enum PointClippingBehavior {
	PointClippingBehaviorAllClipPlanes = 0,
	PointClippingBehaviorUserClipPlanesOnly = 1,
}

#[repr(C)]
pub enum CopyMicromapModeEXT {
	CopyMicromapModeCloneExt = 0,
	CopyMicromapModeSerializeExt = 1,
	CopyMicromapModeDeserializeExt = 2,
	CopyMicromapModeCompactExt = 3,
}

#[repr(C)]
pub enum DescriptorType {
	DescriptorTypeSampler = 0,
	DescriptorTypeCombinedImageSampler = 1,
	DescriptorTypeSampledImage = 2,
	DescriptorTypeStorageImage = 3,
	DescriptorTypeUniformTexelBuffer = 4,
	DescriptorTypeStorageTexelBuffer = 5,
	DescriptorTypeUniformBuffer = 6,
	DescriptorTypeStorageBuffer = 7,
	DescriptorTypeUniformBufferDynamic = 8,
	DescriptorTypeStorageBufferDynamic = 9,
	DescriptorTypeInputAttachment = 10,
}

#[repr(C)]
pub enum PolygonMode {
	PolygonModeFill = 0,
	PolygonModeLine = 1,
	PolygonModePoint = 2,
}

#[repr(C)]
pub enum ComponentSwizzle {
	ComponentSwizzleIdentity = 0,
	ComponentSwizzleZero = 1,
	ComponentSwizzleOne = 2,
	ComponentSwizzleR = 3,
	ComponentSwizzleG = 4,
	ComponentSwizzleB = 5,
	ComponentSwizzleA = 6,
}

#[repr(C)]
pub enum ScopeKHR {
	ScopeDeviceKhr = 1,
	ScopeWorkgroupKhr = 2,
	ScopeSubgroupKhr = 3,
	ScopeQueueFamilyKhr = 5,
}

#[repr(C)]
pub enum PerformanceOverrideTypeINTEL {
	PerformanceOverrideTypeNullHardwareIntel = 0,
	PerformanceOverrideTypeFlushGpuCachesIntel = 1,
}

#[repr(C)]
pub enum OpticalFlowPerformanceLevelNV {
	OpticalFlowPerformanceLevelUnknownNv = 0,
	OpticalFlowPerformanceLevelSlowNv = 1,
	OpticalFlowPerformanceLevelMediumNv = 2,
	OpticalFlowPerformanceLevelFastNv = 3,
}

#[repr(C)]
pub enum LayeredDriverUnderlyingApiMSFT {
	LayeredDriverUnderlyingApiNoneMsft = 0,
	LayeredDriverUnderlyingApiD3d12Msft = 1,
}

#[repr(C)]
pub enum AccelerationStructureMemoryRequirementsTypeNV {
	AccelerationStructureMemoryRequirementsTypeObjectNv = 0,
	AccelerationStructureMemoryRequirementsTypeBuildScratchNv = 1,
	AccelerationStructureMemoryRequirementsTypeUpdateScratchNv = 2,
}

#[repr(C)]
pub enum DeviceMemoryReportEventTypeEXT {
	DeviceMemoryReportEventTypeAllocateExt = 0,
	DeviceMemoryReportEventTypeFreeExt = 1,
	DeviceMemoryReportEventTypeImportExt = 2,
	DeviceMemoryReportEventTypeUnimportExt = 3,
	DeviceMemoryReportEventTypeAllocationFailedExt = 4,
}

#[repr(C)]
pub enum BorderColor {
	BorderColorFloatTransparentBlack = 0,
	BorderColorIntTransparentBlack = 1,
	BorderColorFloatOpaqueBlack = 2,
	BorderColorIntOpaqueBlack = 3,
	BorderColorFloatOpaqueWhite = 4,
	BorderColorIntOpaqueWhite = 5,
}

#[repr(C)]
pub enum PipelineMatchControl {
	PipelineMatchControlApplicationUuidExactMatch = 0,
}

#[repr(C)]
pub enum FragmentShadingRateTypeNV {
	FragmentShadingRateTypeFragmentSizeNv = 0,
	FragmentShadingRateTypeEnumsNv = 1,
}

#[repr(C)]
pub enum CommandBufferLevel {
	CommandBufferLevelPrimary = 0,
	CommandBufferLevelSecondary = 1,
}

#[repr(C)]
pub enum RayTracingShaderGroupTypeKHR {
	RayTracingShaderGroupTypeGeneralKhr = 0,
	RayTracingShaderGroupTypeTrianglesHitGroupKhr = 1,
	RayTracingShaderGroupTypeProceduralHitGroupKhr = 2,
}

#[repr(C)]
pub enum InternalAllocationType {
	InternalAllocationTypeExecutable = 0,
}

#[repr(C)]
pub enum DepthBiasRepresentationEXT {
	DepthBiasRepresentationLeastRepresentableValueFormatExt = 0,
	DepthBiasRepresentationLeastRepresentableValueForceUnormExt = 1,
	DepthBiasRepresentationFloatExt = 2,
}

#[repr(C)]
pub enum DeviceAddressBindingTypeEXT {
	DeviceAddressBindingTypeBindExt = 0,
	DeviceAddressBindingTypeUnbindExt = 1,
}

#[repr(C)]
pub enum ValidationFeatureDisableEXT {
	ValidationFeatureDisableAllExt = 0,
	ValidationFeatureDisableShadersExt = 1,
	ValidationFeatureDisableThreadSafetyExt = 2,
	ValidationFeatureDisableApiParametersExt = 3,
	ValidationFeatureDisableObjectLifetimesExt = 4,
	ValidationFeatureDisableCoreChecksExt = 5,
	ValidationFeatureDisableUniqueHandlesExt = 6,
	ValidationFeatureDisableShaderValidationCacheExt = 7,
}

#[repr(C)]
pub enum DeviceFaultAddressTypeEXT {
	DeviceFaultAddressTypeNoneExt = 0,
	DeviceFaultAddressTypeReadInvalidExt = 1,
	DeviceFaultAddressTypeWriteInvalidExt = 2,
	DeviceFaultAddressTypeExecuteInvalidExt = 3,
	DeviceFaultAddressTypeInstructionPointerUnknownExt = 4,
	DeviceFaultAddressTypeInstructionPointerInvalidExt = 5,
	DeviceFaultAddressTypeInstructionPointerFaultExt = 6,
}

#[repr(C)]
pub enum Result {
	Success = 0,
	NotReady = 1,
	Timeout = 2,
	EventSet = 3,
	EventReset = 4,
	Incomplete = 5,
	ErrorOutOfHostMemory = -1,
	ErrorOutOfDeviceMemory = -2,
	ErrorInitializationFailed = -3,
	ErrorDeviceLost = -4,
	ErrorMemoryMapFailed = -5,
	ErrorLayerNotPresent = -6,
	ErrorExtensionNotPresent = -7,
	ErrorFeatureNotPresent = -8,
	ErrorIncompatibleDriver = -9,
	ErrorTooManyObjects = -10,
	ErrorFormatNotSupported = -11,
	ErrorFragmentedPool = -12,
	ErrorUnknown = -13,
}

#[repr(C)]
pub enum ValidationCheckEXT {
	ValidationCheckAllExt = 0,
	ValidationCheckShadersExt = 1,
}

#[repr(C)]
pub enum PipelineBindPoint {
	PipelineBindPointGraphics = 0,
	PipelineBindPointCompute = 1,
}

#[repr(C)]
pub enum LayerSettingTypeEXT {
	LayerSettingTypeBool32Ext = 0,
	LayerSettingTypeInt32Ext = 1,
	LayerSettingTypeInt64Ext = 2,
	LayerSettingTypeUint32Ext = 3,
	LayerSettingTypeUint64Ext = 4,
	LayerSettingTypeFloat32Ext = 5,
	LayerSettingTypeFloat64Ext = 6,
	LayerSettingTypeStringExt = 7,
}

#[repr(C)]
pub enum ShadingRatePaletteEntryNV {
	ShadingRatePaletteEntryNoInvocationsNv = 0,
	ShadingRatePaletteEntry16InvocationsPerPixelNv = 1,
	ShadingRatePaletteEntry8InvocationsPerPixelNv = 2,
	ShadingRatePaletteEntry4InvocationsPerPixelNv = 3,
	ShadingRatePaletteEntry2InvocationsPerPixelNv = 4,
	ShadingRatePaletteEntry1InvocationPerPixelNv = 5,
	ShadingRatePaletteEntry1InvocationPer2x1PixelsNv = 6,
	ShadingRatePaletteEntry1InvocationPer1x2PixelsNv = 7,
	ShadingRatePaletteEntry1InvocationPer2x2PixelsNv = 8,
	ShadingRatePaletteEntry1InvocationPer4x2PixelsNv = 9,
	ShadingRatePaletteEntry1InvocationPer2x4PixelsNv = 10,
	ShadingRatePaletteEntry1InvocationPer4x4PixelsNv = 11,
}

#[repr(C)]
pub enum GeometryTypeKHR {
	GeometryTypeTrianglesKhr = 0,
	GeometryTypeAabbsKhr = 1,
	GeometryTypeInstancesKhr = 2,
}

#[repr(C)]
pub enum PerformanceParameterTypeINTEL {
	PerformanceParameterTypeHwCountersSupportedIntel = 0,
	PerformanceParameterTypeStreamMarkerValidBitsIntel = 1,
}

#[repr(C)]
pub enum LogicOp {
	LogicOpClear = 0,
	LogicOpAnd = 1,
	LogicOpAndReverse = 2,
	LogicOpCopy = 3,
	LogicOpAndInverted = 4,
	LogicOpNoOp = 5,
	LogicOpXor = 6,
	LogicOpOr = 7,
	LogicOpNor = 8,
	LogicOpEquivalent = 9,
	LogicOpInvert = 10,
	LogicOpOrReverse = 11,
	LogicOpCopyInverted = 12,
	LogicOpOrInverted = 13,
	LogicOpNand = 14,
	LogicOpSet = 15,
}

#[repr(C)]
pub enum ConservativeRasterizationModeEXT {
	ConservativeRasterizationModeDisabledExt = 0,
	ConservativeRasterizationModeOverestimateExt = 1,
	ConservativeRasterizationModeUnderestimateExt = 2,
}

#[repr(C)]
pub enum AccelerationStructureBuildTypeKHR {
	AccelerationStructureBuildTypeHostKhr = 0,
	AccelerationStructureBuildTypeDeviceKhr = 1,
	AccelerationStructureBuildTypeHostOrDeviceKhr = 2,
}

#[repr(C)]
pub enum ShaderCodeTypeEXT {
	ShaderCodeTypeBinaryExt = 0,
	ShaderCodeTypeSpirvExt = 1,
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineTessellationStateCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum MicromapCreateFlagBitsEXT {
	MicromapCreateDeviceAddressCaptureReplayBitExt = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct MicromapCreateFlagsEXT: Flags {
		const MICROMAP_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT = MicromapCreateFlagBitsEXT::MicromapCreateDeviceAddressCaptureReplayBitExt as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineViewportSwizzleStateCreateFlagsNV: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct DescriptorSetLayoutCreateFlags: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct ImagePipeSurfaceCreateFlagsFUCHSIA: Flags {
	}
}

#[repr(u32)]
pub enum GraphicsPipelineLibraryFlagBitsEXT {
	GraphicsPipelineLibraryVertexInputInterfaceBitExt = 1 << 0,
	GraphicsPipelineLibraryPreRasterizationShadersBitExt = 1 << 1,
	GraphicsPipelineLibraryFragmentShaderBitExt = 1 << 2,
	GraphicsPipelineLibraryFragmentOutputInterfaceBitExt = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct GraphicsPipelineLibraryFlagsEXT: Flags {
		const GRAPHICS_PIPELINE_LIBRARY_VERTEX_INPUT_INTERFACE_BIT_EXT = GraphicsPipelineLibraryFlagBitsEXT::GraphicsPipelineLibraryVertexInputInterfaceBitExt as Flags;
		const GRAPHICS_PIPELINE_LIBRARY_PRE_RASTERIZATION_SHADERS_BIT_EXT = GraphicsPipelineLibraryFlagBitsEXT::GraphicsPipelineLibraryPreRasterizationShadersBitExt as Flags;
		const GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_SHADER_BIT_EXT = GraphicsPipelineLibraryFlagBitsEXT::GraphicsPipelineLibraryFragmentShaderBitExt as Flags;
		const GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_OUTPUT_INTERFACE_BIT_EXT = GraphicsPipelineLibraryFlagBitsEXT::GraphicsPipelineLibraryFragmentOutputInterfaceBitExt as Flags;
	}
}

#[repr(u32)]
pub enum ImageCompressionFlagBitsEXT {
	ImageCompressionFixedRateDefaultExt = 1 << 0,
	ImageCompressionFixedRateExplicitExt = 1 << 1,
	ImageCompressionDisabledExt = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct ImageCompressionFlagsEXT: Flags {
		const IMAGE_COMPRESSION_FIXED_RATE_DEFAULT_EXT = ImageCompressionFlagBitsEXT::ImageCompressionFixedRateDefaultExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_EXPLICIT_EXT = ImageCompressionFlagBitsEXT::ImageCompressionFixedRateExplicitExt as Flags;
		const IMAGE_COMPRESSION_DISABLED_EXT = ImageCompressionFlagBitsEXT::ImageCompressionDisabledExt as Flags;
	}
}

#[repr(u32)]
pub enum ImageConstraintsInfoFlagBitsFUCHSIA {
	ImageConstraintsInfoCpuReadRarelyFuchsia = 1 << 0,
	ImageConstraintsInfoCpuReadOftenFuchsia = 1 << 1,
	ImageConstraintsInfoCpuWriteRarelyFuchsia = 1 << 2,
	ImageConstraintsInfoCpuWriteOftenFuchsia = 1 << 3,
	ImageConstraintsInfoProtectedOptionalFuchsia = 1 << 4,
}

bitflags! {
	#[repr(transparent)]
	pub struct ImageConstraintsInfoFlagsFUCHSIA: Flags {
		const IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA::ImageConstraintsInfoCpuReadRarelyFuchsia as Flags;
		const IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA::ImageConstraintsInfoCpuReadOftenFuchsia as Flags;
		const IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA::ImageConstraintsInfoCpuWriteRarelyFuchsia as Flags;
		const IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA::ImageConstraintsInfoCpuWriteOftenFuchsia as Flags;
		const IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA::ImageConstraintsInfoProtectedOptionalFuchsia as Flags;
	}
}

#[repr(u32)]
pub enum FrameBoundaryFlagBitsEXT {
	FrameBoundaryFrameEndBitExt = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct FrameBoundaryFlagsEXT: Flags {
		const FRAME_BOUNDARY_FRAME_END_BIT_EXT = FrameBoundaryFlagBitsEXT::FrameBoundaryFrameEndBitExt as Flags;
	}
}

#[repr(u32)]
pub enum FenceCreateFlagBits {
	FenceCreateSignaledBit = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct FenceCreateFlags: Flags {
		const FENCE_CREATE_SIGNALED_BIT = FenceCreateFlagBits::FenceCreateSignaledBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineMultisampleStateCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum QueryResultFlagBits {
	QueryResult64Bit = 1 << 0,
	QueryResultWaitBit = 1 << 1,
	QueryResultWithAvailabilityBit = 1 << 2,
	QueryResultPartialBit = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct QueryResultFlags: Flags {
		const QUERY_RESULT_64_BIT = QueryResultFlagBits::QueryResult64Bit as Flags;
		const QUERY_RESULT_WAIT_BIT = QueryResultFlagBits::QueryResultWaitBit as Flags;
		const QUERY_RESULT_WITH_AVAILABILITY_BIT = QueryResultFlagBits::QueryResultWithAvailabilityBit as Flags;
		const QUERY_RESULT_PARTIAL_BIT = QueryResultFlagBits::QueryResultPartialBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct DescriptorPoolResetFlags: Flags {
	}
}

#[repr(u32)]
pub enum RenderingFlagBits {
	RenderingContentsSecondaryCommandBuffersBit = 1 << 0,
	RenderingSuspendingBit = 1 << 1,
	RenderingResumingBit = 1 << 2,
}

impl RenderingFlagBits {
	const RenderingContentsSecondaryCommandBuffersBitKhr: u32 = Self::RenderingContentsSecondaryCommandBuffersBitKhr;
	const RenderingSuspendingBitKhr: u32 = Self::RenderingSuspendingBitKhr;
	const RenderingResumingBitKhr: u32 = Self::RenderingResumingBitKhr;
}

bitflags! {
	#[repr(transparent)]
	pub struct RenderingFlags: Flags {
		const RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT = RenderingFlagBits::RenderingContentsSecondaryCommandBuffersBit as Flags;
		const RENDERING_SUSPENDING_BIT = RenderingFlagBits::RenderingSuspendingBit as Flags;
		const RENDERING_RESUMING_BIT = RenderingFlagBits::RenderingResumingBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineRasterizationStateCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum IndirectStateFlagBitsNV {
	IndirectStateFlagFrontfaceBitNv = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct IndirectStateFlagsNV: Flags {
		const INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV = IndirectStateFlagBitsNV::IndirectStateFlagFrontfaceBitNv as Flags;
	}
}

#[repr(u32)]
pub enum PipelineCreationFeedbackFlagBits {
	PipelineCreationFeedbackValidBit = 1 << 0,
	PipelineCreationFeedbackApplicationPipelineCacheHitBit = 1 << 1,
	PipelineCreationFeedbackBasePipelineAccelerationBit = 1 << 2,
}

impl PipelineCreationFeedbackFlagBits {
	const PipelineCreationFeedbackValidBitExt: u32 = Self::PipelineCreationFeedbackValidBitExt;
	const PipelineCreationFeedbackApplicationPipelineCacheHitBitExt: u32 = Self::PipelineCreationFeedbackApplicationPipelineCacheHitBitExt;
	const PipelineCreationFeedbackBasePipelineAccelerationBitExt: u32 = Self::PipelineCreationFeedbackBasePipelineAccelerationBitExt;
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineCreationFeedbackFlags: Flags {
		const PIPELINE_CREATION_FEEDBACK_VALID_BIT = PipelineCreationFeedbackFlagBits::PipelineCreationFeedbackValidBit as Flags;
		const PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT = PipelineCreationFeedbackFlagBits::PipelineCreationFeedbackApplicationPipelineCacheHitBit as Flags;
		const PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT = PipelineCreationFeedbackFlagBits::PipelineCreationFeedbackBasePipelineAccelerationBit as Flags;
	}
}

#[repr(u64)]
pub enum BufferUsageFlagBits2KHR {
	BufferUsage2TransferSrcBitKhr = 1 << 0,
	BufferUsage2TransferDstBitKhr = 1 << 1,
	BufferUsage2UniformTexelBufferBitKhr = 1 << 2,
	BufferUsage2StorageTexelBufferBitKhr = 1 << 3,
	BufferUsage2UniformBufferBitKhr = 1 << 4,
	BufferUsage2StorageBufferBitKhr = 1 << 5,
	BufferUsage2IndexBufferBitKhr = 1 << 6,
	BufferUsage2VertexBufferBitKhr = 1 << 7,
	BufferUsage2IndirectBufferBitKhr = 1 << 8,
}

bitflags! {
	#[repr(transparent)]
	pub struct BufferUsageFlags2KHR: Flags64 {
		const BUFFER_USAGE_2_TRANSFER_SRC_BIT_KHR = BufferUsageFlagBits2KHR::BufferUsage2TransferSrcBitKhr as Flags64;
		const BUFFER_USAGE_2_TRANSFER_DST_BIT_KHR = BufferUsageFlagBits2KHR::BufferUsage2TransferDstBitKhr as Flags64;
		const BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR = BufferUsageFlagBits2KHR::BufferUsage2UniformTexelBufferBitKhr as Flags64;
		const BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT_KHR = BufferUsageFlagBits2KHR::BufferUsage2StorageTexelBufferBitKhr as Flags64;
		const BUFFER_USAGE_2_UNIFORM_BUFFER_BIT_KHR = BufferUsageFlagBits2KHR::BufferUsage2UniformBufferBitKhr as Flags64;
		const BUFFER_USAGE_2_STORAGE_BUFFER_BIT_KHR = BufferUsageFlagBits2KHR::BufferUsage2StorageBufferBitKhr as Flags64;
		const BUFFER_USAGE_2_INDEX_BUFFER_BIT_KHR = BufferUsageFlagBits2KHR::BufferUsage2IndexBufferBitKhr as Flags64;
		const BUFFER_USAGE_2_VERTEX_BUFFER_BIT_KHR = BufferUsageFlagBits2KHR::BufferUsage2VertexBufferBitKhr as Flags64;
		const BUFFER_USAGE_2_INDIRECT_BUFFER_BIT_KHR = BufferUsageFlagBits2KHR::BufferUsage2IndirectBufferBitKhr as Flags64;
	}
}

#[repr(u32)]
pub enum DeviceGroupPresentModeFlagBitsKHR {
	DeviceGroupPresentModeLocalBitKhr = 1 << 0,
	DeviceGroupPresentModeRemoteBitKhr = 1 << 1,
	DeviceGroupPresentModeSumBitKhr = 1 << 2,
	DeviceGroupPresentModeLocalMultiDeviceBitKhr = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct DeviceGroupPresentModeFlagsKHR: Flags {
		const DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR = DeviceGroupPresentModeFlagBitsKHR::DeviceGroupPresentModeLocalBitKhr as Flags;
		const DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR = DeviceGroupPresentModeFlagBitsKHR::DeviceGroupPresentModeRemoteBitKhr as Flags;
		const DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR = DeviceGroupPresentModeFlagBitsKHR::DeviceGroupPresentModeSumBitKhr as Flags;
		const DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR = DeviceGroupPresentModeFlagBitsKHR::DeviceGroupPresentModeLocalMultiDeviceBitKhr as Flags;
	}
}

#[repr(u32)]
pub enum GeometryInstanceFlagBitsKHR {
	GeometryInstanceTriangleFacingCullDisableBitKhr = 1 << 0,
	GeometryInstanceTriangleFlipFacingBitKhr = 1 << 1,
	GeometryInstanceForceOpaqueBitKhr = 1 << 2,
	GeometryInstanceForceNoOpaqueBitKhr = 1 << 3,
}

impl GeometryInstanceFlagBitsKHR {
	const GeometryInstanceTriangleFrontCounterclockwiseBitKhr: u32 = Self::GeometryInstanceTriangleFrontCounterclockwiseBitKhr;
}

bitflags! {
	#[repr(transparent)]
	pub struct GeometryInstanceFlagsKHR: Flags {
		const GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR = GeometryInstanceFlagBitsKHR::GeometryInstanceTriangleFacingCullDisableBitKhr as Flags;
		const GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR = GeometryInstanceFlagBitsKHR::GeometryInstanceTriangleFlipFacingBitKhr as Flags;
		const GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR = GeometryInstanceFlagBitsKHR::GeometryInstanceForceOpaqueBitKhr as Flags;
		const GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR = GeometryInstanceFlagBitsKHR::GeometryInstanceForceNoOpaqueBitKhr as Flags;
	}
}

#[repr(u32)]
pub enum DeviceAddressBindingFlagBitsEXT {
	DeviceAddressBindingInternalObjectBitExt = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct DeviceAddressBindingFlagsEXT: Flags {
		const DEVICE_ADDRESS_BINDING_INTERNAL_OBJECT_BIT_EXT = DeviceAddressBindingFlagBitsEXT::DeviceAddressBindingInternalObjectBitExt as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct DebugUtilsMessengerCallbackDataFlagsEXT: Flags {
	}
}

#[repr(u32)]
pub enum CommandPoolResetFlagBits {
	CommandPoolResetReleaseResourcesBit = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct CommandPoolResetFlags: Flags {
		const COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = CommandPoolResetFlagBits::CommandPoolResetReleaseResourcesBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct SubpassDescriptionFlags: Flags {
	}
}

#[repr(u32)]
pub enum VideoEncodeFeedbackFlagBitsKHR {
	VideoEncodeFeedbackBitstreamBufferOffsetBitKhr = 1 << 0,
	VideoEncodeFeedbackBitstreamBytesWrittenBitKhr = 1 << 1,
	VideoEncodeFeedbackBitstreamHasOverridesBitKhr = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeFeedbackFlagsKHR: Flags {
		const VIDEO_ENCODE_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR = VideoEncodeFeedbackFlagBitsKHR::VideoEncodeFeedbackBitstreamBufferOffsetBitKhr as Flags;
		const VIDEO_ENCODE_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR = VideoEncodeFeedbackFlagBitsKHR::VideoEncodeFeedbackBitstreamBytesWrittenBitKhr as Flags;
		const VIDEO_ENCODE_FEEDBACK_BITSTREAM_HAS_OVERRIDES_BIT_KHR = VideoEncodeFeedbackFlagBitsKHR::VideoEncodeFeedbackBitstreamHasOverridesBitKhr as Flags;
	}
}

#[repr(u32)]
pub enum DeviceDiagnosticsConfigFlagBitsNV {
	DeviceDiagnosticsConfigEnableShaderDebugInfoBitNv = 1 << 0,
	DeviceDiagnosticsConfigEnableResourceTrackingBitNv = 1 << 1,
	DeviceDiagnosticsConfigEnableAutomaticCheckpointsBitNv = 1 << 2,
	DeviceDiagnosticsConfigEnableShaderErrorReportingBitNv = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct DeviceDiagnosticsConfigFlagsNV: Flags {
		const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV = DeviceDiagnosticsConfigFlagBitsNV::DeviceDiagnosticsConfigEnableShaderDebugInfoBitNv as Flags;
		const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV = DeviceDiagnosticsConfigFlagBitsNV::DeviceDiagnosticsConfigEnableResourceTrackingBitNv as Flags;
		const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV = DeviceDiagnosticsConfigFlagBitsNV::DeviceDiagnosticsConfigEnableAutomaticCheckpointsBitNv as Flags;
		const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_ERROR_REPORTING_BIT_NV = DeviceDiagnosticsConfigFlagBitsNV::DeviceDiagnosticsConfigEnableShaderErrorReportingBitNv as Flags;
	}
}

#[repr(u32)]
pub enum ExternalFenceFeatureFlagBits {
	ExternalFenceFeatureExportableBit = 1 << 0,
	ExternalFenceFeatureImportableBit = 1 << 1,
}

bitflags! {
	#[repr(transparent)]
	pub struct ExternalFenceFeatureFlags: Flags {
		const EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT = ExternalFenceFeatureFlagBits::ExternalFenceFeatureExportableBit as Flags;
		const EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT = ExternalFenceFeatureFlagBits::ExternalFenceFeatureImportableBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineCoverageReductionStateCreateFlagsNV: Flags {
	}
}

#[repr(u32)]
pub enum QueryPipelineStatisticFlagBits {
	QueryPipelineStatisticInputAssemblyVerticesBit = 1 << 0,
	QueryPipelineStatisticInputAssemblyPrimitivesBit = 1 << 1,
	QueryPipelineStatisticVertexShaderInvocationsBit = 1 << 2,
	QueryPipelineStatisticGeometryShaderInvocationsBit = 1 << 3,
	QueryPipelineStatisticGeometryShaderPrimitivesBit = 1 << 4,
	QueryPipelineStatisticClippingInvocationsBit = 1 << 5,
	QueryPipelineStatisticClippingPrimitivesBit = 1 << 6,
	QueryPipelineStatisticFragmentShaderInvocationsBit = 1 << 7,
	QueryPipelineStatisticTessellationControlShaderPatchesBit = 1 << 8,
	QueryPipelineStatisticTessellationEvaluationShaderInvocationsBit = 1 << 9,
	QueryPipelineStatisticComputeShaderInvocationsBit = 1 << 10,
}

bitflags! {
	#[repr(transparent)]
	pub struct QueryPipelineStatisticFlags: Flags {
		const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT = QueryPipelineStatisticFlagBits::QueryPipelineStatisticInputAssemblyVerticesBit as Flags;
		const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT = QueryPipelineStatisticFlagBits::QueryPipelineStatisticInputAssemblyPrimitivesBit as Flags;
		const QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT = QueryPipelineStatisticFlagBits::QueryPipelineStatisticVertexShaderInvocationsBit as Flags;
		const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT = QueryPipelineStatisticFlagBits::QueryPipelineStatisticGeometryShaderInvocationsBit as Flags;
		const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT = QueryPipelineStatisticFlagBits::QueryPipelineStatisticGeometryShaderPrimitivesBit as Flags;
		const QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT = QueryPipelineStatisticFlagBits::QueryPipelineStatisticClippingInvocationsBit as Flags;
		const QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT = QueryPipelineStatisticFlagBits::QueryPipelineStatisticClippingPrimitivesBit as Flags;
		const QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT = QueryPipelineStatisticFlagBits::QueryPipelineStatisticFragmentShaderInvocationsBit as Flags;
		const QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT = QueryPipelineStatisticFlagBits::QueryPipelineStatisticTessellationControlShaderPatchesBit as Flags;
		const QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = QueryPipelineStatisticFlagBits::QueryPipelineStatisticTessellationEvaluationShaderInvocationsBit as Flags;
		const QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT = QueryPipelineStatisticFlagBits::QueryPipelineStatisticComputeShaderInvocationsBit as Flags;
	}
}

#[repr(u32)]
pub enum OpticalFlowGridSizeFlagBitsNV {
	OpticalFlowGridSize1x1BitNv = 1 << 0,
	OpticalFlowGridSize2x2BitNv = 1 << 1,
	OpticalFlowGridSize4x4BitNv = 1 << 2,
	OpticalFlowGridSize8x8BitNv = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct OpticalFlowGridSizeFlagsNV: Flags {
		const OPTICAL_FLOW_GRID_SIZE_1X1_BIT_NV = OpticalFlowGridSizeFlagBitsNV::OpticalFlowGridSize1x1BitNv as Flags;
		const OPTICAL_FLOW_GRID_SIZE_2X2_BIT_NV = OpticalFlowGridSizeFlagBitsNV::OpticalFlowGridSize2x2BitNv as Flags;
		const OPTICAL_FLOW_GRID_SIZE_4X4_BIT_NV = OpticalFlowGridSizeFlagBitsNV::OpticalFlowGridSize4x4BitNv as Flags;
		const OPTICAL_FLOW_GRID_SIZE_8X8_BIT_NV = OpticalFlowGridSizeFlagBitsNV::OpticalFlowGridSize8x8BitNv as Flags;
	}
}

#[repr(u32)]
pub enum VideoEncodeRateControlModeFlagBitsKHR {
	VideoEncodeRateControlModeDisabledBitKhr = 1 << 0,
	VideoEncodeRateControlModeCbrBitKhr = 1 << 1,
	VideoEncodeRateControlModeVbrBitKhr = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeRateControlModeFlagsKHR: Flags {
		const VIDEO_ENCODE_RATE_CONTROL_MODE_DISABLED_BIT_KHR = VideoEncodeRateControlModeFlagBitsKHR::VideoEncodeRateControlModeDisabledBitKhr as Flags;
		const VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR = VideoEncodeRateControlModeFlagBitsKHR::VideoEncodeRateControlModeCbrBitKhr as Flags;
		const VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR = VideoEncodeRateControlModeFlagBitsKHR::VideoEncodeRateControlModeVbrBitKhr as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct CommandPoolTrimFlags: Flags {
	}
}

#[repr(u32)]
pub enum VideoDecodeCapabilityFlagBitsKHR {
	VideoDecodeCapabilityDpbAndOutputCoincideBitKhr = 1 << 0,
	VideoDecodeCapabilityDpbAndOutputDistinctBitKhr = 1 << 1,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoDecodeCapabilityFlagsKHR: Flags {
		const VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR = VideoDecodeCapabilityFlagBitsKHR::VideoDecodeCapabilityDpbAndOutputCoincideBitKhr as Flags;
		const VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR = VideoDecodeCapabilityFlagBitsKHR::VideoDecodeCapabilityDpbAndOutputDistinctBitKhr as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineRasterizationDepthClipStateCreateFlagsEXT: Flags {
	}
}

#[repr(u32)]
pub enum BufferCreateFlagBits {
	BufferCreateSparseBindingBit = 1 << 0,
	BufferCreateSparseResidencyBit = 1 << 1,
	BufferCreateSparseAliasedBit = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct BufferCreateFlags: Flags {
		const BUFFER_CREATE_SPARSE_BINDING_BIT = BufferCreateFlagBits::BufferCreateSparseBindingBit as Flags;
		const BUFFER_CREATE_SPARSE_RESIDENCY_BIT = BufferCreateFlagBits::BufferCreateSparseResidencyBit as Flags;
		const BUFFER_CREATE_SPARSE_ALIASED_BIT = BufferCreateFlagBits::BufferCreateSparseAliasedBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct MemoryUnmapFlagsKHR: Flags {
	}
}

#[repr(u32)]
pub enum SurfaceTransformFlagBitsKHR {
	SurfaceTransformIdentityBitKhr = 1 << 0,
	SurfaceTransformRotate90BitKhr = 1 << 1,
	SurfaceTransformRotate180BitKhr = 1 << 2,
	SurfaceTransformRotate270BitKhr = 1 << 3,
	SurfaceTransformHorizontalMirrorBitKhr = 1 << 4,
	SurfaceTransformHorizontalMirrorRotate90BitKhr = 1 << 5,
	SurfaceTransformHorizontalMirrorRotate180BitKhr = 1 << 6,
	SurfaceTransformHorizontalMirrorRotate270BitKhr = 1 << 7,
	SurfaceTransformInheritBitKhr = 1 << 8,
}

bitflags! {
	#[repr(transparent)]
	pub struct SurfaceTransformFlagsKHR: Flags {
		const SURFACE_TRANSFORM_IDENTITY_BIT_KHR = SurfaceTransformFlagBitsKHR::SurfaceTransformIdentityBitKhr as Flags;
		const SURFACE_TRANSFORM_ROTATE_90_BIT_KHR = SurfaceTransformFlagBitsKHR::SurfaceTransformRotate90BitKhr as Flags;
		const SURFACE_TRANSFORM_ROTATE_180_BIT_KHR = SurfaceTransformFlagBitsKHR::SurfaceTransformRotate180BitKhr as Flags;
		const SURFACE_TRANSFORM_ROTATE_270_BIT_KHR = SurfaceTransformFlagBitsKHR::SurfaceTransformRotate270BitKhr as Flags;
		const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR = SurfaceTransformFlagBitsKHR::SurfaceTransformHorizontalMirrorBitKhr as Flags;
		const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = SurfaceTransformFlagBitsKHR::SurfaceTransformHorizontalMirrorRotate90BitKhr as Flags;
		const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = SurfaceTransformFlagBitsKHR::SurfaceTransformHorizontalMirrorRotate180BitKhr as Flags;
		const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = SurfaceTransformFlagBitsKHR::SurfaceTransformHorizontalMirrorRotate270BitKhr as Flags;
		const SURFACE_TRANSFORM_INHERIT_BIT_KHR = SurfaceTransformFlagBitsKHR::SurfaceTransformInheritBitKhr as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineShaderStageCreateFlags: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct DisplaySurfaceCreateFlagsKHR: Flags {
	}
}

#[repr(u32)]
pub enum CommandBufferResetFlagBits {
	CommandBufferResetReleaseResourcesBit = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct CommandBufferResetFlags: Flags {
		const COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = CommandBufferResetFlagBits::CommandBufferResetReleaseResourcesBit as Flags;
	}
}

#[repr(u32)]
pub enum ExternalFenceHandleTypeFlagBits {
	ExternalFenceHandleTypeOpaqueFdBit = 1 << 0,
	ExternalFenceHandleTypeOpaqueWin32Bit = 1 << 1,
	ExternalFenceHandleTypeOpaqueWin32KmtBit = 1 << 2,
	ExternalFenceHandleTypeSyncFdBit = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct ExternalFenceHandleTypeFlags: Flags {
		const EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT = ExternalFenceHandleTypeFlagBits::ExternalFenceHandleTypeOpaqueFdBit as Flags;
		const EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT = ExternalFenceHandleTypeFlagBits::ExternalFenceHandleTypeOpaqueWin32Bit as Flags;
		const EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = ExternalFenceHandleTypeFlagBits::ExternalFenceHandleTypeOpaqueWin32KmtBit as Flags;
		const EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT = ExternalFenceHandleTypeFlagBits::ExternalFenceHandleTypeSyncFdBit as Flags;
	}
}

#[repr(u32)]
pub enum ExternalMemoryFeatureFlagBits {
	ExternalMemoryFeatureDedicatedOnlyBit = 1 << 0,
	ExternalMemoryFeatureExportableBit = 1 << 1,
	ExternalMemoryFeatureImportableBit = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct ExternalMemoryFeatureFlags: Flags {
		const EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT = ExternalMemoryFeatureFlagBits::ExternalMemoryFeatureDedicatedOnlyBit as Flags;
		const EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT = ExternalMemoryFeatureFlagBits::ExternalMemoryFeatureExportableBit as Flags;
		const EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT = ExternalMemoryFeatureFlagBits::ExternalMemoryFeatureImportableBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct DisplayModeCreateFlagsKHR: Flags {
	}
}

#[repr(u32)]
pub enum CommandBufferUsageFlagBits {
	CommandBufferUsageOneTimeSubmitBit = 1 << 0,
	CommandBufferUsageRenderPassContinueBit = 1 << 1,
	CommandBufferUsageSimultaneousUseBit = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct CommandBufferUsageFlags: Flags {
		const COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = CommandBufferUsageFlagBits::CommandBufferUsageOneTimeSubmitBit as Flags;
		const COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = CommandBufferUsageFlagBits::CommandBufferUsageRenderPassContinueBit as Flags;
		const COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = CommandBufferUsageFlagBits::CommandBufferUsageSimultaneousUseBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineDiscardRectangleStateCreateFlagsEXT: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct InstanceCreateFlags: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct DeviceQueueCreateFlags: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct ShaderCorePropertiesFlagsAMD: Flags {
	}
}

#[repr(u32)]
pub enum ConditionalRenderingFlagBitsEXT {
	ConditionalRenderingInvertedBitExt = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct ConditionalRenderingFlagsEXT: Flags {
		const CONDITIONAL_RENDERING_INVERTED_BIT_EXT = ConditionalRenderingFlagBitsEXT::ConditionalRenderingInvertedBitExt as Flags;
	}
}

#[repr(u32)]
pub enum ToolPurposeFlagBits {
	ToolPurposeValidationBit = 1 << 0,
	ToolPurposeProfilingBit = 1 << 1,
	ToolPurposeTracingBit = 1 << 2,
	ToolPurposeAdditionalFeaturesBit = 1 << 3,
	ToolPurposeModifyingFeaturesBit = 1 << 4,
}

impl ToolPurposeFlagBits {
	const ToolPurposeValidationBitExt: u32 = Self::ToolPurposeValidationBitExt;
	const ToolPurposeProfilingBitExt: u32 = Self::ToolPurposeProfilingBitExt;
	const ToolPurposeTracingBitExt: u32 = Self::ToolPurposeTracingBitExt;
	const ToolPurposeAdditionalFeaturesBitExt: u32 = Self::ToolPurposeAdditionalFeaturesBitExt;
	const ToolPurposeModifyingFeaturesBitExt: u32 = Self::ToolPurposeModifyingFeaturesBitExt;
}

bitflags! {
	#[repr(transparent)]
	pub struct ToolPurposeFlags: Flags {
		const TOOL_PURPOSE_VALIDATION_BIT = ToolPurposeFlagBits::ToolPurposeValidationBit as Flags;
		const TOOL_PURPOSE_PROFILING_BIT = ToolPurposeFlagBits::ToolPurposeProfilingBit as Flags;
		const TOOL_PURPOSE_TRACING_BIT = ToolPurposeFlagBits::ToolPurposeTracingBit as Flags;
		const TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT = ToolPurposeFlagBits::ToolPurposeAdditionalFeaturesBit as Flags;
		const TOOL_PURPOSE_MODIFYING_FEATURES_BIT = ToolPurposeFlagBits::ToolPurposeModifyingFeaturesBit as Flags;
	}
}

#[repr(u32)]
pub enum VideoSessionCreateFlagBitsKHR {
	VideoSessionCreateProtectedContentBitKhr = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoSessionCreateFlagsKHR: Flags {
		const VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR = VideoSessionCreateFlagBitsKHR::VideoSessionCreateProtectedContentBitKhr as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct MemoryMapFlags: Flags {
	}
}

#[repr(u32)]
pub enum MemoryHeapFlagBits {
	MemoryHeapDeviceLocalBit = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct MemoryHeapFlags: Flags {
		const MEMORY_HEAP_DEVICE_LOCAL_BIT = MemoryHeapFlagBits::MemoryHeapDeviceLocalBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct EventCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum MemoryPropertyFlagBits {
	MemoryPropertyDeviceLocalBit = 1 << 0,
	MemoryPropertyHostVisibleBit = 1 << 1,
	MemoryPropertyHostCoherentBit = 1 << 2,
	MemoryPropertyHostCachedBit = 1 << 3,
	MemoryPropertyLazilyAllocatedBit = 1 << 4,
}

bitflags! {
	#[repr(transparent)]
	pub struct MemoryPropertyFlags: Flags {
		const MEMORY_PROPERTY_DEVICE_LOCAL_BIT = MemoryPropertyFlagBits::MemoryPropertyDeviceLocalBit as Flags;
		const MEMORY_PROPERTY_HOST_VISIBLE_BIT = MemoryPropertyFlagBits::MemoryPropertyHostVisibleBit as Flags;
		const MEMORY_PROPERTY_HOST_COHERENT_BIT = MemoryPropertyFlagBits::MemoryPropertyHostCoherentBit as Flags;
		const MEMORY_PROPERTY_HOST_CACHED_BIT = MemoryPropertyFlagBits::MemoryPropertyHostCachedBit as Flags;
		const MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = MemoryPropertyFlagBits::MemoryPropertyLazilyAllocatedBit as Flags;
	}
}

#[repr(u32)]
pub enum CommandPoolCreateFlagBits {
	CommandPoolCreateTransientBit = 1 << 0,
	CommandPoolCreateResetCommandBufferBit = 1 << 1,
}

bitflags! {
	#[repr(transparent)]
	pub struct CommandPoolCreateFlags: Flags {
		const COMMAND_POOL_CREATE_TRANSIENT_BIT = CommandPoolCreateFlagBits::CommandPoolCreateTransientBit as Flags;
		const COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = CommandPoolCreateFlagBits::CommandPoolCreateResetCommandBufferBit as Flags;
	}
}

#[repr(u32)]
pub enum PerformanceCounterDescriptionFlagBitsKHR {
	PerformanceCounterDescriptionPerformanceImpactingBitKhr = 1 << 0,
	PerformanceCounterDescriptionConcurrentlyImpactedBitKhr = 1 << 1,
}

impl PerformanceCounterDescriptionFlagBitsKHR {
	const PerformanceCounterDescriptionPerformanceImpactingKhr: u32 = Self::PerformanceCounterDescriptionPerformanceImpactingKhr;
	const PerformanceCounterDescriptionConcurrentlyImpactedKhr: u32 = Self::PerformanceCounterDescriptionConcurrentlyImpactedKhr;
}

bitflags! {
	#[repr(transparent)]
	pub struct PerformanceCounterDescriptionFlagsKHR: Flags {
		const PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR = PerformanceCounterDescriptionFlagBitsKHR::PerformanceCounterDescriptionPerformanceImpactingBitKhr as Flags;
		const PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR = PerformanceCounterDescriptionFlagBitsKHR::PerformanceCounterDescriptionConcurrentlyImpactedBitKhr as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct AccelerationStructureMotionInfoFlagsNV: Flags {
	}
}

#[repr(u32)]
pub enum AccessFlagBits {
	AccessIndirectCommandReadBit = 1 << 0,
	AccessIndexReadBit = 1 << 1,
	AccessVertexAttributeReadBit = 1 << 2,
	AccessUniformReadBit = 1 << 3,
	AccessInputAttachmentReadBit = 1 << 4,
	AccessShaderReadBit = 1 << 5,
	AccessShaderWriteBit = 1 << 6,
	AccessColorAttachmentReadBit = 1 << 7,
	AccessColorAttachmentWriteBit = 1 << 8,
	AccessDepthStencilAttachmentReadBit = 1 << 9,
	AccessDepthStencilAttachmentWriteBit = 1 << 10,
	AccessTransferReadBit = 1 << 11,
	AccessTransferWriteBit = 1 << 12,
	AccessHostReadBit = 1 << 13,
	AccessHostWriteBit = 1 << 14,
	AccessMemoryReadBit = 1 << 15,
	AccessMemoryWriteBit = 1 << 16,
}

bitflags! {
	#[repr(transparent)]
	pub struct AccessFlags: Flags {
		const ACCESS_INDIRECT_COMMAND_READ_BIT = AccessFlagBits::AccessIndirectCommandReadBit as Flags;
		const ACCESS_INDEX_READ_BIT = AccessFlagBits::AccessIndexReadBit as Flags;
		const ACCESS_VERTEX_ATTRIBUTE_READ_BIT = AccessFlagBits::AccessVertexAttributeReadBit as Flags;
		const ACCESS_UNIFORM_READ_BIT = AccessFlagBits::AccessUniformReadBit as Flags;
		const ACCESS_INPUT_ATTACHMENT_READ_BIT = AccessFlagBits::AccessInputAttachmentReadBit as Flags;
		const ACCESS_SHADER_READ_BIT = AccessFlagBits::AccessShaderReadBit as Flags;
		const ACCESS_SHADER_WRITE_BIT = AccessFlagBits::AccessShaderWriteBit as Flags;
		const ACCESS_COLOR_ATTACHMENT_READ_BIT = AccessFlagBits::AccessColorAttachmentReadBit as Flags;
		const ACCESS_COLOR_ATTACHMENT_WRITE_BIT = AccessFlagBits::AccessColorAttachmentWriteBit as Flags;
		const ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT = AccessFlagBits::AccessDepthStencilAttachmentReadBit as Flags;
		const ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = AccessFlagBits::AccessDepthStencilAttachmentWriteBit as Flags;
		const ACCESS_TRANSFER_READ_BIT = AccessFlagBits::AccessTransferReadBit as Flags;
		const ACCESS_TRANSFER_WRITE_BIT = AccessFlagBits::AccessTransferWriteBit as Flags;
		const ACCESS_HOST_READ_BIT = AccessFlagBits::AccessHostReadBit as Flags;
		const ACCESS_HOST_WRITE_BIT = AccessFlagBits::AccessHostWriteBit as Flags;
		const ACCESS_MEMORY_READ_BIT = AccessFlagBits::AccessMemoryReadBit as Flags;
		const ACCESS_MEMORY_WRITE_BIT = AccessFlagBits::AccessMemoryWriteBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct DebugUtilsMessengerCreateFlagsEXT: Flags {
	}
}

#[repr(u32)]
pub enum AccelerationStructureCreateFlagBitsKHR {
	AccelerationStructureCreateDeviceAddressCaptureReplayBitKhr = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct AccelerationStructureCreateFlagsKHR: Flags {
		const ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR = AccelerationStructureCreateFlagBitsKHR::AccelerationStructureCreateDeviceAddressCaptureReplayBitKhr as Flags;
	}
}

#[repr(u32)]
pub enum VideoEncodeH265RateControlFlagBitsEXT {
	VideoEncodeH265RateControlAttemptHrdComplianceBitExt = 1 << 0,
	VideoEncodeH265RateControlRegularGopBitExt = 1 << 1,
	VideoEncodeH265RateControlReferencePatternFlatBitExt = 1 << 2,
	VideoEncodeH265RateControlReferencePatternDyadicBitExt = 1 << 3,
	VideoEncodeH265RateControlTemporalSubLayerPatternDyadicBitExt = 1 << 4,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeH265RateControlFlagsEXT: Flags {
		const VIDEO_ENCODE_H265_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_EXT = VideoEncodeH265RateControlFlagBitsEXT::VideoEncodeH265RateControlAttemptHrdComplianceBitExt as Flags;
		const VIDEO_ENCODE_H265_RATE_CONTROL_REGULAR_GOP_BIT_EXT = VideoEncodeH265RateControlFlagBitsEXT::VideoEncodeH265RateControlRegularGopBitExt as Flags;
		const VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_EXT = VideoEncodeH265RateControlFlagBitsEXT::VideoEncodeH265RateControlReferencePatternFlatBitExt as Flags;
		const VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_EXT = VideoEncodeH265RateControlFlagBitsEXT::VideoEncodeH265RateControlReferencePatternDyadicBitExt as Flags;
		const VIDEO_ENCODE_H265_RATE_CONTROL_TEMPORAL_SUB_LAYER_PATTERN_DYADIC_BIT_EXT = VideoEncodeH265RateControlFlagBitsEXT::VideoEncodeH265RateControlTemporalSubLayerPatternDyadicBitExt as Flags;
	}
}

#[repr(u32)]
pub enum OpticalFlowExecuteFlagBitsNV {
	OpticalFlowExecuteDisableTemporalHintsBitNv = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct OpticalFlowExecuteFlagsNV: Flags {
		const OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_NV = OpticalFlowExecuteFlagBitsNV::OpticalFlowExecuteDisableTemporalHintsBitNv as Flags;
	}
}

#[repr(u32)]
pub enum VideoEncodeH264CapabilityFlagBitsEXT {
	VideoEncodeH264CapabilityHrdComplianceBitExt = 1 << 0,
	VideoEncodeH264CapabilityPredictionWeightTableGeneratedBitExt = 1 << 1,
	VideoEncodeH264CapabilityRowUnalignedSliceBitExt = 1 << 2,
	VideoEncodeH264CapabilityDifferentSliceTypeBitExt = 1 << 3,
	VideoEncodeH264CapabilityBFrameInL0ListBitExt = 1 << 4,
	VideoEncodeH264CapabilityBFrameInL1ListBitExt = 1 << 5,
	VideoEncodeH264CapabilityPerPictureTypeMinMaxQpBitExt = 1 << 6,
	VideoEncodeH264CapabilityPerSliceConstantQpBitExt = 1 << 7,
	VideoEncodeH264CapabilityGeneratePrefixNaluBitExt = 1 << 8,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeH264CapabilityFlagsEXT: Flags {
		const VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_BIT_EXT = VideoEncodeH264CapabilityFlagBitsEXT::VideoEncodeH264CapabilityHrdComplianceBitExt as Flags;
		const VIDEO_ENCODE_H264_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_EXT = VideoEncodeH264CapabilityFlagBitsEXT::VideoEncodeH264CapabilityPredictionWeightTableGeneratedBitExt as Flags;
		const VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_BIT_EXT = VideoEncodeH264CapabilityFlagBitsEXT::VideoEncodeH264CapabilityRowUnalignedSliceBitExt as Flags;
		const VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_EXT = VideoEncodeH264CapabilityFlagBitsEXT::VideoEncodeH264CapabilityDifferentSliceTypeBitExt as Flags;
		const VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_EXT = VideoEncodeH264CapabilityFlagBitsEXT::VideoEncodeH264CapabilityBFrameInL0ListBitExt as Flags;
		const VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_EXT = VideoEncodeH264CapabilityFlagBitsEXT::VideoEncodeH264CapabilityBFrameInL1ListBitExt as Flags;
		const VIDEO_ENCODE_H264_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_EXT = VideoEncodeH264CapabilityFlagBitsEXT::VideoEncodeH264CapabilityPerPictureTypeMinMaxQpBitExt as Flags;
		const VIDEO_ENCODE_H264_CAPABILITY_PER_SLICE_CONSTANT_QP_BIT_EXT = VideoEncodeH264CapabilityFlagBitsEXT::VideoEncodeH264CapabilityPerSliceConstantQpBitExt as Flags;
		const VIDEO_ENCODE_H264_CAPABILITY_GENERATE_PREFIX_NALU_BIT_EXT = VideoEncodeH264CapabilityFlagBitsEXT::VideoEncodeH264CapabilityGeneratePrefixNaluBitExt as Flags;
	}
}

#[repr(u32)]
pub enum ImageCompressionFixedRateFlagBitsEXT {
	ImageCompressionFixedRate1bpcBitExt = 1 << 0,
	ImageCompressionFixedRate2bpcBitExt = 1 << 1,
	ImageCompressionFixedRate3bpcBitExt = 1 << 2,
	ImageCompressionFixedRate4bpcBitExt = 1 << 3,
	ImageCompressionFixedRate5bpcBitExt = 1 << 4,
	ImageCompressionFixedRate6bpcBitExt = 1 << 5,
	ImageCompressionFixedRate7bpcBitExt = 1 << 6,
	ImageCompressionFixedRate8bpcBitExt = 1 << 7,
	ImageCompressionFixedRate9bpcBitExt = 1 << 8,
	ImageCompressionFixedRate10bpcBitExt = 1 << 9,
	ImageCompressionFixedRate11bpcBitExt = 1 << 10,
	ImageCompressionFixedRate12bpcBitExt = 1 << 11,
	ImageCompressionFixedRate13bpcBitExt = 1 << 12,
	ImageCompressionFixedRate14bpcBitExt = 1 << 13,
	ImageCompressionFixedRate15bpcBitExt = 1 << 14,
	ImageCompressionFixedRate16bpcBitExt = 1 << 15,
	ImageCompressionFixedRate17bpcBitExt = 1 << 16,
	ImageCompressionFixedRate18bpcBitExt = 1 << 17,
	ImageCompressionFixedRate19bpcBitExt = 1 << 18,
	ImageCompressionFixedRate20bpcBitExt = 1 << 19,
	ImageCompressionFixedRate21bpcBitExt = 1 << 20,
	ImageCompressionFixedRate22bpcBitExt = 1 << 21,
	ImageCompressionFixedRate23bpcBitExt = 1 << 22,
	ImageCompressionFixedRate24bpcBitExt = 1 << 23,
}

bitflags! {
	#[repr(transparent)]
	pub struct ImageCompressionFixedRateFlagsEXT: Flags {
		const IMAGE_COMPRESSION_FIXED_RATE_1BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate1bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_2BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate2bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_3BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate3bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_4BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate4bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_5BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate5bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_6BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate6bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_7BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate7bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_8BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate8bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_9BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate9bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_10BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate10bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_11BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate11bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_12BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate12bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_13BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate13bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_14BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate14bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_15BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate15bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_16BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate16bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_17BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate17bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_18BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate18bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_19BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate19bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_20BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate20bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_21BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate21bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_22BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate22bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_23BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate23bpcBitExt as Flags;
		const IMAGE_COMPRESSION_FIXED_RATE_24BPC_BIT_EXT = ImageCompressionFixedRateFlagBitsEXT::ImageCompressionFixedRate24bpcBitExt as Flags;
	}
}

#[repr(u32)]
pub enum BuildMicromapFlagBitsEXT {
	BuildMicromapPreferFastTraceBitExt = 1 << 0,
	BuildMicromapPreferFastBuildBitExt = 1 << 1,
	BuildMicromapAllowCompactionBitExt = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct BuildMicromapFlagsEXT: Flags {
		const BUILD_MICROMAP_PREFER_FAST_TRACE_BIT_EXT = BuildMicromapFlagBitsEXT::BuildMicromapPreferFastTraceBitExt as Flags;
		const BUILD_MICROMAP_PREFER_FAST_BUILD_BIT_EXT = BuildMicromapFlagBitsEXT::BuildMicromapPreferFastBuildBitExt as Flags;
		const BUILD_MICROMAP_ALLOW_COMPACTION_BIT_EXT = BuildMicromapFlagBitsEXT::BuildMicromapAllowCompactionBitExt as Flags;
	}
}

#[repr(u32)]
pub enum DisplayPlaneAlphaFlagBitsKHR {
	DisplayPlaneAlphaOpaqueBitKhr = 1 << 0,
	DisplayPlaneAlphaGlobalBitKhr = 1 << 1,
	DisplayPlaneAlphaPerPixelBitKhr = 1 << 2,
	DisplayPlaneAlphaPerPixelPremultipliedBitKhr = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct DisplayPlaneAlphaFlagsKHR: Flags {
		const DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = DisplayPlaneAlphaFlagBitsKHR::DisplayPlaneAlphaOpaqueBitKhr as Flags;
		const DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = DisplayPlaneAlphaFlagBitsKHR::DisplayPlaneAlphaGlobalBitKhr as Flags;
		const DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = DisplayPlaneAlphaFlagBitsKHR::DisplayPlaneAlphaPerPixelBitKhr as Flags;
		const DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = DisplayPlaneAlphaFlagBitsKHR::DisplayPlaneAlphaPerPixelPremultipliedBitKhr as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct WaylandSurfaceCreateFlagsKHR: Flags {
	}
}

#[repr(u32)]
pub enum ExternalSemaphoreHandleTypeFlagBits {
	ExternalSemaphoreHandleTypeOpaqueFdBit = 1 << 0,
	ExternalSemaphoreHandleTypeOpaqueWin32Bit = 1 << 1,
	ExternalSemaphoreHandleTypeOpaqueWin32KmtBit = 1 << 2,
	ExternalSemaphoreHandleTypeD3d12FenceBit = 1 << 3,
	ExternalSemaphoreHandleTypeSyncFdBit = 1 << 4,
}

impl ExternalSemaphoreHandleTypeFlagBits {
	const ExternalSemaphoreHandleTypeD3d11FenceBit: u32 = Self::ExternalSemaphoreHandleTypeD3d11FenceBit;
}

bitflags! {
	#[repr(transparent)]
	pub struct ExternalSemaphoreHandleTypeFlags: Flags {
		const EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT = ExternalSemaphoreHandleTypeFlagBits::ExternalSemaphoreHandleTypeOpaqueFdBit as Flags;
		const EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT = ExternalSemaphoreHandleTypeFlagBits::ExternalSemaphoreHandleTypeOpaqueWin32Bit as Flags;
		const EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = ExternalSemaphoreHandleTypeFlagBits::ExternalSemaphoreHandleTypeOpaqueWin32KmtBit as Flags;
		const EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT = ExternalSemaphoreHandleTypeFlagBits::ExternalSemaphoreHandleTypeD3d12FenceBit as Flags;
		const EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT = ExternalSemaphoreHandleTypeFlagBits::ExternalSemaphoreHandleTypeSyncFdBit as Flags;
	}
}

#[repr(u32)]
pub enum SparseMemoryBindFlagBits {
	SparseMemoryBindMetadataBit = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct SparseMemoryBindFlags: Flags {
		const SPARSE_MEMORY_BIND_METADATA_BIT = SparseMemoryBindFlagBits::SparseMemoryBindMetadataBit as Flags;
	}
}

#[repr(u32)]
pub enum ImageUsageFlagBits {
	ImageUsageTransferSrcBit = 1 << 0,
	ImageUsageTransferDstBit = 1 << 1,
	ImageUsageSampledBit = 1 << 2,
	ImageUsageStorageBit = 1 << 3,
	ImageUsageColorAttachmentBit = 1 << 4,
	ImageUsageDepthStencilAttachmentBit = 1 << 5,
	ImageUsageTransientAttachmentBit = 1 << 6,
	ImageUsageInputAttachmentBit = 1 << 7,
}

bitflags! {
	#[repr(transparent)]
	pub struct ImageUsageFlags: Flags {
		const IMAGE_USAGE_TRANSFER_SRC_BIT = ImageUsageFlagBits::ImageUsageTransferSrcBit as Flags;
		const IMAGE_USAGE_TRANSFER_DST_BIT = ImageUsageFlagBits::ImageUsageTransferDstBit as Flags;
		const IMAGE_USAGE_SAMPLED_BIT = ImageUsageFlagBits::ImageUsageSampledBit as Flags;
		const IMAGE_USAGE_STORAGE_BIT = ImageUsageFlagBits::ImageUsageStorageBit as Flags;
		const IMAGE_USAGE_COLOR_ATTACHMENT_BIT = ImageUsageFlagBits::ImageUsageColorAttachmentBit as Flags;
		const IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = ImageUsageFlagBits::ImageUsageDepthStencilAttachmentBit as Flags;
		const IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = ImageUsageFlagBits::ImageUsageTransientAttachmentBit as Flags;
		const IMAGE_USAGE_INPUT_ATTACHMENT_BIT = ImageUsageFlagBits::ImageUsageInputAttachmentBit as Flags;
	}
}

#[repr(u32)]
pub enum SemaphoreWaitFlagBits {
	SemaphoreWaitAnyBit = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct SemaphoreWaitFlags: Flags {
		const SEMAPHORE_WAIT_ANY_BIT = SemaphoreWaitFlagBits::SemaphoreWaitAnyBit as Flags;
	}
}

#[repr(u64)]
pub enum MemoryDecompressionMethodFlagBitsNV {
	MemoryDecompressionMethodGdeflate10BitNv = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct MemoryDecompressionMethodFlagsNV: Flags64 {
		const MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_NV = MemoryDecompressionMethodFlagBitsNV::MemoryDecompressionMethodGdeflate10BitNv as Flags64;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct IOSSurfaceCreateFlagsMVK: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineRasterizationStateStreamCreateFlagsEXT: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineCacheCreateFlags: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct ValidationCacheCreateFlagsEXT: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineLayoutCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum VideoEncodeCapabilityFlagBitsKHR {
	VideoEncodeCapabilityPrecedingExternallyEncodedBytesBitKhr = 1 << 0,
	VideoEncodeCapabilityInsufficientBitstreamBufferRangeDetectionBitKhr = 1 << 1,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeCapabilityFlagsKHR: Flags {
		const VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR = VideoEncodeCapabilityFlagBitsKHR::VideoEncodeCapabilityPrecedingExternallyEncodedBytesBitKhr as Flags;
		const VIDEO_ENCODE_CAPABILITY_INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_BIT_KHR = VideoEncodeCapabilityFlagBitsKHR::VideoEncodeCapabilityInsufficientBitstreamBufferRangeDetectionBitKhr as Flags;
	}
}

#[repr(u32)]
pub enum DebugUtilsMessageTypeFlagBitsEXT {
	DebugUtilsMessageTypeGeneralBitExt = 1 << 0,
	DebugUtilsMessageTypeValidationBitExt = 1 << 1,
	DebugUtilsMessageTypePerformanceBitExt = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct DebugUtilsMessageTypeFlagsEXT: Flags {
		const DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT = DebugUtilsMessageTypeFlagBitsEXT::DebugUtilsMessageTypeGeneralBitExt as Flags;
		const DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT = DebugUtilsMessageTypeFlagBitsEXT::DebugUtilsMessageTypeValidationBitExt as Flags;
		const DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT = DebugUtilsMessageTypeFlagBitsEXT::DebugUtilsMessageTypePerformanceBitExt as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoCodecOperationFlagsKHR: Flags {
	}
}

#[repr(u32)]
pub enum ShaderStageFlagBits {
	ShaderStageVertexBit = 1 << 0,
	ShaderStageTessellationControlBit = 1 << 1,
	ShaderStageTessellationEvaluationBit = 1 << 2,
	ShaderStageGeometryBit = 1 << 3,
	ShaderStageFragmentBit = 1 << 4,
	ShaderStageComputeBit = 1 << 5,
}

bitflags! {
	#[repr(transparent)]
	pub struct ShaderStageFlags: Flags {
		const SHADER_STAGE_VERTEX_BIT = ShaderStageFlagBits::ShaderStageVertexBit as Flags;
		const SHADER_STAGE_TESSELLATION_CONTROL_BIT = ShaderStageFlagBits::ShaderStageTessellationControlBit as Flags;
		const SHADER_STAGE_TESSELLATION_EVALUATION_BIT = ShaderStageFlagBits::ShaderStageTessellationEvaluationBit as Flags;
		const SHADER_STAGE_GEOMETRY_BIT = ShaderStageFlagBits::ShaderStageGeometryBit as Flags;
		const SHADER_STAGE_FRAGMENT_BIT = ShaderStageFlagBits::ShaderStageFragmentBit as Flags;
		const SHADER_STAGE_COMPUTE_BIT = ShaderStageFlagBits::ShaderStageComputeBit as Flags;
	}
}

#[repr(u32)]
pub enum SampleCountFlagBits {
	SampleCount1Bit = 1 << 0,
	SampleCount2Bit = 1 << 1,
	SampleCount4Bit = 1 << 2,
	SampleCount8Bit = 1 << 3,
	SampleCount16Bit = 1 << 4,
	SampleCount32Bit = 1 << 5,
	SampleCount64Bit = 1 << 6,
}

bitflags! {
	#[repr(transparent)]
	pub struct SampleCountFlags: Flags {
		const SAMPLE_COUNT_1_BIT = SampleCountFlagBits::SampleCount1Bit as Flags;
		const SAMPLE_COUNT_2_BIT = SampleCountFlagBits::SampleCount2Bit as Flags;
		const SAMPLE_COUNT_4_BIT = SampleCountFlagBits::SampleCount4Bit as Flags;
		const SAMPLE_COUNT_8_BIT = SampleCountFlagBits::SampleCount8Bit as Flags;
		const SAMPLE_COUNT_16_BIT = SampleCountFlagBits::SampleCount16Bit as Flags;
		const SAMPLE_COUNT_32_BIT = SampleCountFlagBits::SampleCount32Bit as Flags;
		const SAMPLE_COUNT_64_BIT = SampleCountFlagBits::SampleCount64Bit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineColorBlendStateCreateFlags: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct AndroidSurfaceCreateFlagsKHR: Flags {
	}
}

#[repr(u32)]
pub enum VideoChromaSubsamplingFlagBitsKHR {
	VideoChromaSubsamplingMonochromeBitKhr = 1 << 0,
	VideoChromaSubsampling420BitKhr = 1 << 1,
	VideoChromaSubsampling422BitKhr = 1 << 2,
	VideoChromaSubsampling444BitKhr = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoChromaSubsamplingFlagsKHR: Flags {
		const VIDEO_CHROMA_SUBSAMPLING_MONOCHROME_BIT_KHR = VideoChromaSubsamplingFlagBitsKHR::VideoChromaSubsamplingMonochromeBitKhr as Flags;
		const VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR = VideoChromaSubsamplingFlagBitsKHR::VideoChromaSubsampling420BitKhr as Flags;
		const VIDEO_CHROMA_SUBSAMPLING_422_BIT_KHR = VideoChromaSubsamplingFlagBitsKHR::VideoChromaSubsampling422BitKhr as Flags;
		const VIDEO_CHROMA_SUBSAMPLING_444_BIT_KHR = VideoChromaSubsamplingFlagBitsKHR::VideoChromaSubsampling444BitKhr as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineDynamicStateCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum BufferUsageFlagBits {
	BufferUsageTransferSrcBit = 1 << 0,
	BufferUsageTransferDstBit = 1 << 1,
	BufferUsageUniformTexelBufferBit = 1 << 2,
	BufferUsageStorageTexelBufferBit = 1 << 3,
	BufferUsageUniformBufferBit = 1 << 4,
	BufferUsageStorageBufferBit = 1 << 5,
	BufferUsageIndexBufferBit = 1 << 6,
	BufferUsageVertexBufferBit = 1 << 7,
	BufferUsageIndirectBufferBit = 1 << 8,
}

bitflags! {
	#[repr(transparent)]
	pub struct BufferUsageFlags: Flags {
		const BUFFER_USAGE_TRANSFER_SRC_BIT = BufferUsageFlagBits::BufferUsageTransferSrcBit as Flags;
		const BUFFER_USAGE_TRANSFER_DST_BIT = BufferUsageFlagBits::BufferUsageTransferDstBit as Flags;
		const BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = BufferUsageFlagBits::BufferUsageUniformTexelBufferBit as Flags;
		const BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = BufferUsageFlagBits::BufferUsageStorageTexelBufferBit as Flags;
		const BUFFER_USAGE_UNIFORM_BUFFER_BIT = BufferUsageFlagBits::BufferUsageUniformBufferBit as Flags;
		const BUFFER_USAGE_STORAGE_BUFFER_BIT = BufferUsageFlagBits::BufferUsageStorageBufferBit as Flags;
		const BUFFER_USAGE_INDEX_BUFFER_BIT = BufferUsageFlagBits::BufferUsageIndexBufferBit as Flags;
		const BUFFER_USAGE_VERTEX_BUFFER_BIT = BufferUsageFlagBits::BufferUsageVertexBufferBit as Flags;
		const BUFFER_USAGE_INDIRECT_BUFFER_BIT = BufferUsageFlagBits::BufferUsageIndirectBufferBit as Flags;
	}
}

#[repr(u32)]
pub enum DescriptorBindingFlagBits {
	DescriptorBindingUpdateAfterBindBit = 1 << 0,
	DescriptorBindingUpdateUnusedWhilePendingBit = 1 << 1,
	DescriptorBindingPartiallyBoundBit = 1 << 2,
	DescriptorBindingVariableDescriptorCountBit = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct DescriptorBindingFlags: Flags {
		const DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT = DescriptorBindingFlagBits::DescriptorBindingUpdateAfterBindBit as Flags;
		const DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT = DescriptorBindingFlagBits::DescriptorBindingUpdateUnusedWhilePendingBit as Flags;
		const DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT = DescriptorBindingFlagBits::DescriptorBindingPartiallyBoundBit as Flags;
		const DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT = DescriptorBindingFlagBits::DescriptorBindingVariableDescriptorCountBit as Flags;
	}
}

#[repr(u32)]
pub enum VideoEncodeUsageFlagBitsKHR {
	VideoEncodeUsageTranscodingBitKhr = 1 << 0,
	VideoEncodeUsageStreamingBitKhr = 1 << 1,
	VideoEncodeUsageRecordingBitKhr = 1 << 2,
	VideoEncodeUsageConferencingBitKhr = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeUsageFlagsKHR: Flags {
		const VIDEO_ENCODE_USAGE_TRANSCODING_BIT_KHR = VideoEncodeUsageFlagBitsKHR::VideoEncodeUsageTranscodingBitKhr as Flags;
		const VIDEO_ENCODE_USAGE_STREAMING_BIT_KHR = VideoEncodeUsageFlagBitsKHR::VideoEncodeUsageStreamingBitKhr as Flags;
		const VIDEO_ENCODE_USAGE_RECORDING_BIT_KHR = VideoEncodeUsageFlagBitsKHR::VideoEncodeUsageRecordingBitKhr as Flags;
		const VIDEO_ENCODE_USAGE_CONFERENCING_BIT_KHR = VideoEncodeUsageFlagBitsKHR::VideoEncodeUsageConferencingBitKhr as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct QueryPoolCreateFlags: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct SwapchainCreateFlagsKHR: Flags {
	}
}

#[repr(u32)]
pub enum VideoEncodeH265CapabilityFlagBitsEXT {
	VideoEncodeH265CapabilityHrdComplianceBitExt = 1 << 0,
	VideoEncodeH265CapabilityPredictionWeightTableGeneratedBitExt = 1 << 1,
	VideoEncodeH265CapabilityRowUnalignedSliceSegmentBitExt = 1 << 2,
	VideoEncodeH265CapabilityDifferentSliceSegmentTypeBitExt = 1 << 3,
	VideoEncodeH265CapabilityBFrameInL0ListBitExt = 1 << 4,
	VideoEncodeH265CapabilityBFrameInL1ListBitExt = 1 << 5,
	VideoEncodeH265CapabilityPerPictureTypeMinMaxQpBitExt = 1 << 6,
	VideoEncodeH265CapabilityPerSliceSegmentConstantQpBitExt = 1 << 7,
	VideoEncodeH265CapabilityMultipleTilesPerSliceSegmentBitExt = 1 << 8,
	VideoEncodeH265CapabilityMultipleSliceSegmentsPerTileBitExt = 1 << 9,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeH265CapabilityFlagsEXT: Flags {
		const VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_BIT_EXT = VideoEncodeH265CapabilityFlagBitsEXT::VideoEncodeH265CapabilityHrdComplianceBitExt as Flags;
		const VIDEO_ENCODE_H265_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_EXT = VideoEncodeH265CapabilityFlagBitsEXT::VideoEncodeH265CapabilityPredictionWeightTableGeneratedBitExt as Flags;
		const VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_BIT_EXT = VideoEncodeH265CapabilityFlagBitsEXT::VideoEncodeH265CapabilityRowUnalignedSliceSegmentBitExt as Flags;
		const VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_SEGMENT_TYPE_BIT_EXT = VideoEncodeH265CapabilityFlagBitsEXT::VideoEncodeH265CapabilityDifferentSliceSegmentTypeBitExt as Flags;
		const VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_EXT = VideoEncodeH265CapabilityFlagBitsEXT::VideoEncodeH265CapabilityBFrameInL0ListBitExt as Flags;
		const VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_EXT = VideoEncodeH265CapabilityFlagBitsEXT::VideoEncodeH265CapabilityBFrameInL1ListBitExt as Flags;
		const VIDEO_ENCODE_H265_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_EXT = VideoEncodeH265CapabilityFlagBitsEXT::VideoEncodeH265CapabilityPerPictureTypeMinMaxQpBitExt as Flags;
		const VIDEO_ENCODE_H265_CAPABILITY_PER_SLICE_SEGMENT_CONSTANT_QP_BIT_EXT = VideoEncodeH265CapabilityFlagBitsEXT::VideoEncodeH265CapabilityPerSliceSegmentConstantQpBitExt as Flags;
		const VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILES_PER_SLICE_SEGMENT_BIT_EXT = VideoEncodeH265CapabilityFlagBitsEXT::VideoEncodeH265CapabilityMultipleTilesPerSliceSegmentBitExt as Flags;
		const VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_SEGMENTS_PER_TILE_BIT_EXT = VideoEncodeH265CapabilityFlagBitsEXT::VideoEncodeH265CapabilityMultipleSliceSegmentsPerTileBitExt as Flags;
	}
}

#[repr(u32)]
pub enum VideoEncodeH265CtbSizeFlagBitsEXT {
	VideoEncodeH265CtbSize16BitExt = 1 << 0,
	VideoEncodeH265CtbSize32BitExt = 1 << 1,
	VideoEncodeH265CtbSize64BitExt = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeH265CtbSizeFlagsEXT: Flags {
		const VIDEO_ENCODE_H265_CTB_SIZE_16_BIT_EXT = VideoEncodeH265CtbSizeFlagBitsEXT::VideoEncodeH265CtbSize16BitExt as Flags;
		const VIDEO_ENCODE_H265_CTB_SIZE_32_BIT_EXT = VideoEncodeH265CtbSizeFlagBitsEXT::VideoEncodeH265CtbSize32BitExt as Flags;
		const VIDEO_ENCODE_H265_CTB_SIZE_64_BIT_EXT = VideoEncodeH265CtbSizeFlagBitsEXT::VideoEncodeH265CtbSize64BitExt as Flags;
	}
}

#[repr(u32)]
pub enum ImageAspectFlagBits {
	ImageAspectColorBit = 1 << 0,
	ImageAspectDepthBit = 1 << 1,
	ImageAspectStencilBit = 1 << 2,
	ImageAspectMetadataBit = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct ImageAspectFlags: Flags {
		const IMAGE_ASPECT_COLOR_BIT = ImageAspectFlagBits::ImageAspectColorBit as Flags;
		const IMAGE_ASPECT_DEPTH_BIT = ImageAspectFlagBits::ImageAspectDepthBit as Flags;
		const IMAGE_ASPECT_STENCIL_BIT = ImageAspectFlagBits::ImageAspectStencilBit as Flags;
		const IMAGE_ASPECT_METADATA_BIT = ImageAspectFlagBits::ImageAspectMetadataBit as Flags;
	}
}

#[repr(u32)]
pub enum VideoEncodeContentFlagBitsKHR {
	VideoEncodeContentCameraBitKhr = 1 << 0,
	VideoEncodeContentDesktopBitKhr = 1 << 1,
	VideoEncodeContentRenderedBitKhr = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeContentFlagsKHR: Flags {
		const VIDEO_ENCODE_CONTENT_CAMERA_BIT_KHR = VideoEncodeContentFlagBitsKHR::VideoEncodeContentCameraBitKhr as Flags;
		const VIDEO_ENCODE_CONTENT_DESKTOP_BIT_KHR = VideoEncodeContentFlagBitsKHR::VideoEncodeContentDesktopBitKhr as Flags;
		const VIDEO_ENCODE_CONTENT_RENDERED_BIT_KHR = VideoEncodeContentFlagBitsKHR::VideoEncodeContentRenderedBitKhr as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineVertexInputStateCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum DependencyFlagBits {
	DependencyByRegionBit = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct DependencyFlags: Flags {
		const DEPENDENCY_BY_REGION_BIT = DependencyFlagBits::DependencyByRegionBit as Flags;
	}
}

#[repr(u32)]
pub enum FenceImportFlagBits {
	FenceImportTemporaryBit = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct FenceImportFlags: Flags {
		const FENCE_IMPORT_TEMPORARY_BIT = FenceImportFlagBits::FenceImportTemporaryBit as Flags;
	}
}

#[repr(u32)]
pub enum VideoCapabilityFlagBitsKHR {
	VideoCapabilityProtectedContentBitKhr = 1 << 0,
	VideoCapabilitySeparateReferenceImagesBitKhr = 1 << 1,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoCapabilityFlagsKHR: Flags {
		const VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR = VideoCapabilityFlagBitsKHR::VideoCapabilityProtectedContentBitKhr as Flags;
		const VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR = VideoCapabilityFlagBitsKHR::VideoCapabilitySeparateReferenceImagesBitKhr as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoSessionParametersCreateFlagsKHR: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeFlagsKHR: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineViewportStateCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum FormatFeatureFlagBits {
	FormatFeatureSampledImageBit = 1 << 0,
	FormatFeatureStorageImageBit = 1 << 1,
	FormatFeatureStorageImageAtomicBit = 1 << 2,
	FormatFeatureUniformTexelBufferBit = 1 << 3,
	FormatFeatureStorageTexelBufferBit = 1 << 4,
	FormatFeatureStorageTexelBufferAtomicBit = 1 << 5,
	FormatFeatureVertexBufferBit = 1 << 6,
	FormatFeatureColorAttachmentBit = 1 << 7,
	FormatFeatureColorAttachmentBlendBit = 1 << 8,
	FormatFeatureDepthStencilAttachmentBit = 1 << 9,
	FormatFeatureBlitSrcBit = 1 << 10,
	FormatFeatureBlitDstBit = 1 << 11,
	FormatFeatureSampledImageFilterLinearBit = 1 << 12,
}

bitflags! {
	#[repr(transparent)]
	pub struct FormatFeatureFlags: Flags {
		const FORMAT_FEATURE_SAMPLED_IMAGE_BIT = FormatFeatureFlagBits::FormatFeatureSampledImageBit as Flags;
		const FORMAT_FEATURE_STORAGE_IMAGE_BIT = FormatFeatureFlagBits::FormatFeatureStorageImageBit as Flags;
		const FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT = FormatFeatureFlagBits::FormatFeatureStorageImageAtomicBit as Flags;
		const FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT = FormatFeatureFlagBits::FormatFeatureUniformTexelBufferBit as Flags;
		const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT = FormatFeatureFlagBits::FormatFeatureStorageTexelBufferBit as Flags;
		const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = FormatFeatureFlagBits::FormatFeatureStorageTexelBufferAtomicBit as Flags;
		const FORMAT_FEATURE_VERTEX_BUFFER_BIT = FormatFeatureFlagBits::FormatFeatureVertexBufferBit as Flags;
		const FORMAT_FEATURE_COLOR_ATTACHMENT_BIT = FormatFeatureFlagBits::FormatFeatureColorAttachmentBit as Flags;
		const FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT = FormatFeatureFlagBits::FormatFeatureColorAttachmentBlendBit as Flags;
		const FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT = FormatFeatureFlagBits::FormatFeatureDepthStencilAttachmentBit as Flags;
		const FORMAT_FEATURE_BLIT_SRC_BIT = FormatFeatureFlagBits::FormatFeatureBlitSrcBit as Flags;
		const FORMAT_FEATURE_BLIT_DST_BIT = FormatFeatureFlagBits::FormatFeatureBlitDstBit as Flags;
		const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT = FormatFeatureFlagBits::FormatFeatureSampledImageFilterLinearBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PrivateDataSlotCreateFlags: Flags {
	}
}

#[repr(u64)]
pub enum PipelineCreateFlagBits2KHR {
	PipelineCreate2DisableOptimizationBitKhr = 1 << 0,
	PipelineCreate2AllowDerivativesBitKhr = 1 << 1,
	PipelineCreate2DerivativeBitKhr = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineCreateFlags2KHR: Flags64 {
		const PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT_KHR = PipelineCreateFlagBits2KHR::PipelineCreate2DisableOptimizationBitKhr as Flags64;
		const PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT_KHR = PipelineCreateFlagBits2KHR::PipelineCreate2AllowDerivativesBitKhr as Flags64;
		const PIPELINE_CREATE_2_DERIVATIVE_BIT_KHR = PipelineCreateFlagBits2KHR::PipelineCreate2DerivativeBitKhr as Flags64;
	}
}

#[repr(u32)]
pub enum SwapchainImageUsageFlagBitsANDROID {
	SwapchainImageUsageSharedBitAndroid = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct SwapchainImageUsageFlagsANDROID: Flags {
		const SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_ANDROID = SwapchainImageUsageFlagBitsANDROID::SwapchainImageUsageSharedBitAndroid as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoBeginCodingFlagsKHR: Flags {
	}
}

#[repr(u32)]
pub enum QueryControlFlagBits {
	QueryControlPreciseBit = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct QueryControlFlags: Flags {
		const QUERY_CONTROL_PRECISE_BIT = QueryControlFlagBits::QueryControlPreciseBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct Win32SurfaceCreateFlagsKHR: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineCompilerControlFlagsAMD: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct MetalSurfaceCreateFlagsEXT: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct ViSurfaceCreateFlagsNN: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct HeadlessSurfaceCreateFlagsEXT: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct RenderPassCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum SparseImageFormatFlagBits {
	SparseImageFormatSingleMiptailBit = 1 << 0,
	SparseImageFormatAlignedMipSizeBit = 1 << 1,
	SparseImageFormatNonstandardBlockSizeBit = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct SparseImageFormatFlags: Flags {
		const SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT = SparseImageFormatFlagBits::SparseImageFormatSingleMiptailBit as Flags;
		const SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT = SparseImageFormatFlagBits::SparseImageFormatAlignedMipSizeBit as Flags;
		const SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT = SparseImageFormatFlagBits::SparseImageFormatNonstandardBlockSizeBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct SemaphoreCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum AttachmentDescriptionFlagBits {
	AttachmentDescriptionMayAliasBit = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct AttachmentDescriptionFlags: Flags {
		const ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = AttachmentDescriptionFlagBits::AttachmentDescriptionMayAliasBit as Flags;
	}
}

#[repr(u32)]
pub enum MemoryAllocateFlagBits {
	MemoryAllocateDeviceMaskBit = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct MemoryAllocateFlags: Flags {
		const MEMORY_ALLOCATE_DEVICE_MASK_BIT = MemoryAllocateFlagBits::MemoryAllocateDeviceMaskBit as Flags;
	}
}

#[repr(u32)]
pub enum DebugUtilsMessageSeverityFlagBitsEXT {
	DebugUtilsMessageSeverityVerboseBitExt = 1 << 0,
	DebugUtilsMessageSeverityInfoBitExt = 1 << 4,
	DebugUtilsMessageSeverityWarningBitExt = 1 << 8,
	DebugUtilsMessageSeverityErrorBitExt = 1 << 12,
}

bitflags! {
	#[repr(transparent)]
	pub struct DebugUtilsMessageSeverityFlagsEXT: Flags {
		const DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT = DebugUtilsMessageSeverityFlagBitsEXT::DebugUtilsMessageSeverityVerboseBitExt as Flags;
		const DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT = DebugUtilsMessageSeverityFlagBitsEXT::DebugUtilsMessageSeverityInfoBitExt as Flags;
		const DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT = DebugUtilsMessageSeverityFlagBitsEXT::DebugUtilsMessageSeverityWarningBitExt as Flags;
		const DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT = DebugUtilsMessageSeverityFlagBitsEXT::DebugUtilsMessageSeverityErrorBitExt as Flags;
	}
}

#[repr(u32)]
pub enum ResolveModeFlagBits {
	ResolveModeSampleZeroBit = 1 << 0,
	ResolveModeAverageBit = 1 << 1,
	ResolveModeMinBit = 1 << 2,
	ResolveModeMaxBit = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct ResolveModeFlags: Flags {
		const RESOLVE_MODE_SAMPLE_ZERO_BIT = ResolveModeFlagBits::ResolveModeSampleZeroBit as Flags;
		const RESOLVE_MODE_AVERAGE_BIT = ResolveModeFlagBits::ResolveModeAverageBit as Flags;
		const RESOLVE_MODE_MIN_BIT = ResolveModeFlagBits::ResolveModeMinBit as Flags;
		const RESOLVE_MODE_MAX_BIT = ResolveModeFlagBits::ResolveModeMaxBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeRateControlFlagsKHR: Flags {
	}
}

#[repr(u32)]
pub enum QueueFlagBits {
	QueueGraphicsBit = 1 << 0,
	QueueComputeBit = 1 << 1,
	QueueTransferBit = 1 << 2,
	QueueSparseBindingBit = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct QueueFlags: Flags {
		const QUEUE_GRAPHICS_BIT = QueueFlagBits::QueueGraphicsBit as Flags;
		const QUEUE_COMPUTE_BIT = QueueFlagBits::QueueComputeBit as Flags;
		const QUEUE_TRANSFER_BIT = QueueFlagBits::QueueTransferBit as Flags;
		const QUEUE_SPARSE_BINDING_BIT = QueueFlagBits::QueueSparseBindingBit as Flags;
	}
}

#[repr(u32)]
pub enum SurfaceCounterFlagBitsEXT {
	SurfaceCounterVblankBitExt = 1 << 0,
}

impl SurfaceCounterFlagBitsEXT {
	const SurfaceCounterVblankExt: u32 = Self::SurfaceCounterVblankExt;
}

bitflags! {
	#[repr(transparent)]
	pub struct SurfaceCounterFlagsEXT: Flags {
		const SURFACE_COUNTER_VBLANK_BIT_EXT = SurfaceCounterFlagBitsEXT::SurfaceCounterVblankBitExt as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineCoverageToColorStateCreateFlagsNV: Flags {
	}
}

#[repr(u32)]
pub enum DescriptorPoolCreateFlagBits {
	DescriptorPoolCreateFreeDescriptorSetBit = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct DescriptorPoolCreateFlags: Flags {
		const DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = DescriptorPoolCreateFlagBits::DescriptorPoolCreateFreeDescriptorSetBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct ScreenSurfaceCreateFlagsQNX: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineInputAssemblyStateCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum GeometryFlagBitsKHR {
	GeometryOpaqueBitKhr = 1 << 0,
	GeometryNoDuplicateAnyHitInvocationBitKhr = 1 << 1,
}

bitflags! {
	#[repr(transparent)]
	pub struct GeometryFlagsKHR: Flags {
		const GEOMETRY_OPAQUE_BIT_KHR = GeometryFlagBitsKHR::GeometryOpaqueBitKhr as Flags;
		const GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR = GeometryFlagBitsKHR::GeometryNoDuplicateAnyHitInvocationBitKhr as Flags;
	}
}

#[repr(u64)]
pub enum PipelineStageFlagBits2 {
	PipelineStage2TopOfPipeBit = 1 << 0,
	PipelineStage2DrawIndirectBit = 1 << 1,
	PipelineStage2VertexInputBit = 1 << 2,
	PipelineStage2VertexShaderBit = 1 << 3,
	PipelineStage2TessellationControlShaderBit = 1 << 4,
	PipelineStage2TessellationEvaluationShaderBit = 1 << 5,
	PipelineStage2GeometryShaderBit = 1 << 6,
	PipelineStage2FragmentShaderBit = 1 << 7,
	PipelineStage2EarlyFragmentTestsBit = 1 << 8,
	PipelineStage2LateFragmentTestsBit = 1 << 9,
	PipelineStage2ColorAttachmentOutputBit = 1 << 10,
	PipelineStage2ComputeShaderBit = 1 << 11,
	PipelineStage2AllTransferBit = 1 << 12,
	PipelineStage2BottomOfPipeBit = 1 << 13,
	PipelineStage2HostBit = 1 << 14,
	PipelineStage2AllGraphicsBit = 1 << 15,
	PipelineStage2AllCommandsBit = 1 << 16,
	PipelineStage2CopyBit = 1 << 32,
	PipelineStage2ResolveBit = 1 << 33,
	PipelineStage2BlitBit = 1 << 34,
	PipelineStage2ClearBit = 1 << 35,
	PipelineStage2IndexInputBit = 1 << 36,
	PipelineStage2VertexAttributeInputBit = 1 << 37,
	PipelineStage2PreRasterizationShadersBit = 1 << 38,
}

impl PipelineStageFlagBits2 {
	const PipelineStage2NoneKhr: u64 = Self::PipelineStage2NoneKhr;
	const PipelineStage2TopOfPipeBitKhr: u64 = Self::PipelineStage2TopOfPipeBitKhr;
	const PipelineStage2DrawIndirectBitKhr: u64 = Self::PipelineStage2DrawIndirectBitKhr;
	const PipelineStage2VertexInputBitKhr: u64 = Self::PipelineStage2VertexInputBitKhr;
	const PipelineStage2VertexShaderBitKhr: u64 = Self::PipelineStage2VertexShaderBitKhr;
	const PipelineStage2TessellationControlShaderBitKhr: u64 = Self::PipelineStage2TessellationControlShaderBitKhr;
	const PipelineStage2TessellationEvaluationShaderBitKhr: u64 = Self::PipelineStage2TessellationEvaluationShaderBitKhr;
	const PipelineStage2GeometryShaderBitKhr: u64 = Self::PipelineStage2GeometryShaderBitKhr;
	const PipelineStage2FragmentShaderBitKhr: u64 = Self::PipelineStage2FragmentShaderBitKhr;
	const PipelineStage2EarlyFragmentTestsBitKhr: u64 = Self::PipelineStage2EarlyFragmentTestsBitKhr;
	const PipelineStage2LateFragmentTestsBitKhr: u64 = Self::PipelineStage2LateFragmentTestsBitKhr;
	const PipelineStage2ColorAttachmentOutputBitKhr: u64 = Self::PipelineStage2ColorAttachmentOutputBitKhr;
	const PipelineStage2ComputeShaderBitKhr: u64 = Self::PipelineStage2ComputeShaderBitKhr;
	const PipelineStage2AllTransferBitKhr: u64 = Self::PipelineStage2AllTransferBitKhr;
	const PipelineStage2TransferBit: u64 = Self::PipelineStage2TransferBit;
	const PipelineStage2TransferBitKhr: u64 = Self::PipelineStage2TransferBitKhr;
	const PipelineStage2BottomOfPipeBitKhr: u64 = Self::PipelineStage2BottomOfPipeBitKhr;
	const PipelineStage2HostBitKhr: u64 = Self::PipelineStage2HostBitKhr;
	const PipelineStage2AllGraphicsBitKhr: u64 = Self::PipelineStage2AllGraphicsBitKhr;
	const PipelineStage2AllCommandsBitKhr: u64 = Self::PipelineStage2AllCommandsBitKhr;
	const PipelineStage2CopyBitKhr: u64 = Self::PipelineStage2CopyBitKhr;
	const PipelineStage2ResolveBitKhr: u64 = Self::PipelineStage2ResolveBitKhr;
	const PipelineStage2BlitBitKhr: u64 = Self::PipelineStage2BlitBitKhr;
	const PipelineStage2ClearBitKhr: u64 = Self::PipelineStage2ClearBitKhr;
	const PipelineStage2IndexInputBitKhr: u64 = Self::PipelineStage2IndexInputBitKhr;
	const PipelineStage2VertexAttributeInputBitKhr: u64 = Self::PipelineStage2VertexAttributeInputBitKhr;
	const PipelineStage2PreRasterizationShadersBitKhr: u64 = Self::PipelineStage2PreRasterizationShadersBitKhr;
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineStageFlags2: Flags64 {
		const PIPELINE_STAGE_2_TOP_OF_PIPE_BIT = PipelineStageFlagBits2::PipelineStage2TopOfPipeBit as Flags64;
		const PIPELINE_STAGE_2_DRAW_INDIRECT_BIT = PipelineStageFlagBits2::PipelineStage2DrawIndirectBit as Flags64;
		const PIPELINE_STAGE_2_VERTEX_INPUT_BIT = PipelineStageFlagBits2::PipelineStage2VertexInputBit as Flags64;
		const PIPELINE_STAGE_2_VERTEX_SHADER_BIT = PipelineStageFlagBits2::PipelineStage2VertexShaderBit as Flags64;
		const PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT = PipelineStageFlagBits2::PipelineStage2TessellationControlShaderBit as Flags64;
		const PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT = PipelineStageFlagBits2::PipelineStage2TessellationEvaluationShaderBit as Flags64;
		const PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT = PipelineStageFlagBits2::PipelineStage2GeometryShaderBit as Flags64;
		const PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT = PipelineStageFlagBits2::PipelineStage2FragmentShaderBit as Flags64;
		const PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT = PipelineStageFlagBits2::PipelineStage2EarlyFragmentTestsBit as Flags64;
		const PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT = PipelineStageFlagBits2::PipelineStage2LateFragmentTestsBit as Flags64;
		const PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT = PipelineStageFlagBits2::PipelineStage2ColorAttachmentOutputBit as Flags64;
		const PIPELINE_STAGE_2_COMPUTE_SHADER_BIT = PipelineStageFlagBits2::PipelineStage2ComputeShaderBit as Flags64;
		const PIPELINE_STAGE_2_ALL_TRANSFER_BIT = PipelineStageFlagBits2::PipelineStage2AllTransferBit as Flags64;
		const PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT = PipelineStageFlagBits2::PipelineStage2BottomOfPipeBit as Flags64;
		const PIPELINE_STAGE_2_HOST_BIT = PipelineStageFlagBits2::PipelineStage2HostBit as Flags64;
		const PIPELINE_STAGE_2_ALL_GRAPHICS_BIT = PipelineStageFlagBits2::PipelineStage2AllGraphicsBit as Flags64;
		const PIPELINE_STAGE_2_ALL_COMMANDS_BIT = PipelineStageFlagBits2::PipelineStage2AllCommandsBit as Flags64;
		const PIPELINE_STAGE_2_COPY_BIT = PipelineStageFlagBits2::PipelineStage2CopyBit as Flags64;
		const PIPELINE_STAGE_2_RESOLVE_BIT = PipelineStageFlagBits2::PipelineStage2ResolveBit as Flags64;
		const PIPELINE_STAGE_2_BLIT_BIT = PipelineStageFlagBits2::PipelineStage2BlitBit as Flags64;
		const PIPELINE_STAGE_2_CLEAR_BIT = PipelineStageFlagBits2::PipelineStage2ClearBit as Flags64;
		const PIPELINE_STAGE_2_INDEX_INPUT_BIT = PipelineStageFlagBits2::PipelineStage2IndexInputBit as Flags64;
		const PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT = PipelineStageFlagBits2::PipelineStage2VertexAttributeInputBit as Flags64;
		const PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT = PipelineStageFlagBits2::PipelineStage2PreRasterizationShadersBit as Flags64;
	}
}

#[repr(u64)]
pub enum PhysicalDeviceSchedulingControlsFlagBitsARM {
	PhysicalDeviceSchedulingControlsShaderCoreCountArm = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct PhysicalDeviceSchedulingControlsFlagsARM: Flags64 {
		const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_SHADER_CORE_COUNT_ARM = PhysicalDeviceSchedulingControlsFlagBitsARM::PhysicalDeviceSchedulingControlsShaderCoreCountArm as Flags64;
	}
}

#[repr(u32)]
pub enum PipelineStageFlagBits {
	PipelineStageTopOfPipeBit = 1 << 0,
	PipelineStageDrawIndirectBit = 1 << 1,
	PipelineStageVertexInputBit = 1 << 2,
	PipelineStageVertexShaderBit = 1 << 3,
	PipelineStageTessellationControlShaderBit = 1 << 4,
	PipelineStageTessellationEvaluationShaderBit = 1 << 5,
	PipelineStageGeometryShaderBit = 1 << 6,
	PipelineStageFragmentShaderBit = 1 << 7,
	PipelineStageEarlyFragmentTestsBit = 1 << 8,
	PipelineStageLateFragmentTestsBit = 1 << 9,
	PipelineStageColorAttachmentOutputBit = 1 << 10,
	PipelineStageComputeShaderBit = 1 << 11,
	PipelineStageTransferBit = 1 << 12,
	PipelineStageBottomOfPipeBit = 1 << 13,
	PipelineStageHostBit = 1 << 14,
	PipelineStageAllGraphicsBit = 1 << 15,
	PipelineStageAllCommandsBit = 1 << 16,
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineStageFlags: Flags {
		const PIPELINE_STAGE_TOP_OF_PIPE_BIT = PipelineStageFlagBits::PipelineStageTopOfPipeBit as Flags;
		const PIPELINE_STAGE_DRAW_INDIRECT_BIT = PipelineStageFlagBits::PipelineStageDrawIndirectBit as Flags;
		const PIPELINE_STAGE_VERTEX_INPUT_BIT = PipelineStageFlagBits::PipelineStageVertexInputBit as Flags;
		const PIPELINE_STAGE_VERTEX_SHADER_BIT = PipelineStageFlagBits::PipelineStageVertexShaderBit as Flags;
		const PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT = PipelineStageFlagBits::PipelineStageTessellationControlShaderBit as Flags;
		const PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT = PipelineStageFlagBits::PipelineStageTessellationEvaluationShaderBit as Flags;
		const PIPELINE_STAGE_GEOMETRY_SHADER_BIT = PipelineStageFlagBits::PipelineStageGeometryShaderBit as Flags;
		const PIPELINE_STAGE_FRAGMENT_SHADER_BIT = PipelineStageFlagBits::PipelineStageFragmentShaderBit as Flags;
		const PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT = PipelineStageFlagBits::PipelineStageEarlyFragmentTestsBit as Flags;
		const PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT = PipelineStageFlagBits::PipelineStageLateFragmentTestsBit as Flags;
		const PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT = PipelineStageFlagBits::PipelineStageColorAttachmentOutputBit as Flags;
		const PIPELINE_STAGE_COMPUTE_SHADER_BIT = PipelineStageFlagBits::PipelineStageComputeShaderBit as Flags;
		const PIPELINE_STAGE_TRANSFER_BIT = PipelineStageFlagBits::PipelineStageTransferBit as Flags;
		const PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT = PipelineStageFlagBits::PipelineStageBottomOfPipeBit as Flags;
		const PIPELINE_STAGE_HOST_BIT = PipelineStageFlagBits::PipelineStageHostBit as Flags;
		const PIPELINE_STAGE_ALL_GRAPHICS_BIT = PipelineStageFlagBits::PipelineStageAllGraphicsBit as Flags;
		const PIPELINE_STAGE_ALL_COMMANDS_BIT = PipelineStageFlagBits::PipelineStageAllCommandsBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct XcbSurfaceCreateFlagsKHR: Flags {
	}
}

#[repr(u32)]
pub enum VideoCodingControlFlagBitsKHR {
	VideoCodingControlResetBitKhr = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoCodingControlFlagsKHR: Flags {
		const VIDEO_CODING_CONTROL_RESET_BIT_KHR = VideoCodingControlFlagBitsKHR::VideoCodingControlResetBitKhr as Flags;
	}
}

#[repr(u32)]
pub enum CompositeAlphaFlagBitsKHR {
	CompositeAlphaOpaqueBitKhr = 1 << 0,
	CompositeAlphaPreMultipliedBitKhr = 1 << 1,
	CompositeAlphaPostMultipliedBitKhr = 1 << 2,
	CompositeAlphaInheritBitKhr = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct CompositeAlphaFlagsKHR: Flags {
		const COMPOSITE_ALPHA_OPAQUE_BIT_KHR = CompositeAlphaFlagBitsKHR::CompositeAlphaOpaqueBitKhr as Flags;
		const COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = CompositeAlphaFlagBitsKHR::CompositeAlphaPreMultipliedBitKhr as Flags;
		const COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = CompositeAlphaFlagBitsKHR::CompositeAlphaPostMultipliedBitKhr as Flags;
		const COMPOSITE_ALPHA_INHERIT_BIT_KHR = CompositeAlphaFlagBitsKHR::CompositeAlphaInheritBitKhr as Flags;
	}
}

#[repr(u32)]
pub enum ExternalSemaphoreFeatureFlagBits {
	ExternalSemaphoreFeatureExportableBit = 1 << 0,
	ExternalSemaphoreFeatureImportableBit = 1 << 1,
}

bitflags! {
	#[repr(transparent)]
	pub struct ExternalSemaphoreFeatureFlags: Flags {
		const EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT = ExternalSemaphoreFeatureFlagBits::ExternalSemaphoreFeatureExportableBit as Flags;
		const EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT = ExternalSemaphoreFeatureFlagBits::ExternalSemaphoreFeatureImportableBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct DirectDriverLoadingFlagsLUNARG: Flags {
	}
}

#[repr(u32)]
pub enum VideoEncodeH265TransformBlockSizeFlagBitsEXT {
	VideoEncodeH265TransformBlockSize4BitExt = 1 << 0,
	VideoEncodeH265TransformBlockSize8BitExt = 1 << 1,
	VideoEncodeH265TransformBlockSize16BitExt = 1 << 2,
	VideoEncodeH265TransformBlockSize32BitExt = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeH265TransformBlockSizeFlagsEXT: Flags {
		const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_BIT_EXT = VideoEncodeH265TransformBlockSizeFlagBitsEXT::VideoEncodeH265TransformBlockSize4BitExt as Flags;
		const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_BIT_EXT = VideoEncodeH265TransformBlockSizeFlagBitsEXT::VideoEncodeH265TransformBlockSize8BitExt as Flags;
		const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_BIT_EXT = VideoEncodeH265TransformBlockSizeFlagBitsEXT::VideoEncodeH265TransformBlockSize16BitExt as Flags;
		const VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_BIT_EXT = VideoEncodeH265TransformBlockSizeFlagBitsEXT::VideoEncodeH265TransformBlockSize32BitExt as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineDepthStencilStateCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum VideoEncodeH264StdFlagBitsEXT {
	VideoEncodeH264StdSeparateColorPlaneFlagSetBitExt = 1 << 0,
	VideoEncodeH264StdQpprimeYZeroTransformBypassFlagSetBitExt = 1 << 1,
	VideoEncodeH264StdScalingMatrixPresentFlagSetBitExt = 1 << 2,
	VideoEncodeH264StdChromaQpIndexOffsetBitExt = 1 << 3,
	VideoEncodeH264StdSecondChromaQpIndexOffsetBitExt = 1 << 4,
	VideoEncodeH264StdPicInitQpMinus26BitExt = 1 << 5,
	VideoEncodeH264StdWeightedPredFlagSetBitExt = 1 << 6,
	VideoEncodeH264StdWeightedBipredIdcExplicitBitExt = 1 << 7,
	VideoEncodeH264StdWeightedBipredIdcImplicitBitExt = 1 << 8,
	VideoEncodeH264StdTransform8x8ModeFlagSetBitExt = 1 << 9,
	VideoEncodeH264StdDirectSpatialMvPredFlagUnsetBitExt = 1 << 10,
	VideoEncodeH264StdEntropyCodingModeFlagUnsetBitExt = 1 << 11,
	VideoEncodeH264StdEntropyCodingModeFlagSetBitExt = 1 << 12,
	VideoEncodeH264StdDirect8x8InferenceFlagUnsetBitExt = 1 << 13,
	VideoEncodeH264StdConstrainedIntraPredFlagSetBitExt = 1 << 14,
	VideoEncodeH264StdDeblockingFilterDisabledBitExt = 1 << 15,
	VideoEncodeH264StdDeblockingFilterEnabledBitExt = 1 << 16,
	VideoEncodeH264StdDeblockingFilterPartialBitExt = 1 << 17,
	VideoEncodeH264StdSliceQpDeltaBitExt = 1 << 19,
	VideoEncodeH264StdDifferentSliceQpDeltaBitExt = 1 << 20,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeH264StdFlagsEXT: Flags {
		const VIDEO_ENCODE_H264_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdSeparateColorPlaneFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdQpprimeYZeroTransformBypassFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_SCALING_MATRIX_PRESENT_FLAG_SET_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdScalingMatrixPresentFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_CHROMA_QP_INDEX_OFFSET_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdChromaQpIndexOffsetBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_SECOND_CHROMA_QP_INDEX_OFFSET_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdSecondChromaQpIndexOffsetBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_PIC_INIT_QP_MINUS26_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdPicInitQpMinus26BitExt as Flags;
		const VIDEO_ENCODE_H264_STD_WEIGHTED_PRED_FLAG_SET_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdWeightedPredFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_EXPLICIT_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdWeightedBipredIdcExplicitBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_IMPLICIT_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdWeightedBipredIdcImplicitBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_TRANSFORM_8X8_MODE_FLAG_SET_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdTransform8x8ModeFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdDirectSpatialMvPredFlagUnsetBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_UNSET_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdEntropyCodingModeFlagUnsetBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_SET_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdEntropyCodingModeFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_DIRECT_8X8_INFERENCE_FLAG_UNSET_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdDirect8x8InferenceFlagUnsetBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdConstrainedIntraPredFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_DISABLED_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdDeblockingFilterDisabledBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_ENABLED_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdDeblockingFilterEnabledBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_PARTIAL_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdDeblockingFilterPartialBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_SLICE_QP_DELTA_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdSliceQpDeltaBitExt as Flags;
		const VIDEO_ENCODE_H264_STD_DIFFERENT_SLICE_QP_DELTA_BIT_EXT = VideoEncodeH264StdFlagBitsEXT::VideoEncodeH264StdDifferentSliceQpDeltaBitExt as Flags;
	}
}

#[repr(u32)]
pub enum SemaphoreImportFlagBits {
	SemaphoreImportTemporaryBit = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct SemaphoreImportFlags: Flags {
		const SEMAPHORE_IMPORT_TEMPORARY_BIT = SemaphoreImportFlagBits::SemaphoreImportTemporaryBit as Flags;
	}
}

#[repr(u32)]
pub enum HostImageCopyFlagBitsEXT {
	HostImageCopyMemcpyExt = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct HostImageCopyFlagsEXT: Flags {
		const HOST_IMAGE_COPY_MEMCPY_EXT = HostImageCopyFlagBitsEXT::HostImageCopyMemcpyExt as Flags;
	}
}

#[repr(u64)]
pub enum FormatFeatureFlagBits2 {
	FormatFeature2SampledImageBit = 1 << 0,
	FormatFeature2StorageImageBit = 1 << 1,
	FormatFeature2StorageImageAtomicBit = 1 << 2,
	FormatFeature2UniformTexelBufferBit = 1 << 3,
	FormatFeature2StorageTexelBufferBit = 1 << 4,
	FormatFeature2StorageTexelBufferAtomicBit = 1 << 5,
	FormatFeature2VertexBufferBit = 1 << 6,
	FormatFeature2ColorAttachmentBit = 1 << 7,
	FormatFeature2ColorAttachmentBlendBit = 1 << 8,
	FormatFeature2DepthStencilAttachmentBit = 1 << 9,
	FormatFeature2BlitSrcBit = 1 << 10,
	FormatFeature2BlitDstBit = 1 << 11,
	FormatFeature2SampledImageFilterLinearBit = 1 << 12,
	FormatFeature2SampledImageFilterCubicBit = 1 << 13,
	FormatFeature2TransferSrcBit = 1 << 14,
	FormatFeature2TransferDstBit = 1 << 15,
	FormatFeature2SampledImageFilterMinmaxBit = 1 << 16,
	FormatFeature2MidpointChromaSamplesBit = 1 << 17,
	FormatFeature2SampledImageYcbcrConversionLinearFilterBit = 1 << 18,
	FormatFeature2SampledImageYcbcrConversionSeparateReconstructionFilterBit = 1 << 19,
	FormatFeature2SampledImageYcbcrConversionChromaReconstructionExplicitBit = 1 << 20,
	FormatFeature2SampledImageYcbcrConversionChromaReconstructionExplicitForceableBit = 1 << 21,
	FormatFeature2DisjointBit = 1 << 22,
	FormatFeature2CositedChromaSamplesBit = 1 << 23,
	FormatFeature2StorageReadWithoutFormatBit = 1 << 31,
	FormatFeature2StorageWriteWithoutFormatBit = 1 << 32,
	FormatFeature2SampledImageDepthComparisonBit = 1 << 33,
}

impl FormatFeatureFlagBits2 {
	const FormatFeature2SampledImageBitKhr: u64 = Self::FormatFeature2SampledImageBitKhr;
	const FormatFeature2StorageImageBitKhr: u64 = Self::FormatFeature2StorageImageBitKhr;
	const FormatFeature2StorageImageAtomicBitKhr: u64 = Self::FormatFeature2StorageImageAtomicBitKhr;
	const FormatFeature2UniformTexelBufferBitKhr: u64 = Self::FormatFeature2UniformTexelBufferBitKhr;
	const FormatFeature2StorageTexelBufferBitKhr: u64 = Self::FormatFeature2StorageTexelBufferBitKhr;
	const FormatFeature2StorageTexelBufferAtomicBitKhr: u64 = Self::FormatFeature2StorageTexelBufferAtomicBitKhr;
	const FormatFeature2VertexBufferBitKhr: u64 = Self::FormatFeature2VertexBufferBitKhr;
	const FormatFeature2ColorAttachmentBitKhr: u64 = Self::FormatFeature2ColorAttachmentBitKhr;
	const FormatFeature2ColorAttachmentBlendBitKhr: u64 = Self::FormatFeature2ColorAttachmentBlendBitKhr;
	const FormatFeature2DepthStencilAttachmentBitKhr: u64 = Self::FormatFeature2DepthStencilAttachmentBitKhr;
	const FormatFeature2BlitSrcBitKhr: u64 = Self::FormatFeature2BlitSrcBitKhr;
	const FormatFeature2BlitDstBitKhr: u64 = Self::FormatFeature2BlitDstBitKhr;
	const FormatFeature2SampledImageFilterLinearBitKhr: u64 = Self::FormatFeature2SampledImageFilterLinearBitKhr;
	const FormatFeature2SampledImageFilterCubicBitExt: u64 = Self::FormatFeature2SampledImageFilterCubicBitExt;
	const FormatFeature2TransferSrcBitKhr: u64 = Self::FormatFeature2TransferSrcBitKhr;
	const FormatFeature2TransferDstBitKhr: u64 = Self::FormatFeature2TransferDstBitKhr;
	const FormatFeature2SampledImageFilterMinmaxBitKhr: u64 = Self::FormatFeature2SampledImageFilterMinmaxBitKhr;
	const FormatFeature2MidpointChromaSamplesBitKhr: u64 = Self::FormatFeature2MidpointChromaSamplesBitKhr;
	const FormatFeature2SampledImageYcbcrConversionLinearFilterBitKhr: u64 = Self::FormatFeature2SampledImageYcbcrConversionLinearFilterBitKhr;
	const FormatFeature2SampledImageYcbcrConversionSeparateReconstructionFilterBitKhr: u64 = Self::FormatFeature2SampledImageYcbcrConversionSeparateReconstructionFilterBitKhr;
	const FormatFeature2SampledImageYcbcrConversionChromaReconstructionExplicitBitKhr: u64 = Self::FormatFeature2SampledImageYcbcrConversionChromaReconstructionExplicitBitKhr;
	const FormatFeature2SampledImageYcbcrConversionChromaReconstructionExplicitForceableBitKhr: u64 = Self::FormatFeature2SampledImageYcbcrConversionChromaReconstructionExplicitForceableBitKhr;
	const FormatFeature2DisjointBitKhr: u64 = Self::FormatFeature2DisjointBitKhr;
	const FormatFeature2CositedChromaSamplesBitKhr: u64 = Self::FormatFeature2CositedChromaSamplesBitKhr;
	const FormatFeature2StorageReadWithoutFormatBitKhr: u64 = Self::FormatFeature2StorageReadWithoutFormatBitKhr;
	const FormatFeature2StorageWriteWithoutFormatBitKhr: u64 = Self::FormatFeature2StorageWriteWithoutFormatBitKhr;
	const FormatFeature2SampledImageDepthComparisonBitKhr: u64 = Self::FormatFeature2SampledImageDepthComparisonBitKhr;
}

bitflags! {
	#[repr(transparent)]
	pub struct FormatFeatureFlags2: Flags64 {
		const FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT = FormatFeatureFlagBits2::FormatFeature2SampledImageBit as Flags64;
		const FORMAT_FEATURE_2_STORAGE_IMAGE_BIT = FormatFeatureFlagBits2::FormatFeature2StorageImageBit as Flags64;
		const FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT = FormatFeatureFlagBits2::FormatFeature2StorageImageAtomicBit as Flags64;
		const FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT = FormatFeatureFlagBits2::FormatFeature2UniformTexelBufferBit as Flags64;
		const FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT = FormatFeatureFlagBits2::FormatFeature2StorageTexelBufferBit as Flags64;
		const FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = FormatFeatureFlagBits2::FormatFeature2StorageTexelBufferAtomicBit as Flags64;
		const FORMAT_FEATURE_2_VERTEX_BUFFER_BIT = FormatFeatureFlagBits2::FormatFeature2VertexBufferBit as Flags64;
		const FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT = FormatFeatureFlagBits2::FormatFeature2ColorAttachmentBit as Flags64;
		const FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT = FormatFeatureFlagBits2::FormatFeature2ColorAttachmentBlendBit as Flags64;
		const FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT = FormatFeatureFlagBits2::FormatFeature2DepthStencilAttachmentBit as Flags64;
		const FORMAT_FEATURE_2_BLIT_SRC_BIT = FormatFeatureFlagBits2::FormatFeature2BlitSrcBit as Flags64;
		const FORMAT_FEATURE_2_BLIT_DST_BIT = FormatFeatureFlagBits2::FormatFeature2BlitDstBit as Flags64;
		const FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT = FormatFeatureFlagBits2::FormatFeature2SampledImageFilterLinearBit as Flags64;
		const FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT = FormatFeatureFlagBits2::FormatFeature2SampledImageFilterCubicBit as Flags64;
		const FORMAT_FEATURE_2_TRANSFER_SRC_BIT = FormatFeatureFlagBits2::FormatFeature2TransferSrcBit as Flags64;
		const FORMAT_FEATURE_2_TRANSFER_DST_BIT = FormatFeatureFlagBits2::FormatFeature2TransferDstBit as Flags64;
		const FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT = FormatFeatureFlagBits2::FormatFeature2SampledImageFilterMinmaxBit as Flags64;
		const FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT = FormatFeatureFlagBits2::FormatFeature2MidpointChromaSamplesBit as Flags64;
		const FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT = FormatFeatureFlagBits2::FormatFeature2SampledImageYcbcrConversionLinearFilterBit as Flags64;
		const FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT = FormatFeatureFlagBits2::FormatFeature2SampledImageYcbcrConversionSeparateReconstructionFilterBit as Flags64;
		const FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT = FormatFeatureFlagBits2::FormatFeature2SampledImageYcbcrConversionChromaReconstructionExplicitBit as Flags64;
		const FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT = FormatFeatureFlagBits2::FormatFeature2SampledImageYcbcrConversionChromaReconstructionExplicitForceableBit as Flags64;
		const FORMAT_FEATURE_2_DISJOINT_BIT = FormatFeatureFlagBits2::FormatFeature2DisjointBit as Flags64;
		const FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT = FormatFeatureFlagBits2::FormatFeature2CositedChromaSamplesBit as Flags64;
		const FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT = FormatFeatureFlagBits2::FormatFeature2StorageReadWithoutFormatBit as Flags64;
		const FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT = FormatFeatureFlagBits2::FormatFeature2StorageWriteWithoutFormatBit as Flags64;
		const FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT = FormatFeatureFlagBits2::FormatFeature2SampledImageDepthComparisonBit as Flags64;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct FramebufferCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum PipelineCreateFlagBits {
	PipelineCreateDisableOptimizationBit = 1 << 0,
	PipelineCreateAllowDerivativesBit = 1 << 1,
	PipelineCreateDerivativeBit = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineCreateFlags: Flags {
		const PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = PipelineCreateFlagBits::PipelineCreateDisableOptimizationBit as Flags;
		const PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = PipelineCreateFlagBits::PipelineCreateAllowDerivativesBit as Flags;
		const PIPELINE_CREATE_DERIVATIVE_BIT = PipelineCreateFlagBits::PipelineCreateDerivativeBit as Flags;
	}
}

#[repr(u32)]
pub enum ColorComponentFlagBits {
	ColorComponentRBit = 1 << 0,
	ColorComponentGBit = 1 << 1,
	ColorComponentBBit = 1 << 2,
	ColorComponentABit = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct ColorComponentFlags: Flags {
		const COLOR_COMPONENT_R_BIT = ColorComponentFlagBits::ColorComponentRBit as Flags;
		const COLOR_COMPONENT_G_BIT = ColorComponentFlagBits::ColorComponentGBit as Flags;
		const COLOR_COMPONENT_B_BIT = ColorComponentFlagBits::ColorComponentBBit as Flags;
		const COLOR_COMPONENT_A_BIT = ColorComponentFlagBits::ColorComponentABit as Flags;
	}
}

#[repr(u32)]
pub enum IndirectCommandsLayoutUsageFlagBitsNV {
	IndirectCommandsLayoutUsageExplicitPreprocessBitNv = 1 << 0,
	IndirectCommandsLayoutUsageIndexedSequencesBitNv = 1 << 1,
	IndirectCommandsLayoutUsageUnorderedSequencesBitNv = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct IndirectCommandsLayoutUsageFlagsNV: Flags {
		const INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV = IndirectCommandsLayoutUsageFlagBitsNV::IndirectCommandsLayoutUsageExplicitPreprocessBitNv as Flags;
		const INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV = IndirectCommandsLayoutUsageFlagBitsNV::IndirectCommandsLayoutUsageIndexedSequencesBitNv as Flags;
		const INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV = IndirectCommandsLayoutUsageFlagBitsNV::IndirectCommandsLayoutUsageUnorderedSequencesBitNv as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct AccelerationStructureMotionInstanceFlagsNV: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct MacOSSurfaceCreateFlagsMVK: Flags {
	}
}

#[repr(u32)]
pub enum OpticalFlowUsageFlagBitsNV {
	OpticalFlowUsageInputBitNv = 1 << 0,
	OpticalFlowUsageOutputBitNv = 1 << 1,
	OpticalFlowUsageHintBitNv = 1 << 2,
	OpticalFlowUsageCostBitNv = 1 << 3,
	OpticalFlowUsageGlobalFlowBitNv = 1 << 4,
}

bitflags! {
	#[repr(transparent)]
	pub struct OpticalFlowUsageFlagsNV: Flags {
		const OPTICAL_FLOW_USAGE_INPUT_BIT_NV = OpticalFlowUsageFlagBitsNV::OpticalFlowUsageInputBitNv as Flags;
		const OPTICAL_FLOW_USAGE_OUTPUT_BIT_NV = OpticalFlowUsageFlagBitsNV::OpticalFlowUsageOutputBitNv as Flags;
		const OPTICAL_FLOW_USAGE_HINT_BIT_NV = OpticalFlowUsageFlagBitsNV::OpticalFlowUsageHintBitNv as Flags;
		const OPTICAL_FLOW_USAGE_COST_BIT_NV = OpticalFlowUsageFlagBitsNV::OpticalFlowUsageCostBitNv as Flags;
		const OPTICAL_FLOW_USAGE_GLOBAL_FLOW_BIT_NV = OpticalFlowUsageFlagBitsNV::OpticalFlowUsageGlobalFlowBitNv as Flags;
	}
}

#[repr(u32)]
pub enum PresentScalingFlagBitsEXT {
	PresentScalingOneToOneBitExt = 1 << 0,
	PresentScalingAspectRatioStretchBitExt = 1 << 1,
	PresentScalingStretchBitExt = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct PresentScalingFlagsEXT: Flags {
		const PRESENT_SCALING_ONE_TO_ONE_BIT_EXT = PresentScalingFlagBitsEXT::PresentScalingOneToOneBitExt as Flags;
		const PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_EXT = PresentScalingFlagBitsEXT::PresentScalingAspectRatioStretchBitExt as Flags;
		const PRESENT_SCALING_STRETCH_BIT_EXT = PresentScalingFlagBitsEXT::PresentScalingStretchBitExt as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct ShaderModuleCreateFlags: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct SamplerCreateFlags: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct XlibSurfaceCreateFlagsKHR: Flags {
	}
}

#[repr(u32)]
pub enum ExternalMemoryHandleTypeFlagBits {
	ExternalMemoryHandleTypeOpaqueFdBit = 1 << 0,
	ExternalMemoryHandleTypeOpaqueWin32Bit = 1 << 1,
	ExternalMemoryHandleTypeOpaqueWin32KmtBit = 1 << 2,
	ExternalMemoryHandleTypeD3d11TextureBit = 1 << 3,
	ExternalMemoryHandleTypeD3d11TextureKmtBit = 1 << 4,
	ExternalMemoryHandleTypeD3d12HeapBit = 1 << 5,
	ExternalMemoryHandleTypeD3d12ResourceBit = 1 << 6,
}

bitflags! {
	#[repr(transparent)]
	pub struct ExternalMemoryHandleTypeFlags: Flags {
		const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT = ExternalMemoryHandleTypeFlagBits::ExternalMemoryHandleTypeOpaqueFdBit as Flags;
		const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT = ExternalMemoryHandleTypeFlagBits::ExternalMemoryHandleTypeOpaqueWin32Bit as Flags;
		const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = ExternalMemoryHandleTypeFlagBits::ExternalMemoryHandleTypeOpaqueWin32KmtBit as Flags;
		const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT = ExternalMemoryHandleTypeFlagBits::ExternalMemoryHandleTypeD3d11TextureBit as Flags;
		const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT = ExternalMemoryHandleTypeFlagBits::ExternalMemoryHandleTypeD3d11TextureKmtBit as Flags;
		const EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT = ExternalMemoryHandleTypeFlagBits::ExternalMemoryHandleTypeD3d12HeapBit as Flags;
		const EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT = ExternalMemoryHandleTypeFlagBits::ExternalMemoryHandleTypeD3d12ResourceBit as Flags;
	}
}

#[repr(u32)]
pub enum SubmitFlagBits {
	SubmitProtectedBit = 1 << 0,
}

impl SubmitFlagBits {
	const SubmitProtectedBitKhr: u32 = Self::SubmitProtectedBitKhr;
}

bitflags! {
	#[repr(transparent)]
	pub struct SubmitFlags: Flags {
		const SUBMIT_PROTECTED_BIT = SubmitFlagBits::SubmitProtectedBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct ImageViewCreateFlags: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct BufferViewCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum SubgroupFeatureFlagBits {
	SubgroupFeatureBasicBit = 1 << 0,
	SubgroupFeatureVoteBit = 1 << 1,
	SubgroupFeatureArithmeticBit = 1 << 2,
	SubgroupFeatureBallotBit = 1 << 3,
	SubgroupFeatureShuffleBit = 1 << 4,
	SubgroupFeatureShuffleRelativeBit = 1 << 5,
	SubgroupFeatureClusteredBit = 1 << 6,
	SubgroupFeatureQuadBit = 1 << 7,
}

bitflags! {
	#[repr(transparent)]
	pub struct SubgroupFeatureFlags: Flags {
		const SUBGROUP_FEATURE_BASIC_BIT = SubgroupFeatureFlagBits::SubgroupFeatureBasicBit as Flags;
		const SUBGROUP_FEATURE_VOTE_BIT = SubgroupFeatureFlagBits::SubgroupFeatureVoteBit as Flags;
		const SUBGROUP_FEATURE_ARITHMETIC_BIT = SubgroupFeatureFlagBits::SubgroupFeatureArithmeticBit as Flags;
		const SUBGROUP_FEATURE_BALLOT_BIT = SubgroupFeatureFlagBits::SubgroupFeatureBallotBit as Flags;
		const SUBGROUP_FEATURE_SHUFFLE_BIT = SubgroupFeatureFlagBits::SubgroupFeatureShuffleBit as Flags;
		const SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT = SubgroupFeatureFlagBits::SubgroupFeatureShuffleRelativeBit as Flags;
		const SUBGROUP_FEATURE_CLUSTERED_BIT = SubgroupFeatureFlagBits::SubgroupFeatureClusteredBit as Flags;
		const SUBGROUP_FEATURE_QUAD_BIT = SubgroupFeatureFlagBits::SubgroupFeatureQuadBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct DirectFBSurfaceCreateFlagsEXT: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct DeviceMemoryReportFlagsEXT: Flags {
	}
}

#[repr(u32)]
pub enum ExportMetalObjectTypeFlagBitsEXT {
	ExportMetalObjectTypeMetalDeviceBitExt = 1 << 0,
	ExportMetalObjectTypeMetalCommandQueueBitExt = 1 << 1,
	ExportMetalObjectTypeMetalBufferBitExt = 1 << 2,
	ExportMetalObjectTypeMetalTextureBitExt = 1 << 3,
	ExportMetalObjectTypeMetalIosurfaceBitExt = 1 << 4,
	ExportMetalObjectTypeMetalSharedEventBitExt = 1 << 5,
}

bitflags! {
	#[repr(transparent)]
	pub struct ExportMetalObjectTypeFlagsEXT: Flags {
		const EXPORT_METAL_OBJECT_TYPE_METAL_DEVICE_BIT_EXT = ExportMetalObjectTypeFlagBitsEXT::ExportMetalObjectTypeMetalDeviceBitExt as Flags;
		const EXPORT_METAL_OBJECT_TYPE_METAL_COMMAND_QUEUE_BIT_EXT = ExportMetalObjectTypeFlagBitsEXT::ExportMetalObjectTypeMetalCommandQueueBitExt as Flags;
		const EXPORT_METAL_OBJECT_TYPE_METAL_BUFFER_BIT_EXT = ExportMetalObjectTypeFlagBitsEXT::ExportMetalObjectTypeMetalBufferBitExt as Flags;
		const EXPORT_METAL_OBJECT_TYPE_METAL_TEXTURE_BIT_EXT = ExportMetalObjectTypeFlagBitsEXT::ExportMetalObjectTypeMetalTextureBitExt as Flags;
		const EXPORT_METAL_OBJECT_TYPE_METAL_IOSURFACE_BIT_EXT = ExportMetalObjectTypeFlagBitsEXT::ExportMetalObjectTypeMetalIosurfaceBitExt as Flags;
		const EXPORT_METAL_OBJECT_TYPE_METAL_SHARED_EVENT_BIT_EXT = ExportMetalObjectTypeFlagBitsEXT::ExportMetalObjectTypeMetalSharedEventBitExt as Flags;
	}
}

#[repr(u32)]
pub enum OpticalFlowSessionCreateFlagBitsNV {
	OpticalFlowSessionCreateEnableHintBitNv = 1 << 0,
	OpticalFlowSessionCreateEnableCostBitNv = 1 << 1,
	OpticalFlowSessionCreateEnableGlobalFlowBitNv = 1 << 2,
	OpticalFlowSessionCreateAllowRegionsBitNv = 1 << 3,
	OpticalFlowSessionCreateBothDirectionsBitNv = 1 << 4,
}

bitflags! {
	#[repr(transparent)]
	pub struct OpticalFlowSessionCreateFlagsNV: Flags {
		const OPTICAL_FLOW_SESSION_CREATE_ENABLE_HINT_BIT_NV = OpticalFlowSessionCreateFlagBitsNV::OpticalFlowSessionCreateEnableHintBitNv as Flags;
		const OPTICAL_FLOW_SESSION_CREATE_ENABLE_COST_BIT_NV = OpticalFlowSessionCreateFlagBitsNV::OpticalFlowSessionCreateEnableCostBitNv as Flags;
		const OPTICAL_FLOW_SESSION_CREATE_ENABLE_GLOBAL_FLOW_BIT_NV = OpticalFlowSessionCreateFlagBitsNV::OpticalFlowSessionCreateEnableGlobalFlowBitNv as Flags;
		const OPTICAL_FLOW_SESSION_CREATE_ALLOW_REGIONS_BIT_NV = OpticalFlowSessionCreateFlagBitsNV::OpticalFlowSessionCreateAllowRegionsBitNv as Flags;
		const OPTICAL_FLOW_SESSION_CREATE_BOTH_DIRECTIONS_BIT_NV = OpticalFlowSessionCreateFlagBitsNV::OpticalFlowSessionCreateBothDirectionsBitNv as Flags;
	}
}

#[repr(u32)]
pub enum PresentGravityFlagBitsEXT {
	PresentGravityMinBitExt = 1 << 0,
	PresentGravityMaxBitExt = 1 << 1,
	PresentGravityCenteredBitExt = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct PresentGravityFlagsEXT: Flags {
		const PRESENT_GRAVITY_MIN_BIT_EXT = PresentGravityFlagBitsEXT::PresentGravityMinBitExt as Flags;
		const PRESENT_GRAVITY_MAX_BIT_EXT = PresentGravityFlagBitsEXT::PresentGravityMaxBitExt as Flags;
		const PRESENT_GRAVITY_CENTERED_BIT_EXT = PresentGravityFlagBitsEXT::PresentGravityCenteredBitExt as Flags;
	}
}

#[repr(u32)]
pub enum ShaderCreateFlagBitsEXT {
	ShaderCreateLinkStageBitExt = 1 << 0,
}

bitflags! {
	#[repr(transparent)]
	pub struct ShaderCreateFlagsEXT: Flags {
		const SHADER_CREATE_LINK_STAGE_BIT_EXT = ShaderCreateFlagBitsEXT::ShaderCreateLinkStageBitExt as Flags;
	}
}

#[repr(u32)]
pub enum VideoComponentBitDepthFlagBitsKHR {
	VideoComponentBitDepth8BitKhr = 1 << 0,
	VideoComponentBitDepth10BitKhr = 1 << 2,
	VideoComponentBitDepth12BitKhr = 1 << 4,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoComponentBitDepthFlagsKHR: Flags {
		const VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR = VideoComponentBitDepthFlagBitsKHR::VideoComponentBitDepth8BitKhr as Flags;
		const VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR = VideoComponentBitDepthFlagBitsKHR::VideoComponentBitDepth10BitKhr as Flags;
		const VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR = VideoComponentBitDepthFlagBitsKHR::VideoComponentBitDepth12BitKhr as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct DeviceCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum ImageCreateFlagBits {
	ImageCreateSparseBindingBit = 1 << 0,
	ImageCreateSparseResidencyBit = 1 << 1,
	ImageCreateSparseAliasedBit = 1 << 2,
	ImageCreateMutableFormatBit = 1 << 3,
	ImageCreateCubeCompatibleBit = 1 << 4,
}

bitflags! {
	#[repr(transparent)]
	pub struct ImageCreateFlags: Flags {
		const IMAGE_CREATE_SPARSE_BINDING_BIT = ImageCreateFlagBits::ImageCreateSparseBindingBit as Flags;
		const IMAGE_CREATE_SPARSE_RESIDENCY_BIT = ImageCreateFlagBits::ImageCreateSparseResidencyBit as Flags;
		const IMAGE_CREATE_SPARSE_ALIASED_BIT = ImageCreateFlagBits::ImageCreateSparseAliasedBit as Flags;
		const IMAGE_CREATE_MUTABLE_FORMAT_BIT = ImageCreateFlagBits::ImageCreateMutableFormatBit as Flags;
		const IMAGE_CREATE_CUBE_COMPATIBLE_BIT = ImageCreateFlagBits::ImageCreateCubeCompatibleBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct RefreshObjectFlagsKHR: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoDecodeFlagsKHR: Flags {
	}
}

#[repr(u32)]
pub enum ExternalMemoryHandleTypeFlagBitsNV {
	ExternalMemoryHandleTypeOpaqueWin32BitNv = 1 << 0,
	ExternalMemoryHandleTypeOpaqueWin32KmtBitNv = 1 << 1,
	ExternalMemoryHandleTypeD3d11ImageBitNv = 1 << 2,
	ExternalMemoryHandleTypeD3d11ImageKmtBitNv = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct ExternalMemoryHandleTypeFlagsNV: Flags {
		const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV = ExternalMemoryHandleTypeFlagBitsNV::ExternalMemoryHandleTypeOpaqueWin32BitNv as Flags;
		const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV = ExternalMemoryHandleTypeFlagBitsNV::ExternalMemoryHandleTypeOpaqueWin32KmtBitNv as Flags;
		const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV = ExternalMemoryHandleTypeFlagBitsNV::ExternalMemoryHandleTypeD3d11ImageBitNv as Flags;
		const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV = ExternalMemoryHandleTypeFlagBitsNV::ExternalMemoryHandleTypeD3d11ImageKmtBitNv as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEndCodingFlagsKHR: Flags {
	}
}

#[repr(u32)]
pub enum CullModeFlagBits {
	CullModeFrontBit = 1 << 0,
	CullModeBackBit = 1 << 1,
}

bitflags! {
	#[repr(transparent)]
	pub struct CullModeFlags: Flags {
		const CULL_MODE_FRONT_BIT = CullModeFlagBits::CullModeFrontBit as Flags;
		const CULL_MODE_BACK_BIT = CullModeFlagBits::CullModeBackBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineCoverageModulationStateCreateFlagsNV: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct ImageFormatConstraintsFlagsFUCHSIA: Flags {
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct AcquireProfilingLockFlagsKHR: Flags {
	}
}

#[repr(u32)]
pub enum VideoEncodeH264RateControlFlagBitsEXT {
	VideoEncodeH264RateControlAttemptHrdComplianceBitExt = 1 << 0,
	VideoEncodeH264RateControlRegularGopBitExt = 1 << 1,
	VideoEncodeH264RateControlReferencePatternFlatBitExt = 1 << 2,
	VideoEncodeH264RateControlReferencePatternDyadicBitExt = 1 << 3,
	VideoEncodeH264RateControlTemporalLayerPatternDyadicBitExt = 1 << 4,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeH264RateControlFlagsEXT: Flags {
		const VIDEO_ENCODE_H264_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_EXT = VideoEncodeH264RateControlFlagBitsEXT::VideoEncodeH264RateControlAttemptHrdComplianceBitExt as Flags;
		const VIDEO_ENCODE_H264_RATE_CONTROL_REGULAR_GOP_BIT_EXT = VideoEncodeH264RateControlFlagBitsEXT::VideoEncodeH264RateControlRegularGopBitExt as Flags;
		const VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_EXT = VideoEncodeH264RateControlFlagBitsEXT::VideoEncodeH264RateControlReferencePatternFlatBitExt as Flags;
		const VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_EXT = VideoEncodeH264RateControlFlagBitsEXT::VideoEncodeH264RateControlReferencePatternDyadicBitExt as Flags;
		const VIDEO_ENCODE_H264_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_EXT = VideoEncodeH264RateControlFlagBitsEXT::VideoEncodeH264RateControlTemporalLayerPatternDyadicBitExt as Flags;
	}
}

#[repr(u32)]
pub enum ExternalMemoryFeatureFlagBitsNV {
	ExternalMemoryFeatureDedicatedOnlyBitNv = 1 << 0,
	ExternalMemoryFeatureExportableBitNv = 1 << 1,
	ExternalMemoryFeatureImportableBitNv = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct ExternalMemoryFeatureFlagsNV: Flags {
		const EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV = ExternalMemoryFeatureFlagBitsNV::ExternalMemoryFeatureDedicatedOnlyBitNv as Flags;
		const EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV = ExternalMemoryFeatureFlagBitsNV::ExternalMemoryFeatureExportableBitNv as Flags;
		const EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV = ExternalMemoryFeatureFlagBitsNV::ExternalMemoryFeatureImportableBitNv as Flags;
	}
}

#[repr(u32)]
pub enum VideoEncodeH265StdFlagBitsEXT {
	VideoEncodeH265StdSeparateColorPlaneFlagSetBitExt = 1 << 0,
	VideoEncodeH265StdSampleAdaptiveOffsetEnabledFlagSetBitExt = 1 << 1,
	VideoEncodeH265StdScalingListDataPresentFlagSetBitExt = 1 << 2,
	VideoEncodeH265StdPcmEnabledFlagSetBitExt = 1 << 3,
	VideoEncodeH265StdSpsTemporalMvpEnabledFlagSetBitExt = 1 << 4,
	VideoEncodeH265StdInitQpMinus26BitExt = 1 << 5,
	VideoEncodeH265StdWeightedPredFlagSetBitExt = 1 << 6,
	VideoEncodeH265StdWeightedBipredFlagSetBitExt = 1 << 7,
	VideoEncodeH265StdLog2ParallelMergeLevelMinus2BitExt = 1 << 8,
	VideoEncodeH265StdSignDataHidingEnabledFlagSetBitExt = 1 << 9,
	VideoEncodeH265StdTransformSkipEnabledFlagSetBitExt = 1 << 10,
	VideoEncodeH265StdTransformSkipEnabledFlagUnsetBitExt = 1 << 11,
	VideoEncodeH265StdPpsSliceChromaQpOffsetsPresentFlagSetBitExt = 1 << 12,
	VideoEncodeH265StdTransquantBypassEnabledFlagSetBitExt = 1 << 13,
	VideoEncodeH265StdConstrainedIntraPredFlagSetBitExt = 1 << 14,
	VideoEncodeH265StdEntropyCodingSyncEnabledFlagSetBitExt = 1 << 15,
	VideoEncodeH265StdDeblockingFilterOverrideEnabledFlagSetBitExt = 1 << 16,
	VideoEncodeH265StdDependentSliceSegmentsEnabledFlagSetBitExt = 1 << 17,
	VideoEncodeH265StdDependentSliceSegmentFlagSetBitExt = 1 << 18,
	VideoEncodeH265StdSliceQpDeltaBitExt = 1 << 19,
	VideoEncodeH265StdDifferentSliceQpDeltaBitExt = 1 << 20,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoEncodeH265StdFlagsEXT: Flags {
		const VIDEO_ENCODE_H265_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdSeparateColorPlaneFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdSampleAdaptiveOffsetEnabledFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_SCALING_LIST_DATA_PRESENT_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdScalingListDataPresentFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_PCM_ENABLED_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdPcmEnabledFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdSpsTemporalMvpEnabledFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_INIT_QP_MINUS26_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdInitQpMinus26BitExt as Flags;
		const VIDEO_ENCODE_H265_STD_WEIGHTED_PRED_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdWeightedPredFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_WEIGHTED_BIPRED_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdWeightedBipredFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdLog2ParallelMergeLevelMinus2BitExt as Flags;
		const VIDEO_ENCODE_H265_STD_SIGN_DATA_HIDING_ENABLED_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdSignDataHidingEnabledFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdTransformSkipEnabledFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_UNSET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdTransformSkipEnabledFlagUnsetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdPpsSliceChromaQpOffsetsPresentFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_TRANSQUANT_BYPASS_ENABLED_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdTransquantBypassEnabledFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdConstrainedIntraPredFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdEntropyCodingSyncEnabledFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdDeblockingFilterOverrideEnabledFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdDependentSliceSegmentsEnabledFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENT_FLAG_SET_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdDependentSliceSegmentFlagSetBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_SLICE_QP_DELTA_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdSliceQpDeltaBitExt as Flags;
		const VIDEO_ENCODE_H265_STD_DIFFERENT_SLICE_QP_DELTA_BIT_EXT = VideoEncodeH265StdFlagBitsEXT::VideoEncodeH265StdDifferentSliceQpDeltaBitExt as Flags;
	}
}

#[repr(u32)]
pub enum StencilFaceFlagBits {
	StencilFaceFrontBit = 1 << 0,
	StencilFaceBackBit = 1 << 1,
}

impl StencilFaceFlagBits {
	const StencilFrontAndBack: u32 = Self::StencilFrontAndBack;
}

bitflags! {
	#[repr(transparent)]
	pub struct StencilFaceFlags: Flags {
		const STENCIL_FACE_FRONT_BIT = StencilFaceFlagBits::StencilFaceFrontBit as Flags;
		const STENCIL_FACE_BACK_BIT = StencilFaceFlagBits::StencilFaceBackBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct StreamDescriptorSurfaceCreateFlagsGGP: Flags {
	}
}

#[repr(u32)]
pub enum VideoDecodeUsageFlagBitsKHR {
	VideoDecodeUsageTranscodingBitKhr = 1 << 0,
	VideoDecodeUsageOfflineBitKhr = 1 << 1,
	VideoDecodeUsageStreamingBitKhr = 1 << 2,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoDecodeUsageFlagsKHR: Flags {
		const VIDEO_DECODE_USAGE_TRANSCODING_BIT_KHR = VideoDecodeUsageFlagBitsKHR::VideoDecodeUsageTranscodingBitKhr as Flags;
		const VIDEO_DECODE_USAGE_OFFLINE_BIT_KHR = VideoDecodeUsageFlagBitsKHR::VideoDecodeUsageOfflineBitKhr as Flags;
		const VIDEO_DECODE_USAGE_STREAMING_BIT_KHR = VideoDecodeUsageFlagBitsKHR::VideoDecodeUsageStreamingBitKhr as Flags;
	}
}

#[repr(u32)]
pub enum DebugReportFlagBitsEXT {
	DebugReportInformationBitExt = 1 << 0,
	DebugReportWarningBitExt = 1 << 1,
	DebugReportPerformanceWarningBitExt = 1 << 2,
	DebugReportErrorBitExt = 1 << 3,
	DebugReportDebugBitExt = 1 << 4,
}

bitflags! {
	#[repr(transparent)]
	pub struct DebugReportFlagsEXT: Flags {
		const DEBUG_REPORT_INFORMATION_BIT_EXT = DebugReportFlagBitsEXT::DebugReportInformationBitExt as Flags;
		const DEBUG_REPORT_WARNING_BIT_EXT = DebugReportFlagBitsEXT::DebugReportWarningBitExt as Flags;
		const DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT = DebugReportFlagBitsEXT::DebugReportPerformanceWarningBitExt as Flags;
		const DEBUG_REPORT_ERROR_BIT_EXT = DebugReportFlagBitsEXT::DebugReportErrorBitExt as Flags;
		const DEBUG_REPORT_DEBUG_BIT_EXT = DebugReportFlagBitsEXT::DebugReportDebugBitExt as Flags;
	}
}

#[repr(u32)]
pub enum BuildAccelerationStructureFlagBitsKHR {
	BuildAccelerationStructureAllowUpdateBitKhr = 1 << 0,
	BuildAccelerationStructureAllowCompactionBitKhr = 1 << 1,
	BuildAccelerationStructurePreferFastTraceBitKhr = 1 << 2,
	BuildAccelerationStructurePreferFastBuildBitKhr = 1 << 3,
	BuildAccelerationStructureLowMemoryBitKhr = 1 << 4,
}

bitflags! {
	#[repr(transparent)]
	pub struct BuildAccelerationStructureFlagsKHR: Flags {
		const BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR = BuildAccelerationStructureFlagBitsKHR::BuildAccelerationStructureAllowUpdateBitKhr as Flags;
		const BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR = BuildAccelerationStructureFlagBitsKHR::BuildAccelerationStructureAllowCompactionBitKhr as Flags;
		const BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR = BuildAccelerationStructureFlagBitsKHR::BuildAccelerationStructurePreferFastTraceBitKhr as Flags;
		const BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR = BuildAccelerationStructureFlagBitsKHR::BuildAccelerationStructurePreferFastBuildBitKhr as Flags;
		const BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR = BuildAccelerationStructureFlagBitsKHR::BuildAccelerationStructureLowMemoryBitKhr as Flags;
	}
}

#[repr(u32)]
pub enum PeerMemoryFeatureFlagBits {
	PeerMemoryFeatureCopySrcBit = 1 << 0,
	PeerMemoryFeatureCopyDstBit = 1 << 1,
	PeerMemoryFeatureGenericSrcBit = 1 << 2,
	PeerMemoryFeatureGenericDstBit = 1 << 3,
}

bitflags! {
	#[repr(transparent)]
	pub struct PeerMemoryFeatureFlags: Flags {
		const PEER_MEMORY_FEATURE_COPY_SRC_BIT = PeerMemoryFeatureFlagBits::PeerMemoryFeatureCopySrcBit as Flags;
		const PEER_MEMORY_FEATURE_COPY_DST_BIT = PeerMemoryFeatureFlagBits::PeerMemoryFeatureCopyDstBit as Flags;
		const PEER_MEMORY_FEATURE_GENERIC_SRC_BIT = PeerMemoryFeatureFlagBits::PeerMemoryFeatureGenericSrcBit as Flags;
		const PEER_MEMORY_FEATURE_GENERIC_DST_BIT = PeerMemoryFeatureFlagBits::PeerMemoryFeatureGenericDstBit as Flags;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct PipelineRasterizationConservativeStateCreateFlagsEXT: Flags {
	}
}

#[repr(u64)]
pub enum AccessFlagBits2 {
	Access2IndirectCommandReadBit = 1 << 0,
	Access2IndexReadBit = 1 << 1,
	Access2VertexAttributeReadBit = 1 << 2,
	Access2UniformReadBit = 1 << 3,
	Access2InputAttachmentReadBit = 1 << 4,
	Access2ShaderReadBit = 1 << 5,
	Access2ShaderWriteBit = 1 << 6,
	Access2ColorAttachmentReadBit = 1 << 7,
	Access2ColorAttachmentWriteBit = 1 << 8,
	Access2DepthStencilAttachmentReadBit = 1 << 9,
	Access2DepthStencilAttachmentWriteBit = 1 << 10,
	Access2TransferReadBit = 1 << 11,
	Access2TransferWriteBit = 1 << 12,
	Access2HostReadBit = 1 << 13,
	Access2HostWriteBit = 1 << 14,
	Access2MemoryReadBit = 1 << 15,
	Access2MemoryWriteBit = 1 << 16,
	Access2ShaderSampledReadBit = 1 << 32,
	Access2ShaderStorageReadBit = 1 << 33,
	Access2ShaderStorageWriteBit = 1 << 34,
}

impl AccessFlagBits2 {
	const Access2NoneKhr: u64 = Self::Access2NoneKhr;
	const Access2IndirectCommandReadBitKhr: u64 = Self::Access2IndirectCommandReadBitKhr;
	const Access2IndexReadBitKhr: u64 = Self::Access2IndexReadBitKhr;
	const Access2VertexAttributeReadBitKhr: u64 = Self::Access2VertexAttributeReadBitKhr;
	const Access2UniformReadBitKhr: u64 = Self::Access2UniformReadBitKhr;
	const Access2InputAttachmentReadBitKhr: u64 = Self::Access2InputAttachmentReadBitKhr;
	const Access2ShaderReadBitKhr: u64 = Self::Access2ShaderReadBitKhr;
	const Access2ShaderWriteBitKhr: u64 = Self::Access2ShaderWriteBitKhr;
	const Access2ColorAttachmentReadBitKhr: u64 = Self::Access2ColorAttachmentReadBitKhr;
	const Access2ColorAttachmentWriteBitKhr: u64 = Self::Access2ColorAttachmentWriteBitKhr;
	const Access2DepthStencilAttachmentReadBitKhr: u64 = Self::Access2DepthStencilAttachmentReadBitKhr;
	const Access2DepthStencilAttachmentWriteBitKhr: u64 = Self::Access2DepthStencilAttachmentWriteBitKhr;
	const Access2TransferReadBitKhr: u64 = Self::Access2TransferReadBitKhr;
	const Access2TransferWriteBitKhr: u64 = Self::Access2TransferWriteBitKhr;
	const Access2HostReadBitKhr: u64 = Self::Access2HostReadBitKhr;
	const Access2HostWriteBitKhr: u64 = Self::Access2HostWriteBitKhr;
	const Access2MemoryReadBitKhr: u64 = Self::Access2MemoryReadBitKhr;
	const Access2MemoryWriteBitKhr: u64 = Self::Access2MemoryWriteBitKhr;
	const Access2ShaderSampledReadBitKhr: u64 = Self::Access2ShaderSampledReadBitKhr;
	const Access2ShaderStorageReadBitKhr: u64 = Self::Access2ShaderStorageReadBitKhr;
	const Access2ShaderStorageWriteBitKhr: u64 = Self::Access2ShaderStorageWriteBitKhr;
}

bitflags! {
	#[repr(transparent)]
	pub struct AccessFlags2: Flags64 {
		const ACCESS_2_INDIRECT_COMMAND_READ_BIT = AccessFlagBits2::Access2IndirectCommandReadBit as Flags64;
		const ACCESS_2_INDEX_READ_BIT = AccessFlagBits2::Access2IndexReadBit as Flags64;
		const ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT = AccessFlagBits2::Access2VertexAttributeReadBit as Flags64;
		const ACCESS_2_UNIFORM_READ_BIT = AccessFlagBits2::Access2UniformReadBit as Flags64;
		const ACCESS_2_INPUT_ATTACHMENT_READ_BIT = AccessFlagBits2::Access2InputAttachmentReadBit as Flags64;
		const ACCESS_2_SHADER_READ_BIT = AccessFlagBits2::Access2ShaderReadBit as Flags64;
		const ACCESS_2_SHADER_WRITE_BIT = AccessFlagBits2::Access2ShaderWriteBit as Flags64;
		const ACCESS_2_COLOR_ATTACHMENT_READ_BIT = AccessFlagBits2::Access2ColorAttachmentReadBit as Flags64;
		const ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT = AccessFlagBits2::Access2ColorAttachmentWriteBit as Flags64;
		const ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT = AccessFlagBits2::Access2DepthStencilAttachmentReadBit as Flags64;
		const ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = AccessFlagBits2::Access2DepthStencilAttachmentWriteBit as Flags64;
		const ACCESS_2_TRANSFER_READ_BIT = AccessFlagBits2::Access2TransferReadBit as Flags64;
		const ACCESS_2_TRANSFER_WRITE_BIT = AccessFlagBits2::Access2TransferWriteBit as Flags64;
		const ACCESS_2_HOST_READ_BIT = AccessFlagBits2::Access2HostReadBit as Flags64;
		const ACCESS_2_HOST_WRITE_BIT = AccessFlagBits2::Access2HostWriteBit as Flags64;
		const ACCESS_2_MEMORY_READ_BIT = AccessFlagBits2::Access2MemoryReadBit as Flags64;
		const ACCESS_2_MEMORY_WRITE_BIT = AccessFlagBits2::Access2MemoryWriteBit as Flags64;
		const ACCESS_2_SHADER_SAMPLED_READ_BIT = AccessFlagBits2::Access2ShaderSampledReadBit as Flags64;
		const ACCESS_2_SHADER_STORAGE_READ_BIT = AccessFlagBits2::Access2ShaderStorageReadBit as Flags64;
		const ACCESS_2_SHADER_STORAGE_WRITE_BIT = AccessFlagBits2::Access2ShaderStorageWriteBit as Flags64;
	}
}

bitflags! {
	#[repr(transparent)]
	pub struct DescriptorUpdateTemplateCreateFlags: Flags {
	}
}

#[repr(u32)]
pub enum VideoDecodeH264PictureLayoutFlagBitsKHR {
	VideoDecodeH264PictureLayoutInterlacedInterleavedLinesBitKhr = 1 << 0,
	VideoDecodeH264PictureLayoutInterlacedSeparatePlanesBitKhr = 1 << 1,
}

bitflags! {
	#[repr(transparent)]
	pub struct VideoDecodeH264PictureLayoutFlagsKHR: Flags {
		const VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_KHR = VideoDecodeH264PictureLayoutFlagBitsKHR::VideoDecodeH264PictureLayoutInterlacedInterleavedLinesBitKhr as Flags;
		const VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_KHR = VideoDecodeH264PictureLayoutFlagBitsKHR::VideoDecodeH264PictureLayoutInterlacedSeparatePlanesBitKhr as Flags;
	}
}

pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;

pub type AccessFlags2KHR = AccessFlags2;

pub type SubmitFlagsKHR = SubmitFlags;

pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;

pub type FenceImportFlagsKHR = FenceImportFlags;

pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;

pub type FormatFeatureFlags2KHR = FormatFeatureFlags2;

pub type GeometryInstanceFlagsNV = GeometryInstanceFlagsKHR;

pub type DescriptorUpdateTemplateCreateFlagsKHR = DescriptorUpdateTemplateCreateFlags;

pub type PipelineCreationFeedbackFlagsEXT = PipelineCreationFeedbackFlags;

pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;

pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;

pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;

pub type RenderingFlagsKHR = RenderingFlags;

pub type PipelineStageFlags2KHR = PipelineStageFlags2;

pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;

pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;

pub type BuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsKHR;

pub type SemaphoreImportFlagsKHR = SemaphoreImportFlags;

pub type GeometryFlagsNV = GeometryFlagsKHR;

pub type CommandPoolTrimFlagsKHR = CommandPoolTrimFlags;

pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;

pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;

pub type ResolveModeFlagsKHR = ResolveModeFlags;

pub type ToolPurposeFlagsEXT = ToolPurposeFlags;

pub type PrivateDataSlotCreateFlagsEXT = PrivateDataSlotCreateFlags;

#[repr(C)]
pub struct PhysicalDeviceExternalFormatResolvePropertiesANDROID {
	s_type: StructureType,
	p_next: *mut c_void,
	null_color_attachment_with_external_format_resolve: Bool32,
	external_format_resolve_chroma_offset_x: ChromaLocation,
	external_format_resolve_chroma_offset_y: ChromaLocation,
}

#[repr(C)]
pub struct MemoryGetSciBufInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	memory: DeviceMemory,
	handle_type: ExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceShaderSMBuiltinsPropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_smcount: u32,
	shader_warps_per_sm: u32,
}

#[repr(C)]
pub struct RenderPassSampleLocationsBeginInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	attachment_initial_sample_locations_count: u32,
	p_attachment_initial_sample_locations: *const AttachmentSampleLocationsEXT,
	post_subpass_sample_locations_count: u32,
	p_post_subpass_sample_locations: *const SubpassSampleLocationsEXT,
}

#[repr(C)]
pub struct BindBufferMemoryDeviceGroupInfo {
	s_type: StructureType,
	p_next: *const c_void,
	device_index_count: u32,
	p_device_indices: *const u32,
}

#[repr(C)]
pub struct PipelineCreationFeedbackCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	p_pipeline_creation_feedback: *mut PipelineCreationFeedback,
	pipeline_stage_creation_feedback_count: u32,
	p_pipeline_stage_creation_feedbacks: *mut PipelineCreationFeedback,
}

#[repr(C)]
pub struct InputAttachmentAspectReference {
	subpass: u32,
	input_attachment_index: u32,
	aspect_mask: ImageAspectFlags,
}

#[repr(C)]
pub struct ExternalSemaphoreProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
	compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
	external_semaphore_features: ExternalSemaphoreFeatureFlags,
}

#[repr(C)]
pub struct ImageViewMinLodCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	min_lod: f32,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	fragment_shader_barycentric: Bool32,
}

#[repr(C)]
pub struct SamplerYcbcrConversionImageFormatProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	combined_image_sampler_descriptor_count: u32,
}

#[repr(C)]
pub struct ImageSparseMemoryRequirementsInfo2 {
	s_type: StructureType,
	p_next: *const c_void,
	image: Image,
}

#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelinePropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_group_handle_size: u32,
	max_ray_recursion_depth: u32,
	max_shader_group_stride: u32,
	shader_group_base_alignment: u32,
	shader_group_handle_capture_replay_size: u32,
	max_ray_dispatch_invocation_count: u32,
	shader_group_handle_alignment: u32,
	max_ray_hit_attribute_size: u32,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	fragment_density_offset_granularity: Extent2D,
}

#[repr(C)]
pub struct PerformanceQueryReservationInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	max_performance_queries_per_pool: u32,
}

#[repr(C)]
pub struct SubpassFragmentDensityMapOffsetEndInfoQCOM {
	s_type: StructureType,
	p_next: *const c_void,
	fragment_density_offset_count: u32,
	p_fragment_density_offsets: *const Offset2D,
}

#[repr(C)]
pub struct ImageFormatProperties {
	max_extent: Extent3D,
	max_mip_levels: u32,
	max_array_layers: u32,
	sample_counts: SampleCountFlags,
	max_resource_size: DeviceSize,
}

#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	transform_feedback: Bool32,
	geometry_streams: Bool32,
}

#[repr(C)]
pub struct VideoEncodeH264SessionParametersAddInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	std_spscount: u32,
	p_std_spss: *const StdVideoH264SequenceParameterSet,
	std_ppscount: u32,
	p_std_ppss: *const StdVideoH264PictureParameterSet,
}

#[repr(C)]
pub struct VideoEncodeH264GopRemainingFrameInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	use_gop_remaining_frames: Bool32,
	gop_remaining_i: u32,
	gop_remaining_p: u32,
	gop_remaining_b: u32,
}

#[repr(C)]
pub struct ShaderModuleCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: ShaderModuleCreateFlags,
	code_size: usize,
	p_code: *const u32,
}

#[repr(C)]
pub struct PhysicalDeviceRayTracingInvocationReorderFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	ray_tracing_invocation_reorder: Bool32,
}

#[repr(C)]
pub struct SamplerBlockMatchWindowCreateInfoQCOM {
	s_type: StructureType,
	p_next: *const c_void,
	window_extent: Extent2D,
	window_compare_mode: BlockMatchWindowCompareModeQCOM,
}

#[repr(C)]
pub struct PhysicalDeviceDescriptorBufferPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	combined_image_sampler_descriptor_single_array: Bool32,
	bufferless_push_descriptors: Bool32,
	allow_sampler_image_view_post_submit_creation: Bool32,
	descriptor_buffer_offset_alignment: DeviceSize,
	max_descriptor_buffer_bindings: u32,
	max_resource_descriptor_buffer_bindings: u32,
	max_sampler_descriptor_buffer_bindings: u32,
	max_embedded_immutable_sampler_bindings: u32,
	max_embedded_immutable_samplers: u32,
	buffer_capture_replay_descriptor_data_size: usize,
	image_capture_replay_descriptor_data_size: usize,
	image_view_capture_replay_descriptor_data_size: usize,
	sampler_capture_replay_descriptor_data_size: usize,
	acceleration_structure_capture_replay_descriptor_data_size: usize,
	sampler_descriptor_size: usize,
	combined_image_sampler_descriptor_size: usize,
	sampled_image_descriptor_size: usize,
	storage_image_descriptor_size: usize,
	uniform_texel_buffer_descriptor_size: usize,
	robust_uniform_texel_buffer_descriptor_size: usize,
	storage_texel_buffer_descriptor_size: usize,
	robust_storage_texel_buffer_descriptor_size: usize,
	uniform_buffer_descriptor_size: usize,
	robust_uniform_buffer_descriptor_size: usize,
	storage_buffer_descriptor_size: usize,
	robust_storage_buffer_descriptor_size: usize,
	input_attachment_descriptor_size: usize,
	acceleration_structure_descriptor_size: usize,
	max_sampler_descriptor_buffer_range: DeviceSize,
	max_resource_descriptor_buffer_range: DeviceSize,
	sampler_descriptor_buffer_address_space_size: DeviceSize,
	resource_descriptor_buffer_address_space_size: DeviceSize,
	descriptor_buffer_address_space_size: DeviceSize,
}

#[repr(C)]
pub struct PhysicalDevicePCIBusInfoPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	pci_domain: u32,
	pci_bus: u32,
	pci_device: u32,
	pci_function: u32,
}

#[repr(C)]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	representative_fragment_test_enable: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	border_color_swizzle: Bool32,
	border_color_swizzle_from_image: Bool32,
}

#[repr(C)]
pub struct CommandBufferInheritanceInfo {
	s_type: StructureType,
	p_next: *const c_void,
	render_pass: RenderPass,
	subpass: u32,
	framebuffer: Framebuffer,
	occlusion_query_enable: Bool32,
	query_flags: QueryControlFlags,
	pipeline_statistics: QueryPipelineStatisticFlags,
}

#[repr(C)]
pub struct PipelineLayoutCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineLayoutCreateFlags,
	set_layout_count: u32,
	p_set_layouts: *const DescriptorSetLayout,
	push_constant_range_count: u32,
	p_push_constant_ranges: *const PushConstantRange,
}

#[repr(C)]
pub struct PipelineTessellationDomainOriginStateCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	domain_origin: TessellationDomainOrigin,
}

#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_input_attachment_array_dynamic_indexing: Bool32,
	shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
	shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
	shader_uniform_buffer_array_non_uniform_indexing: Bool32,
	shader_sampled_image_array_non_uniform_indexing: Bool32,
	shader_storage_buffer_array_non_uniform_indexing: Bool32,
	shader_storage_image_array_non_uniform_indexing: Bool32,
	shader_input_attachment_array_non_uniform_indexing: Bool32,
	shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
	shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
	descriptor_binding_uniform_buffer_update_after_bind: Bool32,
	descriptor_binding_sampled_image_update_after_bind: Bool32,
	descriptor_binding_storage_image_update_after_bind: Bool32,
	descriptor_binding_storage_buffer_update_after_bind: Bool32,
	descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
	descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
	descriptor_binding_update_unused_while_pending: Bool32,
	descriptor_binding_partially_bound: Bool32,
	descriptor_binding_variable_descriptor_count: Bool32,
	runtime_descriptor_array: Bool32,
}

#[repr(C)]
pub struct ImageStencilUsageCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	stencil_usage: ImageUsageFlags,
}

#[repr(C)]
pub struct PhysicalDeviceImageRobustnessFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	robust_image_access: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceExternalMemoryRDMAFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	external_memory_rdma: Bool32,
}

#[repr(C)]
pub struct ComputePipelineCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineCreateFlags,
	stage: PipelineShaderStageCreateInfo,
	layout: PipelineLayout,
	base_pipeline_handle: Pipeline,
	base_pipeline_index: i32,
}

#[repr(C)]
pub struct GeometryDataNV {
	triangles: GeometryTrianglesNV,
	aabbs: GeometryAABBNV,
}

#[repr(C)]
pub struct CuFunctionCreateInfoNVX {
	s_type: StructureType,
	p_next: *const c_void,
	module: CuModuleNVX,
	p_name: *const u8,
}

#[repr(C)]
pub struct PhysicalDeviceShaderObjectFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_object: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderObjectPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_binary_uuid: [u8; UUID_SIZE],
	shader_binary_version: u32,
}

#[repr(C)]
pub struct PhysicalDeviceProperties {
	api_version: u32,
	driver_version: u32,
	vendor_id: u32,
	device_id: u32,
	device_type: PhysicalDeviceType,
	device_name: [u8; MAX_PHYSICAL_DEVICE_NAME_SIZE],
	pipeline_cache_uuid: [u8; UUID_SIZE],
	limits: PhysicalDeviceLimits,
	sparse_properties: PhysicalDeviceSparseProperties,
}

#[repr(C)]
pub struct ExternalMemoryImageCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	handle_types: ExternalMemoryHandleTypeFlagsNV,
}

#[repr(C)]
pub struct CommandBufferInheritanceRenderingInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: RenderingFlags,
	view_mask: u32,
	#[cfg(vulkan)]
	color_attachment_count: u32,
	#[cfg(vulkansc)]
	color_attachment_count: u32,
	p_color_attachment_formats: *const Format,
	depth_attachment_format: Format,
	stencil_attachment_format: Format,
	rasterization_samples: SampleCountFlagBits,
}

#[repr(C)]
pub struct ImportSemaphoreSciSyncInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	semaphore: Semaphore,
	handle_type: ExternalSemaphoreHandleTypeFlagBits,
	handle: *mut c_void,
}

#[repr(C)]
pub struct SemaphoreGetFdInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	semaphore: Semaphore,
	handle_type: ExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct PresentRegionKHR {
	rectangle_count: u32,
	p_rectangles: *const RectLayerKHR,
}

#[repr(C)]
pub struct MicromapCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	create_flags: MicromapCreateFlagsEXT,
	buffer: Buffer,
	offset: DeviceSize,
	size: DeviceSize,
	r#type: MicromapTypeEXT,
	device_address: DeviceAddress,
}

#[repr(C)]
pub struct CommandPoolMemoryReservationCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	command_pool_reserved_size: DeviceSize,
	command_pool_max_command_buffers: u32,
}

#[repr(C)]
pub struct SamplerYcbcrConversionCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	format: Format,
	ycbcr_model: SamplerYcbcrModelConversion,
	ycbcr_range: SamplerYcbcrRange,
	components: ComponentMapping,
	x_chroma_offset: ChromaLocation,
	y_chroma_offset: ChromaLocation,
	chroma_filter: Filter,
	force_explicit_reconstruction: Bool32,
}

#[repr(C)]
pub struct MemoryDedicatedAllocateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	image: Image,
	buffer: Buffer,
}

#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	custom_border_colors: Bool32,
	custom_border_color_without_format: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderTileImageFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_tile_image_color_read_access: Bool32,
	shader_tile_image_depth_read_access: Bool32,
	shader_tile_image_stencil_read_access: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	max_custom_border_color_samplers: u32,
}

#[repr(C)]
pub struct BindAccelerationStructureMemoryInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	acceleration_structure: AccelerationStructureNV,
	memory: DeviceMemory,
	memory_offset: DeviceSize,
	device_index_count: u32,
	p_device_indices: *const u32,
}

#[repr(C)]
pub struct RefreshObjectKHR {
	object_type: ObjectType,
	object_handle: u64,
	flags: RefreshObjectFlagsKHR,
}

#[repr(C)]
pub struct AndroidHardwareBufferFormatProperties2ANDROID {
	s_type: StructureType,
	p_next: *mut c_void,
	format: Format,
	external_format: u64,
	format_features: FormatFeatureFlags2,
	sampler_ycbcr_conversion_components: ComponentMapping,
	suggested_ycbcr_model: SamplerYcbcrModelConversion,
	suggested_ycbcr_range: SamplerYcbcrRange,
	suggested_xchroma_offset: ChromaLocation,
	suggested_ychroma_offset: ChromaLocation,
}

#[repr(C)]
pub struct PhysicalDeviceOpacityMicromapPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	max_opacity2_state_subdivision_level: u32,
	max_opacity4_state_subdivision_level: u32,
}

#[repr(C)]
pub struct ImportMetalSharedEventInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	mtl_shared_event: MTLSharedEvent_id,
}

#[repr(C)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	sample_order_type: CoarseSampleOrderTypeNV,
	custom_sample_order_count: u32,
	p_custom_sample_orders: *const CoarseSampleOrderCustomNV,
}

#[repr(C)]
pub struct RayTracingPipelineCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineCreateFlags,
	stage_count: u32,
	p_stages: *const PipelineShaderStageCreateInfo,
	group_count: u32,
	p_groups: *const RayTracingShaderGroupCreateInfoKHR,
	max_pipeline_ray_recursion_depth: u32,
	p_library_info: *const PipelineLibraryCreateInfoKHR,
	p_library_interface: *const RayTracingPipelineInterfaceCreateInfoKHR,
	p_dynamic_state: *const PipelineDynamicStateCreateInfo,
	layout: PipelineLayout,
	base_pipeline_handle: Pipeline,
	base_pipeline_index: i32,
}

#[repr(C)]
pub struct PhysicalDeviceSparseProperties {
	residency_standard2_dblock_shape: Bool32,
	residency_standard2_dmultisample_block_shape: Bool32,
	residency_standard3_dblock_shape: Bool32,
	residency_aligned_mip_size: Bool32,
	residency_non_resident_strict: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	vertex_attribute_instance_rate_divisor: Bool32,
	vertex_attribute_instance_rate_zero_divisor: Bool32,
}

#[repr(C)]
pub struct ConformanceVersion {
	major: u8,
	minor: u8,
	subminor: u8,
	patch: u8,
}

#[repr(C)]
pub struct PhysicalDeviceSubgroupProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	subgroup_size: u32,
	supported_stages: ShaderStageFlags,
	supported_operations: SubgroupFeatureFlags,
	quad_operations_in_all_stages: Bool32,
}

#[repr(C)]
pub struct ShaderResourceUsageAMD {
	num_used_vgprs: u32,
	num_used_sgprs: u32,
	lds_size_per_local_work_group: u32,
	lds_usage_size_in_bytes: usize,
	scratch_mem_usage_in_bytes: usize,
}

#[repr(C)]
pub struct ExportFenceCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	handle_types: ExternalFenceHandleTypeFlags,
}

#[repr(C)]
pub struct MemoryGetRemoteAddressInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	memory: DeviceMemory,
	handle_type: ExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	texel_buffer_alignment: Bool32,
}

#[repr(C)]
pub struct CopyImageToBufferInfo2 {
	s_type: StructureType,
	p_next: *const c_void,
	src_image: Image,
	src_image_layout: ImageLayout,
	dst_buffer: Buffer,
	region_count: u32,
	p_regions: *const BufferImageCopy2,
}

#[repr(C)]
pub struct PhysicalDeviceDescriptorBufferFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	descriptor_buffer: Bool32,
	descriptor_buffer_capture_replay: Bool32,
	descriptor_buffer_image_layout_ignored: Bool32,
	descriptor_buffer_push_descriptors: Bool32,
}

#[repr(C)]
pub struct DescriptorPoolCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: DescriptorPoolCreateFlags,
	max_sets: u32,
	pool_size_count: u32,
	p_pool_sizes: *const DescriptorPoolSize,
}

#[repr(C)]
pub struct QueueFamilyVideoPropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	video_codec_operations: VideoCodecOperationFlagsKHR,
}

#[repr(C)]
pub struct MicromapTriangleEXT {
	data_offset: u32,
	subdivision_level: u16,
	format: u16,
}

#[repr(C)]
pub struct SwapchainPresentModeInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	swapchain_count: u32,
	p_present_modes: *const PresentModeKHR,
}

#[repr(C)]
pub struct ExportSemaphoreSciSyncInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	p_attributes: NvSciSyncAttrList,
}

#[repr(C)]
pub struct ComputePipelineIndirectBufferInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	device_address: DeviceAddress,
	size: DeviceSize,
	pipeline_device_address_capture_replay: DeviceAddress,
}

#[repr(C)]
pub struct FaultCallbackInfo {
	s_type: StructureType,
	p_next: *const c_void,
	fault_count: u32,
	p_faults: *mut FaultData,
	pfn_fault_callback: PFN_vkFaultCallbackFunction,
}

#[repr(C)]
pub struct PhysicalDeviceMultiDrawFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	multi_draw: Bool32,
}

#[repr(C)]
pub struct BufferCaptureDescriptorDataInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	buffer: Buffer,
}

#[repr(C)]
pub struct DedicatedAllocationBufferCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	dedicated_allocation: Bool32,
}

#[repr(C)]
pub struct GraphicsShaderGroupCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	stage_count: u32,
	p_stages: *const PipelineShaderStageCreateInfo,
	p_vertex_input_state: *const PipelineVertexInputStateCreateInfo,
	p_tessellation_state: *const PipelineTessellationStateCreateInfo,
}

#[repr(C)]
pub struct ExternalBufferProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	external_memory_properties: ExternalMemoryProperties,
}

#[repr(C)]
pub struct PhysicalDeviceMeshShaderPropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	max_draw_mesh_tasks_count: u32,
	max_task_work_group_invocations: u32,
	max_task_work_group_size: [u32; 3],
	max_task_total_memory_size: u32,
	max_task_output_count: u32,
	max_mesh_work_group_invocations: u32,
	max_mesh_work_group_size: [u32; 3],
	max_mesh_total_memory_size: u32,
	max_mesh_output_vertices: u32,
	max_mesh_output_primitives: u32,
	max_mesh_multiview_view_count: u32,
	mesh_output_per_vertex_granularity: u32,
	mesh_output_per_primitive_granularity: u32,
}

#[repr(C)]
pub struct VideoDecodeH265SessionParametersAddInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	std_vpscount: u32,
	p_std_vpss: *const StdVideoH265VideoParameterSet,
	std_spscount: u32,
	p_std_spss: *const StdVideoH265SequenceParameterSet,
	std_ppscount: u32,
	p_std_ppss: *const StdVideoH265PictureParameterSet,
}

#[repr(C)]
pub struct PhysicalDeviceShaderCorePropertiesAMD {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_engine_count: u32,
	shader_arrays_per_engine_count: u32,
	compute_units_per_shader_array: u32,
	simd_per_compute_unit: u32,
	wavefronts_per_simd: u32,
	wavefront_size: u32,
	sgprs_per_simd: u32,
	min_sgpr_allocation: u32,
	max_sgpr_allocation: u32,
	sgpr_allocation_granularity: u32,
	vgprs_per_simd: u32,
	min_vgpr_allocation: u32,
	max_vgpr_allocation: u32,
	vgpr_allocation_granularity: u32,
}

#[repr(C)]
pub struct AccelerationStructureSRTMotionInstanceNV {
	transform_t0: SRTDataNV,
	transform_t1: SRTDataNV,
	instance_custom_index: u32,
	mask: u32,
	instance_shader_binding_table_record_offset: u32,
	flags: GeometryInstanceFlagsKHR,
	acceleration_structure_reference: u64,
}

#[repr(C)]
pub struct PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	attachment_feedback_loop_layout: Bool32,
}

#[repr(C)]
pub struct DeviceFaultVendorInfoEXT {
	description: [u8; MAX_DESCRIPTION_SIZE],
	vendor_fault_code: u64,
	vendor_fault_data: u64,
}

#[repr(C)]
pub struct BufferMemoryBarrier {
	s_type: StructureType,
	p_next: *const c_void,
	src_access_mask: AccessFlags,
	dst_access_mask: AccessFlags,
	src_queue_family_index: u32,
	dst_queue_family_index: u32,
	buffer: Buffer,
	offset: DeviceSize,
	size: DeviceSize,
}

#[repr(C)]
pub struct ExtensionProperties {
	extension_name: [u8; MAX_EXTENSION_NAME_SIZE],
	spec_version: u32,
}

#[repr(C)]
pub struct ImageFormatListCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	view_format_count: u32,
	p_view_formats: *const Format,
}

#[repr(C)]
pub struct PhysicalDeviceSubpassShadingFeaturesHUAWEI {
	s_type: StructureType,
	p_next: *mut c_void,
	subpass_shading: Bool32,
}

#[repr(C)]
pub struct CommandBufferSubmitInfo {
	s_type: StructureType,
	p_next: *const c_void,
	command_buffer: CommandBuffer,
	device_mask: u32,
}

#[repr(C)]
pub struct PipelineCacheSafetyCriticalIndexEntry {
	pipeline_identifier: [u8; UUID_SIZE],
	pipeline_memory_size: u64,
	json_size: u64,
	json_offset: u64,
	stage_index_count: u32,
	stage_index_stride: u32,
	stage_index_offset: u64,
}

#[repr(C)]
pub struct PhysicalDeviceVulkan11Features {
	s_type: StructureType,
	p_next: *mut c_void,
	storage_buffer16_bit_access: Bool32,
	uniform_and_storage_buffer16_bit_access: Bool32,
	storage_push_constant16: Bool32,
	storage_input_output16: Bool32,
	multiview: Bool32,
	multiview_geometry_shader: Bool32,
	multiview_tessellation_shader: Bool32,
	variable_pointers_storage_buffer: Bool32,
	variable_pointers: Bool32,
	protected_memory: Bool32,
	sampler_ycbcr_conversion: Bool32,
	shader_draw_parameters: Bool32,
}

#[repr(C)]
pub struct ColorBlendEquationEXT {
	src_color_blend_factor: BlendFactor,
	dst_color_blend_factor: BlendFactor,
	color_blend_op: BlendOp,
	src_alpha_blend_factor: BlendFactor,
	dst_alpha_blend_factor: BlendFactor,
	alpha_blend_op: BlendOp,
}

#[repr(C)]
pub struct CommandBufferBeginInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: CommandBufferUsageFlags,
	p_inheritance_info: *const CommandBufferInheritanceInfo,
}

#[repr(C)]
pub struct PhysicalDeviceDeviceMemoryReportFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	device_memory_report: Bool32,
}

#[repr(C)]
pub struct DebugMarkerMarkerInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	p_marker_name: *const u8,
	color: [f32; 4],
}

#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	max_timeline_semaphore_value_difference: u64,
}

#[repr(C)]
pub struct VideoEncodeH265RateControlLayerInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	use_min_qp: Bool32,
	min_qp: VideoEncodeH265QpEXT,
	use_max_qp: Bool32,
	max_qp: VideoEncodeH265QpEXT,
	use_max_frame_size: Bool32,
	max_frame_size: VideoEncodeH265FrameSizeEXT,
}

#[repr(C)]
pub struct MicromapBuildSizesInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	micromap_size: DeviceSize,
	build_scratch_size: DeviceSize,
	discardable: Bool32,
}

#[repr(C)]
pub struct PhysicalDevicePortabilitySubsetFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	ant_alpha_color_blend_factors: Bool32,
	events: Bool32,
	image_view_format_reinterpretation: Bool32,
	image_view_format_swizzle: Bool32,
	image_view2_don3_dimage: Bool32,
	multisample_array_image: Bool32,
	mutable_comparison_samplers: Bool32,
	point_polygons: Bool32,
	sampler_mip_lod_bias: Bool32,
	separate_stencil_mask_ref: Bool32,
	shader_sample_rate_interpolation_functions: Bool32,
	tessellation_isolines: Bool32,
	tessellation_point_mode: Bool32,
	triangle_fans: Bool32,
	vertex_attribute_access_beyond_stride: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	buffer_device_address: Bool32,
	buffer_device_address_capture_replay: Bool32,
	buffer_device_address_multi_device: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceRayTracingMotionBlurFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	ray_tracing_motion_blur: Bool32,
	ray_tracing_motion_blur_pipeline_trace_rays_indirect: Bool32,
}

#[repr(C)]
pub struct SurfacePresentModeEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	present_mode: PresentModeKHR,
}

#[repr(C)]
pub struct DeviceGroupPresentInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	swapchain_count: u32,
	p_device_masks: *const u32,
	mode: DeviceGroupPresentModeFlagBitsKHR,
}

#[repr(C)]
pub struct PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	extended_sparse_address_space: Bool32,
}

#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	r#type: RayTracingShaderGroupTypeKHR,
	general_shader: u32,
	closest_hit_shader: u32,
	any_hit_shader: u32,
	intersection_shader: u32,
	p_shader_group_capture_replay_handle: *const c_void,
}

#[repr(C)]
pub struct PhysicalDevicePipelineProtectedAccessFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	pipeline_protected_access: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	cooperative_matrix: Bool32,
	cooperative_matrix_robust_buffer_access: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceVideoFormatInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	image_usage: ImageUsageFlags,
}

#[repr(C)]
pub struct PhysicalDeviceMemoryProperties {
	memory_type_count: u32,
	memory_types: [MemoryType; MAX_MEMORY_TYPES],
	memory_heap_count: u32,
	memory_heaps: [MemoryHeap; MAX_MEMORY_HEAPS],
}

#[repr(C)]
pub struct SemaphoreGetZirconHandleInfoFUCHSIA {
	s_type: StructureType,
	p_next: *const c_void,
	semaphore: Semaphore,
	handle_type: ExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct IndirectCommandsLayoutTokenNV {
	s_type: StructureType,
	p_next: *const c_void,
	token_type: IndirectCommandsTokenTypeNV,
	stream: u32,
	offset: u32,
	vertex_binding_unit: u32,
	vertex_dynamic_stride: Bool32,
	pushconstant_pipeline_layout: PipelineLayout,
	pushconstant_shader_stage_flags: ShaderStageFlags,
	pushconstant_offset: u32,
	pushconstant_size: u32,
	indirect_state_flags: IndirectStateFlagsNV,
	index_type_count: u32,
	p_index_types: *const IndexType,
	p_index_type_values: *const u32,
}

#[repr(C)]
pub struct FenceGetSciSyncInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	fence: Fence,
	handle_type: ExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct DisplayPlaneCapabilities2KHR {
	s_type: StructureType,
	p_next: *mut c_void,
	capabilities: DisplayPlaneCapabilitiesKHR,
}

#[repr(C)]
pub struct DrmFormatModifierPropertiesList2EXT {
	s_type: StructureType,
	p_next: *mut c_void,
	drm_format_modifier_count: u32,
	p_drm_format_modifier_properties: *mut DrmFormatModifierProperties2EXT,
}

#[repr(C)]
pub struct ShaderModuleIdentifierEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	identifier_size: u32,
	identifier: [u8; MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT],
}

#[repr(C)]
pub struct DeviceFaultAddressInfoEXT {
	address_type: DeviceFaultAddressTypeEXT,
	reported_address: DeviceAddress,
	address_precision: DeviceSize,
}

#[repr(C)]
pub struct OpticalFlowImageFormatInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	usage: OpticalFlowUsageFlagsNV,
}

#[repr(C)]
pub struct ImportFenceFdInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	fence: Fence,
	flags: FenceImportFlags,
	handle_type: ExternalFenceHandleTypeFlagBits,
	fd: isize,
}

#[repr(C)]
pub struct CudaFunctionCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	module: CudaModuleNV,
	p_name: *const u8,
}

#[repr(C)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	performance_counter_query_pools: Bool32,
	performance_counter_multiple_query_pools: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceSchedulingControlsFeaturesARM {
	s_type: StructureType,
	p_next: *mut c_void,
	scheduling_controls: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceRayTracingInvocationReorderPropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	ray_tracing_invocation_reorder_reordering_hint: RayTracingInvocationReorderModeNV,
}

#[repr(C)]
pub struct IOSSurfaceCreateInfoMVK {
	s_type: StructureType,
	p_next: *const c_void,
	flags: IOSSurfaceCreateFlagsMVK,
	p_view: *const c_void,
}

#[repr(C)]
pub struct Rect2D {
	offset: Offset2D,
	extent: Extent2D,
}

#[repr(C)]
pub struct PhysicalDeviceLayeredDriverPropertiesMSFT {
	s_type: StructureType,
	p_next: *mut c_void,
	underlying_api: LayeredDriverUnderlyingApiMSFT,
}

#[repr(C)]
pub struct SamplerCubicWeightsCreateInfoQCOM {
	s_type: StructureType,
	p_next: *const c_void,
	cubic_weights: CubicFilterWeightsQCOM,
}

#[repr(C)]
pub struct PhysicalDeviceShaderCoreBuiltinsFeaturesARM {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_core_builtins: Bool32,
}

#[repr(C)]
pub struct AccelerationStructureCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	create_flags: AccelerationStructureCreateFlagsKHR,
	buffer: Buffer,
	offset: DeviceSize,
	size: DeviceSize,
	r#type: AccelerationStructureTypeKHR,
	device_address: DeviceAddress,
}

#[repr(C)]
pub struct PhysicalDeviceShaderTerminateInvocationFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_terminate_invocation: Bool32,
}

#[repr(C)]
pub struct VertexInputAttributeDescription2EXT {
	s_type: StructureType,
	p_next: *mut c_void,
	location: u32,
	binding: u32,
	format: Format,
	offset: u32,
}

#[repr(C)]
pub struct CommandBufferAllocateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	command_pool: CommandPool,
	level: CommandBufferLevel,
	command_buffer_count: u32,
}

#[repr(C)]
pub struct DirectFBSurfaceCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: DirectFBSurfaceCreateFlagsEXT,
	dfb: *mut IDirectFB,
	surface: *mut IDirectFBSurface,
}

#[repr(C)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIA {
	s_type: StructureType,
	p_next: *const c_void,
	flags: ImagePipeSurfaceCreateFlagsFUCHSIA,
	image_pipe_handle: zx_handle_t,
}

#[repr(C)]
pub struct MemoryPriorityAllocateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	priority: f32,
}

#[repr(C)]
pub struct ColorBlendAdvancedEXT {
	advanced_blend_op: BlendOp,
	src_premultiplied: Bool32,
	dst_premultiplied: Bool32,
	blend_overlap: BlendOverlapEXT,
	clamp_results: Bool32,
}

#[repr(C)]
pub struct VideoPictureResourceInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	coded_offset: Offset2D,
	coded_extent: Extent2D,
	base_array_layer: u32,
	image_view_binding: ImageView,
}

#[repr(C)]
pub struct PhysicalDeviceShaderModuleIdentifierPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_module_identifier_algorithm_uuid: [u8; UUID_SIZE],
}

#[repr(C)]
pub struct ExportMemoryWin32HandleInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	p_attributes: *const SECURITY_ATTRIBUTES,
	dw_access: DWORD,
}

#[repr(C)]
pub struct StridedDeviceAddressRegionKHR {
	device_address: DeviceAddress,
	stride: DeviceSize,
	size: DeviceSize,
}

#[repr(C)]
pub struct VideoEncodeQualityLevelInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	quality_level: u32,
}

#[repr(C)]
pub struct SwapchainLatencyCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	latency_mode_enable: Bool32,
}

#[repr(C)]
pub struct BufferCollectionConstraintsInfoFUCHSIA {
	s_type: StructureType,
	p_next: *const c_void,
	min_buffer_count: u32,
	max_buffer_count: u32,
	min_buffer_count_for_camping: u32,
	min_buffer_count_for_dedicated_slack: u32,
	min_buffer_count_for_shared_slack: u32,
}

#[repr(C)]
pub struct PhysicalDeviceDisplacementMicromapPropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	max_displacement_micromap_subdivision_level: u32,
}

#[repr(C)]
pub struct PhysicalDeviceDepthClipControlFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	depth_clip_control: Bool32,
}

#[repr(C)]
pub struct DisplayModePropertiesKHR {
	display_mode: DisplayModeKHR,
	parameters: DisplayModeParametersKHR,
}

#[repr(C)]
pub struct ImageViewCaptureDescriptorDataInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	image_view: ImageView,
}

#[repr(C)]
pub struct PhysicalDeviceShaderIntegerDotProductFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_integer_dot_product: Bool32,
}

#[repr(C)]
pub struct MemoryAllocateFlagsInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: MemoryAllocateFlags,
	device_mask: u32,
}

#[repr(C)]
pub struct FrameBoundaryEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: FrameBoundaryFlagsEXT,
	frame_id: u64,
	image_count: u32,
	p_images: *const Image,
	buffer_count: u32,
	p_buffers: *const Buffer,
	tag_name: u64,
	tag_size: usize,
	p_tag: *const c_void,
}

#[repr(C)]
pub struct TraceRaysIndirectCommand2KHR {
	raygen_shader_record_address: DeviceAddress,
	raygen_shader_record_size: DeviceSize,
	miss_shader_binding_table_address: DeviceAddress,
	miss_shader_binding_table_size: DeviceSize,
	miss_shader_binding_table_stride: DeviceSize,
	hit_shader_binding_table_address: DeviceAddress,
	hit_shader_binding_table_size: DeviceSize,
	hit_shader_binding_table_stride: DeviceSize,
	callable_shader_binding_table_address: DeviceAddress,
	callable_shader_binding_table_size: DeviceSize,
	callable_shader_binding_table_stride: DeviceSize,
	width: u32,
	height: u32,
	depth: u32,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	fragment_density_map: Bool32,
	fragment_density_map_dynamic: Bool32,
	fragment_density_map_non_subsampled_images: Bool32,
}

#[repr(C)]
pub struct PipelineShaderStageCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineShaderStageCreateFlags,
	stage: ShaderStageFlagBits,
	module: ShaderModule,
	#[cfg(vulkan)]
	p_name: *const u8,
	#[cfg(vulkansc)]
	p_name: *const u8,
	p_specialization_info: *const SpecializationInfo,
}

#[repr(C)]
pub struct PipelineDepthStencilStateCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineDepthStencilStateCreateFlags,
	depth_test_enable: Bool32,
	depth_write_enable: Bool32,
	depth_compare_op: CompareOp,
	depth_bounds_test_enable: Bool32,
	stencil_test_enable: Bool32,
	front: StencilOpState,
	back: StencilOpState,
	min_depth_bounds: f32,
	max_depth_bounds: f32,
}

#[repr(C)]
pub struct ImportMemoryHostPointerInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	handle_type: ExternalMemoryHandleTypeFlagBits,
	p_host_pointer: *mut c_void,
}

#[repr(C)]
pub struct SciSyncAttributesInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	client_type: SciSyncClientTypeNV,
	primitive_type: SciSyncPrimitiveTypeNV,
}

#[repr(C)]
pub struct PhysicalDeviceColorWriteEnableFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	color_write_enable: Bool32,
}

#[repr(C)]
pub struct VideoDecodeH264DpbSlotInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	p_std_reference_info: *const StdVideoDecodeH264ReferenceInfo,
}

#[repr(C)]
pub struct VideoEncodeH264QualityLevelPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	preferred_rate_control_flags: VideoEncodeH264RateControlFlagsEXT,
	preferred_gop_frame_count: u32,
	preferred_idr_period: u32,
	preferred_consecutive_bframe_count: u32,
	preferred_temporal_layer_count: u32,
	preferred_constant_qp: VideoEncodeH264QpEXT,
	preferred_max_l0_reference_count: u32,
	preferred_max_l1_reference_count: u32,
	preferred_std_entropy_coding_mode_flag: Bool32,
}

#[repr(C)]
pub struct ImportMetalTextureInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	plane: ImageAspectFlagBits,
	mtl_texture: MTLTexture_id,
}

#[repr(C)]
pub struct PhysicalDeviceMemoryDecompressionFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	memory_decompression: Bool32,
}

#[repr(C)]
pub struct BindBufferMemoryInfo {
	s_type: StructureType,
	p_next: *const c_void,
	buffer: Buffer,
	memory: DeviceMemory,
	memory_offset: DeviceSize,
}

#[repr(C)]
pub struct SpecializationInfo {
	map_entry_count: u32,
	p_map_entries: *const SpecializationMapEntry,
	data_size: usize,
	p_data: *const c_void,
}

#[repr(C)]
pub struct PhysicalDeviceToolProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	name: [u8; MAX_EXTENSION_NAME_SIZE],
	version: [u8; MAX_EXTENSION_NAME_SIZE],
	purposes: ToolPurposeFlags,
	description: [u8; MAX_DESCRIPTION_SIZE],
	layer: [u8; MAX_EXTENSION_NAME_SIZE],
}

#[repr(C)]
pub struct PhysicalDeviceNonSeamlessCubeMapFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	non_seamless_cube_map: Bool32,
}

#[repr(C)]
pub struct PipelineCacheHeaderVersionSafetyCriticalOne {
	header_version_one: PipelineCacheHeaderVersionOne,
	validation_version: PipelineCacheValidationVersion,
	implementation_data: u32,
	pipeline_index_count: u32,
	pipeline_index_stride: u32,
	pipeline_index_offset: u64,
}

#[repr(C)]
pub struct LatencySubmissionPresentIdNV {
	s_type: StructureType,
	p_next: *const c_void,
	present_id: u64,
}

#[repr(C)]
pub struct AccelerationStructureDeviceAddressInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	acceleration_structure: AccelerationStructureKHR,
}

#[repr(C)]
pub struct CuModuleCreateInfoNVX {
	s_type: StructureType,
	p_next: *const c_void,
	data_size: usize,
	p_data: *const c_void,
}

#[repr(C)]
pub struct AttachmentSampleLocationsEXT {
	attachment_index: u32,
	sample_locations_info: SampleLocationsInfoEXT,
}

#[repr(C)]
pub struct DebugMarkerObjectTagInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	object_type: DebugReportObjectTypeEXT,
	object: u64,
	tag_name: u64,
	tag_size: usize,
	p_tag: *const c_void,
}

#[repr(C)]
pub struct BufferMemoryRequirementsInfo2 {
	s_type: StructureType,
	p_next: *const c_void,
	buffer: Buffer,
}

#[repr(C)]
pub struct MacOSSurfaceCreateInfoMVK {
	s_type: StructureType,
	p_next: *const c_void,
	flags: MacOSSurfaceCreateFlagsMVK,
	p_view: *const c_void,
}

#[repr(C)]
pub struct DescriptorGetInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	r#type: DescriptorType,
	data: DescriptorDataEXT,
}

#[repr(C)]
pub struct PhysicalDeviceCopyMemoryIndirectPropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	supported_queues: QueueFlags,
}

#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicState3PropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	dynamic_primitive_topology_unrestricted: Bool32,
}

#[repr(C)]
pub struct BufferConstraintsInfoFUCHSIA {
	s_type: StructureType,
	p_next: *const c_void,
	create_info: BufferCreateInfo,
	required_format_features: FormatFeatureFlags,
	buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
}

#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	max_update_after_bind_descriptors_in_all_pools: u32,
	shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
	shader_sampled_image_array_non_uniform_indexing_native: Bool32,
	shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
	shader_storage_image_array_non_uniform_indexing_native: Bool32,
	shader_input_attachment_array_non_uniform_indexing_native: Bool32,
	robust_buffer_access_update_after_bind: Bool32,
	quad_divergent_implicit_lod: Bool32,
	max_per_stage_descriptor_update_after_bind_samplers: u32,
	max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
	max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
	max_per_stage_descriptor_update_after_bind_sampled_images: u32,
	max_per_stage_descriptor_update_after_bind_storage_images: u32,
	max_per_stage_descriptor_update_after_bind_input_attachments: u32,
	max_per_stage_update_after_bind_resources: u32,
	max_descriptor_set_update_after_bind_samplers: u32,
	max_descriptor_set_update_after_bind_uniform_buffers: u32,
	max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
	max_descriptor_set_update_after_bind_storage_buffers: u32,
	max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
	max_descriptor_set_update_after_bind_sampled_images: u32,
	max_descriptor_set_update_after_bind_storage_images: u32,
	max_descriptor_set_update_after_bind_input_attachments: u32,
}

#[repr(C)]
pub struct ImageSubresourceRange {
	aspect_mask: ImageAspectFlags,
	base_mip_level: u32,
	level_count: u32,
	base_array_layer: u32,
	layer_count: u32,
}

#[repr(C)]
pub struct SubresourceLayout {
	offset: DeviceSize,
	size: DeviceSize,
	row_pitch: DeviceSize,
	array_pitch: DeviceSize,
	depth_pitch: DeviceSize,
}

#[repr(C)]
pub struct SurfacePresentScalingCapabilitiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	supported_present_scaling: PresentScalingFlagsEXT,
	supported_present_gravity_x: PresentGravityFlagsEXT,
	supported_present_gravity_y: PresentGravityFlagsEXT,
	min_scaled_image_extent: Extent2D,
	max_scaled_image_extent: Extent2D,
}

#[repr(C)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	memory_priority: Bool32,
}

#[repr(C)]
pub struct VideoEncodeH264NaluSliceInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	ant_qp: i32,
	p_std_slice_header: *const StdVideoEncodeH264SliceHeader,
}

#[repr(C)]
pub struct PresentTimesInfoGOOGLE {
	s_type: StructureType,
	p_next: *const c_void,
	swapchain_count: u32,
	p_times: *const PresentTimeGOOGLE,
}

#[repr(C)]
pub struct VideoEncodeUsageInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	video_usage_hints: VideoEncodeUsageFlagsKHR,
	video_content_hints: VideoEncodeContentFlagsKHR,
	tuning_mode: VideoEncodeTuningModeKHR,
}

#[repr(C)]
pub struct PhysicalDeviceVulkan13Properties {
	s_type: StructureType,
	p_next: *mut c_void,
	min_subgroup_size: u32,
	max_subgroup_size: u32,
	max_compute_workgroup_subgroups: u32,
	required_subgroup_size_stages: ShaderStageFlags,
	max_inline_uniform_block_size: u32,
	max_per_stage_descriptor_inline_uniform_blocks: u32,
	max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
	max_descriptor_set_inline_uniform_blocks: u32,
	max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
	max_inline_uniform_total_size: u32,
	integer_dot_product8_bit_unsigned_accelerated: Bool32,
	integer_dot_product8_bit_signed_accelerated: Bool32,
	integer_dot_product8_bit_mixed_signedness_accelerated: Bool32,
	integer_dot_product4x8_bit_packed_unsigned_accelerated: Bool32,
	integer_dot_product4x8_bit_packed_signed_accelerated: Bool32,
	integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Bool32,
	integer_dot_product16_bit_unsigned_accelerated: Bool32,
	integer_dot_product16_bit_signed_accelerated: Bool32,
	integer_dot_product16_bit_mixed_signedness_accelerated: Bool32,
	integer_dot_product32_bit_unsigned_accelerated: Bool32,
	integer_dot_product32_bit_signed_accelerated: Bool32,
	integer_dot_product32_bit_mixed_signedness_accelerated: Bool32,
	integer_dot_product64_bit_unsigned_accelerated: Bool32,
	integer_dot_product64_bit_signed_accelerated: Bool32,
	integer_dot_product64_bit_mixed_signedness_accelerated: Bool32,
	integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Bool32,
	integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Bool32,
	integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Bool32,
	integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Bool32,
	integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Bool32,
	integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: Bool32,
	integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Bool32,
	integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Bool32,
	integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Bool32,
	integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Bool32,
	integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Bool32,
	integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Bool32,
	integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Bool32,
	integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Bool32,
	integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Bool32,
	storage_texel_buffer_offset_alignment_bytes: DeviceSize,
	storage_texel_buffer_offset_single_texel_alignment: Bool32,
	uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
	uniform_texel_buffer_offset_single_texel_alignment: Bool32,
	max_buffer_size: DeviceSize,
}

#[repr(C)]
pub struct ImageCopy2 {
	s_type: StructureType,
	p_next: *const c_void,
	src_subresource: ImageSubresourceLayers,
	src_offset: Offset3D,
	dst_subresource: ImageSubresourceLayers,
	dst_offset: Offset3D,
	extent: Extent3D,
}

#[repr(C)]
pub struct PhysicalDeviceImageFormatInfo2 {
	s_type: StructureType,
	p_next: *const c_void,
	format: Format,
	r#type: ImageType,
	tiling: ImageTiling,
	usage: ImageUsageFlags,
	flags: ImageCreateFlags,
}

#[repr(C)]
pub struct AndroidSurfaceCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: AndroidSurfaceCreateFlagsKHR,
	window: *mut ANativeWindow,
}

#[repr(C)]
pub struct DebugMarkerObjectNameInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	object_type: DebugReportObjectTypeEXT,
	object: u64,
	p_object_name: *const u8,
}

#[repr(C)]
pub struct GetLatencyMarkerInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	timing_count: u32,
	p_timings: *mut LatencyTimingsFrameReportNV,
}

#[repr(C)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	separate_depth_stencil_layouts: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceCudaKernelLaunchPropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	compute_capability_minor: u32,
	compute_capability_major: u32,
}

#[repr(C)]
pub struct Offset2D {
	x: i32,
	y: i32,
}

#[repr(C)]
pub struct PhysicalDeviceCudaKernelLaunchFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	cuda_kernel_launch_features: Bool32,
}

#[repr(C)]
pub struct CommandBufferInheritanceViewportScissorInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	viewport_scissor2_d: Bool32,
	viewport_depth_count: u32,
	p_viewport_depths: *const Viewport,
}

#[repr(C)]
pub struct CudaModuleCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	data_size: usize,
	p_data: *const c_void,
}

#[repr(C)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	pipeline_executable_info: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelineFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	ray_tracing_pipeline: Bool32,
	ray_tracing_pipeline_shader_group_handle_capture_replay: Bool32,
	ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: Bool32,
	ray_tracing_pipeline_trace_rays_indirect: Bool32,
	ray_traversal_primitive_culling: Bool32,
}

#[repr(C)]
pub struct QueueFamilyGlobalPriorityPropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	priority_count: u32,
	priorities: [QueueGlobalPriorityKHR; MAX_GLOBAL_PRIORITY_SIZE_KHR],
}

#[repr(C)]
pub struct VideoEncodeH264SessionParametersGetInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	write_std_sps: Bool32,
	write_std_pps: Bool32,
	std_spsid: u32,
	std_ppsid: u32,
}

#[repr(C)]
pub struct PhysicalDeviceLimits {
	max_image_dimension1_d: u32,
	max_image_dimension2_d: u32,
	max_image_dimension3_d: u32,
	max_image_dimension_cube: u32,
	max_image_array_layers: u32,
	max_texel_buffer_elements: u32,
	max_uniform_buffer_range: u32,
	max_storage_buffer_range: u32,
	max_push_constants_size: u32,
	max_memory_allocation_count: u32,
	max_sampler_allocation_count: u32,
	buffer_image_granularity: DeviceSize,
	sparse_address_space_size: DeviceSize,
	max_bound_descriptor_sets: u32,
	max_per_stage_descriptor_samplers: u32,
	max_per_stage_descriptor_uniform_buffers: u32,
	max_per_stage_descriptor_storage_buffers: u32,
	max_per_stage_descriptor_sampled_images: u32,
	max_per_stage_descriptor_storage_images: u32,
	max_per_stage_descriptor_input_attachments: u32,
	max_per_stage_resources: u32,
	max_descriptor_set_samplers: u32,
	max_descriptor_set_uniform_buffers: u32,
	max_descriptor_set_uniform_buffers_dynamic: u32,
	max_descriptor_set_storage_buffers: u32,
	max_descriptor_set_storage_buffers_dynamic: u32,
	max_descriptor_set_sampled_images: u32,
	max_descriptor_set_storage_images: u32,
	max_descriptor_set_input_attachments: u32,
	max_vertex_input_attributes: u32,
	max_vertex_input_bindings: u32,
	max_vertex_input_attribute_offset: u32,
	max_vertex_input_binding_stride: u32,
	max_vertex_output_components: u32,
	max_tessellation_generation_level: u32,
	max_tessellation_patch_size: u32,
	max_tessellation_control_per_vertex_input_components: u32,
	max_tessellation_control_per_vertex_output_components: u32,
	max_tessellation_control_per_patch_output_components: u32,
	max_tessellation_control_total_output_components: u32,
	max_tessellation_evaluation_input_components: u32,
	max_tessellation_evaluation_output_components: u32,
	max_geometry_shader_invocations: u32,
	max_geometry_input_components: u32,
	max_geometry_output_components: u32,
	max_geometry_output_vertices: u32,
	max_geometry_total_output_components: u32,
	max_fragment_input_components: u32,
	max_fragment_output_attachments: u32,
	max_fragment_dual_src_attachments: u32,
	max_fragment_combined_output_resources: u32,
	max_compute_shared_memory_size: u32,
	max_compute_work_group_count: [u32; 3],
	max_compute_work_group_invocations: u32,
	max_compute_work_group_size: [u32; 3],
	sub_pixel_precision_bits: u32,
	sub_texel_precision_bits: u32,
	mipmap_precision_bits: u32,
	max_draw_indexed_index_value: u32,
	max_draw_indirect_count: u32,
	max_sampler_lod_bias: f32,
	max_sampler_anisotropy: f32,
	max_viewports: u32,
	max_viewport_dimensions: [u32; 2],
	viewport_bounds_range: [f32; 2],
	viewport_sub_pixel_bits: u32,
	min_memory_map_alignment: usize,
	min_texel_buffer_offset_alignment: DeviceSize,
	min_uniform_buffer_offset_alignment: DeviceSize,
	min_storage_buffer_offset_alignment: DeviceSize,
	min_texel_offset: i32,
	max_texel_offset: u32,
	min_texel_gather_offset: i32,
	max_texel_gather_offset: u32,
	min_interpolation_offset: f32,
	max_interpolation_offset: f32,
	sub_pixel_interpolation_offset_bits: u32,
	max_framebuffer_width: u32,
	max_framebuffer_height: u32,
	max_framebuffer_layers: u32,
	framebuffer_color_sample_counts: SampleCountFlags,
	framebuffer_depth_sample_counts: SampleCountFlags,
	framebuffer_stencil_sample_counts: SampleCountFlags,
	framebuffer_no_attachments_sample_counts: SampleCountFlags,
	max_color_attachments: u32,
	sampled_image_color_sample_counts: SampleCountFlags,
	sampled_image_integer_sample_counts: SampleCountFlags,
	sampled_image_depth_sample_counts: SampleCountFlags,
	sampled_image_stencil_sample_counts: SampleCountFlags,
	storage_image_sample_counts: SampleCountFlags,
	max_sample_mask_words: u32,
	timestamp_compute_and_graphics: Bool32,
	timestamp_period: f32,
	max_clip_distances: u32,
	max_cull_distances: u32,
	max_combined_clip_and_cull_distances: u32,
	discrete_queue_priorities: u32,
	point_size_range: [f32; 2],
	line_width_range: [f32; 2],
	point_size_granularity: f32,
	line_width_granularity: f32,
	strict_lines: Bool32,
	standard_sample_locations: Bool32,
	optimal_buffer_copy_offset_alignment: DeviceSize,
	optimal_buffer_copy_row_pitch_alignment: DeviceSize,
	non_coherent_atom_size: DeviceSize,
}

#[repr(C)]
pub struct IndirectCommandsStreamNV {
	buffer: Buffer,
	offset: DeviceSize,
}

#[repr(C)]
pub struct PerformanceQuerySubmitInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	counter_pass_index: u32,
}

#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	acquire_count: u32,
	p_acquire_syncs: *const DeviceMemory,
	p_acquire_keys: *const u64,
	p_acquire_timeouts: *const u32,
	release_count: u32,
	p_release_syncs: *const DeviceMemory,
	p_release_keys: *const u64,
}

#[repr(C)]
pub struct AccelerationStructureVersionInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	p_version_data: *const u8,
}

#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	buffer_device_address: Bool32,
	buffer_device_address_capture_replay: Bool32,
	buffer_device_address_multi_device: Bool32,
}

#[repr(C)]
pub struct ImportMemoryBufferCollectionFUCHSIA {
	s_type: StructureType,
	p_next: *const c_void,
	collection: BufferCollectionFUCHSIA,
	index: u32,
}

#[repr(C)]
pub struct AccelerationStructureGeometryInstancesDataKHR {
	s_type: StructureType,
	p_next: *const c_void,
	array_of_pointers: Bool32,
	data: DeviceOrHostAddressConstKHR,
}

#[repr(C)]
pub struct PhysicalDeviceDriverProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	driver_id: DriverId,
	driver_name: [u8; MAX_DRIVER_NAME_SIZE],
	driver_info: [u8; MAX_DRIVER_INFO_SIZE],
	conformance_version: ConformanceVersion,
}

#[repr(C)]
pub struct RenderPassCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: RenderPassCreateFlags,
	attachment_count: u32,
	p_attachments: *const AttachmentDescription,
	subpass_count: u32,
	p_subpasses: *const SubpassDescription,
	dependency_count: u32,
	p_dependencies: *const SubpassDependency,
}

#[repr(C)]
pub struct NativeBufferUsage2ANDROID {
	consumer: u64,
	producer: u64,
}

#[repr(C)]
pub struct PhysicalDevicePresentationPropertiesANDROID {
	s_type: StructureType,
	p_next: *const c_void,
	shared_image: Bool32,
}

#[repr(C)]
pub struct DeviceFaultVendorBinaryHeaderVersionOneEXT {
	header_size: u32,
	header_version: DeviceFaultVendorBinaryHeaderVersionEXT,
	vendor_id: u32,
	device_id: u32,
	driver_version: u32,
	pipeline_cache_uuid: [u8; UUID_SIZE],
	application_name_offset: u32,
	application_version: u32,
	engine_name_offset: u32,
	engine_version: u32,
	api_version: u32,
}

#[repr(C)]
pub struct SemaphoreCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: SemaphoreCreateFlags,
}

#[repr(C)]
pub struct CalibratedTimestampInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	time_domain: TimeDomainEXT,
}

#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	max_transform_feedback_streams: u32,
	max_transform_feedback_buffers: u32,
	max_transform_feedback_buffer_size: DeviceSize,
	max_transform_feedback_stream_data_size: u32,
	max_transform_feedback_buffer_data_size: u32,
	max_transform_feedback_buffer_data_stride: u32,
	transform_feedback_queries: Bool32,
	transform_feedback_streams_lines_triangles: Bool32,
	transform_feedback_rasterization_stream_select: Bool32,
	transform_feedback_draw: Bool32,
}

#[repr(C)]
pub struct PipelineCacheStageValidationIndexEntry {
	code_size: u64,
	code_offset: u64,
}

#[repr(C)]
pub struct PhysicalDeviceShadingRateImagePropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	shading_rate_texel_size: Extent2D,
	shading_rate_palette_size: u32,
	shading_rate_max_coarse_samples: u32,
}

#[repr(C)]
pub struct RenderPassStripeSubmitInfoARM {
	s_type: StructureType,
	p_next: *const c_void,
	stripe_semaphore_info_count: u32,
	p_stripe_semaphore_infos: *const SemaphoreSubmitInfo,
}

#[repr(C)]
pub struct PhysicalDeviceDrmPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	has_primary: Bool32,
	has_render: Bool32,
	primary_major: i64,
	primary_minor: i64,
	render_major: i64,
	render_minor: i64,
}

#[repr(C)]
pub struct PhysicalDeviceMaintenance4Features {
	s_type: StructureType,
	p_next: *mut c_void,
	maintenance4: Bool32,
}

#[repr(C)]
pub struct XYColorEXT {
	x: f32,
	y: f32,
}

#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureNV {
	s_type: StructureType,
	p_next: *const c_void,
	acceleration_structure_count: u32,
	p_acceleration_structures: *const AccelerationStructureNV,
}

#[repr(C)]
pub struct FilterCubicImageViewImageFormatPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	filter_cubic: Bool32,
	filter_cubic_minmax: Bool32,
}

#[repr(C)]
pub struct PipelineCoverageReductionStateCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineCoverageReductionStateCreateFlagsNV,
	coverage_reduction_mode: CoverageReductionModeNV,
}

#[repr(C)]
pub struct DirectDriverLoadingInfoLUNARG {
	s_type: StructureType,
	p_next: *mut c_void,
	flags: DirectDriverLoadingFlagsLUNARG,
	pfn_get_instance_proc_addr: PFN_vkGetInstanceProcAddrLUNARG,
}

#[repr(C)]
pub struct RenderPassBeginInfo {
	s_type: StructureType,
	p_next: *const c_void,
	render_pass: RenderPass,
	framebuffer: Framebuffer,
	render_area: Rect2D,
	clear_value_count: u32,
	p_clear_values: *const ClearValue,
}

#[repr(C)]
pub struct PhysicalDevicePrivateDataFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	private_data: Bool32,
}

#[repr(C)]
pub struct PhysicalDevicePerformanceQueryPropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	allow_command_buffer_query_copies: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	subsampled_loads: Bool32,
	subsampled_coarse_reconstruction_early_access: Bool32,
	max_subsampled_array_layers: u32,
	max_descriptor_set_subsampled_samplers: u32,
}

#[repr(C)]
pub struct HeadlessSurfaceCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: HeadlessSurfaceCreateFlagsEXT,
}

#[repr(C)]
pub struct VideoEncodeRateControlLayerInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	average_bitrate: u64,
	max_bitrate: u64,
	frame_rate_numerator: u32,
	frame_rate_denominator: u32,
}

#[repr(C)]
pub struct AllocationCallbacks {
	p_user_data: *mut c_void,
	pfn_allocation: PFN_vkAllocationFunction,
	pfn_reallocation: PFN_vkReallocationFunction,
	pfn_free: PFN_vkFreeFunction,
	pfn_internal_allocation: PFN_vkInternalAllocationNotification,
	pfn_internal_free: PFN_vkInternalFreeNotification,
}

#[repr(C)]
pub struct SamplerCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: SamplerCreateFlags,
	mag_filter: Filter,
	min_filter: Filter,
	mipmap_mode: SamplerMipmapMode,
	address_mode_u: SamplerAddressMode,
	address_mode_v: SamplerAddressMode,
	address_mode_w: SamplerAddressMode,
	mip_lod_bias: f32,
	anisotropy_enable: Bool32,
	max_anisotropy: f32,
	compare_enable: Bool32,
	compare_op: CompareOp,
	min_lod: f32,
	max_lod: f32,
	border_color: BorderColor,
	unnormalized_coordinates: Bool32,
}

#[repr(C)]
pub struct TransformMatrixKHR {
	matrix: [[f32; 3]; 4],
}

#[repr(C)]
pub struct VideoCapabilitiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	flags: VideoCapabilityFlagsKHR,
	min_bitstream_buffer_offset_alignment: DeviceSize,
	min_bitstream_buffer_size_alignment: DeviceSize,
	picture_access_granularity: Extent2D,
	min_coded_extent: Extent2D,
	max_coded_extent: Extent2D,
	max_dpb_slots: u32,
	max_active_reference_pictures: u32,
	std_header_version: ExtensionProperties,
}

#[repr(C)]
pub struct PhysicalDevicePresentBarrierFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	present_barrier: Bool32,
}

#[repr(C)]
pub struct PhysicalDevicePipelineRobustnessFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	pipeline_robustness: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	uniform_buffer_standard_layout: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceProperties2 {
	s_type: StructureType,
	p_next: *mut c_void,
	properties: PhysicalDeviceProperties,
}

#[repr(C)]
pub struct SubpassDependency {
	src_subpass: u32,
	dst_subpass: u32,
	src_stage_mask: PipelineStageFlags,
	dst_stage_mask: PipelineStageFlags,
	src_access_mask: AccessFlags,
	dst_access_mask: AccessFlags,
	dependency_flags: DependencyFlags,
}

#[repr(C)]
pub struct SRTDataNV {
	sx: f32,
	a: f32,
	b: f32,
	pvx: f32,
	sy: f32,
	c: f32,
	pvy: f32,
	sz: f32,
	pvz: f32,
	qx: f32,
	qy: f32,
	qz: f32,
	qw: f32,
	tx: f32,
	ty: f32,
	tz: f32,
}

#[repr(C)]
pub struct PhysicalDeviceCubicWeightsFeaturesQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	selectable_cubic_weights: Bool32,
}

#[repr(C)]
pub struct BufferCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: BufferCreateFlags,
	size: DeviceSize,
	usage: BufferUsageFlags,
	sharing_mode: SharingMode,
	queue_family_index_count: u32,
	p_queue_family_indices: *const u32,
}

#[repr(C)]
pub struct DisplayPresentInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	src_rect: Rect2D,
	dst_rect: Rect2D,
	persistent: Bool32,
}

#[repr(C)]
pub struct DeviceSemaphoreSciSyncPoolReservationCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	semaphore_sci_sync_pool_request_count: u32,
}

#[repr(C)]
pub struct ClearDepthStencilValue {
	depth: f32,
	stencil: u32,
}

#[repr(C)]
pub struct VideoEncodeH265SessionParametersCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	max_std_vpscount: u32,
	max_std_spscount: u32,
	max_std_ppscount: u32,
	p_parameters_add_info: *const VideoEncodeH265SessionParametersAddInfoEXT,
}

#[repr(C)]
pub struct PhysicalDeviceProvokingVertexPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	provoking_vertex_mode_per_pipeline: Bool32,
	transform_feedback_preserves_triangle_fan_provoking_vertex: Bool32,
}

#[repr(C)]
pub struct BufferCollectionBufferCreateInfoFUCHSIA {
	s_type: StructureType,
	p_next: *const c_void,
	collection: BufferCollectionFUCHSIA,
	index: u32,
}

#[repr(C)]
pub struct AmigoProfilingSubmitInfoSEC {
	s_type: StructureType,
	p_next: *const c_void,
	first_draw_timestamp: u64,
	swap_buffer_timestamp: u64,
}

#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	descriptor_set_count: u32,
	p_descriptor_counts: *const u32,
}

#[repr(C)]
pub struct ImageViewUsageCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	usage: ImageUsageFlags,
}

#[repr(C)]
pub struct SamplerYcbcrConversionInfo {
	s_type: StructureType,
	p_next: *const c_void,
	conversion: SamplerYcbcrConversion,
}

#[repr(C)]
pub struct SubpassDescription {
	flags: SubpassDescriptionFlags,
	pipeline_bind_point: PipelineBindPoint,
	input_attachment_count: u32,
	p_input_attachments: *const AttachmentReference,
	color_attachment_count: u32,
	p_color_attachments: *const AttachmentReference,
	p_resolve_attachments: *const AttachmentReference,
	p_depth_stencil_attachment: *const AttachmentReference,
	preserve_attachment_count: u32,
	p_preserve_attachments: *const u32,
}

#[repr(C)]
pub struct PhysicalDeviceImageProcessingPropertiesQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	max_weight_filter_phases: u32,
	max_weight_filter_dimension: Extent2D,
	max_block_match_region: Extent2D,
	max_box_filter_block_size: Extent2D,
}

#[repr(C)]
pub struct PipelinePropertiesIdentifierEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	pipeline_identifier: [u8; UUID_SIZE],
}

#[repr(C)]
pub struct PipelinePoolSize {
	s_type: StructureType,
	p_next: *const c_void,
	pool_entry_size: DeviceSize,
	pool_entry_count: u32,
}

#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
	s_type: StructureType,
	p_next: *mut c_void,
	per_view_position_all_components: Bool32,
}

#[repr(C)]
pub struct DeviceImageMemoryRequirements {
	s_type: StructureType,
	p_next: *const c_void,
	p_create_info: *const ImageCreateInfo,
	plane_aspect: ImageAspectFlagBits,
}

#[repr(C)]
pub struct VideoSessionMemoryRequirementsKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	memory_bind_index: u32,
	memory_requirements: MemoryRequirements,
}

#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	cooperative_matrix: Bool32,
	cooperative_matrix_robust_buffer_access: Bool32,
}

#[repr(C)]
pub struct SamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	enable_ydegamma: Bool32,
	enable_cb_cr_degamma: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceImageProcessing2PropertiesQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	max_block_match_window: Extent2D,
}

#[repr(C)]
pub struct PresentTimeGOOGLE {
	present_id: u32,
	desired_present_time: u64,
}

#[repr(C)]
pub struct SpecializationMapEntry {
	ant_id: u32,
	offset: u32,
	size: usize,
}

#[repr(C)]
pub struct InstanceCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: InstanceCreateFlags,
	p_application_info: *const ApplicationInfo,
	enabled_layer_count: u32,
	pp_enabled_layer_names: *const *const u8,
	enabled_extension_count: u32,
	pp_enabled_extension_names: *const *const u8,
}

#[repr(C)]
pub struct ExportMemorySciBufInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	p_attributes: NvSciBufAttrList,
}

#[repr(C)]
pub struct IndirectCommandsLayoutCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	flags: IndirectCommandsLayoutUsageFlagsNV,
	pipeline_bind_point: PipelineBindPoint,
	token_count: u32,
	p_tokens: *const IndirectCommandsLayoutTokenNV,
	stream_count: u32,
	p_stream_strides: *const u32,
}

#[repr(C)]
pub struct AccelerationStructureGeometryAabbsDataKHR {
	s_type: StructureType,
	p_next: *const c_void,
	data: DeviceOrHostAddressConstKHR,
	stride: DeviceSize,
}

#[repr(C)]
pub struct ImageToMemoryCopyEXT {
	s_type: StructureType,
	p_next: *const c_void,
	p_host_pointer: *mut c_void,
	memory_row_length: u32,
	memory_image_height: u32,
	image_subresource: ImageSubresourceLayers,
	image_offset: Offset3D,
	image_extent: Extent3D,
}

#[repr(C)]
pub struct PhysicalDeviceConservativeRasterizationPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	primitive_overestimation_size: f32,
	max_extra_primitive_overestimation_size: f32,
	extra_primitive_overestimation_size_granularity: f32,
	primitive_underestimation: Bool32,
	conservative_point_and_line_rasterization: Bool32,
	degenerate_triangles_rasterized: Bool32,
	degenerate_lines_rasterized: Bool32,
	fully_covered_fragment_shader_input_variable: Bool32,
	conservative_rasterization_post_depth_coverage: Bool32,
}

#[repr(C)]
pub struct DeviceQueueGlobalPriorityCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	global_priority: QueueGlobalPriorityKHR,
}

#[repr(C)]
pub struct InitializePerformanceApiInfoINTEL {
	s_type: StructureType,
	p_next: *const c_void,
	p_user_data: *mut c_void,
}

#[repr(C)]
pub struct PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	attachment_feedback_loop_dynamic_state: Bool32,
}

#[repr(C)]
pub struct AccelerationStructureInstanceKHR {
	transform: TransformMatrixKHR,
	instance_custom_index: u32,
	mask: u32,
	instance_shader_binding_table_record_offset: u32,
	flags: GeometryInstanceFlagsKHR,
	acceleration_structure_reference: u64,
}

#[repr(C)]
pub struct SurfacePresentModeCompatibilityEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	present_mode_count: u32,
	p_present_modes: *mut PresentModeKHR,
}

#[repr(C)]
pub struct QueryPoolPerformanceQueryCreateInfoINTEL {
	s_type: StructureType,
	p_next: *const c_void,
	performance_counters_sampling: QueryPoolSamplingModeINTEL,
}

#[repr(C)]
pub struct CopyImageInfo2 {
	s_type: StructureType,
	p_next: *const c_void,
	src_image: Image,
	src_image_layout: ImageLayout,
	dst_image: Image,
	dst_image_layout: ImageLayout,
	region_count: u32,
	p_regions: *const ImageCopy2,
}

#[repr(C)]
pub struct DescriptorBufferBindingInfoEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	address: DeviceAddress,
	usage: BufferUsageFlags,
}

#[repr(C)]
pub struct PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
	s_type: StructureType,
	p_next: *mut c_void,
	descriptor_set_host_mapping: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	rasterization_order_color_attachment_access: Bool32,
	rasterization_order_depth_attachment_access: Bool32,
	rasterization_order_stencil_attachment_access: Bool32,
}

#[repr(C)]
pub struct PerformanceCounterDescriptionKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	flags: PerformanceCounterDescriptionFlagsKHR,
	name: [u8; MAX_DESCRIPTION_SIZE],
	category: [u8; MAX_DESCRIPTION_SIZE],
	description: [u8; MAX_DESCRIPTION_SIZE],
}

#[repr(C)]
pub struct PerformanceConfigurationAcquireInfoINTEL {
	s_type: StructureType,
	p_next: *const c_void,
	r#type: PerformanceConfigurationTypeINTEL,
}

#[repr(C)]
pub struct ExportMetalCommandQueueInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	queue: Queue,
	mtl_command_queue: MTLCommandQueue_id,
}

#[repr(C)]
pub struct PhysicalDeviceShaderCorePropertiesARM {
	s_type: StructureType,
	p_next: *mut c_void,
	pixel_rate: u32,
	texel_rate: u32,
	fma_rate: u32,
}

#[repr(C)]
pub struct ImageResolve {
	src_subresource: ImageSubresourceLayers,
	src_offset: Offset3D,
	dst_subresource: ImageSubresourceLayers,
	dst_offset: Offset3D,
	extent: Extent3D,
}

#[repr(C)]
pub struct PhysicalDeviceSparseImageFormatInfo2 {
	s_type: StructureType,
	p_next: *const c_void,
	format: Format,
	r#type: ImageType,
	samples: SampleCountFlagBits,
	usage: ImageUsageFlags,
	tiling: ImageTiling,
}

#[repr(C)]
pub struct VideoEncodeH265SessionParametersGetInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	write_std_vps: Bool32,
	write_std_sps: Bool32,
	write_std_pps: Bool32,
	std_vpsid: u32,
	std_spsid: u32,
	std_ppsid: u32,
}

#[repr(C)]
pub struct BindVertexBufferIndirectCommandNV {
	buffer_address: DeviceAddress,
	size: u32,
	stride: u32,
}

#[repr(C)]
pub struct DrmFormatModifierPropertiesListEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	drm_format_modifier_count: u32,
	p_drm_format_modifier_properties: *mut DrmFormatModifierPropertiesEXT,
}

#[repr(C)]
pub struct SparseImageOpaqueMemoryBindInfo {
	image: Image,
	bind_count: u32,
	p_binds: *const SparseMemoryBind,
}

#[repr(C)]
pub struct VideoEncodeH265DpbSlotInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	p_std_reference_info: *const StdVideoEncodeH265ReferenceInfo,
}

#[repr(C)]
pub struct PhysicalDeviceSubpassShadingPropertiesHUAWEI {
	s_type: StructureType,
	p_next: *mut c_void,
	max_subpass_shading_workgroup_size_aspect_ratio: u32,
}

#[repr(C)]
pub struct MutableDescriptorTypeListEXT {
	descriptor_type_count: u32,
	p_descriptor_types: *const DescriptorType,
}

#[repr(C)]
pub struct AndroidHardwareBufferFormatPropertiesANDROID {
	s_type: StructureType,
	p_next: *mut c_void,
	format: Format,
	external_format: u64,
	format_features: FormatFeatureFlags,
	sampler_ycbcr_conversion_components: ComponentMapping,
	suggested_ycbcr_model: SamplerYcbcrModelConversion,
	suggested_ycbcr_range: SamplerYcbcrRange,
	suggested_xchroma_offset: ChromaLocation,
	suggested_ychroma_offset: ChromaLocation,
}

#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicState3FeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	extended_dynamic_state3_tessellation_domain_origin: Bool32,
	extended_dynamic_state3_depth_clamp_enable: Bool32,
	extended_dynamic_state3_polygon_mode: Bool32,
	extended_dynamic_state3_rasterization_samples: Bool32,
	extended_dynamic_state3_sample_mask: Bool32,
	extended_dynamic_state3_alpha_to_coverage_enable: Bool32,
	extended_dynamic_state3_alpha_to_one_enable: Bool32,
	extended_dynamic_state3_logic_op_enable: Bool32,
	extended_dynamic_state3_color_blend_enable: Bool32,
	extended_dynamic_state3_color_blend_equation: Bool32,
	extended_dynamic_state3_color_write_mask: Bool32,
	extended_dynamic_state3_rasterization_stream: Bool32,
	extended_dynamic_state3_conservative_rasterization_mode: Bool32,
	extended_dynamic_state3_extra_primitive_overestimation_size: Bool32,
	extended_dynamic_state3_depth_clip_enable: Bool32,
	extended_dynamic_state3_sample_locations_enable: Bool32,
	extended_dynamic_state3_color_blend_advanced: Bool32,
	extended_dynamic_state3_provoking_vertex_mode: Bool32,
	extended_dynamic_state3_line_rasterization_mode: Bool32,
	extended_dynamic_state3_line_stipple_enable: Bool32,
	extended_dynamic_state3_depth_clip_negative_one_to_one: Bool32,
	extended_dynamic_state3_viewport_wscaling_enable: Bool32,
	extended_dynamic_state3_viewport_swizzle: Bool32,
	extended_dynamic_state3_coverage_to_color_enable: Bool32,
	extended_dynamic_state3_coverage_to_color_location: Bool32,
	extended_dynamic_state3_coverage_modulation_mode: Bool32,
	extended_dynamic_state3_coverage_modulation_table_enable: Bool32,
	extended_dynamic_state3_coverage_modulation_table: Bool32,
	extended_dynamic_state3_coverage_reduction_mode: Bool32,
	extended_dynamic_state3_representative_fragment_test_enable: Bool32,
	extended_dynamic_state3_shading_rate_image_enable: Bool32,
}

#[repr(C)]
pub struct SemaphoreSignalInfo {
	s_type: StructureType,
	p_next: *const c_void,
	semaphore: Semaphore,
	value: u64,
}

#[repr(C)]
pub struct PipelineRasterizationLineStateCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	line_rasterization_mode: LineRasterizationModeEXT,
	stippled_line_enable: Bool32,
	line_stipple_factor: u32,
	line_stipple_pattern: u16,
}

#[repr(C)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	src_premultiplied: Bool32,
	dst_premultiplied: Bool32,
	blend_overlap: BlendOverlapEXT,
}

#[repr(C)]
pub struct PhysicalDeviceDepthStencilResolveProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	supported_depth_resolve_modes: ResolveModeFlags,
	supported_stencil_resolve_modes: ResolveModeFlags,
	independent_resolve_none: Bool32,
	independent_resolve: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	fragment_density_map_offset: Bool32,
}

#[repr(C)]
pub struct ApplicationInfo {
	s_type: StructureType,
	p_next: *const c_void,
	p_application_name: *const u8,
	application_version: u32,
	p_engine_name: *const u8,
	engine_version: u32,
	api_version: u32,
}

#[repr(C)]
pub struct ImagePlaneMemoryRequirementsInfo {
	s_type: StructureType,
	p_next: *const c_void,
	plane_aspect: ImageAspectFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	coverage_reduction_mode: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceHostImageCopyFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	host_image_copy: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	heap_budget: [DeviceSize; MAX_MEMORY_HEAPS],
	heap_usage: [DeviceSize; MAX_MEMORY_HEAPS],
}

#[repr(C)]
pub struct VideoSessionCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	queue_family_index: u32,
	flags: VideoSessionCreateFlagsKHR,
	p_video_profile: *const VideoProfileInfoKHR,
	picture_format: Format,
	max_coded_extent: Extent2D,
	reference_picture_format: Format,
	max_dpb_slots: u32,
	max_active_reference_pictures: u32,
	p_std_header_version: *const ExtensionProperties,
}

#[repr(C)]
pub struct VideoEncodeRateControlInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: VideoEncodeRateControlFlagsKHR,
	rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
	layer_count: u32,
	p_layers: *const VideoEncodeRateControlLayerInfoKHR,
	virtual_buffer_size_in_ms: u32,
	initial_virtual_buffer_size_in_ms: u32,
}

#[repr(C)]
pub struct PhysicalDevicePipelineRobustnessPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	default_robustness_storage_buffers: PipelineRobustnessBufferBehaviorEXT,
	default_robustness_uniform_buffers: PipelineRobustnessBufferBehaviorEXT,
	default_robustness_vertex_inputs: PipelineRobustnessBufferBehaviorEXT,
	default_robustness_images: PipelineRobustnessImageBehaviorEXT,
}

#[repr(C)]
pub struct LayerProperties {
	layer_name: [u8; MAX_EXTENSION_NAME_SIZE],
	spec_version: u32,
	implementation_version: u32,
	description: [u8; MAX_DESCRIPTION_SIZE],
}

#[repr(C)]
pub struct XlibSurfaceCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: XlibSurfaceCreateFlagsKHR,
	dpy: *mut Display,
	window: Window,
}

#[repr(C)]
pub struct DisplayPlaneProperties2KHR {
	s_type: StructureType,
	p_next: *mut c_void,
	display_plane_properties: DisplayPlanePropertiesKHR,
}

#[repr(C)]
pub struct FramebufferMixedSamplesCombinationNV {
	s_type: StructureType,
	p_next: *mut c_void,
	coverage_reduction_mode: CoverageReductionModeNV,
	rasterization_samples: SampleCountFlagBits,
	depth_stencil_samples: SampleCountFlags,
	color_samples: SampleCountFlags,
}

#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	max_graphics_shader_group_count: u32,
	max_indirect_sequence_count: u32,
	max_indirect_commands_token_count: u32,
	max_indirect_commands_stream_count: u32,
	max_indirect_commands_token_offset: u32,
	max_indirect_commands_stream_stride: u32,
	min_sequences_count_buffer_offset_alignment: u32,
	min_sequences_index_buffer_offset_alignment: u32,
	min_indirect_commands_buffer_offset_alignment: u32,
}

#[repr(C)]
pub struct AttachmentDescription {
	flags: AttachmentDescriptionFlags,
	format: Format,
	samples: SampleCountFlagBits,
	load_op: AttachmentLoadOp,
	store_op: AttachmentStoreOp,
	stencil_load_op: AttachmentLoadOp,
	stencil_store_op: AttachmentStoreOp,
	initial_layout: ImageLayout,
	final_layout: ImageLayout,
}

#[repr(C)]
pub struct VideoEncodeH265FrameSizeEXT {
	frame_isize: u32,
	frame_psize: u32,
	frame_bsize: u32,
}

#[repr(C)]
pub struct RenderingInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: RenderingFlags,
	render_area: Rect2D,
	layer_count: u32,
	view_mask: u32,
	color_attachment_count: u32,
	p_color_attachments: *const RenderingAttachmentInfo,
	p_depth_attachment: *const RenderingAttachmentInfo,
	p_stencil_attachment: *const RenderingAttachmentInfo,
}

#[repr(C)]
pub struct SetStateFlagsIndirectCommandNV {
	data: u32,
}

#[repr(C)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	full_screen_exclusive_supported: Bool32,
}

#[repr(C)]
pub struct OpaqueCaptureDescriptorDataCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	opaque_capture_descriptor_data: *const c_void,
}

#[repr(C)]
pub struct ValidationCacheCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: ValidationCacheCreateFlagsEXT,
	initial_data_size: usize,
	p_initial_data: *const c_void,
}

#[repr(C)]
pub struct DisplayPlaneCapabilitiesKHR {
	supported_alpha: DisplayPlaneAlphaFlagsKHR,
	min_src_position: Offset2D,
	max_src_position: Offset2D,
	min_src_extent: Extent2D,
	max_src_extent: Extent2D,
	min_dst_position: Offset2D,
	max_dst_position: Offset2D,
	min_dst_extent: Extent2D,
	max_dst_extent: Extent2D,
}

#[repr(C)]
pub struct ImportMemorySciBufInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	handle_type: ExternalMemoryHandleTypeFlagBits,
	handle: NvSciBufObj,
}

#[repr(C)]
pub struct DescriptorPoolSize {
	r#type: DescriptorType,
	descriptor_count: u32,
}

#[repr(C)]
pub struct LayerSettingsCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	setting_count: u32,
	p_settings: *const LayerSettingEXT,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRatePropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	min_fragment_shading_rate_attachment_texel_size: Extent2D,
	max_fragment_shading_rate_attachment_texel_size: Extent2D,
	max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
	primitive_fragment_shading_rate_with_multiple_viewports: Bool32,
	layered_shading_rate_attachments: Bool32,
	fragment_shading_rate_non_trivial_combiner_ops: Bool32,
	max_fragment_size: Extent2D,
	max_fragment_size_aspect_ratio: u32,
	max_fragment_shading_rate_coverage_samples: u32,
	max_fragment_shading_rate_rasterization_samples: SampleCountFlagBits,
	fragment_shading_rate_with_shader_depth_stencil_writes: Bool32,
	fragment_shading_rate_with_sample_mask: Bool32,
	fragment_shading_rate_with_shader_sample_mask: Bool32,
	fragment_shading_rate_with_conservative_rasterization: Bool32,
	fragment_shading_rate_with_fragment_shader_interlock: Bool32,
	fragment_shading_rate_with_custom_sample_locations: Bool32,
	fragment_shading_rate_strict_multiply_combiner: Bool32,
}

#[repr(C)]
pub struct GraphicsPipelineLibraryCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: GraphicsPipelineLibraryFlagsEXT,
}

#[repr(C)]
pub struct PhysicalDeviceImageCompressionControlFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	image_compression_control: Bool32,
}

#[repr(C)]
pub struct RenderPassStripeBeginInfoARM {
	s_type: StructureType,
	p_next: *const c_void,
	stripe_info_count: u32,
	p_stripe_infos: *mut RenderPassStripeInfoARM,
}

#[repr(C)]
pub struct QueryPoolVideoEncodeFeedbackCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	encode_feedback_flags: VideoEncodeFeedbackFlagsKHR,
}

#[repr(C)]
pub struct ExternalMemoryProperties {
	external_memory_features: ExternalMemoryFeatureFlags,
	export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
	compatible_handle_types: ExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	fragment_density_map_deferred: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceSubgroupSizeControlProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	min_subgroup_size: u32,
	max_subgroup_size: u32,
	max_compute_workgroup_subgroups: u32,
	required_subgroup_size_stages: ShaderStageFlags,
}

#[repr(C)]
pub struct SetLatencyMarkerInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	present_id: u64,
	marker: LatencyMarkerNV,
}

#[repr(C)]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMD {
	s_type: StructureType,
	p_next: *mut c_void,
	device_coherent_memory: Bool32,
}

#[repr(C)]
pub struct DrmFormatModifierPropertiesEXT {
	drm_format_modifier: u64,
	drm_format_modifier_plane_count: u32,
	drm_format_modifier_tiling_features: FormatFeatureFlags,
}

#[repr(C)]
pub struct VideoProfileInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	video_codec_operation: VideoCodecOperationFlagBitsKHR,
	chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
	luma_bit_depth: VideoComponentBitDepthFlagsKHR,
	chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
}

#[repr(C)]
pub struct PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	image_compression_control_swapchain: Bool32,
}

#[repr(C)]
pub struct PipelineFragmentShadingRateStateCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	fragment_size: Extent2D,
	combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
}

#[repr(C)]
pub struct BindShaderGroupIndirectCommandNV {
	group_index: u32,
}

#[repr(C)]
pub struct PhysicalDeviceNestedCommandBufferFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	nested_command_buffer: Bool32,
	nested_command_buffer_rendering: Bool32,
	nested_command_buffer_simultaneous_use: Bool32,
}

#[repr(C)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	drm_format_modifier: u64,
	drm_format_modifier_plane_count: u32,
	p_plane_layouts: *const SubresourceLayout,
}

#[repr(C)]
pub struct PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_image_int64_atomics: Bool32,
	sparse_image_int64_atomics: Bool32,
}

#[repr(C)]
pub struct AndroidHardwareBufferFormatResolvePropertiesANDROID {
	s_type: StructureType,
	p_next: *mut c_void,
	color_attachment_format: Format,
}

#[repr(C)]
pub struct TraceRaysIndirectCommandKHR {
	width: u32,
	height: u32,
	depth: u32,
}

#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	cooperative_matrix_supported_stages: ShaderStageFlags,
}

#[repr(C)]
pub struct SubpassEndInfo {
	s_type: StructureType,
	p_next: *const c_void,
}

#[repr(C)]
pub struct PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	multisampled_render_to_single_sampled: Bool32,
}

#[repr(C)]
pub struct DisplayPlanePropertiesKHR {
	current_display: DisplayKHR,
	current_stack_index: u32,
}

#[repr(C)]
pub struct DebugReportCallbackCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: DebugReportFlagsEXT,
	pfn_callback: PFN_vkDebugReportCallbackEXT,
	p_user_data: *mut c_void,
}

#[repr(C)]
pub struct PhysicalDeviceVariablePointersFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	variable_pointers_storage_buffer: Bool32,
	variable_pointers: Bool32,
}

#[repr(C)]
pub struct ExportFenceWin32HandleInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	p_attributes: *const SECURITY_ATTRIBUTES,
	dw_access: DWORD,
	name: LPCWSTR,
}

#[repr(C)]
pub struct SurfaceFormat2KHR {
	s_type: StructureType,
	p_next: *mut c_void,
	surface_format: SurfaceFormatKHR,
}

#[repr(C)]
pub struct AcquireNextImageInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	swapchain: SwapchainKHR,
	timeout: u64,
	semaphore: Semaphore,
	fence: Fence,
	device_mask: u32,
}

#[repr(C)]
pub struct MemoryOpaqueCaptureAddressAllocateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	opaque_capture_address: u64,
}

#[repr(C)]
pub struct PhysicalDeviceShaderDrawParametersFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_draw_parameters: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceImageViewMinLodFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	min_lod: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceExternalImageFormatInfo {
	s_type: StructureType,
	p_next: *const c_void,
	handle_type: ExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VideoEncodeH265QualityLevelPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	preferred_rate_control_flags: VideoEncodeH265RateControlFlagsEXT,
	preferred_gop_frame_count: u32,
	preferred_idr_period: u32,
	preferred_consecutive_bframe_count: u32,
	preferred_sub_layer_count: u32,
	preferred_constant_qp: VideoEncodeH265QpEXT,
	preferred_max_l0_reference_count: u32,
	preferred_max_l1_reference_count: u32,
}

#[repr(C)]
pub struct DisplayEventInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	display_event: DisplayEventTypeEXT,
}

#[repr(C)]
pub struct ExportMetalObjectsInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
}

#[repr(C)]
pub struct FramebufferCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: FramebufferCreateFlags,
	render_pass: RenderPass,
	attachment_count: u32,
	p_attachments: *const ImageView,
	width: u32,
	height: u32,
	layers: u32,
}

#[repr(C)]
pub struct AccelerationStructureBuildSizesInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	acceleration_structure_size: DeviceSize,
	update_scratch_size: DeviceSize,
	build_scratch_size: DeviceSize,
}

#[repr(C)]
pub struct CoarseSampleLocationNV {
	pixel_x: u32,
	pixel_y: u32,
	sample: u32,
}

#[repr(C)]
pub struct AndroidHardwareBufferUsageANDROID {
	s_type: StructureType,
	p_next: *mut c_void,
	android_hardware_buffer_usage: u64,
}

#[repr(C)]
pub struct DeviceQueueCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: DeviceQueueCreateFlags,
	queue_family_index: u32,
	queue_count: u32,
	p_queue_priorities: *const f32,
}

#[repr(C)]
pub struct PipelineCoverageToColorStateCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineCoverageToColorStateCreateFlagsNV,
	coverage_to_color_enable: Bool32,
	coverage_to_color_location: u32,
}

#[repr(C)]
pub struct SurfaceFormatKHR {
	format: Format,
	color_space: ColorSpaceKHR,
}

#[repr(C)]
pub struct PerformanceMarkerInfoINTEL {
	s_type: StructureType,
	p_next: *const c_void,
	marker: u64,
}

#[repr(C)]
pub struct CheckpointData2NV {
	s_type: StructureType,
	p_next: *mut c_void,
	stage: PipelineStageFlags2,
	p_checkpoint_marker: *mut c_void,
}

#[repr(C)]
pub struct AttachmentReferenceStencilLayout {
	s_type: StructureType,
	p_next: *mut c_void,
	stencil_layout: ImageLayout,
}

#[repr(C)]
pub struct PhysicalDeviceRayTracingPositionFetchFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	ray_tracing_position_fetch: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceExternalMemorySciBufFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	sci_buf_import: Bool32,
	sci_buf_export: Bool32,
}

#[repr(C)]
pub struct PipelineExecutableInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	pipeline: Pipeline,
	executable_index: u32,
}

#[repr(C)]
pub struct PhysicalDeviceOpacityMicromapFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	micromap: Bool32,
	micromap_capture_replay: Bool32,
	micromap_host_commands: Bool32,
}

#[repr(C)]
pub struct QueryPoolCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: QueryPoolCreateFlags,
	query_type: QueryType,
	query_count: u32,
	pipeline_statistics: QueryPipelineStatisticFlags,
}

#[repr(C)]
pub struct PhysicalDeviceScalarBlockLayoutFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	scalar_block_layout: Bool32,
}

#[repr(C)]
pub struct ValidationFeaturesEXT {
	s_type: StructureType,
	p_next: *const c_void,
	enabled_validation_feature_count: u32,
	p_enabled_validation_features: *const ValidationFeatureEnableEXT,
	disabled_validation_feature_count: u32,
	p_disabled_validation_features: *const ValidationFeatureDisableEXT,
}

#[repr(C)]
pub struct PhysicalDeviceInlineUniformBlockFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	inline_uniform_block: Bool32,
	descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
}

#[repr(C)]
pub struct BaseOutStructure {
	s_type: StructureType,
	p_next: *mut BaseOutStructure,
}

#[repr(C)]
pub struct MemoryHostPointerPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	memory_type_bits: u32,
}

#[repr(C)]
pub struct CopyAccelerationStructureToMemoryInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	src: AccelerationStructureKHR,
	dst: DeviceOrHostAddressKHR,
	mode: CopyAccelerationStructureModeKHR,
}

#[repr(C)]
pub struct SharedPresentSurfaceCapabilitiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	shared_present_supported_usage_flags: ImageUsageFlags,
}

#[repr(C)]
pub struct AndroidHardwareBufferPropertiesANDROID {
	s_type: StructureType,
	p_next: *mut c_void,
	allocation_size: DeviceSize,
	memory_type_bits: u32,
}

#[repr(C)]
pub struct DeviceDiagnosticsConfigCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	flags: DeviceDiagnosticsConfigFlagsNV,
}

#[repr(C)]
pub struct DepthBiasRepresentationInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	depth_bias_representation: DepthBiasRepresentationEXT,
	depth_bias_exact: Bool32,
}

#[repr(C)]
pub struct AccelerationStructureCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	compacted_size: DeviceSize,
	info: AccelerationStructureInfoNV,
}

#[repr(C)]
pub struct ImageViewASTCDecodeModeEXT {
	s_type: StructureType,
	p_next: *const c_void,
	decode_mode: Format,
}

#[repr(C)]
pub struct DeviceGroupSwapchainCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	modes: DeviceGroupPresentModeFlagsKHR,
}

#[repr(C)]
pub struct PhysicalDeviceShaderAtomicInt64Features {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_buffer_int64_atomics: Bool32,
	shader_shared_int64_atomics: Bool32,
}

#[repr(C)]
pub struct AccelerationStructureMatrixMotionInstanceNV {
	transform_t0: TransformMatrixKHR,
	transform_t1: TransformMatrixKHR,
	instance_custom_index: u32,
	mask: u32,
	instance_shader_binding_table_record_offset: u32,
	flags: GeometryInstanceFlagsKHR,
	acceleration_structure_reference: u64,
}

#[repr(C)]
pub struct SparseImageFormatProperties {
	aspect_mask: ImageAspectFlags,
	image_granularity: Extent3D,
	flags: SparseImageFormatFlags,
}

#[repr(C)]
pub struct ImageBlit {
	src_subresource: ImageSubresourceLayers,
	src_offsets: [Offset3D; 2],
	dst_subresource: ImageSubresourceLayers,
	dst_offsets: [Offset3D; 2],
}

#[repr(C)]
pub struct ConditionalRenderingBeginInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	buffer: Buffer,
	offset: DeviceSize,
	flags: ConditionalRenderingFlagsEXT,
}

#[repr(C)]
pub struct PipelineCacheHeaderVersionOne {
	header_size: u32,
	header_version: PipelineCacheHeaderVersion,
	vendor_id: u32,
	device_id: u32,
	pipeline_cache_uuid: [u8; UUID_SIZE],
}

#[repr(C)]
pub struct VertexInputBindingDescription2EXT {
	s_type: StructureType,
	p_next: *mut c_void,
	binding: u32,
	stride: u32,
	input_rate: VertexInputRate,
	divisor: u32,
}

#[repr(C)]
pub struct ProtectedSubmitInfo {
	s_type: StructureType,
	p_next: *const c_void,
	protected_submit: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	multiview_per_view_render_areas: Bool32,
}

#[repr(C)]
pub struct QueryLowLatencySupportNV {
	s_type: StructureType,
	p_next: *const c_void,
	p_queried_low_latency_data: *mut c_void,
}

#[repr(C)]
pub struct PhysicalDevice8BitStorageFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	storage_buffer8_bit_access: Bool32,
	uniform_and_storage_buffer8_bit_access: Bool32,
	storage_push_constant8: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	image_footprint: Bool32,
}

#[repr(C)]
pub struct BufferCopy2 {
	s_type: StructureType,
	p_next: *const c_void,
	src_offset: DeviceSize,
	dst_offset: DeviceSize,
	size: DeviceSize,
}

#[repr(C)]
pub struct BindImageMemoryInfo {
	s_type: StructureType,
	p_next: *const c_void,
	image: Image,
	memory: DeviceMemory,
	memory_offset: DeviceSize,
}

#[repr(C)]
pub struct PhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	pipeline_library_group_handles: Bool32,
}

#[repr(C)]
pub struct VideoReferenceSlotInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	slot_index: i32,
	p_picture_resource: *const VideoPictureResourceInfoKHR,
}

#[repr(C)]
pub struct PhysicalDeviceRayQueryFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	ray_query: Bool32,
}

#[repr(C)]
pub struct VideoEncodeH265PictureInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	nalu_slice_segment_entry_count: u32,
	p_nalu_slice_segment_entries: *const VideoEncodeH265NaluSliceSegmentInfoEXT,
	p_std_picture_info: *const StdVideoEncodeH265PictureInfo,
}

#[repr(C)]
pub struct PhysicalDeviceLegacyDitheringFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	legacy_dithering: Bool32,
}

#[repr(C)]
pub struct QueueFamilyProperties2 {
	s_type: StructureType,
	p_next: *mut c_void,
	queue_family_properties: QueueFamilyProperties,
}

#[repr(C)]
pub struct CopyMemoryToImageIndirectCommandNV {
	src_address: DeviceAddress,
	buffer_row_length: u32,
	buffer_image_height: u32,
	image_subresource: ImageSubresourceLayers,
	image_offset: Offset3D,
	image_extent: Extent3D,
}

#[repr(C)]
pub struct AccelerationStructureMemoryRequirementsInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	r#type: AccelerationStructureMemoryRequirementsTypeNV,
	acceleration_structure: AccelerationStructureNV,
}

#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	max_vertex_attrib_divisor: u32,
}

#[repr(C)]
pub struct FormatProperties3 {
	s_type: StructureType,
	p_next: *mut c_void,
	linear_tiling_features: FormatFeatureFlags2,
	optimal_tiling_features: FormatFeatureFlags2,
	buffer_features: FormatFeatureFlags2,
}

#[repr(C)]
pub struct VertexInputAttributeDescription {
	location: u32,
	binding: u32,
	format: Format,
	offset: u32,
}

#[repr(C)]
pub struct ImportMemoryWin32HandleInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	handle_type: ExternalMemoryHandleTypeFlagBits,
	handle: HANDLE,
	name: LPCWSTR,
}

#[repr(C)]
pub struct MultisampledRenderToSingleSampledInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	multisampled_render_to_single_sampled_enable: Bool32,
	rasterization_samples: SampleCountFlagBits,
}

#[repr(C)]
pub struct PipelineVertexInputStateCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineVertexInputStateCreateFlags,
	vertex_binding_description_count: u32,
	p_vertex_binding_descriptions: *const VertexInputBindingDescription,
	vertex_attribute_description_count: u32,
	p_vertex_attribute_descriptions: *const VertexInputAttributeDescription,
}

#[repr(C)]
pub struct ExternalFormatANDROID {
	s_type: StructureType,
	p_next: *mut c_void,
	external_format: u64,
}

#[repr(C)]
pub struct PhysicalDeviceClusterCullingShaderFeaturesHUAWEI {
	s_type: StructureType,
	p_next: *mut c_void,
	clusterculling_shader: Bool32,
	multiview_cluster_culling_shader: Bool32,
}

#[repr(C)]
pub struct VideoDecodeH265PictureInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	p_std_picture_info: *const StdVideoDecodeH265PictureInfo,
	slice_segment_count: u32,
	p_slice_segment_offsets: *const u32,
}

#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	acquire_count: u32,
	p_acquire_syncs: *const DeviceMemory,
	p_acquire_keys: *const u64,
	p_acquire_timeout_milliseconds: *const u32,
	release_count: u32,
	p_release_syncs: *const DeviceMemory,
	p_release_keys: *const u64,
}

#[repr(C)]
pub struct PhysicalDeviceShaderCoreBuiltinsPropertiesARM {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_core_mask: u64,
	shader_core_count: u32,
	shader_warps_per_core: u32,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	pipeline_fragment_shading_rate: Bool32,
	primitive_fragment_shading_rate: Bool32,
	attachment_fragment_shading_rate: Bool32,
}

#[repr(C)]
pub struct BufferOpaqueCaptureAddressCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	opaque_capture_address: u64,
}

#[repr(C)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	conditional_rendering_enable: Bool32,
}

#[repr(C)]
pub struct SubpassSampleLocationsEXT {
	subpass_index: u32,
	sample_locations_info: SampleLocationsInfoEXT,
}

#[repr(C)]
pub struct PipelineMultisampleStateCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineMultisampleStateCreateFlags,
	rasterization_samples: SampleCountFlagBits,
	sample_shading_enable: Bool32,
	min_sample_shading: f32,
	p_sample_mask: *const SampleMask,
	alpha_to_coverage_enable: Bool32,
	alpha_to_one_enable: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderCoreProperties2AMD {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_core_features: ShaderCorePropertiesFlagsAMD,
	active_compute_unit_count: u32,
}

#[repr(C)]
pub struct DescriptorSetLayoutBinding {
	binding: u32,
	descriptor_type: DescriptorType,
	descriptor_count: u32,
	stage_flags: ShaderStageFlags,
	p_immutable_samplers: *const Sampler,
}

#[repr(C)]
pub struct CopyAccelerationStructureInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	src: AccelerationStructureKHR,
	dst: AccelerationStructureKHR,
	mode: CopyAccelerationStructureModeKHR,
}

#[repr(C)]
pub struct VideoSessionParametersUpdateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	update_sequence_count: u32,
}

#[repr(C)]
pub struct GraphicsPipelineCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineCreateFlags,
	stage_count: u32,
	#[cfg(vulkan)]
	p_stages: *const PipelineShaderStageCreateInfo,
	#[cfg(vulkansc)]
	p_stages: *const PipelineShaderStageCreateInfo,
	p_vertex_input_state: *const PipelineVertexInputStateCreateInfo,
	p_input_assembly_state: *const PipelineInputAssemblyStateCreateInfo,
	p_tessellation_state: *const PipelineTessellationStateCreateInfo,
	p_viewport_state: *const PipelineViewportStateCreateInfo,
	p_rasterization_state: *const PipelineRasterizationStateCreateInfo,
	p_multisample_state: *const PipelineMultisampleStateCreateInfo,
	p_depth_stencil_state: *const PipelineDepthStencilStateCreateInfo,
	p_color_blend_state: *const PipelineColorBlendStateCreateInfo,
	p_dynamic_state: *const PipelineDynamicStateCreateInfo,
	layout: PipelineLayout,
	render_pass: RenderPass,
	subpass: u32,
	base_pipeline_handle: Pipeline,
	base_pipeline_index: i32,
}

#[repr(C)]
pub struct SampleLocationsInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	sample_locations_per_pixel: SampleCountFlagBits,
	sample_location_grid_size: Extent2D,
	sample_locations_count: u32,
	p_sample_locations: *const SampleLocationEXT,
}

#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_buffer_float16_atomics: Bool32,
	shader_buffer_float16_atomic_add: Bool32,
	shader_buffer_float16_atomic_min_max: Bool32,
	shader_buffer_float32_atomic_min_max: Bool32,
	shader_buffer_float64_atomic_min_max: Bool32,
	shader_shared_float16_atomics: Bool32,
	shader_shared_float16_atomic_add: Bool32,
	shader_shared_float16_atomic_min_max: Bool32,
	shader_shared_float32_atomic_min_max: Bool32,
	shader_shared_float64_atomic_min_max: Bool32,
	shader_image_float32_atomic_min_max: Bool32,
	sparse_image_float32_atomic_min_max: Bool32,
}

#[repr(C)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMD {
	s_type: StructureType,
	p_next: *const c_void,
	local_dimming_enable: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceYcbcrDegammaFeaturesQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	ycbcr_degamma: Bool32,
}

#[repr(C)]
pub struct TextureLODGatherFormatPropertiesAMD {
	s_type: StructureType,
	p_next: *mut c_void,
	supports_texture_gather_lodbias_amd: Bool32,
}

#[repr(C)]
pub struct RenderPassSubpassFeedbackInfoEXT {
	subpass_merge_status: SubpassMergeStatusEXT,
	description: [u8; MAX_DESCRIPTION_SIZE],
	post_merge_index: u32,
}

#[repr(C)]
pub struct ExportMetalIOSurfaceInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	image: Image,
	io_surface: IOSurfaceRef,
}

#[repr(C)]
pub struct VertexInputBindingDescription {
	binding: u32,
	stride: u32,
	input_rate: VertexInputRate,
}

#[repr(C)]
pub struct PhysicalDevicePresentWaitFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	present_wait: Bool32,
}

#[repr(C)]
pub struct DisplayPropertiesKHR {
	display: DisplayKHR,
	display_name: *const u8,
	physical_dimensions: Extent2D,
	physical_resolution: Extent2D,
	supported_transforms: SurfaceTransformFlagsKHR,
	plane_reorder_possible: Bool32,
	persistent_content: Bool32,
}

#[repr(C)]
pub struct BindImagePlaneMemoryInfo {
	s_type: StructureType,
	p_next: *const c_void,
	plane_aspect: ImageAspectFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceASTCDecodeFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	decode_mode_shared_exponent: Bool32,
}

#[repr(C)]
pub struct Viewport {
	x: f32,
	y: f32,
	width: f32,
	height: f32,
	min_depth: f32,
	max_depth: f32,
}

#[repr(C)]
pub struct DisplayModeProperties2KHR {
	s_type: StructureType,
	p_next: *mut c_void,
	display_mode_properties: DisplayModePropertiesKHR,
}

#[repr(C)]
pub struct MemoryToImageCopyEXT {
	s_type: StructureType,
	p_next: *const c_void,
	p_host_pointer: *const c_void,
	memory_row_length: u32,
	memory_image_height: u32,
	image_subresource: ImageSubresourceLayers,
	image_offset: Offset3D,
	image_extent: Extent3D,
}

#[repr(C)]
pub struct DeviceImageSubresourceInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	p_create_info: *const ImageCreateInfo,
	p_subresource: *const ImageSubresource2KHR,
}

#[repr(C)]
pub struct DeviceBufferMemoryRequirements {
	s_type: StructureType,
	p_next: *const c_void,
	p_create_info: *const BufferCreateInfo,
}

#[repr(C)]
pub struct AcquireProfilingLockInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: AcquireProfilingLockFlagsKHR,
	timeout: u64,
}

#[repr(C)]
pub struct ImageSubresource2KHR {
	s_type: StructureType,
	p_next: *mut c_void,
	image_subresource: ImageSubresource,
}

#[repr(C)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	drm_format_modifier: u64,
	sharing_mode: SharingMode,
	queue_family_index_count: u32,
	p_queue_family_indices: *const u32,
}

#[repr(C)]
pub struct DeviceCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: DeviceCreateFlags,
	queue_create_info_count: u32,
	p_queue_create_infos: *const DeviceQueueCreateInfo,
	#[deprecated(note = "Ignored")]
	enabled_layer_count: u32,
	#[deprecated(note = "Ignored")]
	pp_enabled_layer_names: *const *const u8,
	enabled_extension_count: u32,
	pp_enabled_extension_names: *const *const u8,
	p_enabled_features: *const PhysicalDeviceFeatures,
}

#[repr(C)]
pub struct VideoEncodeH264ProfileInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	std_profile_idc: StdVideoH264ProfileIdc,
}

#[repr(C)]
pub struct SurfaceCapabilities2KHR {
	s_type: StructureType,
	p_next: *mut c_void,
	surface_capabilities: SurfaceCapabilitiesKHR,
}

#[repr(C)]
pub struct LatencySurfaceCapabilitiesNV {
	s_type: StructureType,
	p_next: *const c_void,
	present_mode_count: u32,
	p_present_modes: *mut PresentModeKHR,
}

#[repr(C)]
pub struct BufferViewCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: BufferViewCreateFlags,
	buffer: Buffer,
	format: Format,
	offset: DeviceSize,
	range: DeviceSize,
}

#[repr(C)]
pub struct ImageBlit2 {
	s_type: StructureType,
	p_next: *const c_void,
	src_subresource: ImageSubresourceLayers,
	src_offsets: [Offset3D; 2],
	dst_subresource: ImageSubresourceLayers,
	dst_offsets: [Offset3D; 2],
}

#[repr(C)]
pub struct BindImageMemorySwapchainInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	swapchain: SwapchainKHR,
	image_index: u32,
}

#[repr(C)]
pub struct DisplayProperties2KHR {
	s_type: StructureType,
	p_next: *mut c_void,
	display_properties: DisplayPropertiesKHR,
}

#[repr(C)]
pub struct PhysicalDeviceHostQueryResetFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	host_query_reset: Bool32,
}

#[repr(C)]
pub struct MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM {
	s_type: StructureType,
	p_next: *const c_void,
	per_view_render_area_count: u32,
	p_per_view_render_areas: *const Rect2D,
}

#[repr(C)]
pub struct ExportSemaphoreWin32HandleInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	p_attributes: *const SECURITY_ATTRIBUTES,
	dw_access: DWORD,
	name: LPCWSTR,
}

#[repr(C)]
pub struct PipelineColorWriteCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	attachment_count: u32,
	p_color_write_enables: *const Bool32,
}

#[repr(C)]
pub struct DescriptorAddressInfoEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	address: DeviceAddress,
	range: DeviceSize,
	format: Format,
}

#[repr(C)]
pub struct DeviceGroupSubmitInfo {
	s_type: StructureType,
	p_next: *const c_void,
	wait_semaphore_count: u32,
	p_wait_semaphore_device_indices: *const u32,
	command_buffer_count: u32,
	p_command_buffer_device_masks: *const u32,
	signal_semaphore_count: u32,
	p_signal_semaphore_device_indices: *const u32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_early_and_late_fragment_tests: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_smbuiltins: Bool32,
}

#[repr(C)]
pub struct MemoryGetFdInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	memory: DeviceMemory,
	handle_type: ExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct BindImageMemoryDeviceGroupInfo {
	s_type: StructureType,
	p_next: *const c_void,
	device_index_count: u32,
	p_device_indices: *const u32,
	split_instance_bind_region_count: u32,
	p_split_instance_bind_regions: *const Rect2D,
}

#[repr(C)]
pub struct Extent2D {
	width: u32,
	height: u32,
}

#[repr(C)]
pub struct SubmitInfo2 {
	s_type: StructureType,
	p_next: *const c_void,
	flags: SubmitFlags,
	wait_semaphore_info_count: u32,
	p_wait_semaphore_infos: *const SemaphoreSubmitInfo,
	command_buffer_info_count: u32,
	p_command_buffer_infos: *const CommandBufferSubmitInfo,
	signal_semaphore_info_count: u32,
	p_signal_semaphore_infos: *const SemaphoreSubmitInfo,
}

#[repr(C)]
pub struct QueueFamilyCheckpointProperties2NV {
	s_type: StructureType,
	p_next: *mut c_void,
	checkpoint_execution_stage_mask: PipelineStageFlags2,
}

#[repr(C)]
pub struct VideoEncodeSessionParametersFeedbackInfoKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	has_overrides: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	graphics_pipeline_library: Bool32,
}

#[repr(C)]
pub struct DeviceMemoryOpaqueCaptureAddressInfo {
	s_type: StructureType,
	p_next: *const c_void,
	memory: DeviceMemory,
}

#[repr(C)]
pub struct DeviceAddressBindingCallbackDataEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	flags: DeviceAddressBindingFlagsEXT,
	base_address: DeviceAddress,
	size: DeviceSize,
	binding_type: DeviceAddressBindingTypeEXT,
}

#[repr(C)]
pub struct MemoryDedicatedRequirements {
	s_type: StructureType,
	p_next: *mut c_void,
	prefers_dedicated_allocation: Bool32,
	requires_dedicated_allocation: Bool32,
}

#[repr(C)]
pub struct VideoProfileListInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	profile_count: u32,
	p_profiles: *const VideoProfileInfoKHR,
}

#[repr(C)]
pub struct ExportFenceSciSyncInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	p_attributes: NvSciSyncAttrList,
}

#[repr(C)]
pub struct PhysicalDeviceDynamicRenderingFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	dynamic_rendering: Bool32,
}

#[repr(C)]
pub struct FormatProperties2 {
	s_type: StructureType,
	p_next: *mut c_void,
	format_properties: FormatProperties,
}

#[repr(C)]
pub struct AccelerationStructureBuildGeometryInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	r#type: AccelerationStructureTypeKHR,
	flags: BuildAccelerationStructureFlagsKHR,
	mode: BuildAccelerationStructureModeKHR,
	src_acceleration_structure: AccelerationStructureKHR,
	dst_acceleration_structure: AccelerationStructureKHR,
	geometry_count: u32,
	p_geometries: *const AccelerationStructureGeometryKHR,
	pp_geometries: *const *const AccelerationStructureGeometryKHR,
	scratch_data: DeviceOrHostAddressKHR,
}

#[repr(C)]
pub struct AccelerationStructureGeometryMotionTrianglesDataNV {
	s_type: StructureType,
	p_next: *const c_void,
	vertex_data: DeviceOrHostAddressConstKHR,
}

#[repr(C)]
pub struct CopyMicromapInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	src: MicromapEXT,
	dst: MicromapEXT,
	mode: CopyMicromapModeEXT,
}

#[repr(C)]
pub struct PipelineFragmentShadingRateEnumStateCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	shading_rate_type: FragmentShadingRateTypeNV,
	shading_rate: FragmentShadingRateNV,
	combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
}

#[repr(C)]
pub struct RenderingFragmentDensityMapAttachmentInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	image_view: ImageView,
	image_layout: ImageLayout,
}

#[repr(C)]
pub struct ImageViewAddressPropertiesNVX {
	s_type: StructureType,
	p_next: *mut c_void,
	device_address: DeviceAddress,
	size: DeviceSize,
}

#[repr(C)]
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_demote_to_helper_invocation: Bool32,
}

#[repr(C)]
pub struct ExportMetalDeviceInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	mtl_device: MTLDevice_id,
}

#[repr(C)]
pub struct CheckpointDataNV {
	s_type: StructureType,
	p_next: *mut c_void,
	stage: PipelineStageFlagBits,
	p_checkpoint_marker: *mut c_void,
}

#[repr(C)]
pub struct CoarseSampleOrderCustomNV {
	shading_rate: ShadingRatePaletteEntryNV,
	sample_count: u32,
	sample_location_count: u32,
	p_sample_locations: *const CoarseSampleLocationNV,
}

#[repr(C)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	group_count: u32,
	p_groups: *const GraphicsShaderGroupCreateInfoNV,
	pipeline_count: u32,
	p_pipelines: *const Pipeline,
}

#[repr(C)]
pub struct PhysicalDeviceAccelerationStructureFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	acceleration_structure: Bool32,
	acceleration_structure_capture_replay: Bool32,
	acceleration_structure_indirect_build: Bool32,
	acceleration_structure_host_commands: Bool32,
	descriptor_binding_acceleration_structure_update_after_bind: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceTexelBufferAlignmentProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	storage_texel_buffer_offset_alignment_bytes: DeviceSize,
	storage_texel_buffer_offset_single_texel_alignment: Bool32,
	uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
	uniform_texel_buffer_offset_single_texel_alignment: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_zero_initialize_workgroup_memory: Bool32,
}

#[repr(C)]
pub struct SamplerCustomBorderColorCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	custom_border_color: ClearColorValue,
	format: Format,
}

#[repr(C)]
pub struct ExportMetalTextureInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	image: Image,
	image_view: ImageView,
	buffer_view: BufferView,
	plane: ImageAspectFlagBits,
	mtl_texture: MTLTexture_id,
}

#[repr(C)]
pub struct PrivateDataSlotCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PrivateDataSlotCreateFlags,
}

#[repr(C)]
pub struct MicromapBuildInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	r#type: MicromapTypeEXT,
	flags: BuildMicromapFlagsEXT,
	mode: BuildMicromapModeEXT,
	dst_micromap: MicromapEXT,
	usage_counts_count: u32,
	p_usage_counts: *const MicromapUsageEXT,
	pp_usage_counts: *const *const MicromapUsageEXT,
	data: DeviceOrHostAddressConstKHR,
	scratch_data: DeviceOrHostAddressKHR,
	triangle_array: DeviceOrHostAddressConstKHR,
	triangle_array_stride: DeviceSize,
}

#[repr(C)]
pub struct VideoDecodeH265ProfileInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	std_profile_idc: StdVideoH265ProfileIdc,
}

#[repr(C)]
pub struct DescriptorBufferBindingPushDescriptorBufferHandleEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	buffer: Buffer,
}

#[repr(C)]
pub struct RenderPassMultiviewCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	subpass_count: u32,
	p_view_masks: *const u32,
	dependency_count: u32,
	p_view_offsets: *const i32,
	correlation_mask_count: u32,
	p_correlation_masks: *const u32,
}

#[repr(C)]
pub struct ImageViewCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: ImageViewCreateFlags,
	image: Image,
	view_type: ImageViewType,
	format: Format,
	components: ComponentMapping,
	subresource_range: ImageSubresourceRange,
}

#[repr(C)]
pub struct RenderingAttachmentInfo {
	s_type: StructureType,
	p_next: *const c_void,
	image_view: ImageView,
	image_layout: ImageLayout,
	resolve_mode: ResolveModeFlagBits,
	resolve_image_view: ImageView,
	resolve_image_layout: ImageLayout,
	load_op: AttachmentLoadOp,
	store_op: AttachmentStoreOp,
	clear_value: ClearValue,
}

#[repr(C)]
pub struct PhysicalDeviceFrameBoundaryFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	frame_boundary: Bool32,
}

#[repr(C)]
pub struct VideoEncodeH264CapabilitiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	flags: VideoEncodeH264CapabilityFlagsEXT,
	max_level_idc: StdVideoH264LevelIdc,
	max_slice_count: u32,
	max_ppicture_l0_reference_count: u32,
	max_bpicture_l0_reference_count: u32,
	max_l1_reference_count: u32,
	max_temporal_layer_count: u32,
	expect_dyadic_temporal_layer_pattern: Bool32,
	min_qp: i32,
	max_qp: i32,
	prefers_gop_remaining_frames: Bool32,
	requires_gop_remaining_frames: Bool32,
	std_syntax_flags: VideoEncodeH264StdFlagsEXT,
}

#[repr(C)]
pub struct SemaphoreGetSciSyncInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	semaphore: Semaphore,
	handle_type: ExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct ImageCopy {
	src_subresource: ImageSubresourceLayers,
	src_offset: Offset3D,
	dst_subresource: ImageSubresourceLayers,
	dst_offset: Offset3D,
	extent: Extent3D,
}

#[repr(C)]
pub struct CopyMemoryToImageInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: HostImageCopyFlagsEXT,
	dst_image: Image,
	dst_image_layout: ImageLayout,
	region_count: u32,
	p_regions: *const MemoryToImageCopyEXT,
}

#[repr(C)]
pub struct DependencyInfo {
	s_type: StructureType,
	p_next: *const c_void,
	dependency_flags: DependencyFlags,
	memory_barrier_count: u32,
	p_memory_barriers: *const MemoryBarrier2,
	buffer_memory_barrier_count: u32,
	p_buffer_memory_barriers: *const BufferMemoryBarrier2,
	image_memory_barrier_count: u32,
	p_image_memory_barriers: *const ImageMemoryBarrier2,
}

#[repr(C)]
pub struct SparseImageMemoryBindInfo {
	image: Image,
	bind_count: u32,
	p_binds: *const SparseImageMemoryBind,
}

#[repr(C)]
pub struct PhysicalDeviceRayTracingPropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_group_handle_size: u32,
	max_recursion_depth: u32,
	max_shader_group_stride: u32,
	shader_group_base_alignment: u32,
	max_geometry_count: u64,
	max_instance_count: u64,
	max_triangle_count: u64,
	max_descriptor_set_acceleration_structures: u32,
}

#[repr(C)]
pub struct AttachmentReference {
	attachment: u32,
	layout: ImageLayout,
}

#[repr(C)]
pub struct MemoryFdPropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	memory_type_bits: u32,
}

#[repr(C)]
pub struct ExternalImageFormatPropertiesNV {
	image_format_properties: ImageFormatProperties,
	external_memory_features: ExternalMemoryFeatureFlagsNV,
	export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNV,
	compatible_handle_types: ExternalMemoryHandleTypeFlagsNV,
}

#[repr(C)]
pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfo {
	s_type: StructureType,
	p_next: *mut c_void,
	required_subgroup_size: u32,
}

#[repr(C)]
pub struct DebugUtilsLabelEXT {
	s_type: StructureType,
	p_next: *const c_void,
	p_label_name: *const u8,
	color: [f32; 4],
}

#[repr(C)]
pub struct MicromapVersionInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	p_version_data: *const u8,
}

#[repr(C)]
pub struct ImportSemaphoreZirconHandleInfoFUCHSIA {
	s_type: StructureType,
	p_next: *const c_void,
	semaphore: Semaphore,
	flags: SemaphoreImportFlags,
	handle_type: ExternalSemaphoreHandleTypeFlagBits,
	zircon_handle: zx_handle_t,
}

#[repr(C)]
pub struct PhysicalDeviceGroupProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	physical_device_count: u32,
	physical_devices: [PhysicalDevice; MAX_DEVICE_GROUP_SIZE],
	subset_allocation: Bool32,
}

#[repr(C)]
pub struct CommandPoolCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: CommandPoolCreateFlags,
	queue_family_index: u32,
}

#[repr(C)]
pub struct FenceGetWin32HandleInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	fence: Fence,
	handle_type: ExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct RenderPassAttachmentBeginInfo {
	s_type: StructureType,
	p_next: *const c_void,
	attachment_count: u32,
	p_attachments: *const ImageView,
}

#[repr(C)]
pub struct VideoDecodeH264ProfileInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	std_profile_idc: StdVideoH264ProfileIdc,
	picture_layout: VideoDecodeH264PictureLayoutFlagBitsKHR,
}

#[repr(C)]
pub struct DescriptorSetLayoutCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: DescriptorSetLayoutCreateFlags,
	binding_count: u32,
	p_bindings: *const DescriptorSetLayoutBinding,
}

#[repr(C)]
pub struct OpticalFlowSessionCreatePrivateDataInfoNV {
	s_type: StructureType,
	p_next: *mut c_void,
	id: u32,
	size: u32,
	p_private_data: *const c_void,
}

#[repr(C)]
pub struct CooperativeMatrixPropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	msize: u32,
	nsize: u32,
	ksize: u32,
	atype: ComponentTypeKHR,
	btype: ComponentTypeKHR,
	ctype: ComponentTypeKHR,
	result_type: ComponentTypeKHR,
	saturating_accumulation: Bool32,
	scope: ScopeKHR,
}

#[repr(C)]
pub struct PipelineExecutableInternalRepresentationKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	name: [u8; MAX_DESCRIPTION_SIZE],
	description: [u8; MAX_DESCRIPTION_SIZE],
	is_text: Bool32,
	data_size: usize,
	p_data: *mut c_void,
}

#[repr(C)]
pub struct PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	format_rgba10x6_without_ycb_cr_sampler: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	sample_counts: SampleCountFlags,
	fragment_size: Extent2D,
}

#[repr(C)]
pub struct RenderPassCreationFeedbackInfoEXT {
	post_merge_subpass_count: u32,
}

#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	protected_memory: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShadingRateImageFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	shading_rate_image: Bool32,
	shading_rate_coarse_sample_order: Bool32,
}

#[repr(C)]
pub struct BufferImageCopy2 {
	s_type: StructureType,
	p_next: *const c_void,
	buffer_offset: DeviceSize,
	buffer_row_length: u32,
	buffer_image_height: u32,
	image_subresource: ImageSubresourceLayers,
	image_offset: Offset3D,
	image_extent: Extent3D,
}

#[repr(C)]
pub struct SwapchainPresentScalingCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	scaling_behavior: PresentScalingFlagsEXT,
	present_gravity_x: PresentGravityFlagsEXT,
	present_gravity_y: PresentGravityFlagsEXT,
}

#[repr(C)]
pub struct PhysicalDeviceInlineUniformBlockProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	max_inline_uniform_block_size: u32,
	max_per_stage_descriptor_inline_uniform_blocks: u32,
	max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
	max_descriptor_set_inline_uniform_blocks: u32,
	max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
}

#[repr(C)]
pub struct GeneratedCommandsMemoryRequirementsInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	pipeline_bind_point: PipelineBindPoint,
	pipeline: Pipeline,
	indirect_commands_layout: IndirectCommandsLayoutNV,
	max_sequences_count: u32,
}

#[repr(C)]
pub struct VideoEncodeH264QpEXT {
	qp_i: i32,
	qp_p: i32,
	qp_b: i32,
}

#[repr(C)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	depth_clip_enable: Bool32,
}

#[repr(C)]
pub struct XcbSurfaceCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: XcbSurfaceCreateFlagsKHR,
	connection: *mut xcb_connection_t,
	window: xcb_window_t,
}

#[repr(C)]
pub struct FormatProperties {
	linear_tiling_features: FormatFeatureFlags,
	optimal_tiling_features: FormatFeatureFlags,
	buffer_features: FormatFeatureFlags,
}

#[repr(C)]
pub struct PhysicalDeviceMultiDrawPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	max_multi_draw_count: u32,
}

#[repr(C)]
pub struct PhysicalDeviceMeshShaderFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	task_shader: Bool32,
	mesh_shader: Bool32,
}

#[repr(C)]
pub struct MetalSurfaceCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: MetalSurfaceCreateFlagsEXT,
	p_layer: *const CAMetalLayer,
}

#[repr(C)]
pub struct PhysicalDeviceExternalFormatResolveFeaturesANDROID {
	s_type: StructureType,
	p_next: *mut c_void,
	external_format_resolve: Bool32,
}

#[repr(C)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	exclusive_scissor_count: u32,
	p_exclusive_scissors: *const Rect2D,
}

#[repr(C)]
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	transform: SurfaceTransformFlagBitsKHR,
	render_area: Rect2D,
}

#[repr(C)]
pub struct VideoDecodeUsageInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	video_usage_hints: VideoDecodeUsageFlagsKHR,
}

#[repr(C)]
pub struct SparseBufferMemoryBindInfo {
	buffer: Buffer,
	bind_count: u32,
	p_binds: *const SparseMemoryBind,
}

#[repr(C)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	compute_derivative_group_quads: Bool32,
	compute_derivative_group_linear: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderFloat16Int8Features {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_float16: Bool32,
	shader_int8: Bool32,
}

#[repr(C)]
pub struct SparseImageMemoryBind {
	subresource: ImageSubresource,
	offset: Offset3D,
	extent: Extent3D,
	memory: DeviceMemory,
	memory_offset: DeviceSize,
	flags: SparseMemoryBindFlags,
}

#[repr(C)]
pub struct PhysicalDeviceShaderTileImagePropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_tile_image_coherent_read_accelerated: Bool32,
	shader_tile_image_read_sample_from_pixel_rate_invocation: Bool32,
	shader_tile_image_read_from_helper_invocation: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceOpticalFlowFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	optical_flow: Bool32,
}

#[repr(C)]
pub struct PipelineInputAssemblyStateCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineInputAssemblyStateCreateFlags,
	topology: PrimitiveTopology,
	primitive_restart_enable: Bool32,
}

#[repr(C)]
pub struct ShadingRatePaletteNV {
	shading_rate_palette_entry_count: u32,
	p_shading_rate_palette_entries: *const ShadingRatePaletteEntryNV,
}

#[repr(C)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	dedicated_allocation_image_aliasing: Bool32,
}

#[repr(C)]
pub struct PipelineShaderStageModuleIdentifierCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	identifier_size: u32,
	p_identifier: *const u8,
}

#[repr(C)]
pub struct DrawMeshTasksIndirectCommandEXT {
	group_count_x: u32,
	group_count_y: u32,
	group_count_z: u32,
}

#[repr(C)]
pub struct PhysicalDeviceCopyMemoryIndirectFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	indirect_copy: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceSurfaceInfo2KHR {
	s_type: StructureType,
	p_next: *const c_void,
	surface: SurfaceKHR,
}

#[repr(C)]
pub struct SamplerCaptureDescriptorDataInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	sampler: Sampler,
}

#[repr(C)]
pub struct ScreenSurfaceCreateInfoQNX {
	s_type: StructureType,
	p_next: *const c_void,
	flags: ScreenSurfaceCreateFlagsQNX,
	context: *mut _screen_context,
	window: *mut _screen_window,
}

#[repr(C)]
pub struct MemoryGetZirconHandleInfoFUCHSIA {
	s_type: StructureType,
	p_next: *const c_void,
	memory: DeviceMemory,
	handle_type: ExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct RenderPassFragmentDensityMapCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	fragment_density_map_attachment: AttachmentReference,
}

#[repr(C)]
pub struct PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	dynamic_rendering_unused_attachments: Bool32,
}

#[repr(C)]
pub struct DevicePrivateDataCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	private_data_slot_request_count: u32,
}

#[repr(C)]
pub struct PipelineExecutablePropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	stages: ShaderStageFlags,
	name: [u8; MAX_DESCRIPTION_SIZE],
	description: [u8; MAX_DESCRIPTION_SIZE],
	subgroup_size: u32,
}

#[repr(C)]
pub struct VideoEncodeH264DpbSlotInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	p_std_reference_info: *const StdVideoEncodeH264ReferenceInfo,
}

#[repr(C)]
pub struct PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX {
	s_type: StructureType,
	p_next: *mut c_void,
	screen_buffer_import: Bool32,
}

#[repr(C)]
pub struct SemaphoreGetWin32HandleInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	semaphore: Semaphore,
	handle_type: ExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct DescriptorSetLayoutHostMappingInfoVALVE {
	s_type: StructureType,
	p_next: *mut c_void,
	descriptor_offset: usize,
	descriptor_size: u32,
}

#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	advanced_blend_coherent_operations: Bool32,
}

#[repr(C)]
pub struct DisplayPlaneInfo2KHR {
	s_type: StructureType,
	p_next: *const c_void,
	mode: DisplayModeKHR,
	plane_index: u32,
}

#[repr(C)]
pub struct MemoryUnmapInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: MemoryUnmapFlagsKHR,
	memory: DeviceMemory,
}

#[repr(C)]
pub struct ImageDrmFormatModifierPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	drm_format_modifier: u64,
}

#[repr(C)]
pub struct PerformanceStreamMarkerInfoINTEL {
	s_type: StructureType,
	p_next: *const c_void,
	marker: u32,
}

#[repr(C)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	ycbcr_image_arrays: Bool32,
}

#[repr(C)]
pub struct DecompressMemoryRegionNV {
	src_address: DeviceAddress,
	dst_address: DeviceAddress,
	compressed_size: DeviceSize,
	decompressed_size: DeviceSize,
	decompression_method: MemoryDecompressionMethodFlagsNV,
}

#[repr(C)]
pub struct PhysicalDeviceSamplerFilterMinmaxProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	filter_minmax_single_component_formats: Bool32,
	filter_minmax_image_component_mapping: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceFloatControlsProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	denorm_behavior_independence: ShaderFloatControlsIndependence,
	rounding_mode_independence: ShaderFloatControlsIndependence,
	shader_signed_zero_inf_nan_preserve_float16: Bool32,
	shader_signed_zero_inf_nan_preserve_float32: Bool32,
	shader_signed_zero_inf_nan_preserve_float64: Bool32,
	shader_denorm_preserve_float16: Bool32,
	shader_denorm_preserve_float32: Bool32,
	shader_denorm_preserve_float64: Bool32,
	shader_denorm_flush_to_zero_float16: Bool32,
	shader_denorm_flush_to_zero_float32: Bool32,
	shader_denorm_flush_to_zero_float64: Bool32,
	shader_rounding_mode_rtefloat16: Bool32,
	shader_rounding_mode_rtefloat32: Bool32,
	shader_rounding_mode_rtefloat64: Bool32,
	shader_rounding_mode_rtzfloat16: Bool32,
	shader_rounding_mode_rtzfloat32: Bool32,
	shader_rounding_mode_rtzfloat64: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceMultiviewProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	max_multiview_view_count: u32,
	max_multiview_instance_index: u32,
}

#[repr(C)]
pub struct GeometryAABBNV {
	s_type: StructureType,
	p_next: *const c_void,
	aabb_data: Buffer,
	num_aabbs: u32,
	stride: u32,
	offset: DeviceSize,
}

#[repr(C)]
pub struct DeviceGroupPresentCapabilitiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	present_mask: [u32; MAX_DEVICE_GROUP_SIZE],
	modes: DeviceGroupPresentModeFlagsKHR,
}

#[repr(C)]
pub struct AccelerationStructureInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	r#type: AccelerationStructureTypeNV,
	flags: BuildAccelerationStructureFlagsNV,
	instance_count: u32,
	geometry_count: u32,
	p_geometries: *const GeometryNV,
}

#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_subgroup_uniform_control_flow: Bool32,
}

#[repr(C)]
pub struct ExternalImageFormatProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	external_memory_properties: ExternalMemoryProperties,
}

#[repr(C)]
pub struct PhysicalDeviceExternalBufferInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: BufferCreateFlags,
	usage: BufferUsageFlags,
	handle_type: ExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceDescriptorPoolOverallocationFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	descriptor_pool_overallocation: Bool32,
}

#[repr(C)]
pub struct RenderPassInputAttachmentAspectCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	aspect_reference_count: u32,
	p_aspect_references: *const InputAttachmentAspectReference,
}

#[repr(C)]
pub struct ImportMetalIOSurfaceInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	io_surface: IOSurfaceRef,
}

#[repr(C)]
pub struct PhysicalDeviceShaderEnqueuePropertiesAMDX {
	s_type: StructureType,
	p_next: *mut c_void,
	max_execution_graph_depth: u32,
	max_execution_graph_shader_output_nodes: u32,
	max_execution_graph_shader_payload_size: u32,
	max_execution_graph_shader_payload_count: u32,
	execution_graph_dispatch_address_alignment: u32,
}

#[repr(C)]
pub struct DeviceGroupCommandBufferBeginInfo {
	s_type: StructureType,
	p_next: *const c_void,
	device_mask: u32,
}

#[repr(C)]
pub struct SubpassShadingPipelineCreateInfoHUAWEI {
	s_type: StructureType,
	p_next: *mut c_void,
	render_pass: RenderPass,
	subpass: u32,
}

#[repr(C)]
pub struct ImportAndroidHardwareBufferInfoANDROID {
	s_type: StructureType,
	p_next: *const c_void,
	buffer: *mut AHardwareBuffer,
}

#[repr(C)]
pub struct PhysicalDeviceMultiviewFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	multiview: Bool32,
	multiview_geometry_shader: Bool32,
	multiview_tessellation_shader: Bool32,
}

#[repr(C)]
pub struct SurfaceCapabilities2EXT {
	s_type: StructureType,
	p_next: *mut c_void,
	min_image_count: u32,
	max_image_count: u32,
	current_extent: Extent2D,
	min_image_extent: Extent2D,
	max_image_extent: Extent2D,
	max_image_array_layers: u32,
	supported_transforms: SurfaceTransformFlagsKHR,
	current_transform: SurfaceTransformFlagBitsKHR,
	supported_composite_alpha: CompositeAlphaFlagsKHR,
	supported_usage_flags: ImageUsageFlags,
	supported_surface_counters: SurfaceCounterFlagsEXT,
}

#[repr(C)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	sampler_ycbcr_conversion: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceMeshShaderFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	task_shader: Bool32,
	mesh_shader: Bool32,
	multiview_mesh_shader: Bool32,
	primitive_fragment_shading_rate_mesh_shader: Bool32,
	mesh_shader_queries: Bool32,
}

#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	r#type: RayTracingShaderGroupTypeKHR,
	general_shader: u32,
	closest_hit_shader: u32,
	any_hit_shader: u32,
	intersection_shader: u32,
}

#[repr(C)]
pub struct ImageDrmFormatModifierListCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	drm_format_modifier_count: u32,
	p_drm_format_modifiers: *const u64,
}

#[repr(C)]
pub struct CopyMemoryIndirectCommandNV {
	src_address: DeviceAddress,
	dst_address: DeviceAddress,
	size: DeviceSize,
}

#[repr(C)]
pub struct CopyCommandTransformInfoQCOM {
	s_type: StructureType,
	p_next: *const c_void,
	transform: SurfaceTransformFlagBitsKHR,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	fragment_shader_sample_interlock: Bool32,
	fragment_shader_pixel_interlock: Bool32,
	fragment_shader_shading_rate_interlock: Bool32,
}

#[repr(C)]
pub struct ImportSemaphoreFdInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	semaphore: Semaphore,
	flags: SemaphoreImportFlags,
	handle_type: ExternalSemaphoreHandleTypeFlagBits,
	fd: isize,
}

#[repr(C)]
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	index_type_uint8: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceVulkan12Properties {
	s_type: StructureType,
	p_next: *mut c_void,
	driver_id: DriverId,
	driver_name: [u8; MAX_DRIVER_NAME_SIZE],
	driver_info: [u8; MAX_DRIVER_INFO_SIZE],
	conformance_version: ConformanceVersion,
	denorm_behavior_independence: ShaderFloatControlsIndependence,
	rounding_mode_independence: ShaderFloatControlsIndependence,
	shader_signed_zero_inf_nan_preserve_float16: Bool32,
	shader_signed_zero_inf_nan_preserve_float32: Bool32,
	shader_signed_zero_inf_nan_preserve_float64: Bool32,
	shader_denorm_preserve_float16: Bool32,
	shader_denorm_preserve_float32: Bool32,
	shader_denorm_preserve_float64: Bool32,
	shader_denorm_flush_to_zero_float16: Bool32,
	shader_denorm_flush_to_zero_float32: Bool32,
	shader_denorm_flush_to_zero_float64: Bool32,
	shader_rounding_mode_rtefloat16: Bool32,
	shader_rounding_mode_rtefloat32: Bool32,
	shader_rounding_mode_rtefloat64: Bool32,
	shader_rounding_mode_rtzfloat16: Bool32,
	shader_rounding_mode_rtzfloat32: Bool32,
	shader_rounding_mode_rtzfloat64: Bool32,
	max_update_after_bind_descriptors_in_all_pools: u32,
	shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
	shader_sampled_image_array_non_uniform_indexing_native: Bool32,
	shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
	shader_storage_image_array_non_uniform_indexing_native: Bool32,
	shader_input_attachment_array_non_uniform_indexing_native: Bool32,
	robust_buffer_access_update_after_bind: Bool32,
	quad_divergent_implicit_lod: Bool32,
	max_per_stage_descriptor_update_after_bind_samplers: u32,
	max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
	max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
	max_per_stage_descriptor_update_after_bind_sampled_images: u32,
	max_per_stage_descriptor_update_after_bind_storage_images: u32,
	max_per_stage_descriptor_update_after_bind_input_attachments: u32,
	max_per_stage_update_after_bind_resources: u32,
	max_descriptor_set_update_after_bind_samplers: u32,
	max_descriptor_set_update_after_bind_uniform_buffers: u32,
	max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
	max_descriptor_set_update_after_bind_storage_buffers: u32,
	max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
	max_descriptor_set_update_after_bind_sampled_images: u32,
	max_descriptor_set_update_after_bind_storage_images: u32,
	max_descriptor_set_update_after_bind_input_attachments: u32,
	supported_depth_resolve_modes: ResolveModeFlags,
	supported_stencil_resolve_modes: ResolveModeFlags,
	independent_resolve_none: Bool32,
	independent_resolve: Bool32,
	filter_minmax_single_component_formats: Bool32,
	filter_minmax_image_component_mapping: Bool32,
	max_timeline_semaphore_value_difference: u64,
	framebuffer_integer_color_sample_counts: SampleCountFlags,
}

#[repr(C)]
pub struct VideoEncodeH264SessionParametersCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	max_std_spscount: u32,
	max_std_ppscount: u32,
	p_parameters_add_info: *const VideoEncodeH264SessionParametersAddInfoEXT,
}

#[repr(C)]
pub struct MutableDescriptorTypeCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	mutable_descriptor_type_list_count: u32,
	p_mutable_descriptor_type_lists: *const MutableDescriptorTypeListEXT,
}

#[repr(C)]
pub struct ExportMemoryAllocateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	handle_types: ExternalMemoryHandleTypeFlagsNV,
}

#[repr(C)]
pub struct PresentIdKHR {
	s_type: StructureType,
	p_next: *const c_void,
	swapchain_count: u32,
	p_present_ids: *const u64,
}

#[repr(C)]
pub struct RenderPassCreationControlEXT {
	s_type: StructureType,
	p_next: *const c_void,
	disallow_merging: Bool32,
}

#[repr(C)]
pub struct DrawMeshTasksIndirectCommandNV {
	task_count: u32,
	first_task: u32,
}

#[repr(C)]
pub struct WriteDescriptorSetInlineUniformBlock {
	s_type: StructureType,
	p_next: *const c_void,
	data_size: u32,
	p_data: *const c_void,
}

#[repr(C)]
pub struct PhysicalDeviceSubgroupSizeControlFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	subgroup_size_control: Bool32,
	compute_full_subgroups: Bool32,
}

#[repr(C)]
pub struct CopyBufferInfo2 {
	s_type: StructureType,
	p_next: *const c_void,
	src_buffer: Buffer,
	dst_buffer: Buffer,
	region_count: u32,
	p_regions: *const BufferCopy2,
}

#[repr(C)]
pub struct GeometryTrianglesNV {
	s_type: StructureType,
	p_next: *const c_void,
	vertex_data: Buffer,
	vertex_offset: DeviceSize,
	vertex_count: u32,
	vertex_stride: DeviceSize,
	vertex_format: Format,
	index_data: Buffer,
	index_offset: DeviceSize,
	index_count: u32,
	index_type: IndexType,
	transform_data: Buffer,
	transform_offset: DeviceSize,
}

#[repr(C)]
pub struct DisplayPowerInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	power_state: DisplayPowerStateEXT,
}

#[repr(C)]
pub struct ImportScreenBufferInfoQNX {
	s_type: StructureType,
	p_next: *const c_void,
	buffer: *mut _screen_buffer,
}

#[repr(C)]
pub struct DescriptorSetBindingReferenceVALVE {
	s_type: StructureType,
	p_next: *const c_void,
	descriptor_set_layout: DescriptorSetLayout,
	binding: u32,
}

#[repr(C)]
pub struct VideoDecodeH264SessionParametersAddInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	std_spscount: u32,
	p_std_spss: *const StdVideoH264SequenceParameterSet,
	std_ppscount: u32,
	p_std_ppss: *const StdVideoH264PictureParameterSet,
}

#[repr(C)]
pub struct DeviceMemoryReportCallbackDataEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	flags: DeviceMemoryReportFlagsEXT,
	r#type: DeviceMemoryReportEventTypeEXT,
	memory_object_id: u64,
	size: DeviceSize,
	object_type: ObjectType,
	object_handle: u64,
	heap_index: u32,
}

#[repr(C)]
pub struct StencilOpState {
	fail_op: StencilOp,
	pass_op: StencilOp,
	depth_fail_op: StencilOp,
	compare_op: CompareOp,
	compare_mask: u32,
	write_mask: u32,
	reference: u32,
}

#[repr(C)]
pub struct PipelineCacheCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineCacheCreateFlags,
	#[cfg(vulkan)]
	initial_data_size: usize,
	#[cfg(vulkansc)]
	initial_data_size: usize,
	p_initial_data: *const c_void,
}

#[repr(C)]
pub struct OpticalFlowExecuteInfoNV {
	s_type: StructureType,
	p_next: *mut c_void,
	flags: OpticalFlowExecuteFlagsNV,
	region_count: u32,
	p_regions: *const Rect2D,
}

#[repr(C)]
pub struct DeviceGroupRenderPassBeginInfo {
	s_type: StructureType,
	p_next: *const c_void,
	device_mask: u32,
	device_render_area_count: u32,
	p_device_render_areas: *const Rect2D,
}

#[repr(C)]
pub struct MemoryZirconHandlePropertiesFUCHSIA {
	s_type: StructureType,
	p_next: *mut c_void,
	memory_type_bits: u32,
}

#[repr(C)]
pub struct ImageFormatProperties2 {
	s_type: StructureType,
	p_next: *mut c_void,
	image_format_properties: ImageFormatProperties,
}

#[repr(C)]
pub struct ScreenBufferFormatPropertiesQNX {
	s_type: StructureType,
	p_next: *mut c_void,
	format: Format,
	external_format: u64,
	screen_usage: u64,
	format_features: FormatFeatureFlags,
	sampler_ycbcr_conversion_components: ComponentMapping,
	suggested_ycbcr_model: SamplerYcbcrModelConversion,
	suggested_ycbcr_range: SamplerYcbcrRange,
	suggested_xchroma_offset: ChromaLocation,
	suggested_ychroma_offset: ChromaLocation,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	tri_strip_vertex_order_independent_of_provoking_vertex: Bool32,
}

#[repr(C)]
pub struct BindIndexBufferIndirectCommandNV {
	buffer_address: DeviceAddress,
	size: u32,
	index_type: IndexType,
}

#[repr(C)]
pub struct AabbPositionsKHR {
	min_x: f32,
	min_y: f32,
	min_z: f32,
	max_x: f32,
	max_y: f32,
	max_z: f32,
}

#[repr(C)]
pub struct Extent3D {
	width: u32,
	height: u32,
	depth: u32,
}

#[repr(C)]
pub struct ImageCompressionControlEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: ImageCompressionFlagsEXT,
	compression_control_plane_count: u32,
	p_fixed_rate_flags: *mut ImageCompressionFixedRateFlagsEXT,
}

#[repr(C)]
pub struct QueueFamilyQueryResultStatusPropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	query_result_status_support: Bool32,
}

#[repr(C)]
pub struct DeviceFaultCountsEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	address_info_count: u32,
	vendor_info_count: u32,
	vendor_binary_size: DeviceSize,
}

#[repr(C)]
pub struct VideoEncodeH265NaluSliceSegmentInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	ant_qp: i32,
	p_std_slice_segment_header: *const StdVideoEncodeH265SliceSegmentHeader,
}

#[repr(C)]
pub struct RenderPassCreationFeedbackCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	p_render_pass_feedback: *mut RenderPassCreationFeedbackInfoEXT,
}

#[repr(C)]
pub struct SurfaceCapabilitiesKHR {
	min_image_count: u32,
	max_image_count: u32,
	current_extent: Extent2D,
	min_image_extent: Extent2D,
	max_image_extent: Extent2D,
	max_image_array_layers: u32,
	supported_transforms: SurfaceTransformFlagsKHR,
	current_transform: SurfaceTransformFlagBitsKHR,
	supported_composite_alpha: CompositeAlphaFlagsKHR,
	supported_usage_flags: ImageUsageFlags,
}

#[repr(C)]
pub struct SubpassDescriptionDepthStencilResolve {
	s_type: StructureType,
	p_next: *const c_void,
	depth_resolve_mode: ResolveModeFlagBits,
	stencil_resolve_mode: ResolveModeFlagBits,
	p_depth_stencil_resolve_attachment: *const AttachmentReference2,
}

#[repr(C)]
pub struct PhysicalDeviceExternalSemaphoreInfo {
	s_type: StructureType,
	p_next: *const c_void,
	handle_type: ExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceVideoEncodeQualityLevelInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	p_video_profile: *const VideoProfileInfoKHR,
	quality_level: u32,
}

#[repr(C)]
pub struct ExecutionGraphPipelineScratchSizeAMDX {
	s_type: StructureType,
	p_next: *mut c_void,
	size: DeviceSize,
}

#[repr(C)]
pub struct DispatchGraphCountInfoAMDX {
	count: u32,
	infos: DeviceOrHostAddressConstAMDX,
	stride: u64,
}

#[repr(C)]
pub struct VideoEncodeCapabilitiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	flags: VideoEncodeCapabilityFlagsKHR,
	rate_control_modes: VideoEncodeRateControlModeFlagsKHR,
	max_rate_control_layers: u32,
	max_bitrate: u64,
	max_quality_levels: u32,
	encode_input_picture_granularity: Extent2D,
	supported_encode_feedback_flags: VideoEncodeFeedbackFlagsKHR,
}

#[repr(C)]
pub struct AccelerationStructureGeometryKHR {
	s_type: StructureType,
	p_next: *const c_void,
	geometry_type: GeometryTypeKHR,
	geometry: AccelerationStructureGeometryDataKHR,
	flags: GeometryFlagsKHR,
}

#[repr(C)]
pub struct AccelerationStructureCaptureDescriptorDataInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	acceleration_structure: AccelerationStructureKHR,
	acceleration_structure_nv: AccelerationStructureNV,
}

#[repr(C)]
pub struct DebugUtilsMessengerCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: DebugUtilsMessengerCreateFlagsEXT,
	message_severity: DebugUtilsMessageSeverityFlagsEXT,
	message_type: DebugUtilsMessageTypeFlagsEXT,
	pfn_user_callback: PFN_vkDebugUtilsMessengerCallbackEXT,
	p_user_data: *mut c_void,
}

#[repr(C)]
pub struct DeviceMemoryOverallocationCreateInfoAMD {
	s_type: StructureType,
	p_next: *const c_void,
	overallocation_behavior: MemoryOverallocationBehaviorAMD,
}

#[repr(C)]
pub struct PhysicalDeviceMutableDescriptorTypeFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	mutable_descriptor_type: Bool32,
}

#[repr(C)]
pub struct MemoryAllocateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	allocation_size: DeviceSize,
	memory_type_index: u32,
}

#[repr(C)]
pub struct BufferCollectionCreateInfoFUCHSIA {
	s_type: StructureType,
	p_next: *const c_void,
	collection_token: zx_handle_t,
}

#[repr(C)]
pub struct ImageViewHandleInfoNVX {
	s_type: StructureType,
	p_next: *const c_void,
	image_view: ImageView,
	descriptor_type: DescriptorType,
	sampler: Sampler,
}

#[repr(C)]
pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	primitive_topology_list_restart: Bool32,
	primitive_topology_patch_list_restart: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceRenderPassStripedPropertiesARM {
	s_type: StructureType,
	p_next: *mut c_void,
	render_pass_stripe_granularity: Extent2D,
	max_render_pass_stripes: u32,
}

#[repr(C)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	max_discard_rectangles: u32,
}

#[repr(C)]
pub struct ShaderStatisticsInfoAMD {
	shader_stage_mask: ShaderStageFlags,
	resource_usage: ShaderResourceUsageAMD,
	num_physical_vgprs: u32,
	num_physical_sgprs: u32,
	num_available_vgprs: u32,
	num_available_sgprs: u32,
	compute_work_group_size: [u32; 3],
}

#[repr(C)]
pub struct DescriptorPoolInlineUniformBlockCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	max_inline_uniform_block_bindings: u32,
}

#[repr(C)]
pub struct AttachmentReference2 {
	s_type: StructureType,
	p_next: *const c_void,
	attachment: u32,
	layout: ImageLayout,
	aspect_mask: ImageAspectFlags,
}

#[repr(C)]
pub struct TimelineSemaphoreSubmitInfo {
	s_type: StructureType,
	p_next: *const c_void,
	wait_semaphore_value_count: u32,
	p_wait_semaphore_values: *const u64,
	signal_semaphore_value_count: u32,
	p_signal_semaphore_values: *const u64,
}

#[repr(C)]
pub struct NativeBufferANDROID {
	s_type: StructureType,
	p_next: *const c_void,
	handle: *const c_void,
	stride: isize,
	format: isize,
	usage: isize,
	usage2: NativeBufferUsage2ANDROID,
}

#[repr(C)]
pub struct BufferCopy {
	src_offset: DeviceSize,
	dst_offset: DeviceSize,
	size: DeviceSize,
}

#[repr(C)]
pub struct ViSurfaceCreateInfoNN {
	s_type: StructureType,
	p_next: *const c_void,
	flags: ViSurfaceCreateFlagsNN,
	window: *mut c_void,
}

#[repr(C)]
pub struct SwapchainCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: SwapchainCreateFlagsKHR,
	surface: SurfaceKHR,
	min_image_count: u32,
	image_format: Format,
	image_color_space: ColorSpaceKHR,
	image_extent: Extent2D,
	image_array_layers: u32,
	image_usage: ImageUsageFlags,
	image_sharing_mode: SharingMode,
	queue_family_index_count: u32,
	p_queue_family_indices: *const u32,
	pre_transform: SurfaceTransformFlagBitsKHR,
	composite_alpha: CompositeAlphaFlagBitsKHR,
	present_mode: PresentModeKHR,
	clipped: Bool32,
	#[cfg(vulkan)]
	old_swapchain: SwapchainKHR,
	#[cfg(vulkansc)]
	old_swapchain: SwapchainKHR,
}

#[repr(C)]
pub struct BufferDeviceAddressInfo {
	s_type: StructureType,
	p_next: *const c_void,
	buffer: Buffer,
}

#[repr(C)]
pub struct CooperativeMatrixPropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	msize: u32,
	nsize: u32,
	ksize: u32,
	atype: ComponentTypeNV,
	btype: ComponentTypeNV,
	ctype: ComponentTypeNV,
	dtype: ComponentTypeNV,
	scope: ScopeNV,
}

#[repr(C)]
pub struct PhysicalDeviceImage2DViewOf3DFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	image2_dview_of3_d: Bool32,
	sampler2_dview_of3_d: Bool32,
}

#[repr(C)]
pub struct VideoDecodeH265CapabilitiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	max_level_idc: StdVideoH265LevelIdc,
}

#[repr(C)]
pub struct OpticalFlowSessionCreateInfoNV {
	s_type: StructureType,
	p_next: *mut c_void,
	width: u32,
	height: u32,
	image_format: Format,
	flow_vector_format: Format,
	cost_format: Format,
	output_grid_size: OpticalFlowGridSizeFlagsNV,
	hint_grid_size: OpticalFlowGridSizeFlagsNV,
	performance_level: OpticalFlowPerformanceLevelNV,
	flags: OpticalFlowSessionCreateFlagsNV,
}

#[repr(C)]
pub struct PhysicalDeviceFaultFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	device_fault: Bool32,
	device_fault_vendor_binary: Bool32,
}

#[repr(C)]
pub struct DeviceDeviceMemoryReportCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: DeviceMemoryReportFlagsEXT,
	pfn_user_callback: PFN_vkDeviceMemoryReportCallbackEXT,
	p_user_data: *mut c_void,
}

#[repr(C)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID {
	s_type: StructureType,
	p_next: *const c_void,
	memory: DeviceMemory,
}

#[repr(C)]
pub struct PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	ycbcr2plane444_formats: Bool32,
}

#[repr(C)]
pub struct PipelineDynamicStateCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineDynamicStateCreateFlags,
	dynamic_state_count: u32,
	p_dynamic_states: *const DynamicState,
}

#[repr(C)]
pub struct PhysicalDeviceOpticalFlowPropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	supported_output_grid_sizes: OpticalFlowGridSizeFlagsNV,
	supported_hint_grid_sizes: OpticalFlowGridSizeFlagsNV,
	hint_supported: Bool32,
	cost_supported: Bool32,
	bidirectional_flow_supported: Bool32,
	global_flow_supported: Bool32,
	min_width: u32,
	min_height: u32,
	max_width: u32,
	max_height: u32,
	max_num_regions_of_interest: u32,
}

#[repr(C)]
pub struct PhysicalDeviceMaintenance4Properties {
	s_type: StructureType,
	p_next: *mut c_void,
	max_buffer_size: DeviceSize,
}

#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	timeline_semaphore: Bool32,
}

#[repr(C)]
pub struct BaseInStructure {
	s_type: StructureType,
	p_next: *const BaseInStructure,
}

#[repr(C)]
pub struct VideoEncodeH265RateControlInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: VideoEncodeH265RateControlFlagsEXT,
	gop_frame_count: u32,
	idr_period: u32,
	consecutive_bframe_count: u32,
	sub_layer_count: u32,
}

#[repr(C)]
pub struct PhysicalDevice16BitStorageFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	storage_buffer16_bit_access: Bool32,
	uniform_and_storage_buffer16_bit_access: Bool32,
	storage_push_constant16: Bool32,
	storage_input_output16: Bool32,
}

#[repr(C)]
pub struct AttachmentDescription2 {
	s_type: StructureType,
	p_next: *const c_void,
	flags: AttachmentDescriptionFlags,
	format: Format,
	samples: SampleCountFlagBits,
	load_op: AttachmentLoadOp,
	store_op: AttachmentStoreOp,
	stencil_load_op: AttachmentLoadOp,
	stencil_store_op: AttachmentStoreOp,
	initial_layout: ImageLayout,
	final_layout: ImageLayout,
}

#[repr(C)]
pub struct PhysicalDeviceClusterCullingShaderPropertiesHUAWEI {
	s_type: StructureType,
	p_next: *mut c_void,
	max_work_group_count: [u32; 3],
	max_work_group_size: [u32; 3],
	max_output_cluster_count: u32,
	indirect_buffer_offset_alignment: DeviceSize,
}

#[repr(C)]
pub struct PipelineRasterizationStateCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineRasterizationStateCreateFlags,
	depth_clamp_enable: Bool32,
	rasterizer_discard_enable: Bool32,
	polygon_mode: PolygonMode,
	cull_mode: CullModeFlags,
	front_face: FrontFace,
	depth_bias_enable: Bool32,
	depth_bias_constant_factor: f32,
	depth_bias_clamp: f32,
	depth_bias_slope_factor: f32,
	line_width: f32,
}

#[repr(C)]
pub struct DisplayModeParametersKHR {
	visible_region: Extent2D,
	refresh_rate: u32,
}

#[repr(C)]
pub struct PerformanceValueINTEL {
	r#type: PerformanceValueTypeINTEL,
	data: PerformanceValueDataINTEL,
}

#[repr(C)]
pub struct DispatchGraphInfoAMDX {
	node_index: u32,
	payload_count: u32,
	payloads: DeviceOrHostAddressConstAMDX,
	payload_stride: u64,
}

#[repr(C)]
pub struct SubmitInfo {
	s_type: StructureType,
	p_next: *const c_void,
	wait_semaphore_count: u32,
	p_wait_semaphores: *const Semaphore,
	p_wait_dst_stage_mask: *const PipelineStageFlags,
	command_buffer_count: u32,
	p_command_buffers: *const CommandBuffer,
	signal_semaphore_count: u32,
	p_signal_semaphores: *const Semaphore,
}

#[repr(C)]
pub struct PhysicalDeviceImageSlicedViewOf3DFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	image_sliced_view_of3_d: Bool32,
}

#[repr(C)]
pub struct VideoDecodeH265DpbSlotInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	p_std_reference_info: *const StdVideoDecodeH265ReferenceInfo,
}

#[repr(C)]
pub struct MultiviewPerViewAttributesInfoNVX {
	s_type: StructureType,
	p_next: *const c_void,
	per_view_attributes: Bool32,
	per_view_attributes_position_xonly: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceNestedCommandBufferPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	max_command_buffer_nesting_level: u32,
}

#[repr(C)]
pub struct PhysicalDeviceMaintenance5FeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	maintenance5: Bool32,
}

#[repr(C)]
pub struct PipelineCompilerControlCreateInfoAMD {
	s_type: StructureType,
	p_next: *const c_void,
	compiler_control_flags: PipelineCompilerControlFlagsAMD,
}

#[repr(C)]
pub struct ExportMetalObjectCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	export_object_type: ExportMetalObjectTypeFlagBitsEXT,
}

#[repr(C)]
pub struct AttachmentSampleCountInfoAMD {
	s_type: StructureType,
	p_next: *const c_void,
	color_attachment_count: u32,
	p_color_attachment_samples: *const SampleCountFlagBits,
	depth_stencil_attachment_samples: SampleCountFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceCubicClampFeaturesQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	cubic_range_clamp: Bool32,
}

#[repr(C)]
pub struct CopyMemoryToAccelerationStructureInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	src: DeviceOrHostAddressConstKHR,
	dst: AccelerationStructureKHR,
	mode: CopyAccelerationStructureModeKHR,
}

#[repr(C)]
pub struct PhysicalDeviceHostImageCopyPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	copy_src_layout_count: u32,
	p_copy_src_layouts: *mut ImageLayout,
	copy_dst_layout_count: u32,
	p_copy_dst_layouts: *mut ImageLayout,
	optimal_tiling_layout_uuid: [u8; UUID_SIZE],
	identical_memory_type_requirements: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	diagnostics_config: Bool32,
}

#[repr(C)]
pub struct ComponentMapping {
	r: ComponentSwizzle,
	g: ComponentSwizzle,
	b: ComponentSwizzle,
	a: ComponentSwizzle,
}

#[repr(C)]
pub struct VideoEncodeH265SessionParametersAddInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	std_vpscount: u32,
	p_std_vpss: *const StdVideoH265VideoParameterSet,
	std_spscount: u32,
	p_std_spss: *const StdVideoH265SequenceParameterSet,
	std_ppscount: u32,
	p_std_ppss: *const StdVideoH265PictureParameterSet,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	max_fragment_shading_rate_invocation_count: SampleCountFlagBits,
}

#[repr(C)]
pub struct PhysicalDevicePresentIdFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	present_id: Bool32,
}

#[repr(C)]
pub struct LayerSettingEXT {
	p_layer_name: *const u8,
	p_setting_name: *const u8,
	r#type: LayerSettingTypeEXT,
	value_count: u32,
	p_values: *const c_void,
}

#[repr(C)]
pub struct CopyBufferToImageInfo2 {
	s_type: StructureType,
	p_next: *const c_void,
	src_buffer: Buffer,
	dst_image: Image,
	dst_image_layout: ImageLayout,
	region_count: u32,
	p_regions: *const BufferImageCopy2,
}

#[repr(C)]
pub struct SampleLocationEXT {
	x: f32,
	y: f32,
}

#[repr(C)]
pub struct ViewportWScalingNV {
	xcoeff: f32,
	ycoeff: f32,
}

#[repr(C)]
pub struct PhysicalDeviceDisplacementMicromapFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	displacement_micromap: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	combined_image_sampler_density_map_descriptor_size: usize,
}

#[repr(C)]
pub struct CuLaunchInfoNVX {
	s_type: StructureType,
	p_next: *const c_void,
	function: CuFunctionNVX,
	grid_dim_x: u32,
	grid_dim_y: u32,
	grid_dim_z: u32,
	block_dim_x: u32,
	block_dim_y: u32,
	block_dim_z: u32,
	shared_mem_bytes: u32,
	param_count: usize,
	p_params: *const *const c_void,
	extra_count: usize,
	p_extras: *const *const c_void,
}

#[repr(C)]
pub struct ExternalFenceProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
	compatible_handle_types: ExternalFenceHandleTypeFlags,
	external_fence_features: ExternalFenceFeatureFlags,
}

#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	protected_no_fault: Bool32,
}

#[repr(C)]
pub struct DescriptorBufferInfo {
	buffer: Buffer,
	offset: DeviceSize,
	range: DeviceSize,
}

#[repr(C)]
pub struct PhysicalDeviceImageProcessingFeaturesQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	texture_sample_weighted: Bool32,
	texture_box_filter: Bool32,
	texture_block_match: Bool32,
}

#[repr(C)]
pub struct FragmentShadingRateAttachmentInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	p_fragment_shading_rate_attachment: *const AttachmentReference2,
	shading_rate_attachment_texel_size: Extent2D,
}

#[repr(C)]
pub struct VideoEncodeH265GopRemainingFrameInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	use_gop_remaining_frames: Bool32,
	gop_remaining_i: u32,
	gop_remaining_p: u32,
	gop_remaining_b: u32,
}

#[repr(C)]
pub struct FramebufferAttachmentImageInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: ImageCreateFlags,
	usage: ImageUsageFlags,
	width: u32,
	height: u32,
	layer_count: u32,
	view_format_count: u32,
	p_view_formats: *const Format,
}

#[repr(C)]
pub struct ImageCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: ImageCreateFlags,
	image_type: ImageType,
	format: Format,
	extent: Extent3D,
	mip_levels: u32,
	array_layers: u32,
	samples: SampleCountFlagBits,
	tiling: ImageTiling,
	usage: ImageUsageFlags,
	sharing_mode: SharingMode,
	queue_family_index_count: u32,
	p_queue_family_indices: *const u32,
	initial_layout: ImageLayout,
}

#[repr(C)]
pub struct TilePropertiesQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	tile_size: Extent3D,
	apron_size: Extent2D,
	origin: Offset2D,
}

#[repr(C)]
pub struct DescriptorUpdateTemplateCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: DescriptorUpdateTemplateCreateFlags,
	descriptor_update_entry_count: u32,
	p_descriptor_update_entries: *const DescriptorUpdateTemplateEntry,
	template_type: DescriptorUpdateTemplateType,
	descriptor_set_layout: DescriptorSetLayout,
	pipeline_bind_point: PipelineBindPoint,
	pipeline_layout: PipelineLayout,
	set: u32,
}

#[repr(C)]
pub struct PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	graphics_pipeline_library_fast_linking: Bool32,
	graphics_pipeline_library_independent_interpolation_decoration: Bool32,
}

#[repr(C)]
pub struct ImageMemoryRequirementsInfo2 {
	s_type: StructureType,
	p_next: *const c_void,
	image: Image,
}

#[repr(C)]
pub struct SysmemColorSpaceFUCHSIA {
	s_type: StructureType,
	p_next: *const c_void,
	color_space: u32,
}

#[repr(C)]
pub struct DeviceQueueInfo2 {
	s_type: StructureType,
	p_next: *const c_void,
	flags: DeviceQueueCreateFlags,
	queue_family_index: u32,
	queue_index: u32,
}

#[repr(C)]
pub struct RectLayerKHR {
	offset: Offset2D,
	extent: Extent2D,
	layer: u32,
}

#[repr(C)]
pub struct ExportSemaphoreCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	handle_types: ExternalSemaphoreHandleTypeFlags,
}

#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	advanced_blend_max_color_attachments: u32,
	advanced_blend_independent_blend: Bool32,
	advanced_blend_non_premultiplied_src_color: Bool32,
	advanced_blend_non_premultiplied_dst_color: Bool32,
	advanced_blend_correlated_overlap: Bool32,
	advanced_blend_all_operations: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceDepthBiasControlFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	depth_bias_control: Bool32,
	least_representable_value_force_unorm_representation: Bool32,
	float_representation: Bool32,
	depth_bias_exact: Bool32,
}

#[repr(C)]
pub struct VideoEncodeH265SessionCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	use_max_level_idc: Bool32,
	max_level_idc: StdVideoH265LevelIdc,
}

#[repr(C)]
pub struct VideoEncodeH265ProfileInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	std_profile_idc: StdVideoH265ProfileIdc,
}

#[repr(C)]
pub struct AttachmentDescriptionStencilLayout {
	s_type: StructureType,
	p_next: *mut c_void,
	stencil_initial_layout: ImageLayout,
	stencil_final_layout: ImageLayout,
}

#[repr(C)]
pub struct PastPresentationTimingGOOGLE {
	present_id: u32,
	desired_present_time: u64,
	actual_present_time: u64,
	earliest_present_time: u64,
	present_margin: u64,
}

#[repr(C)]
pub struct VertexInputBindingDivisorDescriptionEXT {
	binding: u32,
	divisor: u32,
}

#[repr(C)]
pub struct SurfaceFullScreenExclusiveInfoEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	full_screen_exclusive: FullScreenExclusiveEXT,
}

#[repr(C)]
pub struct PhysicalDeviceDepthClampZeroOneFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	depth_clamp_zero_one: Bool32,
}

#[repr(C)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	hmonitor: HMONITOR,
}

#[repr(C)]
pub struct DrmFormatModifierProperties2EXT {
	drm_format_modifier: u64,
	drm_format_modifier_plane_count: u32,
	drm_format_modifier_tiling_features: FormatFeatureFlags2,
}

#[repr(C)]
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD {
	s_type: StructureType,
	p_next: *mut c_void,
	local_dimming_support: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceLinearColorAttachmentFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	linear_color_attachment: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	cooperative_matrix_supported_stages: ShaderStageFlags,
}

#[repr(C)]
pub struct PhysicalDeviceShaderModuleIdentifierFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_module_identifier: Bool32,
}

#[repr(C)]
pub struct QueueFamilyCheckpointPropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	checkpoint_execution_stage_mask: PipelineStageFlags,
}

#[repr(C)]
pub struct BufferDeviceAddressCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	device_address: DeviceAddress,
}

#[repr(C)]
pub struct ExternalFormatQNX {
	s_type: StructureType,
	p_next: *mut c_void,
	external_format: u64,
}

#[repr(C)]
pub struct SemaphoreSciSyncPoolCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	handle: NvSciSyncObj,
}

#[repr(C)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	binding_count: u32,
	p_binding_flags: *const DescriptorBindingFlags,
}

#[repr(C)]
pub struct RefreshObjectListKHR {
	s_type: StructureType,
	p_next: *const c_void,
	object_count: u32,
	p_objects: *const RefreshObjectKHR,
}

#[repr(C)]
pub struct SparseMemoryBind {
	resource_offset: DeviceSize,
	size: DeviceSize,
	memory: DeviceMemory,
	memory_offset: DeviceSize,
	flags: SparseMemoryBindFlags,
}

#[repr(C)]
pub struct SemaphoreSubmitInfo {
	s_type: StructureType,
	p_next: *const c_void,
	semaphore: Semaphore,
	value: u64,
	stage_mask: PipelineStageFlags2,
	device_index: u32,
}

#[repr(C)]
pub struct ImageSubresource {
	aspect_mask: ImageAspectFlags,
	mip_level: u32,
	array_layer: u32,
}

#[repr(C)]
pub struct PhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	subpass_merge_feedback: Bool32,
}

#[repr(C)]
pub struct ImportSemaphoreWin32HandleInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	semaphore: Semaphore,
	flags: SemaphoreImportFlags,
	handle_type: ExternalSemaphoreHandleTypeFlagBits,
	handle: HANDLE,
	name: LPCWSTR,
}

#[repr(C)]
pub struct VideoDecodeH264PictureInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	p_std_picture_info: *const StdVideoDecodeH264PictureInfo,
	slice_count: u32,
	p_slice_offsets: *const u32,
}

#[repr(C)]
pub struct BufferCollectionImageCreateInfoFUCHSIA {
	s_type: StructureType,
	p_next: *const c_void,
	collection: BufferCollectionFUCHSIA,
	index: u32,
}

#[repr(C)]
pub struct RenderingFragmentShadingRateAttachmentInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	image_view: ImageView,
	image_layout: ImageLayout,
	shading_rate_attachment_texel_size: Extent2D,
}

#[repr(C)]
pub struct RenderingAreaInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	view_mask: u32,
	color_attachment_count: u32,
	p_color_attachment_formats: *const Format,
	depth_attachment_format: Format,
	stencil_attachment_format: Format,
}

#[repr(C)]
pub struct RenderPassCreateInfo2 {
	s_type: StructureType,
	p_next: *const c_void,
	flags: RenderPassCreateFlags,
	attachment_count: u32,
	p_attachments: *const AttachmentDescription2,
	subpass_count: u32,
	p_subpasses: *const SubpassDescription2,
	dependency_count: u32,
	p_dependencies: *const SubpassDependency2,
	correlated_view_mask_count: u32,
	p_correlated_view_masks: *const u32,
}

#[repr(C)]
pub struct PhysicalDeviceProvokingVertexFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	provoking_vertex_last: Bool32,
	transform_feedback_preserves_provoking_vertex: Bool32,
}

#[repr(C)]
pub struct ClearAttachment {
	aspect_mask: ImageAspectFlags,
	color_attachment: u32,
	clear_value: ClearValue,
}

#[repr(C)]
pub struct DebugUtilsMessengerCallbackDataEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: DebugUtilsMessengerCallbackDataFlagsEXT,
	p_message_id_name: *const u8,
	message_id_number: i32,
	p_message: *const u8,
	queue_label_count: u32,
	p_queue_labels: *const DebugUtilsLabelEXT,
	cmd_buf_label_count: u32,
	p_cmd_buf_labels: *const DebugUtilsLabelEXT,
	object_count: u32,
	p_objects: *const DebugUtilsObjectNameInfoEXT,
}

#[repr(C)]
pub struct PerformanceOverrideInfoINTEL {
	s_type: StructureType,
	p_next: *const c_void,
	r#type: PerformanceOverrideTypeINTEL,
	enable: Bool32,
	parameter: u64,
}

#[repr(C)]
pub struct BufferImageCopy {
	buffer_offset: DeviceSize,
	buffer_row_length: u32,
	buffer_image_height: u32,
	image_subresource: ImageSubresourceLayers,
	image_offset: Offset3D,
	image_extent: Extent3D,
}

#[repr(C)]
pub struct MemoryRequirements {
	size: DeviceSize,
	alignment: DeviceSize,
	memory_type_bits: u32,
}

#[repr(C)]
pub struct EventCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: EventCreateFlags,
}

#[repr(C)]
pub struct PhysicalDeviceRayTracingMaintenance1FeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	ray_tracing_maintenance1: Bool32,
	ray_tracing_pipeline_trace_rays_indirect2: Bool32,
}

#[repr(C)]
pub struct FaultData {
	s_type: StructureType,
	p_next: *mut c_void,
	fault_level: FaultLevel,
	fault_type: FaultType,
}

#[repr(C)]
pub struct ShaderCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: ShaderCreateFlagsEXT,
	stage: ShaderStageFlagBits,
	next_stage: ShaderStageFlags,
	code_type: ShaderCodeTypeEXT,
	code_size: usize,
	p_code: *const c_void,
	p_name: *const u8,
	set_layout_count: u32,
	p_set_layouts: *const DescriptorSetLayout,
	push_constant_range_count: u32,
	p_push_constant_ranges: *const PushConstantRange,
	p_specialization_info: *const SpecializationInfo,
}

#[repr(C)]
pub struct BufferUsageFlags2CreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	usage: BufferUsageFlags2KHR,
}

#[repr(C)]
pub struct SwapchainPresentFenceInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	swapchain_count: u32,
	p_fences: *const Fence,
}

#[repr(C)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	vertex_binding_divisor_count: u32,
	p_vertex_binding_divisors: *const VertexInputBindingDivisorDescriptionEXT,
}

#[repr(C)]
pub struct DebugUtilsObjectTagInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	object_type: ObjectType,
	object_handle: u64,
	tag_name: u64,
	tag_size: usize,
	p_tag: *const c_void,
}

#[repr(C)]
pub struct OpticalFlowImageFormatPropertiesNV {
	s_type: StructureType,
	p_next: *const c_void,
	format: Format,
}

#[repr(C)]
pub struct PipelineCreationFeedback {
	flags: PipelineCreationFeedbackFlags,
	duration: u64,
}

#[repr(C)]
pub struct PipelineCoverageModulationStateCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineCoverageModulationStateCreateFlagsNV,
	coverage_modulation_mode: CoverageModulationModeNV,
	coverage_modulation_table_enable: Bool32,
	coverage_modulation_table_count: u32,
	p_coverage_modulation_table: *const f32,
}

#[repr(C)]
pub struct PipelineOfflineCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	pipeline_identifier: [u8; UUID_SIZE],
	match_control: PipelineMatchControl,
	pool_entry_size: DeviceSize,
}

#[repr(C)]
pub struct PerformanceCounterKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	unit: PerformanceCounterUnitKHR,
	scope: PerformanceCounterScopeKHR,
	storage: PerformanceCounterStorageKHR,
	uuid: [u8; UUID_SIZE],
}

#[repr(C)]
pub struct VideoEncodeH265SessionParametersFeedbackInfoEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	has_std_vpsoverrides: Bool32,
	has_std_spsoverrides: Bool32,
	has_std_ppsoverrides: Bool32,
}

#[repr(C)]
pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	provoking_vertex_mode: ProvokingVertexModeEXT,
}

#[repr(C)]
pub struct BlitImageInfo2 {
	s_type: StructureType,
	p_next: *const c_void,
	src_image: Image,
	src_image_layout: ImageLayout,
	dst_image: Image,
	dst_image_layout: ImageLayout,
	region_count: u32,
	p_regions: *const ImageBlit2,
	filter: Filter,
}

#[repr(C)]
pub struct PhysicalDeviceMemoryProperties2 {
	s_type: StructureType,
	p_next: *mut c_void,
	memory_properties: PhysicalDeviceMemoryProperties,
}

#[repr(C)]
pub struct Win32SurfaceCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: Win32SurfaceCreateFlagsKHR,
	hinstance: HINSTANCE,
	hwnd: HWND,
}

#[repr(C)]
pub struct VideoCodingControlInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: VideoCodingControlFlagsKHR,
}

#[repr(C)]
pub struct ScreenBufferPropertiesQNX {
	s_type: StructureType,
	p_next: *mut c_void,
	allocation_size: DeviceSize,
	memory_type_bits: u32,
}

#[repr(C)]
pub struct AccelerationStructureGeometryTrianglesDataKHR {
	s_type: StructureType,
	p_next: *const c_void,
	vertex_format: Format,
	vertex_data: DeviceOrHostAddressConstKHR,
	vertex_stride: DeviceSize,
	max_vertex: u32,
	index_type: IndexType,
	index_data: DeviceOrHostAddressConstKHR,
	transform_data: DeviceOrHostAddressConstKHR,
}

#[repr(C)]
pub struct PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	primitives_generated_query: Bool32,
	primitives_generated_query_with_rasterizer_discard: Bool32,
	primitives_generated_query_with_non_zero_streams: Bool32,
}

#[repr(C)]
pub struct WriteDescriptorSet {
	s_type: StructureType,
	p_next: *const c_void,
	dst_set: DescriptorSet,
	dst_binding: u32,
	dst_array_element: u32,
	descriptor_count: u32,
	descriptor_type: DescriptorType,
	p_image_info: *const DescriptorImageInfo,
	p_buffer_info: *const DescriptorBufferInfo,
	p_texel_buffer_view: *const BufferView,
}

#[repr(C)]
pub struct ImageViewSampleWeightCreateInfoQCOM {
	s_type: StructureType,
	p_next: *const c_void,
	filter_center: Offset2D,
	filter_size: Extent2D,
	num_phases: u32,
}

#[repr(C)]
pub struct DisplayModeCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: DisplayModeCreateFlagsKHR,
	parameters: DisplayModeParametersKHR,
}

#[repr(C)]
pub struct ExternalMemoryAcquireUnmodifiedEXT {
	s_type: StructureType,
	p_next: *const c_void,
	acquire_unmodified_memory: Bool32,
}

#[repr(C)]
pub struct DedicatedAllocationImageCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	dedicated_allocation: Bool32,
}

#[repr(C)]
pub struct VideoDecodeCapabilitiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	flags: VideoDecodeCapabilityFlagsKHR,
}

#[repr(C)]
pub struct DirectDriverLoadingListLUNARG {
	s_type: StructureType,
	p_next: *const c_void,
	mode: DirectDriverLoadingModeLUNARG,
	driver_count: u32,
	p_drivers: *const DirectDriverLoadingInfoLUNARG,
}

#[repr(C)]
pub struct DescriptorSetLayoutSupport {
	s_type: StructureType,
	p_next: *mut c_void,
	supported: Bool32,
}

#[repr(C)]
pub struct VideoBeginCodingInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: VideoBeginCodingFlagsKHR,
	video_session: VideoSessionKHR,
	video_session_parameters: VideoSessionParametersKHR,
	reference_slot_count: u32,
	p_reference_slots: *const VideoReferenceSlotInfoKHR,
}

#[repr(C)]
pub struct ValidationFlagsEXT {
	s_type: StructureType,
	p_next: *const c_void,
	disabled_validation_check_count: u32,
	p_disabled_validation_checks: *const ValidationCheckEXT,
}

#[repr(C)]
pub struct SurfaceProtectedCapabilitiesKHR {
	s_type: StructureType,
	p_next: *const c_void,
	supports_protected: Bool32,
}

#[repr(C)]
pub struct CopyImageToMemoryInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: HostImageCopyFlagsEXT,
	src_image: Image,
	src_image_layout: ImageLayout,
	region_count: u32,
	p_regions: *const ImageToMemoryCopyEXT,
}

#[repr(C)]
pub struct VideoFormatPropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	format: Format,
	component_mapping: ComponentMapping,
	image_create_flags: ImageCreateFlags,
	image_type: ImageType,
	image_tiling: ImageTiling,
	image_usage_flags: ImageUsageFlags,
}

#[repr(C)]
pub struct DrawIndexedIndirectCommand {
	index_count: u32,
	instance_count: u32,
	first_index: u32,
	vertex_offset: i32,
	first_instance: u32,
}

#[repr(C)]
pub struct AccelerationStructureMotionInstanceNV {
	r#type: AccelerationStructureMotionInstanceTypeNV,
	flags: AccelerationStructureMotionInstanceFlagsNV,
	data: AccelerationStructureMotionInstanceDataNV,
}

#[repr(C)]
pub struct ExternalMemoryBufferCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	handle_types: ExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct PipelineColorBlendStateCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineColorBlendStateCreateFlags,
	logic_op_enable: Bool32,
	logic_op: LogicOp,
	attachment_count: u32,
	p_attachments: *const PipelineColorBlendAttachmentState,
	blend_constants: [f32; 4],
}

#[repr(C)]
pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	vertex_input_dynamic_state: Bool32,
}

#[repr(C)]
pub struct Offset3D {
	x: i32,
	y: i32,
	z: i32,
}

#[repr(C)]
pub struct PipelineViewportSwizzleStateCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineViewportSwizzleStateCreateFlagsNV,
	viewport_count: u32,
	p_viewport_swizzles: *const ViewportSwizzleNV,
}

#[repr(C)]
pub struct MemoryGetWin32HandleInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	memory: DeviceMemory,
	handle_type: ExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceMaintenance3Properties {
	s_type: StructureType,
	p_next: *mut c_void,
	max_per_set_descriptors: u32,
	max_memory_allocation_size: DeviceSize,
}

#[repr(C)]
pub struct PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI {
	s_type: StructureType,
	p_next: *mut c_void,
	cluster_shading_rate: Bool32,
}

#[repr(C)]
pub struct VideoDecodeH264CapabilitiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	max_level_idc: StdVideoH264LevelIdc,
	field_offset_granularity: Offset2D,
}

#[repr(C)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	exclusive_scissor: Bool32,
}

#[repr(C)]
pub struct SwapchainImageCreateInfoANDROID {
	s_type: StructureType,
	p_next: *const c_void,
	usage: SwapchainImageUsageFlagsANDROID,
}

#[repr(C)]
pub struct CopyMemoryToMicromapInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	src: DeviceOrHostAddressConstKHR,
	dst: MicromapEXT,
	mode: CopyMicromapModeEXT,
}

#[repr(C)]
pub struct ExportMemoryAllocateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	handle_types: ExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct CopyImageToImageInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: HostImageCopyFlagsEXT,
	src_image: Image,
	src_image_layout: ImageLayout,
	dst_image: Image,
	dst_image_layout: ImageLayout,
	region_count: u32,
	p_regions: *const ImageCopy2,
}

#[repr(C)]
pub struct VideoEncodeInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: VideoEncodeFlagsKHR,
	dst_buffer: Buffer,
	dst_buffer_offset: DeviceSize,
	dst_buffer_range: DeviceSize,
	src_picture_resource: VideoPictureResourceInfoKHR,
	p_setup_reference_slot: *const VideoReferenceSlotInfoKHR,
	reference_slot_count: u32,
	p_reference_slots: *const VideoReferenceSlotInfoKHR,
	preceding_externally_encoded_bytes: u32,
}

#[repr(C)]
pub struct PipelineDiscardRectangleStateCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineDiscardRectangleStateCreateFlagsEXT,
	discard_rectangle_mode: DiscardRectangleModeEXT,
	discard_rectangle_count: u32,
	p_discard_rectangles: *const Rect2D,
}

#[repr(C)]
pub struct PipelineLibraryCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	library_count: u32,
	p_libraries: *const Pipeline,
}

#[repr(C)]
pub struct PhysicalDeviceShaderClockFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_subgroup_clock: Bool32,
	shader_device_clock: Bool32,
}

#[repr(C)]
pub struct BufferMemoryBarrier2 {
	s_type: StructureType,
	p_next: *const c_void,
	src_stage_mask: PipelineStageFlags2,
	src_access_mask: AccessFlags2,
	dst_stage_mask: PipelineStageFlags2,
	dst_access_mask: AccessFlags2,
	src_queue_family_index: u32,
	dst_queue_family_index: u32,
	buffer: Buffer,
	offset: DeviceSize,
	size: DeviceSize,
}

#[repr(C)]
pub struct PhysicalDeviceTilePropertiesFeaturesQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	tile_properties: Bool32,
}

#[repr(C)]
pub struct PipelineRasterizationStateRasterizationOrderAMD {
	s_type: StructureType,
	p_next: *const c_void,
	rasterization_order: RasterizationOrderAMD,
}

#[repr(C)]
pub struct DeviceGroupBindSparseInfo {
	s_type: StructureType,
	p_next: *const c_void,
	resource_device_index: u32,
	memory_device_index: u32,
}

#[repr(C)]
pub struct ExportMetalSharedEventInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	semaphore: Semaphore,
	event: Event,
	mtl_shared_event: MTLSharedEvent_id,
}

#[repr(C)]
pub struct PhysicalDeviceVulkanMemoryModelFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	vulkan_memory_model: Bool32,
	vulkan_memory_model_device_scope: Bool32,
	vulkan_memory_model_availability_visibility_chains: Bool32,
}

#[repr(C)]
pub struct ImageCompressionPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	image_compression_flags: ImageCompressionFlagsEXT,
	image_compression_fixed_rate_flags: ImageCompressionFixedRateFlagsEXT,
}

#[repr(C)]
pub struct MemoryWin32HandlePropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	memory_type_bits: u32,
}

#[repr(C)]
pub struct RenderPassTransformBeginInfoQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	transform: SurfaceTransformFlagBitsKHR,
}

#[repr(C)]
pub struct PhysicalDeviceRelaxedLineRasterizationFeaturesIMG {
	s_type: StructureType,
	p_next: *mut c_void,
	relaxed_line_rasterization: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	min_fragment_density_texel_size: Extent2D,
	max_fragment_density_texel_size: Extent2D,
	fragment_density_invocations: Bool32,
}

#[repr(C)]
pub struct BindPipelineIndirectCommandNV {
	pipeline_address: DeviceAddress,
}

#[repr(C)]
pub struct PhysicalDeviceRenderPassStripedFeaturesARM {
	s_type: StructureType,
	p_next: *mut c_void,
	render_pass_striped: Bool32,
}

#[repr(C)]
pub struct HdrMetadataEXT {
	s_type: StructureType,
	p_next: *const c_void,
	display_primary_red: XYColorEXT,
	display_primary_green: XYColorEXT,
	display_primary_blue: XYColorEXT,
	white_point: XYColorEXT,
	max_luminance: f32,
	min_luminance: f32,
	max_content_light_level: f32,
	max_frame_average_light_level: f32,
}

#[repr(C)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineRasterizationConservativeStateCreateFlagsEXT,
	conservative_rasterization_mode: ConservativeRasterizationModeEXT,
	extra_primitive_overestimation_size: f32,
}

#[repr(C)]
pub struct PhysicalDevicePortabilitySubsetPropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	min_vertex_input_binding_stride_alignment: u32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_subgroup_extended_types: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	conditional_rendering: Bool32,
	inherited_conditional_rendering: Bool32,
}

#[repr(C)]
pub struct VideoEncodeQualityLevelPropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	preferred_rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
	preferred_rate_control_layer_count: u32,
}

#[repr(C)]
pub struct VideoDecodeH264SessionParametersCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	max_std_spscount: u32,
	max_std_ppscount: u32,
	p_parameters_add_info: *const VideoDecodeH264SessionParametersAddInfoKHR,
}

#[repr(C)]
pub struct AccelerationStructureTrianglesOpacityMicromapEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	index_type: IndexType,
	index_buffer: DeviceOrHostAddressConstKHR,
	index_stride: DeviceSize,
	base_triangle: u32,
	usage_counts_count: u32,
	p_usage_counts: *const MicromapUsageEXT,
	pp_usage_counts: *const *const MicromapUsageEXT,
	micromap: MicromapEXT,
}

#[repr(C)]
pub struct PhysicalDeviceVulkan11Properties {
	s_type: StructureType,
	p_next: *mut c_void,
	device_uuid: [u8; UUID_SIZE],
	driver_uuid: [u8; UUID_SIZE],
	device_luid: [u8; LUID_SIZE],
	device_node_mask: u32,
	device_luidvalid: Bool32,
	subgroup_size: u32,
	subgroup_supported_stages: ShaderStageFlags,
	subgroup_supported_operations: SubgroupFeatureFlags,
	subgroup_quad_operations_in_all_stages: Bool32,
	point_clipping_behavior: PointClippingBehavior,
	max_multiview_view_count: u32,
	max_multiview_instance_index: u32,
	protected_no_fault: Bool32,
	max_per_set_descriptors: u32,
	max_memory_allocation_size: DeviceSize,
}

#[repr(C)]
pub struct MemoryHeap {
	size: DeviceSize,
	flags: MemoryHeapFlags,
}

#[repr(C)]
pub struct RayTracingPipelineInterfaceCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	max_pipeline_ray_payload_size: u32,
	max_pipeline_ray_hit_attribute_size: u32,
}

#[repr(C)]
pub struct DepthBiasInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	depth_bias_constant_factor: f32,
	depth_bias_clamp: f32,
	depth_bias_slope_factor: f32,
}

#[repr(C)]
pub struct MultiDrawInfoEXT {
	first_vertex: u32,
	vertex_count: u32,
}

#[repr(C)]
pub struct PhysicalDeviceVulkan12Features {
	s_type: StructureType,
	p_next: *mut c_void,
	sampler_mirror_clamp_to_edge: Bool32,
	draw_indirect_count: Bool32,
	storage_buffer8_bit_access: Bool32,
	uniform_and_storage_buffer8_bit_access: Bool32,
	storage_push_constant8: Bool32,
	shader_buffer_int64_atomics: Bool32,
	shader_shared_int64_atomics: Bool32,
	shader_float16: Bool32,
	shader_int8: Bool32,
	descriptor_indexing: Bool32,
	shader_input_attachment_array_dynamic_indexing: Bool32,
	shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
	shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
	shader_uniform_buffer_array_non_uniform_indexing: Bool32,
	shader_sampled_image_array_non_uniform_indexing: Bool32,
	shader_storage_buffer_array_non_uniform_indexing: Bool32,
	shader_storage_image_array_non_uniform_indexing: Bool32,
	shader_input_attachment_array_non_uniform_indexing: Bool32,
	shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
	shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
	descriptor_binding_uniform_buffer_update_after_bind: Bool32,
	descriptor_binding_sampled_image_update_after_bind: Bool32,
	descriptor_binding_storage_image_update_after_bind: Bool32,
	descriptor_binding_storage_buffer_update_after_bind: Bool32,
	descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
	descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
	descriptor_binding_update_unused_while_pending: Bool32,
	descriptor_binding_partially_bound: Bool32,
	descriptor_binding_variable_descriptor_count: Bool32,
	runtime_descriptor_array: Bool32,
	sampler_filter_minmax: Bool32,
	scalar_block_layout: Bool32,
	imageless_framebuffer: Bool32,
	uniform_buffer_standard_layout: Bool32,
	shader_subgroup_extended_types: Bool32,
	separate_depth_stencil_layouts: Bool32,
	host_query_reset: Bool32,
	timeline_semaphore: Bool32,
	buffer_device_address: Bool32,
	buffer_device_address_capture_replay: Bool32,
	buffer_device_address_multi_device: Bool32,
	vulkan_memory_model: Bool32,
	vulkan_memory_model_device_scope: Bool32,
	vulkan_memory_model_availability_visibility_chains: Bool32,
	shader_output_viewport_index: Bool32,
	shader_output_layer: Bool32,
	subgroup_broadcast_dynamic_id: Bool32,
}

#[repr(C)]
pub struct DescriptorImageInfo {
	sampler: Sampler,
	image_view: ImageView,
	image_layout: ImageLayout,
}

#[repr(C)]
pub struct PipelineViewportStateCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineViewportStateCreateFlags,
	viewport_count: u32,
	p_viewports: *const Viewport,
	scissor_count: u32,
	p_scissors: *const Rect2D,
}

#[repr(C)]
pub struct MultisamplePropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	max_sample_location_grid_size: Extent2D,
}

#[repr(C)]
pub struct DeviceGroupDeviceCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	physical_device_count: u32,
	p_physical_devices: *const PhysicalDevice,
}

#[repr(C)]
pub struct PushConstantRange {
	stage_flags: ShaderStageFlags,
	offset: u32,
	size: u32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderEnqueueFeaturesAMDX {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_enqueue: Bool32,
}

#[repr(C)]
pub struct PhysicalDevicePipelinePropertiesFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	pipeline_properties_identifier: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceTextureCompressionASTCHDRFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	texture_compression_astc_hdr: Bool32,
}

#[repr(C)]
pub struct MemoryRequirements2 {
	s_type: StructureType,
	p_next: *mut c_void,
	memory_requirements: MemoryRequirements,
}

#[repr(C)]
pub struct FenceGetFdInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	fence: Fence,
	handle_type: ExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	multiview_per_view_viewports: Bool32,
}

#[repr(C)]
pub struct PipelineIndirectDeviceAddressInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	pipeline_bind_point: PipelineBindPoint,
	pipeline: Pipeline,
}

#[repr(C)]
pub struct PresentRegionsKHR {
	s_type: StructureType,
	p_next: *const c_void,
	swapchain_count: u32,
	p_regions: *const PresentRegionKHR,
}

#[repr(C)]
pub struct HostImageCopyDevicePerformanceQueryEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	optimal_device_access: Bool32,
	identical_memory_layout: Bool32,
}

#[repr(C)]
pub struct VideoEndCodingInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: VideoEndCodingFlagsKHR,
}

#[repr(C)]
pub struct ImportFenceWin32HandleInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	fence: Fence,
	flags: FenceImportFlags,
	handle_type: ExternalFenceHandleTypeFlagBits,
	handle: HANDLE,
	name: LPCWSTR,
}

#[repr(C)]
pub struct MicromapUsageEXT {
	count: u32,
	subdivision_level: u32,
	format: u32,
}

#[repr(C)]
pub struct PhysicalDeviceSwapchainMaintenance1FeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	swapchain_maintenance1: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceVulkan13Features {
	s_type: StructureType,
	p_next: *mut c_void,
	robust_image_access: Bool32,
	inline_uniform_block: Bool32,
	descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
	pipeline_creation_cache_control: Bool32,
	private_data: Bool32,
	shader_demote_to_helper_invocation: Bool32,
	shader_terminate_invocation: Bool32,
	subgroup_size_control: Bool32,
	compute_full_subgroups: Bool32,
	synchronization2: Bool32,
	texture_compression_astc_hdr: Bool32,
	shader_zero_initialize_workgroup_memory: Bool32,
	dynamic_rendering: Bool32,
	shader_integer_dot_product: Bool32,
	maintenance4: Bool32,
}

#[repr(C)]
pub struct ExecutionGraphPipelineCreateInfoAMDX {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineCreateFlags,
	stage_count: u32,
	p_stages: *const PipelineShaderStageCreateInfo,
	p_library_info: *const PipelineLibraryCreateInfoKHR,
	layout: PipelineLayout,
	base_pipeline_handle: Pipeline,
	base_pipeline_index: i32,
}

#[repr(C)]
pub struct AccelerationStructureTrianglesDisplacementMicromapNV {
	s_type: StructureType,
	p_next: *mut c_void,
	displacement_bias_and_scale_format: Format,
	displacement_vector_format: Format,
	displacement_bias_and_scale_buffer: DeviceOrHostAddressConstKHR,
	displacement_bias_and_scale_stride: DeviceSize,
	displacement_vector_buffer: DeviceOrHostAddressConstKHR,
	displacement_vector_stride: DeviceSize,
	displaced_micromap_primitive_flags: DeviceOrHostAddressConstKHR,
	displaced_micromap_primitive_flags_stride: DeviceSize,
	index_type: IndexType,
	index_buffer: DeviceOrHostAddressConstKHR,
	index_stride: DeviceSize,
	base_triangle: u32,
	usage_counts_count: u32,
	p_usage_counts: *const MicromapUsageEXT,
	pp_usage_counts: *const *const MicromapUsageEXT,
	micromap: MicromapEXT,
}

#[repr(C)]
pub struct VideoDecodeH265SessionParametersCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	max_std_vpscount: u32,
	max_std_spscount: u32,
	max_std_ppscount: u32,
	p_parameters_add_info: *const VideoDecodeH265SessionParametersAddInfoKHR,
}

#[repr(C)]
pub struct ImportMetalBufferInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	mtl_buffer: MTLBuffer_id,
}

#[repr(C)]
pub struct SparseImageMemoryRequirements2 {
	s_type: StructureType,
	p_next: *mut c_void,
	memory_requirements: SparseImageMemoryRequirements,
}

#[repr(C)]
pub struct PhysicalDeviceMeshShaderPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	max_task_work_group_total_count: u32,
	max_task_work_group_count: [u32; 3],
	max_task_work_group_invocations: u32,
	max_task_work_group_size: [u32; 3],
	max_task_payload_size: u32,
	max_task_shared_memory_size: u32,
	max_task_payload_and_shared_memory_size: u32,
	max_mesh_work_group_total_count: u32,
	max_mesh_work_group_count: [u32; 3],
	max_mesh_work_group_invocations: u32,
	max_mesh_work_group_size: [u32; 3],
	max_mesh_shared_memory_size: u32,
	max_mesh_payload_and_shared_memory_size: u32,
	max_mesh_output_memory_size: u32,
	max_mesh_payload_and_output_memory_size: u32,
	max_mesh_output_components: u32,
	max_mesh_output_vertices: u32,
	max_mesh_output_primitives: u32,
	max_mesh_output_layers: u32,
	max_mesh_multiview_view_count: u32,
	mesh_output_per_vertex_granularity: u32,
	mesh_output_per_primitive_granularity: u32,
	max_preferred_task_work_group_invocations: u32,
	max_preferred_mesh_work_group_invocations: u32,
	prefers_local_invocation_vertex_output: Bool32,
	prefers_local_invocation_primitive_output: Bool32,
	prefers_compact_vertex_output: Bool32,
	prefers_compact_primitive_output: Bool32,
}

#[repr(C)]
pub struct DescriptorUpdateTemplateEntry {
	dst_binding: u32,
	dst_array_element: u32,
	descriptor_count: u32,
	descriptor_type: DescriptorType,
	offset: usize,
	stride: usize,
}

#[repr(C)]
pub struct PipelineViewportWScalingStateCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	viewport_wscaling_enable: Bool32,
	viewport_count: u32,
	p_viewport_wscalings: *const ViewportWScalingNV,
}

#[repr(C)]
pub struct ImageCaptureDescriptorDataInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	image: Image,
}

#[repr(C)]
pub struct ShaderModuleValidationCacheCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	validation_cache: ValidationCacheEXT,
}

#[repr(C)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	sample_location_sample_counts: SampleCountFlags,
	max_sample_location_grid_size: Extent2D,
	sample_location_coordinate_range: [f32; 2],
	sample_location_sub_pixel_bits: u32,
	variable_sample_locations: Bool32,
}

#[repr(C)]
pub struct FramebufferAttachmentsCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	attachment_image_info_count: u32,
	p_attachment_image_infos: *const FramebufferAttachmentImageInfo,
}

#[repr(C)]
pub struct MappedMemoryRange {
	s_type: StructureType,
	p_next: *const c_void,
	memory: DeviceMemory,
	offset: DeviceSize,
	size: DeviceSize,
}

#[repr(C)]
pub struct VideoEncodeH264FrameSizeEXT {
	frame_isize: u32,
	frame_psize: u32,
	frame_bsize: u32,
}

#[repr(C)]
pub struct DeviceEventInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	device_event: DeviceEventTypeEXT,
}

#[repr(C)]
pub struct PipelineInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	pipeline: Pipeline,
}

#[repr(C)]
pub struct SwapchainCounterCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	surface_counters: SurfaceCounterFlagsEXT,
}

#[repr(C)]
pub struct SubpassBeginInfo {
	s_type: StructureType,
	p_next: *const c_void,
	contents: SubpassContents,
}

#[repr(C)]
pub struct PhysicalDeviceInvocationMaskFeaturesHUAWEI {
	s_type: StructureType,
	p_next: *mut c_void,
	invocation_mask: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	extended_dynamic_state2: Bool32,
	extended_dynamic_state2_logic_op: Bool32,
	extended_dynamic_state2_patch_control_points: Bool32,
}

#[repr(C)]
pub struct ViewportSwizzleNV {
	x: ViewportCoordinateSwizzleNV,
	y: ViewportCoordinateSwizzleNV,
	z: ViewportCoordinateSwizzleNV,
	w: ViewportCoordinateSwizzleNV,
}

#[repr(C)]
pub struct PipelineCreateFlags2CreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineCreateFlags2KHR,
}

#[repr(C)]
pub struct FenceCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: FenceCreateFlags,
}

#[repr(C)]
pub struct ImageFormatConstraintsInfoFUCHSIA {
	s_type: StructureType,
	p_next: *const c_void,
	image_create_info: ImageCreateInfo,
	required_format_features: FormatFeatureFlags,
	flags: ImageFormatConstraintsFlagsFUCHSIA,
	sysmem_pixel_format: u64,
	color_space_count: u32,
	p_color_spaces: *const SysmemColorSpaceFUCHSIA,
}

#[repr(C)]
pub struct PhysicalDeviceSynchronization2Features {
	s_type: StructureType,
	p_next: *mut c_void,
	synchronization2: Bool32,
}

#[repr(C)]
pub struct SubpassDescription2 {
	s_type: StructureType,
	p_next: *const c_void,
	flags: SubpassDescriptionFlags,
	pipeline_bind_point: PipelineBindPoint,
	view_mask: u32,
	input_attachment_count: u32,
	p_input_attachments: *const AttachmentReference2,
	color_attachment_count: u32,
	p_color_attachments: *const AttachmentReference2,
	p_resolve_attachments: *const AttachmentReference2,
	p_depth_stencil_attachment: *const AttachmentReference2,
	preserve_attachment_count: u32,
	p_preserve_attachments: *const u32,
}

#[repr(C)]
pub struct SwapchainPresentModesCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	present_mode_count: u32,
	p_present_modes: *const PresentModeKHR,
}

#[repr(C)]
pub struct ExportMemoryWin32HandleInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	p_attributes: *const SECURITY_ATTRIBUTES,
	dw_access: DWORD,
	name: LPCWSTR,
}

#[repr(C)]
pub struct PhysicalDeviceInheritedViewportScissorFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	inherited_viewport_scissor2_d: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceVulkanSC10Features {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_atomic_instructions: Bool32,
}

#[repr(C)]
pub struct VideoEncodeH264RateControlInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: VideoEncodeH264RateControlFlagsEXT,
	gop_frame_count: u32,
	idr_period: u32,
	consecutive_bframe_count: u32,
	temporal_layer_count: u32,
}

#[repr(C)]
pub struct PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	workgroup_memory_explicit_layout: Bool32,
	workgroup_memory_explicit_layout_scalar_block_layout: Bool32,
	workgroup_memory_explicit_layout8_bit_access: Bool32,
	workgroup_memory_explicit_layout16_bit_access: Bool32,
}

#[repr(C)]
pub struct ImportMemoryFdInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	handle_type: ExternalMemoryHandleTypeFlagBits,
	fd: isize,
}

#[repr(C)]
pub struct LatencySleepInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	signal_semaphore: Semaphore,
	value: u64,
}

#[repr(C)]
pub struct CommandPoolMemoryConsumption {
	s_type: StructureType,
	p_next: *mut c_void,
	command_pool_allocated: DeviceSize,
	command_pool_reserved_size: DeviceSize,
	command_buffer_allocated: DeviceSize,
}

#[repr(C)]
pub struct PhysicalDeviceImagelessFramebufferFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	imageless_framebuffer: Bool32,
}

#[repr(C)]
pub struct DeviceFaultInfoEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	description: [u8; MAX_DESCRIPTION_SIZE],
	p_address_infos: *mut DeviceFaultAddressInfoEXT,
	p_vendor_infos: *mut DeviceFaultVendorInfoEXT,
	p_vendor_binary_data: *mut c_void,
}

#[repr(C)]
pub struct SurfaceCapabilitiesPresentBarrierNV {
	s_type: StructureType,
	p_next: *mut c_void,
	present_barrier_supported: Bool32,
}

#[repr(C)]
pub struct DebugUtilsObjectNameInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	object_type: ObjectType,
	object_handle: u64,
	p_object_name: *const u8,
}

#[repr(C)]
pub struct PhysicalDeviceLineRasterizationFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	rectangular_lines: Bool32,
	bresenham_lines: Bool32,
	smooth_lines: Bool32,
	stippled_rectangular_lines: Bool32,
	stippled_bresenham_lines: Bool32,
	stippled_smooth_lines: Bool32,
}

#[repr(C)]
pub struct ImageResolve2 {
	s_type: StructureType,
	p_next: *const c_void,
	src_subresource: ImageSubresourceLayers,
	src_offset: Offset3D,
	dst_subresource: ImageSubresourceLayers,
	dst_offset: Offset3D,
	extent: Extent3D,
}

#[repr(C)]
pub struct VideoSessionParametersCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: VideoSessionParametersCreateFlagsKHR,
	video_session_parameters_template: VideoSessionParametersKHR,
	video_session: VideoSessionKHR,
}

#[repr(C)]
pub struct SemaphoreSciSyncCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	semaphore_pool: SemaphoreSciSyncPoolNV,
	p_fence: *const NvSciSyncFence,
}

#[repr(C)]
pub struct MemoryType {
	property_flags: MemoryPropertyFlags,
	heap_index: u32,
}

#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupport {
	s_type: StructureType,
	p_next: *mut c_void,
	max_variable_descriptor_count: u32,
}

#[repr(C)]
pub struct PhysicalDevicePointClippingProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	point_clipping_behavior: PointClippingBehavior,
}

#[repr(C)]
pub struct SemaphoreWaitInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: SemaphoreWaitFlags,
	semaphore_count: u32,
	p_semaphores: *const Semaphore,
	p_values: *const u64,
}

#[repr(C)]
pub struct PipelineRasterizationStateStreamCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineRasterizationStateStreamCreateFlagsEXT,
	rasterization_stream: u32,
}

#[repr(C)]
pub struct PipelineExecutableStatisticKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	name: [u8; MAX_DESCRIPTION_SIZE],
	description: [u8; MAX_DESCRIPTION_SIZE],
	format: PipelineExecutableStatisticFormatKHR,
	value: PipelineExecutableStatisticValueKHR,
}

#[repr(C)]
pub struct DispatchIndirectCommand {
	x: u32,
	y: u32,
	z: u32,
}

#[repr(C)]
pub struct BufferCollectionPropertiesFUCHSIA {
	s_type: StructureType,
	p_next: *mut c_void,
	memory_type_bits: u32,
	buffer_count: u32,
	create_info_index: u32,
	sysmem_pixel_format: u64,
	format_features: FormatFeatureFlags,
	sysmem_color_space_index: SysmemColorSpaceFUCHSIA,
	sampler_ycbcr_conversion_components: ComponentMapping,
	suggested_ycbcr_model: SamplerYcbcrModelConversion,
	suggested_ycbcr_range: SamplerYcbcrRange,
	suggested_xchroma_offset: ChromaLocation,
	suggested_ychroma_offset: ChromaLocation,
}

#[repr(C)]
pub struct PipelineRobustnessCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	storage_buffers: PipelineRobustnessBufferBehaviorEXT,
	uniform_buffers: PipelineRobustnessBufferBehaviorEXT,
	vertex_inputs: PipelineRobustnessBufferBehaviorEXT,
	images: PipelineRobustnessImageBehaviorEXT,
}

#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_buffer_float32_atomics: Bool32,
	shader_buffer_float32_atomic_add: Bool32,
	shader_buffer_float64_atomics: Bool32,
	shader_buffer_float64_atomic_add: Bool32,
	shader_shared_float32_atomics: Bool32,
	shader_shared_float32_atomic_add: Bool32,
	shader_shared_float64_atomics: Bool32,
	shader_shared_float64_atomic_add: Bool32,
	shader_image_float32_atomics: Bool32,
	shader_image_float32_atomic_add: Bool32,
	sparse_image_float32_atomics: Bool32,
	sparse_image_float32_atomic_add: Bool32,
}

#[repr(C)]
pub struct AccelerationStructureMotionInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	max_instances: u32,
	flags: AccelerationStructureMotionInfoFlagsNV,
}

#[repr(C)]
pub struct RenderPassSubpassFeedbackCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	p_subpass_feedback: *mut RenderPassSubpassFeedbackInfoEXT,
}

#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	device_generated_commands: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_integer_functions2: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	representative_fragment_test: Bool32,
}

#[repr(C)]
pub struct PresentFrameTokenGGP {
	s_type: StructureType,
	p_next: *const c_void,
	frame_token: GgpFrameToken,
}

#[repr(C)]
pub struct PipelineViewportDepthClipControlCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	negative_one_to_one: Bool32,
}

#[repr(C)]
pub struct DrawIndirectCommand {
	vertex_count: u32,
	instance_count: u32,
	first_vertex: u32,
	first_instance: u32,
}

#[repr(C)]
pub struct GeometryNV {
	s_type: StructureType,
	p_next: *const c_void,
	geometry_type: GeometryTypeKHR,
	geometry: GeometryDataNV,
	flags: GeometryFlagsKHR,
}

#[repr(C)]
pub struct SamplerBorderColorComponentMappingCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	components: ComponentMapping,
	srgb: Bool32,
}

#[repr(C)]
pub struct PipelineRenderingCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	view_mask: u32,
	color_attachment_count: u32,
	p_color_attachment_formats: *const Format,
	depth_attachment_format: Format,
	stencil_attachment_format: Format,
}

#[repr(C)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	min_imported_host_pointer_alignment: DeviceSize,
}

#[repr(C)]
pub struct BindSparseInfo {
	s_type: StructureType,
	p_next: *const c_void,
	wait_semaphore_count: u32,
	p_wait_semaphores: *const Semaphore,
	buffer_bind_count: u32,
	p_buffer_binds: *const SparseBufferMemoryBindInfo,
	image_opaque_bind_count: u32,
	p_image_opaque_binds: *const SparseImageOpaqueMemoryBindInfo,
	image_bind_count: u32,
	p_image_binds: *const SparseImageMemoryBindInfo,
	signal_semaphore_count: u32,
	p_signal_semaphores: *const Semaphore,
}

#[repr(C)]
pub struct DisplaySurfaceCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: DisplaySurfaceCreateFlagsKHR,
	display_mode: DisplayModeKHR,
	plane_index: u32,
	plane_stack_index: u32,
	transform: SurfaceTransformFlagBitsKHR,
	global_alpha: f32,
	alpha_mode: DisplayPlaneAlphaFlagBitsKHR,
	image_extent: Extent2D,
}

#[repr(C)]
pub struct LatencySleepModeInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	low_latency_mode: Bool32,
	low_latency_boost: Bool32,
	minimum_interval_us: u32,
}

#[repr(C)]
pub struct ImageSwapchainCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	swapchain: SwapchainKHR,
}

#[repr(C)]
pub struct MemoryMapInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: MemoryMapFlags,
	memory: DeviceMemory,
	offset: DeviceSize,
	size: DeviceSize,
}

#[repr(C)]
pub struct MultiDrawIndexedInfoEXT {
	first_index: u32,
	index_count: u32,
	vertex_offset: i32,
}

#[repr(C)]
pub struct PhysicalDevicePipelineCreationCacheControlFeatures {
	s_type: StructureType,
	p_next: *mut c_void,
	pipeline_creation_cache_control: Bool32,
}

#[repr(C)]
pub struct ImageSubresourceLayers {
	aspect_mask: ImageAspectFlags,
	mip_level: u32,
	base_array_layer: u32,
	layer_count: u32,
}

#[repr(C)]
pub struct PhysicalDevice4444FormatsFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	format_a4_r4_g4_b4: Bool32,
	format_a4_b4_g4_r4: Bool32,
}

#[repr(C)]
pub struct BlitImageCubicWeightsInfoQCOM {
	s_type: StructureType,
	p_next: *const c_void,
	cubic_weights: CubicFilterWeightsQCOM,
}

#[repr(C)]
pub struct SemaphoreTypeCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	semaphore_type: SemaphoreType,
	initial_value: u64,
}

#[repr(C)]
pub struct HostImageLayoutTransitionInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	image: Image,
	old_layout: ImageLayout,
	new_layout: ImageLayout,
	subresource_range: ImageSubresourceRange,
}

#[repr(C)]
pub struct PhysicalDeviceShaderIntegerDotProductProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	integer_dot_product8_bit_unsigned_accelerated: Bool32,
	integer_dot_product8_bit_signed_accelerated: Bool32,
	integer_dot_product8_bit_mixed_signedness_accelerated: Bool32,
	integer_dot_product4x8_bit_packed_unsigned_accelerated: Bool32,
	integer_dot_product4x8_bit_packed_signed_accelerated: Bool32,
	integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Bool32,
	integer_dot_product16_bit_unsigned_accelerated: Bool32,
	integer_dot_product16_bit_signed_accelerated: Bool32,
	integer_dot_product16_bit_mixed_signedness_accelerated: Bool32,
	integer_dot_product32_bit_unsigned_accelerated: Bool32,
	integer_dot_product32_bit_signed_accelerated: Bool32,
	integer_dot_product32_bit_mixed_signedness_accelerated: Bool32,
	integer_dot_product64_bit_unsigned_accelerated: Bool32,
	integer_dot_product64_bit_signed_accelerated: Bool32,
	integer_dot_product64_bit_mixed_signedness_accelerated: Bool32,
	integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Bool32,
	integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Bool32,
	integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Bool32,
	integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Bool32,
	integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Bool32,
	integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: Bool32,
	integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Bool32,
	integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Bool32,
	integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Bool32,
	integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Bool32,
	integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Bool32,
	integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Bool32,
	integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Bool32,
	integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Bool32,
	integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Bool32,
}

#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureKHR {
	s_type: StructureType,
	p_next: *const c_void,
	acceleration_structure_count: u32,
	p_acceleration_structures: *const AccelerationStructureKHR,
}

#[repr(C)]
pub struct RayTracingPipelineCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineCreateFlags,
	stage_count: u32,
	p_stages: *const PipelineShaderStageCreateInfo,
	group_count: u32,
	p_groups: *const RayTracingShaderGroupCreateInfoNV,
	max_recursion_depth: u32,
	layout: PipelineLayout,
	base_pipeline_handle: Pipeline,
	base_pipeline_index: i32,
}

#[repr(C)]
pub struct PhysicalDeviceAmigoProfilingFeaturesSEC {
	s_type: StructureType,
	p_next: *mut c_void,
	amigo_profiling: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceIDProperties {
	s_type: StructureType,
	p_next: *mut c_void,
	device_uuid: [u8; UUID_SIZE],
	driver_uuid: [u8; UUID_SIZE],
	device_luid: [u8; LUID_SIZE],
	device_node_mask: u32,
	device_luidvalid: Bool32,
}

#[repr(C)]
pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	pageable_device_local_memory: Bool32,
}

#[repr(C)]
pub struct ImportMemoryWin32HandleInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	handle_type: ExternalMemoryHandleTypeFlagsNV,
	handle: HANDLE,
}

#[repr(C)]
pub struct RenderPassStripeInfoARM {
	s_type: StructureType,
	p_next: *const c_void,
	stripe_area: Rect2D,
}

#[repr(C)]
pub struct PhysicalDevicePushDescriptorPropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	max_push_descriptors: u32,
}

#[repr(C)]
pub struct MemoryBarrier2 {
	s_type: StructureType,
	p_next: *const c_void,
	src_stage_mask: PipelineStageFlags2,
	src_access_mask: AccessFlags2,
	dst_stage_mask: PipelineStageFlags2,
	dst_access_mask: AccessFlags2,
}

#[repr(C)]
pub struct ExternalMemoryImageCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	handle_types: ExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	image_view_type: ImageViewType,
}

#[repr(C)]
pub struct LatencyTimingsFrameReportNV {
	s_type: StructureType,
	p_next: *const c_void,
	present_id: u64,
	input_sample_time_us: u64,
	sim_start_time_us: u64,
	sim_end_time_us: u64,
	render_submit_start_time_us: u64,
	render_submit_end_time_us: u64,
	present_start_time_us: u64,
	present_end_time_us: u64,
	driver_start_time_us: u64,
	driver_end_time_us: u64,
	os_render_queue_start_time_us: u64,
	os_render_queue_end_time_us: u64,
	gpu_render_start_time_us: u64,
	gpu_render_end_time_us: u64,
}

#[repr(C)]
pub struct ImportFenceSciSyncInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	fence: Fence,
	handle_type: ExternalFenceHandleTypeFlagBits,
	handle: *mut c_void,
}

#[repr(C)]
pub struct SubpassDependency2 {
	s_type: StructureType,
	p_next: *const c_void,
	src_subpass: u32,
	dst_subpass: u32,
	src_stage_mask: PipelineStageFlags,
	dst_stage_mask: PipelineStageFlags,
	src_access_mask: AccessFlags,
	dst_access_mask: AccessFlags,
	dependency_flags: DependencyFlags,
	view_offset: i32,
}

#[repr(C)]
pub struct PhysicalDeviceExternalFenceInfo {
	s_type: StructureType,
	p_next: *const c_void,
	handle_type: ExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct ImageMemoryBarrier2 {
	s_type: StructureType,
	p_next: *const c_void,
	src_stage_mask: PipelineStageFlags2,
	src_access_mask: AccessFlags2,
	dst_stage_mask: PipelineStageFlags2,
	dst_access_mask: AccessFlags2,
	old_layout: ImageLayout,
	new_layout: ImageLayout,
	src_queue_family_index: u32,
	dst_queue_family_index: u32,
	image: Image,
	subresource_range: ImageSubresourceRange,
}

#[repr(C)]
pub struct PresentInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	wait_semaphore_count: u32,
	p_wait_semaphores: *const Semaphore,
	swapchain_count: u32,
	p_swapchains: *const SwapchainKHR,
	p_image_indices: *const u32,
	p_results: *mut Result,
}

#[repr(C)]
pub struct ExportMetalBufferInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	memory: DeviceMemory,
	mtl_buffer: MTLBuffer_id,
}

#[repr(C)]
pub struct PhysicalDeviceSchedulingControlsPropertiesARM {
	s_type: StructureType,
	p_next: *mut c_void,
	scheduling_controls_flags: PhysicalDeviceSchedulingControlsFlagsARM,
}

#[repr(C)]
pub struct D3D12FenceSubmitInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	wait_semaphore_values_count: u32,
	p_wait_semaphore_values: *const u64,
	signal_semaphore_values_count: u32,
	p_signal_semaphore_values: *const u64,
}

#[repr(C)]
pub struct CopyDescriptorSet {
	s_type: StructureType,
	p_next: *const c_void,
	src_set: DescriptorSet,
	src_binding: u32,
	src_array_element: u32,
	dst_set: DescriptorSet,
	dst_binding: u32,
	dst_array_element: u32,
	descriptor_count: u32,
}

#[repr(C)]
pub struct PhysicalDeviceExternalSciSync2FeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	sci_sync_fence: Bool32,
	sci_sync_semaphore2: Bool32,
	sci_sync_import: Bool32,
	sci_sync_export: Bool32,
}

#[repr(C)]
pub struct PipelineTessellationStateCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineTessellationStateCreateFlags,
	patch_control_points: u32,
}

#[repr(C)]
pub struct SparseImageMemoryRequirements {
	format_properties: SparseImageFormatProperties,
	image_mip_tail_first_lod: u32,
	image_mip_tail_size: DeviceSize,
	image_mip_tail_offset: DeviceSize,
	image_mip_tail_stride: DeviceSize,
}

#[repr(C)]
pub struct OutOfBandQueueTypeInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	queue_type: OutOfBandQueueTypeNV,
}

#[repr(C)]
pub struct SubresourceHostMemcpySizeEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	size: DeviceSize,
}

#[repr(C)]
pub struct QueueFamilyProperties {
	queue_flags: QueueFlags,
	queue_count: u32,
	timestamp_valid_bits: u32,
	min_image_transfer_granularity: Extent3D,
}

#[repr(C)]
pub struct DescriptorSetAllocateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	descriptor_pool: DescriptorPool,
	descriptor_set_count: u32,
	p_set_layouts: *const DescriptorSetLayout,
}

#[repr(C)]
pub struct PhysicalDeviceFeatures2 {
	s_type: StructureType,
	p_next: *mut c_void,
	features: PhysicalDeviceFeatures,
}

#[repr(C)]
pub struct PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	global_priority_query: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	corner_sampled_image: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceAccelerationStructurePropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	max_geometry_count: u64,
	max_instance_count: u64,
	max_primitive_count: u64,
	max_per_stage_descriptor_acceleration_structures: u32,
	max_per_stage_descriptor_update_after_bind_acceleration_structures: u32,
	max_descriptor_set_acceleration_structures: u32,
	max_descriptor_set_update_after_bind_acceleration_structures: u32,
	min_acceleration_structure_scratch_offset_alignment: u32,
}

#[repr(C)]
pub struct GeneratedCommandsInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	pipeline_bind_point: PipelineBindPoint,
	pipeline: Pipeline,
	indirect_commands_layout: IndirectCommandsLayoutNV,
	stream_count: u32,
	p_streams: *const IndirectCommandsStreamNV,
	sequences_count: u32,
	preprocess_buffer: Buffer,
	preprocess_offset: DeviceSize,
	preprocess_size: DeviceSize,
	sequences_count_buffer: Buffer,
	sequences_count_offset: DeviceSize,
	sequences_index_buffer: Buffer,
	sequences_index_offset: DeviceSize,
}

#[repr(C)]
pub struct BindVideoSessionMemoryInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	memory_bind_index: u32,
	memory: DeviceMemory,
	memory_offset: DeviceSize,
	memory_size: DeviceSize,
}

#[repr(C)]
pub struct StreamDescriptorSurfaceCreateInfoGGP {
	s_type: StructureType,
	p_next: *const c_void,
	flags: StreamDescriptorSurfaceCreateFlagsGGP,
	stream_descriptor: GgpStreamDescriptor,
}

#[repr(C)]
pub struct PhysicalDeviceRobustness2FeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	robust_buffer_access2: Bool32,
	robust_image_access2: Bool32,
	null_descriptor: Bool32,
}

#[repr(C)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	flags: PipelineRasterizationDepthClipStateCreateFlagsEXT,
	depth_clip_enable: Bool32,
}

#[repr(C)]
pub struct VideoEncodeH264SessionCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	use_max_level_idc: Bool32,
	max_level_idc: StdVideoH264LevelIdc,
}

#[repr(C)]
pub struct VideoEncodeH264RateControlLayerInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	use_min_qp: Bool32,
	min_qp: VideoEncodeH264QpEXT,
	use_max_qp: Bool32,
	max_qp: VideoEncodeH264QpEXT,
	use_max_frame_size: Bool32,
	max_frame_size: VideoEncodeH264FrameSizeEXT,
}

#[repr(C)]
pub struct PhysicalDeviceFeatures {
	robust_buffer_access: Bool32,
	full_draw_index_uint32: Bool32,
	image_cube_array: Bool32,
	independent_blend: Bool32,
	geometry_shader: Bool32,
	tessellation_shader: Bool32,
	sample_rate_shading: Bool32,
	dual_src_blend: Bool32,
	logic_op: Bool32,
	multi_draw_indirect: Bool32,
	draw_indirect_first_instance: Bool32,
	depth_clamp: Bool32,
	depth_bias_clamp: Bool32,
	fill_mode_non_solid: Bool32,
	depth_bounds: Bool32,
	wide_lines: Bool32,
	large_points: Bool32,
	alpha_to_one: Bool32,
	multi_viewport: Bool32,
	sampler_anisotropy: Bool32,
	texture_compression_etc2: Bool32,
	texture_compression_astc_ldr: Bool32,
	texture_compression_bc: Bool32,
	occlusion_query_precise: Bool32,
	pipeline_statistics_query: Bool32,
	vertex_pipeline_stores_and_atomics: Bool32,
	fragment_stores_and_atomics: Bool32,
	shader_tessellation_and_geometry_point_size: Bool32,
	shader_image_gather_extended: Bool32,
	shader_storage_image_extended_formats: Bool32,
	shader_storage_image_multisample: Bool32,
	shader_storage_image_read_without_format: Bool32,
	shader_storage_image_write_without_format: Bool32,
	shader_uniform_buffer_array_dynamic_indexing: Bool32,
	shader_sampled_image_array_dynamic_indexing: Bool32,
	shader_storage_buffer_array_dynamic_indexing: Bool32,
	shader_storage_image_array_dynamic_indexing: Bool32,
	shader_clip_distance: Bool32,
	shader_cull_distance: Bool32,
	shader_float64: Bool32,
	shader_int64: Bool32,
	shader_int16: Bool32,
	shader_resource_residency: Bool32,
	shader_resource_min_lod: Bool32,
	sparse_binding: Bool32,
	sparse_residency_buffer: Bool32,
	sparse_residency_image2_d: Bool32,
	sparse_residency_image3_d: Bool32,
	sparse_residency2_samples: Bool32,
	sparse_residency4_samples: Bool32,
	sparse_residency8_samples: Bool32,
	sparse_residency16_samples: Bool32,
	sparse_residency_aliased: Bool32,
	variable_multisample_rate: Bool32,
	inherited_queries: Bool32,
}

#[repr(C)]
pub struct ResolveImageInfo2 {
	s_type: StructureType,
	p_next: *const c_void,
	src_image: Image,
	src_image_layout: ImageLayout,
	dst_image: Image,
	dst_image_layout: ImageLayout,
	region_count: u32,
	p_regions: *const ImageResolve2,
}

#[repr(C)]
pub struct SparseImageFormatProperties2 {
	s_type: StructureType,
	p_next: *mut c_void,
	properties: SparseImageFormatProperties,
}

#[repr(C)]
pub struct CopyMicromapToMemoryInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	src: MicromapEXT,
	dst: DeviceOrHostAddressKHR,
	mode: CopyMicromapModeEXT,
}

#[repr(C)]
pub struct ImageConstraintsInfoFUCHSIA {
	s_type: StructureType,
	p_next: *const c_void,
	format_constraints_count: u32,
	p_format_constraints: *const ImageFormatConstraintsInfoFUCHSIA,
	buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
	flags: ImageConstraintsInfoFlagsFUCHSIA,
}

#[repr(C)]
pub struct SubpassResolvePerformanceQueryEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	optimal: Bool32,
}

#[repr(C)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	shading_rate_image_enable: Bool32,
	viewport_count: u32,
	p_shading_rate_palettes: *const ShadingRatePaletteNV,
}

#[repr(C)]
pub struct PhysicalDeviceMemoryDecompressionPropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	decompression_methods: MemoryDecompressionMethodFlagsNV,
	max_decompression_indirect_count: u64,
}

#[repr(C)]
pub struct MemoryBarrier {
	s_type: StructureType,
	p_next: *const c_void,
	src_access_mask: AccessFlags,
	dst_access_mask: AccessFlags,
}

#[repr(C)]
pub struct ImageMemoryBarrier {
	s_type: StructureType,
	p_next: *const c_void,
	src_access_mask: AccessFlags,
	dst_access_mask: AccessFlags,
	old_layout: ImageLayout,
	new_layout: ImageLayout,
	src_queue_family_index: u32,
	dst_queue_family_index: u32,
	image: Image,
	subresource_range: ImageSubresourceRange,
}

#[repr(C)]
pub struct SwapchainPresentBarrierCreateInfoNV {
	s_type: StructureType,
	p_next: *mut c_void,
	present_barrier_enable: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceLineRasterizationPropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	line_sub_pixel_precision_bits: u32,
}

#[repr(C)]
pub struct PhysicalDeviceRobustness2PropertiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	robust_storage_buffer_access_size_alignment: DeviceSize,
	robust_uniform_buffer_access_size_alignment: DeviceSize,
}

#[repr(C)]
pub struct DeviceObjectReservationCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	pipeline_cache_create_info_count: u32,
	p_pipeline_cache_create_infos: *const PipelineCacheCreateInfo,
	pipeline_pool_size_count: u32,
	p_pipeline_pool_sizes: *const PipelinePoolSize,
	semaphore_request_count: u32,
	command_buffer_request_count: u32,
	fence_request_count: u32,
	device_memory_request_count: u32,
	buffer_request_count: u32,
	image_request_count: u32,
	event_request_count: u32,
	query_pool_request_count: u32,
	buffer_view_request_count: u32,
	image_view_request_count: u32,
	layered_image_view_request_count: u32,
	pipeline_cache_request_count: u32,
	pipeline_layout_request_count: u32,
	render_pass_request_count: u32,
	graphics_pipeline_request_count: u32,
	compute_pipeline_request_count: u32,
	descriptor_set_layout_request_count: u32,
	sampler_request_count: u32,
	descriptor_pool_request_count: u32,
	descriptor_set_request_count: u32,
	framebuffer_request_count: u32,
	command_pool_request_count: u32,
	sampler_ycbcr_conversion_request_count: u32,
	surface_request_count: u32,
	swapchain_request_count: u32,
	display_mode_request_count: u32,
	subpass_description_request_count: u32,
	attachment_description_request_count: u32,
	descriptor_set_layout_binding_request_count: u32,
	descriptor_set_layout_binding_limit: u32,
	max_image_view_mip_levels: u32,
	max_image_view_array_layers: u32,
	max_layered_image_view_mip_levels: u32,
	max_occlusion_queries_per_pool: u32,
	max_pipeline_statistics_queries_per_pool: u32,
	max_timestamp_queries_per_pool: u32,
	max_immutable_samplers_per_descriptor_set_layout: u32,
}

#[repr(C)]
pub struct PipelineShaderStageNodeCreateInfoAMDX {
	s_type: StructureType,
	p_next: *const c_void,
	p_name: *const u8,
	index: u32,
}

#[repr(C)]
pub struct ApplicationParametersEXT {
	s_type: StructureType,
	p_next: *const c_void,
	vendor_id: u32,
	device_id: u32,
	key: u32,
	value: u64,
}

#[repr(C)]
pub struct PhysicalDeviceExtendedSparseAddressSpacePropertiesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	extended_sparse_address_space_size: DeviceSize,
	extended_sparse_image_usage_flags: ImageUsageFlags,
	extended_sparse_buffer_usage_flags: BufferUsageFlags,
}

#[repr(C)]
pub struct PhysicalDeviceExternalSciSyncFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	sci_sync_fence: Bool32,
	sci_sync_semaphore: Bool32,
	sci_sync_import: Bool32,
	sci_sync_export: Bool32,
}

#[repr(C)]
pub struct AccelerationStructureBuildRangeInfoKHR {
	primitive_count: u32,
	primitive_offset: u32,
	first_vertex: u32,
	transform_offset: u32,
}

#[repr(C)]
pub struct ImportMemoryZirconHandleInfoFUCHSIA {
	s_type: StructureType,
	p_next: *const c_void,
	handle_type: ExternalMemoryHandleTypeFlagBits,
	handle: zx_handle_t,
}

#[repr(C)]
pub struct PhysicalDeviceVulkanSC10Properties {
	s_type: StructureType,
	p_next: *mut c_void,
	device_no_dynamic_host_allocations: Bool32,
	device_destroy_frees_memory: Bool32,
	command_pool_multiple_command_buffers_recording: Bool32,
	command_pool_reset_command_buffer: Bool32,
	command_buffer_simultaneous_use: Bool32,
	secondary_command_buffer_null_or_imageless_framebuffer: Bool32,
	recycle_descriptor_set_memory: Bool32,
	recycle_pipeline_memory: Bool32,
	max_render_pass_subpasses: u32,
	max_render_pass_dependencies: u32,
	max_subpass_input_attachments: u32,
	max_subpass_preserve_attachments: u32,
	max_framebuffer_attachments: u32,
	max_descriptor_set_layout_bindings: u32,
	max_query_fault_count: u32,
	max_callback_fault_count: u32,
	max_command_pool_command_buffers: u32,
	max_command_buffer_size: DeviceSize,
}

#[repr(C)]
pub struct CudaLaunchInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	function: CudaFunctionNV,
	grid_dim_x: u32,
	grid_dim_y: u32,
	grid_dim_z: u32,
	block_dim_x: u32,
	block_dim_y: u32,
	block_dim_z: u32,
	shared_mem_bytes: u32,
	param_count: usize,
	p_params: *const *const c_void,
	extra_count: usize,
	p_extras: *const *const c_void,
}

#[repr(C)]
pub struct PhysicalDeviceImageProcessing2FeaturesQCOM {
	s_type: StructureType,
	p_next: *mut c_void,
	texture_block_match2: Bool32,
}

#[repr(C)]
pub struct WaylandSurfaceCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: WaylandSurfaceCreateFlagsKHR,
	display: *mut wl_display,
	surface: *mut wl_surface,
}

#[repr(C)]
pub struct VideoEncodeH264PictureInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	nalu_slice_entry_count: u32,
	p_nalu_slice_entries: *const VideoEncodeH264NaluSliceInfoEXT,
	p_std_picture_info: *const StdVideoEncodeH264PictureInfo,
	generate_prefix_nalu: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	extended_dynamic_state: Bool32,
}

#[repr(C)]
pub struct QueryPoolPerformanceCreateInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	queue_family_index: u32,
	counter_index_count: u32,
	p_counter_indices: *const u32,
}

#[repr(C)]
pub struct PipelineColorBlendAttachmentState {
	blend_enable: Bool32,
	src_color_blend_factor: BlendFactor,
	dst_color_blend_factor: BlendFactor,
	color_blend_op: BlendOp,
	src_alpha_blend_factor: BlendFactor,
	dst_alpha_blend_factor: BlendFactor,
	alpha_blend_op: BlendOp,
	color_write_mask: ColorComponentFlags,
}

#[repr(C)]
pub struct DedicatedAllocationMemoryAllocateInfoNV {
	s_type: StructureType,
	p_next: *const c_void,
	image: Image,
	buffer: Buffer,
}

#[repr(C)]
pub struct SamplerReductionModeCreateInfo {
	s_type: StructureType,
	p_next: *const c_void,
	reduction_mode: SamplerReductionMode,
}

#[repr(C)]
pub struct VideoDecodeInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	flags: VideoDecodeFlagsKHR,
	src_buffer: Buffer,
	src_buffer_offset: DeviceSize,
	src_buffer_range: DeviceSize,
	dst_picture_resource: VideoPictureResourceInfoKHR,
	p_setup_reference_slot: *const VideoReferenceSlotInfoKHR,
	reference_slot_count: u32,
	p_reference_slots: *const VideoReferenceSlotInfoKHR,
}

#[repr(C)]
pub struct VideoEncodeH265CapabilitiesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	flags: VideoEncodeH265CapabilityFlagsEXT,
	max_level_idc: StdVideoH265LevelIdc,
	max_slice_segment_count: u32,
	max_tiles: Extent2D,
	ctb_sizes: VideoEncodeH265CtbSizeFlagsEXT,
	transform_block_sizes: VideoEncodeH265TransformBlockSizeFlagsEXT,
	max_ppicture_l0_reference_count: u32,
	max_bpicture_l0_reference_count: u32,
	max_l1_reference_count: u32,
	max_sub_layer_count: u32,
	expect_dyadic_temporal_sub_layer_pattern: Bool32,
	min_qp: i32,
	max_qp: i32,
	prefers_gop_remaining_frames: Bool32,
	requires_gop_remaining_frames: Bool32,
	std_syntax_flags: VideoEncodeH265StdFlagsEXT,
}

#[repr(C)]
pub struct ReleaseSwapchainImagesInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	swapchain: SwapchainKHR,
	image_index_count: u32,
	p_image_indices: *const u32,
}

#[repr(C)]
pub struct VideoEncodeSessionParametersGetInfoKHR {
	s_type: StructureType,
	p_next: *const c_void,
	video_session_parameters: VideoSessionParametersKHR,
}

#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	device_generated_compute: Bool32,
	device_generated_compute_pipelines: Bool32,
	device_generated_compute_capture_replay: Bool32,
}

#[repr(C)]
pub struct DeviceQueueShaderCoreControlCreateInfoARM {
	s_type: StructureType,
	p_next: *mut c_void,
	shader_core_count: u32,
}

#[repr(C)]
pub struct MemorySciBufPropertiesNV {
	s_type: StructureType,
	p_next: *const c_void,
	memory_type_bits: u32,
}

#[repr(C)]
pub struct RefreshCycleDurationGOOGLE {
	refresh_duration: u64,
}

#[repr(C)]
pub struct ImageViewSlicedCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	slice_offset: u32,
	slice_count: u32,
}

#[repr(C)]
pub struct VideoEncodeH265QpEXT {
	qp_i: i32,
	qp_p: i32,
	qp_b: i32,
}

#[repr(C)]
pub struct SubresourceLayout2KHR {
	s_type: StructureType,
	p_next: *mut c_void,
	subresource_layout: SubresourceLayout,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
	s_type: StructureType,
	p_next: *mut c_void,
	fragment_shading_rate_enums: Bool32,
	supersample_fragment_shading_rates: Bool32,
	no_invocation_fragment_shading_rates: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceAddressBindingReportFeaturesEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	report_address_binding: Bool32,
}

#[repr(C)]
pub struct VideoEncodeH264SessionParametersFeedbackInfoEXT {
	s_type: StructureType,
	p_next: *mut c_void,
	has_std_spsoverrides: Bool32,
	has_std_ppsoverrides: Bool32,
}

#[repr(C)]
pub struct ClearRect {
	rect: Rect2D,
	base_array_layer: u32,
	layer_count: u32,
}

#[repr(C)]
pub struct PhysicalDeviceMaintenance5PropertiesKHR {
	s_type: StructureType,
	p_next: *mut c_void,
	early_fragment_multisample_coverage_after_sample_counting: Bool32,
	early_fragment_sample_mask_test_before_sample_counting: Bool32,
	depth_stencil_swizzle_one_support: Bool32,
	polygon_mode_point_size: Bool32,
	non_strict_single_pixel_wide_lines_use_parallelogram: Bool32,
	non_strict_wide_lines_use_parallelogram: Bool32,
}

#[repr(C)]
pub struct PipelineSampleLocationsStateCreateInfoEXT {
	s_type: StructureType,
	p_next: *const c_void,
	sample_locations_enable: Bool32,
	sample_locations_info: SampleLocationsInfoEXT,
}

