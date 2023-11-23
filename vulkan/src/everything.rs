#[repr(C)]
pub struct VkBaseOutStructure {
s_type: VkStructureType,
p_next: VkBaseOutStructure,
}

#[repr(C)]
pub struct VkBaseInStructure {
s_type: VkStructureType,
p_next: VkBaseInStructure,
}

#[repr(C)]
pub struct VkOffset2D {
x: i32,
y: i32,
}

#[repr(C)]
pub struct VkOffset3D {
x: i32,
y: i32,
z: i32,
}

#[repr(C)]
pub struct VkExtent2D {
width: u32,
height: u32,
}

#[repr(C)]
pub struct VkExtent3D {
width: u32,
height: u32,
depth: u32,
}

#[repr(C)]
pub struct VkViewport {
x: f32,
y: f32,
width: f32,
height: f32,
min_depth: f32,
max_depth: f32,
}

#[repr(C)]
pub struct VkRect2D {
offset: VkOffset2D,
extent: VkExtent2D,
}

#[repr(C)]
pub struct VkClearRect {
rect: VkRect2D,
base_array_layer: u32,
layer_count: u32,
}

#[repr(C)]
pub struct VkComponentMapping {
r: VkComponentSwizzle,
g: VkComponentSwizzle,
b: VkComponentSwizzle,
a: VkComponentSwizzle,
}

#[repr(C)]
pub struct VkPhysicalDeviceProperties {
api_version: u32,
driver_version: u32,
vendor_id: u32,
device_id: u32,
device_type: VkPhysicalDeviceType,
device_name: u8,
pipeline_cache_uuid: u8,
limits: VkPhysicalDeviceLimits,
sparse_properties: VkPhysicalDeviceSparseProperties,
}

#[repr(C)]
pub struct VkExtensionProperties {
extension_name: u8,
spec_version: u32,
}

#[repr(C)]
pub struct VkLayerProperties {
layer_name: u8,
spec_version: u32,
implementation_version: u32,
description: u8,
}

#[repr(C)]
pub struct VkApplicationInfo {
s_type: VkStructureType,
p_next: c_void,
p_application_name: u8,
application_version: u32,
p_engine_name: u8,
engine_version: u32,
api_version: u32,
}

#[repr(C)]
pub struct VkAllocationCallbacks {
p_user_data: c_void,
pfn_allocation: PFN_vkAllocationFunction,
pfn_reallocation: PFN_vkReallocationFunction,
pfn_free: PFN_vkFreeFunction,
pfn_internal_allocation: PFN_vkInternalAllocationNotification,
pfn_internal_free: PFN_vkInternalFreeNotification,
}

#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkDeviceQueueCreateFlags,
queue_family_index: u32,
queue_count: u32,
p_queue_priorities: f32,
}

#[repr(C)]
pub struct VkDeviceCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkDeviceCreateFlags,
queue_create_info_count: u32,
p_queue_create_infos: VkDeviceQueueCreateInfo,
#[deprecated(note = "Ignored")]
enabled_layer_count: u32,
#[deprecated(note = "Ignored")]
pp_enabled_layer_names: u8,
enabled_extension_count: u32,
pp_enabled_extension_names: u8,
p_enabled_features: VkPhysicalDeviceFeatures,
}

#[repr(C)]
pub struct VkInstanceCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkInstanceCreateFlags,
p_application_info: VkApplicationInfo,
enabled_layer_count: u32,
pp_enabled_layer_names: u8,
enabled_extension_count: u32,
pp_enabled_extension_names: u8,
}

#[repr(C)]
pub struct VkQueueFamilyProperties {
queue_flags: VkQueueFlags,
queue_count: u32,
timestamp_valid_bits: u32,
min_image_transfer_granularity: VkExtent3D,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
memory_type_count: u32,
memory_types: VkMemoryType,
memory_heap_count: u32,
memory_heaps: VkMemoryHeap,
}

#[repr(C)]
pub struct VkMemoryAllocateInfo {
s_type: VkStructureType,
p_next: c_void,
allocation_size: VkDeviceSize,
memory_type_index: u32,
}

#[repr(C)]
pub struct VkMemoryRequirements {
size: VkDeviceSize,
alignment: VkDeviceSize,
memory_type_bits: u32,
}

#[repr(C)]
pub struct VkSparseImageFormatProperties {
aspect_mask: VkImageAspectFlags,
image_granularity: VkExtent3D,
flags: VkSparseImageFormatFlags,
}

#[repr(C)]
pub struct VkSparseImageMemoryRequirements {
format_properties: VkSparseImageFormatProperties,
image_mip_tail_first_lod: u32,
image_mip_tail_size: VkDeviceSize,
image_mip_tail_offset: VkDeviceSize,
image_mip_tail_stride: VkDeviceSize,
}

#[repr(C)]
pub struct VkMemoryType {
property_flags: VkMemoryPropertyFlags,
heap_index: u32,
}

#[repr(C)]
pub struct VkMemoryHeap {
size: VkDeviceSize,
flags: VkMemoryHeapFlags,
}

#[repr(C)]
pub struct VkMappedMemoryRange {
s_type: VkStructureType,
p_next: c_void,
memory: VkDeviceMemory,
offset: VkDeviceSize,
size: VkDeviceSize,
}

#[repr(C)]
pub struct VkFormatProperties {
linear_tiling_features: VkFormatFeatureFlags,
optimal_tiling_features: VkFormatFeatureFlags,
buffer_features: VkFormatFeatureFlags,
}

#[repr(C)]
pub struct VkImageFormatProperties {
max_extent: VkExtent3D,
max_mip_levels: u32,
max_array_layers: u32,
sample_counts: VkSampleCountFlags,
max_resource_size: VkDeviceSize,
}

#[repr(C)]
pub struct VkDescriptorBufferInfo {
buffer: VkBuffer,
offset: VkDeviceSize,
range: VkDeviceSize,
}

#[repr(C)]
pub struct VkDescriptorImageInfo {
sampler: VkSampler,
image_view: VkImageView,
image_layout: VkImageLayout,
}

#[repr(C)]
pub struct VkWriteDescriptorSet {
s_type: VkStructureType,
p_next: c_void,
dst_set: VkDescriptorSet,
dst_binding: u32,
dst_array_element: u32,
descriptor_count: u32,
descriptor_type: VkDescriptorType,
p_image_info: VkDescriptorImageInfo,
p_buffer_info: VkDescriptorBufferInfo,
p_texel_buffer_view: VkBufferView,
}

#[repr(C)]
pub struct VkCopyDescriptorSet {
s_type: VkStructureType,
p_next: c_void,
src_set: VkDescriptorSet,
src_binding: u32,
src_array_element: u32,
dst_set: VkDescriptorSet,
dst_binding: u32,
dst_array_element: u32,
descriptor_count: u32,
}

#[repr(C)]
pub struct VkBufferUsageFlags2CreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
usage: VkBufferUsageFlags2KHR,
}

#[repr(C)]
pub struct VkBufferCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkBufferCreateFlags,
size: VkDeviceSize,
usage: VkBufferUsageFlags,
sharing_mode: VkSharingMode,
queue_family_index_count: u32,
p_queue_family_indices: u32,
}

#[repr(C)]
pub struct VkBufferViewCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkBufferViewCreateFlags,
buffer: VkBuffer,
format: VkFormat,
offset: VkDeviceSize,
range: VkDeviceSize,
}

#[repr(C)]
pub struct VkImageSubresource {
aspect_mask: VkImageAspectFlags,
mip_level: u32,
array_layer: u32,
}

#[repr(C)]
pub struct VkImageSubresourceLayers {
aspect_mask: VkImageAspectFlags,
mip_level: u32,
base_array_layer: u32,
layer_count: u32,
}

#[repr(C)]
pub struct VkImageSubresourceRange {
aspect_mask: VkImageAspectFlags,
base_mip_level: u32,
level_count: u32,
base_array_layer: u32,
layer_count: u32,
}

#[repr(C)]
pub struct VkMemoryBarrier {
s_type: VkStructureType,
p_next: c_void,
src_access_mask: VkAccessFlags,
dst_access_mask: VkAccessFlags,
}

#[repr(C)]
pub struct VkBufferMemoryBarrier {
s_type: VkStructureType,
p_next: c_void,
src_access_mask: VkAccessFlags,
dst_access_mask: VkAccessFlags,
src_queue_family_index: u32,
dst_queue_family_index: u32,
buffer: VkBuffer,
offset: VkDeviceSize,
size: VkDeviceSize,
}

#[repr(C)]
pub struct VkImageMemoryBarrier {
s_type: VkStructureType,
p_next: c_void,
src_access_mask: VkAccessFlags,
dst_access_mask: VkAccessFlags,
old_layout: VkImageLayout,
new_layout: VkImageLayout,
src_queue_family_index: u32,
dst_queue_family_index: u32,
image: VkImage,
subresource_range: VkImageSubresourceRange,
}

#[repr(C)]
pub struct VkImageCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkImageCreateFlags,
image_type: VkImageType,
format: VkFormat,
extent: VkExtent3D,
mip_levels: u32,
array_layers: u32,
samples: VkSampleCountFlagBits,
tiling: VkImageTiling,
usage: VkImageUsageFlags,
sharing_mode: VkSharingMode,
queue_family_index_count: u32,
p_queue_family_indices: u32,
initial_layout: VkImageLayout,
}

#[repr(C)]
pub struct VkSubresourceLayout {
offset: VkDeviceSize,
size: VkDeviceSize,
row_pitch: VkDeviceSize,
array_pitch: VkDeviceSize,
depth_pitch: VkDeviceSize,
}

#[repr(C)]
pub struct VkImageViewCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkImageViewCreateFlags,
image: VkImage,
view_type: VkImageViewType,
format: VkFormat,
components: VkComponentMapping,
subresource_range: VkImageSubresourceRange,
}

#[repr(C)]
pub struct VkBufferCopy {
src_offset: VkDeviceSize,
dst_offset: VkDeviceSize,
size: VkDeviceSize,
}

#[repr(C)]
pub struct VkSparseMemoryBind {
resource_offset: VkDeviceSize,
size: VkDeviceSize,
memory: VkDeviceMemory,
memory_offset: VkDeviceSize,
flags: VkSparseMemoryBindFlags,
}

#[repr(C)]
pub struct VkSparseImageMemoryBind {
subresource: VkImageSubresource,
offset: VkOffset3D,
extent: VkExtent3D,
memory: VkDeviceMemory,
memory_offset: VkDeviceSize,
flags: VkSparseMemoryBindFlags,
}

#[repr(C)]
pub struct VkSparseBufferMemoryBindInfo {
buffer: VkBuffer,
bind_count: u32,
p_binds: VkSparseMemoryBind,
}

#[repr(C)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
image: VkImage,
bind_count: u32,
p_binds: VkSparseMemoryBind,
}

#[repr(C)]
pub struct VkSparseImageMemoryBindInfo {
image: VkImage,
bind_count: u32,
p_binds: VkSparseImageMemoryBind,
}

#[repr(C)]
pub struct VkBindSparseInfo {
s_type: VkStructureType,
p_next: c_void,
wait_semaphore_count: u32,
p_wait_semaphores: VkSemaphore,
buffer_bind_count: u32,
p_buffer_binds: VkSparseBufferMemoryBindInfo,
image_opaque_bind_count: u32,
p_image_opaque_binds: VkSparseImageOpaqueMemoryBindInfo,
image_bind_count: u32,
p_image_binds: VkSparseImageMemoryBindInfo,
signal_semaphore_count: u32,
p_signal_semaphores: VkSemaphore,
}

#[repr(C)]
pub struct VkImageCopy {
src_subresource: VkImageSubresourceLayers,
src_offset: VkOffset3D,
dst_subresource: VkImageSubresourceLayers,
dst_offset: VkOffset3D,
extent: VkExtent3D,
}

#[repr(C)]
pub struct VkImageBlit {
src_subresource: VkImageSubresourceLayers,
src_offsets: VkOffset3D,
dst_subresource: VkImageSubresourceLayers,
dst_offsets: VkOffset3D,
}

#[repr(C)]
pub struct VkBufferImageCopy {
buffer_offset: VkDeviceSize,
buffer_row_length: u32,
buffer_image_height: u32,
image_subresource: VkImageSubresourceLayers,
image_offset: VkOffset3D,
image_extent: VkExtent3D,
}

#[repr(C)]
pub struct VkCopyMemoryIndirectCommandNV {
src_address: VkDeviceAddress,
dst_address: VkDeviceAddress,
size: VkDeviceSize,
}

#[repr(C)]
pub struct VkCopyMemoryToImageIndirectCommandNV {
src_address: VkDeviceAddress,
buffer_row_length: u32,
buffer_image_height: u32,
image_subresource: VkImageSubresourceLayers,
image_offset: VkOffset3D,
image_extent: VkExtent3D,
}

#[repr(C)]
pub struct VkImageResolve {
src_subresource: VkImageSubresourceLayers,
src_offset: VkOffset3D,
dst_subresource: VkImageSubresourceLayers,
dst_offset: VkOffset3D,
extent: VkExtent3D,
}

#[repr(C)]
pub struct VkShaderModuleCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkShaderModuleCreateFlags,
code_size: usize,
p_code: u32,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutBinding {
binding: u32,
descriptor_type: VkDescriptorType,
descriptor_count: u32,
stage_flags: VkShaderStageFlags,
p_immutable_samplers: VkSampler,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkDescriptorSetLayoutCreateFlags,
binding_count: u32,
p_bindings: VkDescriptorSetLayoutBinding,
}

#[repr(C)]
pub struct VkDescriptorPoolSize {
r#type: VkDescriptorType,
descriptor_count: u32,
}

#[repr(C)]
pub struct VkDescriptorPoolCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkDescriptorPoolCreateFlags,
max_sets: u32,
pool_size_count: u32,
p_pool_sizes: VkDescriptorPoolSize,
}

#[repr(C)]
pub struct VkDescriptorSetAllocateInfo {
s_type: VkStructureType,
p_next: c_void,
descriptor_pool: VkDescriptorPool,
descriptor_set_count: u32,
p_set_layouts: VkDescriptorSetLayout,
}

#[repr(C)]
pub struct VkSpecializationMapEntry {
ant_id: u32,
offset: u32,
size: usize,
}

#[repr(C)]
pub struct VkSpecializationInfo {
map_entry_count: u32,
p_map_entries: VkSpecializationMapEntry,
data_size: usize,
p_data: c_void,
}

#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineShaderStageCreateFlags,
stage: VkShaderStageFlagBits,
module: VkShaderModule,
#[cfg(vulkan)]
p_name: u8,
#[cfg(vulkansc)]
p_name: u8,
p_specialization_info: VkSpecializationInfo,
}

#[repr(C)]
pub struct VkComputePipelineCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineCreateFlags,
stage: VkPipelineShaderStageCreateInfo,
layout: VkPipelineLayout,
base_pipeline_handle: VkPipeline,
base_pipeline_index: i32,
}

#[repr(C)]
pub struct VkComputePipelineIndirectBufferInfoNV {
s_type: VkStructureType,
p_next: c_void,
device_address: VkDeviceAddress,
size: VkDeviceSize,
pipeline_device_address_capture_replay: VkDeviceAddress,
}

#[repr(C)]
pub struct VkPipelineCreateFlags2CreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineCreateFlags2KHR,
}

#[repr(C)]
pub struct VkVertexInputBindingDescription {
binding: u32,
stride: u32,
input_rate: VkVertexInputRate,
}

#[repr(C)]
pub struct VkVertexInputAttributeDescription {
location: u32,
binding: u32,
format: VkFormat,
offset: u32,
}

#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineVertexInputStateCreateFlags,
vertex_binding_description_count: u32,
p_vertex_binding_descriptions: VkVertexInputBindingDescription,
vertex_attribute_description_count: u32,
p_vertex_attribute_descriptions: VkVertexInputAttributeDescription,
}

#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineInputAssemblyStateCreateFlags,
topology: VkPrimitiveTopology,
primitive_restart_enable: VkBool32,
}

#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineTessellationStateCreateFlags,
patch_control_points: u32,
}

#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineViewportStateCreateFlags,
viewport_count: u32,
p_viewports: VkViewport,
scissor_count: u32,
p_scissors: VkRect2D,
}

#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineRasterizationStateCreateFlags,
depth_clamp_enable: VkBool32,
rasterizer_discard_enable: VkBool32,
polygon_mode: VkPolygonMode,
cull_mode: VkCullModeFlags,
front_face: VkFrontFace,
depth_bias_enable: VkBool32,
depth_bias_constant_factor: f32,
depth_bias_clamp: f32,
depth_bias_slope_factor: f32,
line_width: f32,
}

#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineMultisampleStateCreateFlags,
rasterization_samples: VkSampleCountFlagBits,
sample_shading_enable: VkBool32,
min_sample_shading: f32,
p_sample_mask: VkSampleMask,
alpha_to_coverage_enable: VkBool32,
alpha_to_one_enable: VkBool32,
}

#[repr(C)]
pub struct VkPipelineColorBlendAttachmentState {
blend_enable: VkBool32,
src_color_blend_factor: VkBlendFactor,
dst_color_blend_factor: VkBlendFactor,
color_blend_op: VkBlendOp,
src_alpha_blend_factor: VkBlendFactor,
dst_alpha_blend_factor: VkBlendFactor,
alpha_blend_op: VkBlendOp,
color_write_mask: VkColorComponentFlags,
}

#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineColorBlendStateCreateFlags,
logic_op_enable: VkBool32,
logic_op: VkLogicOp,
attachment_count: u32,
p_attachments: VkPipelineColorBlendAttachmentState,
blend_constants: f32,
}

#[repr(C)]
pub struct VkPipelineDynamicStateCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineDynamicStateCreateFlags,
dynamic_state_count: u32,
p_dynamic_states: VkDynamicState,
}

#[repr(C)]
pub struct VkStencilOpState {
fail_op: VkStencilOp,
pass_op: VkStencilOp,
depth_fail_op: VkStencilOp,
compare_op: VkCompareOp,
compare_mask: u32,
write_mask: u32,
reference: u32,
}

#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineDepthStencilStateCreateFlags,
depth_test_enable: VkBool32,
depth_write_enable: VkBool32,
depth_compare_op: VkCompareOp,
depth_bounds_test_enable: VkBool32,
stencil_test_enable: VkBool32,
front: VkStencilOpState,
back: VkStencilOpState,
min_depth_bounds: f32,
max_depth_bounds: f32,
}

#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineCreateFlags,
stage_count: u32,
#[cfg(vulkan)]
p_stages: VkPipelineShaderStageCreateInfo,
#[cfg(vulkansc)]
p_stages: VkPipelineShaderStageCreateInfo,
p_vertex_input_state: VkPipelineVertexInputStateCreateInfo,
p_input_assembly_state: VkPipelineInputAssemblyStateCreateInfo,
p_tessellation_state: VkPipelineTessellationStateCreateInfo,
p_viewport_state: VkPipelineViewportStateCreateInfo,
p_rasterization_state: VkPipelineRasterizationStateCreateInfo,
p_multisample_state: VkPipelineMultisampleStateCreateInfo,
p_depth_stencil_state: VkPipelineDepthStencilStateCreateInfo,
p_color_blend_state: VkPipelineColorBlendStateCreateInfo,
p_dynamic_state: VkPipelineDynamicStateCreateInfo,
layout: VkPipelineLayout,
render_pass: VkRenderPass,
subpass: u32,
base_pipeline_handle: VkPipeline,
base_pipeline_index: i32,
}

#[repr(C)]
pub struct VkPipelineCacheCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineCacheCreateFlags,
#[cfg(vulkan)]
initial_data_size: usize,
#[cfg(vulkansc)]
initial_data_size: usize,
p_initial_data: c_void,
}

#[repr(C)]
pub struct VkPipelineCacheHeaderVersionOne {
header_size: u32,
header_version: VkPipelineCacheHeaderVersion,
vendor_id: u32,
device_id: u32,
pipeline_cache_uuid: u8,
}

#[repr(C)]
pub struct VkPipelineCacheStageValidationIndexEntry {
code_size: u64,
code_offset: u64,
}

#[repr(C)]
pub struct VkPipelineCacheSafetyCriticalIndexEntry {
pipeline_identifier: u8,
pipeline_memory_size: u64,
json_size: u64,
json_offset: u64,
stage_index_count: u32,
stage_index_stride: u32,
stage_index_offset: u64,
}

#[repr(C)]
pub struct VkPipelineCacheHeaderVersionSafetyCriticalOne {
header_version_one: VkPipelineCacheHeaderVersionOne,
validation_version: VkPipelineCacheValidationVersion,
implementation_data: u32,
pipeline_index_count: u32,
pipeline_index_stride: u32,
pipeline_index_offset: u64,
}

#[repr(C)]
pub struct VkPushConstantRange {
stage_flags: VkShaderStageFlags,
offset: u32,
size: u32,
}

#[repr(C)]
pub struct VkPipelineLayoutCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineLayoutCreateFlags,
set_layout_count: u32,
p_set_layouts: VkDescriptorSetLayout,
push_constant_range_count: u32,
p_push_constant_ranges: VkPushConstantRange,
}

#[repr(C)]
pub struct VkSamplerCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkSamplerCreateFlags,
mag_filter: VkFilter,
min_filter: VkFilter,
mipmap_mode: VkSamplerMipmapMode,
address_mode_u: VkSamplerAddressMode,
address_mode_v: VkSamplerAddressMode,
address_mode_w: VkSamplerAddressMode,
mip_lod_bias: f32,
anisotropy_enable: VkBool32,
max_anisotropy: f32,
compare_enable: VkBool32,
compare_op: VkCompareOp,
min_lod: f32,
max_lod: f32,
border_color: VkBorderColor,
unnormalized_coordinates: VkBool32,
}

#[repr(C)]
pub struct VkCommandPoolCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkCommandPoolCreateFlags,
queue_family_index: u32,
}

#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
s_type: VkStructureType,
p_next: c_void,
command_pool: VkCommandPool,
level: VkCommandBufferLevel,
command_buffer_count: u32,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceInfo {
s_type: VkStructureType,
p_next: c_void,
render_pass: VkRenderPass,
subpass: u32,
framebuffer: VkFramebuffer,
occlusion_query_enable: VkBool32,
query_flags: VkQueryControlFlags,
pipeline_statistics: VkQueryPipelineStatisticFlags,
}

#[repr(C)]
pub struct VkCommandBufferBeginInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkCommandBufferUsageFlags,
p_inheritance_info: VkCommandBufferInheritanceInfo,
}

#[repr(C)]
pub struct VkRenderPassBeginInfo {
s_type: VkStructureType,
p_next: c_void,
render_pass: VkRenderPass,
framebuffer: VkFramebuffer,
render_area: VkRect2D,
clear_value_count: u32,
p_clear_values: VkClearValue,
}

#[repr(C)]
pub struct VkClearDepthStencilValue {
depth: f32,
stencil: u32,
}

#[repr(C)]
pub struct VkClearAttachment {
aspect_mask: VkImageAspectFlags,
color_attachment: u32,
clear_value: VkClearValue,
}

#[repr(C)]
pub struct VkAttachmentDescription {
flags: VkAttachmentDescriptionFlags,
format: VkFormat,
samples: VkSampleCountFlagBits,
load_op: VkAttachmentLoadOp,
store_op: VkAttachmentStoreOp,
stencil_load_op: VkAttachmentLoadOp,
stencil_store_op: VkAttachmentStoreOp,
initial_layout: VkImageLayout,
final_layout: VkImageLayout,
}

#[repr(C)]
pub struct VkAttachmentReference {
attachment: u32,
layout: VkImageLayout,
}

#[repr(C)]
pub struct VkSubpassDescription {
flags: VkSubpassDescriptionFlags,
pipeline_bind_point: VkPipelineBindPoint,
input_attachment_count: u32,
p_input_attachments: VkAttachmentReference,
color_attachment_count: u32,
p_color_attachments: VkAttachmentReference,
p_resolve_attachments: VkAttachmentReference,
p_depth_stencil_attachment: VkAttachmentReference,
preserve_attachment_count: u32,
p_preserve_attachments: u32,
}

#[repr(C)]
pub struct VkSubpassDependency {
src_subpass: u32,
dst_subpass: u32,
src_stage_mask: VkPipelineStageFlags,
dst_stage_mask: VkPipelineStageFlags,
src_access_mask: VkAccessFlags,
dst_access_mask: VkAccessFlags,
dependency_flags: VkDependencyFlags,
}

#[repr(C)]
pub struct VkRenderPassCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkRenderPassCreateFlags,
attachment_count: u32,
p_attachments: VkAttachmentDescription,
subpass_count: u32,
p_subpasses: VkSubpassDescription,
dependency_count: u32,
p_dependencies: VkSubpassDependency,
}

#[repr(C)]
pub struct VkEventCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkEventCreateFlags,
}

#[repr(C)]
pub struct VkFenceCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkFenceCreateFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceFeatures {
robust_buffer_access: VkBool32,
full_draw_index_uint32: VkBool32,
image_cube_array: VkBool32,
independent_blend: VkBool32,
geometry_shader: VkBool32,
tessellation_shader: VkBool32,
sample_rate_shading: VkBool32,
dual_src_blend: VkBool32,
logic_op: VkBool32,
multi_draw_indirect: VkBool32,
draw_indirect_first_instance: VkBool32,
depth_clamp: VkBool32,
depth_bias_clamp: VkBool32,
fill_mode_non_solid: VkBool32,
depth_bounds: VkBool32,
wide_lines: VkBool32,
large_points: VkBool32,
alpha_to_one: VkBool32,
multi_viewport: VkBool32,
sampler_anisotropy: VkBool32,
texture_compression_etc2: VkBool32,
texture_compression_astc_ldr: VkBool32,
texture_compression_bc: VkBool32,
occlusion_query_precise: VkBool32,
pipeline_statistics_query: VkBool32,
vertex_pipeline_stores_and_atomics: VkBool32,
fragment_stores_and_atomics: VkBool32,
shader_tessellation_and_geometry_point_size: VkBool32,
shader_image_gather_extended: VkBool32,
shader_storage_image_extended_formats: VkBool32,
shader_storage_image_multisample: VkBool32,
shader_storage_image_read_without_format: VkBool32,
shader_storage_image_write_without_format: VkBool32,
shader_uniform_buffer_array_dynamic_indexing: VkBool32,
shader_sampled_image_array_dynamic_indexing: VkBool32,
shader_storage_buffer_array_dynamic_indexing: VkBool32,
shader_storage_image_array_dynamic_indexing: VkBool32,
shader_clip_distance: VkBool32,
shader_cull_distance: VkBool32,
shader_float64: VkBool32,
shader_int64: VkBool32,
shader_int16: VkBool32,
shader_resource_residency: VkBool32,
shader_resource_min_lod: VkBool32,
sparse_binding: VkBool32,
sparse_residency_buffer: VkBool32,
sparse_residency_image2_d: VkBool32,
sparse_residency_image3_d: VkBool32,
sparse_residency2_samples: VkBool32,
sparse_residency4_samples: VkBool32,
sparse_residency8_samples: VkBool32,
sparse_residency16_samples: VkBool32,
sparse_residency_aliased: VkBool32,
variable_multisample_rate: VkBool32,
inherited_queries: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
residency_standard2_dblock_shape: VkBool32,
residency_standard2_dmultisample_block_shape: VkBool32,
residency_standard3_dblock_shape: VkBool32,
residency_aligned_mip_size: VkBool32,
residency_non_resident_strict: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceLimits {
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
buffer_image_granularity: VkDeviceSize,
sparse_address_space_size: VkDeviceSize,
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
max_compute_work_group_count: u32,
max_compute_work_group_invocations: u32,
max_compute_work_group_size: u32,
sub_pixel_precision_bits: u32,
sub_texel_precision_bits: u32,
mipmap_precision_bits: u32,
max_draw_indexed_index_value: u32,
max_draw_indirect_count: u32,
max_sampler_lod_bias: f32,
max_sampler_anisotropy: f32,
max_viewports: u32,
max_viewport_dimensions: u32,
viewport_bounds_range: f32,
viewport_sub_pixel_bits: u32,
min_memory_map_alignment: usize,
min_texel_buffer_offset_alignment: VkDeviceSize,
min_uniform_buffer_offset_alignment: VkDeviceSize,
min_storage_buffer_offset_alignment: VkDeviceSize,
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
framebuffer_color_sample_counts: VkSampleCountFlags,
framebuffer_depth_sample_counts: VkSampleCountFlags,
framebuffer_stencil_sample_counts: VkSampleCountFlags,
framebuffer_no_attachments_sample_counts: VkSampleCountFlags,
max_color_attachments: u32,
sampled_image_color_sample_counts: VkSampleCountFlags,
sampled_image_integer_sample_counts: VkSampleCountFlags,
sampled_image_depth_sample_counts: VkSampleCountFlags,
sampled_image_stencil_sample_counts: VkSampleCountFlags,
storage_image_sample_counts: VkSampleCountFlags,
max_sample_mask_words: u32,
timestamp_compute_and_graphics: VkBool32,
timestamp_period: f32,
max_clip_distances: u32,
max_cull_distances: u32,
max_combined_clip_and_cull_distances: u32,
discrete_queue_priorities: u32,
point_size_range: f32,
line_width_range: f32,
point_size_granularity: f32,
line_width_granularity: f32,
strict_lines: VkBool32,
standard_sample_locations: VkBool32,
optimal_buffer_copy_offset_alignment: VkDeviceSize,
optimal_buffer_copy_row_pitch_alignment: VkDeviceSize,
non_coherent_atom_size: VkDeviceSize,
}

#[repr(C)]
pub struct VkSemaphoreCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkSemaphoreCreateFlags,
}

#[repr(C)]
pub struct VkQueryPoolCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkQueryPoolCreateFlags,
query_type: VkQueryType,
query_count: u32,
pipeline_statistics: VkQueryPipelineStatisticFlags,
}

#[repr(C)]
pub struct VkFramebufferCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkFramebufferCreateFlags,
render_pass: VkRenderPass,
attachment_count: u32,
p_attachments: VkImageView,
width: u32,
height: u32,
layers: u32,
}

#[repr(C)]
pub struct VkDrawIndirectCommand {
vertex_count: u32,
instance_count: u32,
first_vertex: u32,
first_instance: u32,
}

#[repr(C)]
pub struct VkDrawIndexedIndirectCommand {
index_count: u32,
instance_count: u32,
first_index: u32,
vertex_offset: i32,
first_instance: u32,
}

#[repr(C)]
pub struct VkDispatchIndirectCommand {
x: u32,
y: u32,
z: u32,
}

#[repr(C)]
pub struct VkMultiDrawInfoEXT {
first_vertex: u32,
vertex_count: u32,
}

#[repr(C)]
pub struct VkMultiDrawIndexedInfoEXT {
first_index: u32,
index_count: u32,
vertex_offset: i32,
}

#[repr(C)]
pub struct VkSubmitInfo {
s_type: VkStructureType,
p_next: c_void,
wait_semaphore_count: u32,
p_wait_semaphores: VkSemaphore,
p_wait_dst_stage_mask: VkPipelineStageFlags,
command_buffer_count: u32,
p_command_buffers: VkCommandBuffer,
signal_semaphore_count: u32,
p_signal_semaphores: VkSemaphore,
}

#[repr(C)]
pub struct VkDisplayPropertiesKHR {
display: VkDisplayKHR,
display_name: u8,
physical_dimensions: VkExtent2D,
physical_resolution: VkExtent2D,
supported_transforms: VkSurfaceTransformFlagsKHR,
plane_reorder_possible: VkBool32,
persistent_content: VkBool32,
}

#[repr(C)]
pub struct VkDisplayPlanePropertiesKHR {
current_display: VkDisplayKHR,
current_stack_index: u32,
}

#[repr(C)]
pub struct VkDisplayModeParametersKHR {
visible_region: VkExtent2D,
refresh_rate: u32,
}

#[repr(C)]
pub struct VkDisplayModePropertiesKHR {
display_mode: VkDisplayModeKHR,
parameters: VkDisplayModeParametersKHR,
}

#[repr(C)]
pub struct VkDisplayModeCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkDisplayModeCreateFlagsKHR,
parameters: VkDisplayModeParametersKHR,
}

#[repr(C)]
pub struct VkDisplayPlaneCapabilitiesKHR {
supported_alpha: VkDisplayPlaneAlphaFlagsKHR,
min_src_position: VkOffset2D,
max_src_position: VkOffset2D,
min_src_extent: VkExtent2D,
max_src_extent: VkExtent2D,
min_dst_position: VkOffset2D,
max_dst_position: VkOffset2D,
min_dst_extent: VkExtent2D,
max_dst_extent: VkExtent2D,
}

#[repr(C)]
pub struct VkDisplaySurfaceCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkDisplaySurfaceCreateFlagsKHR,
display_mode: VkDisplayModeKHR,
plane_index: u32,
plane_stack_index: u32,
transform: VkSurfaceTransformFlagBitsKHR,
global_alpha: f32,
alpha_mode: VkDisplayPlaneAlphaFlagBitsKHR,
image_extent: VkExtent2D,
}

#[repr(C)]
pub struct VkDisplayPresentInfoKHR {
s_type: VkStructureType,
p_next: c_void,
src_rect: VkRect2D,
dst_rect: VkRect2D,
persistent: VkBool32,
}

#[repr(C)]
pub struct VkSurfaceCapabilitiesKHR {
min_image_count: u32,
max_image_count: u32,
current_extent: VkExtent2D,
min_image_extent: VkExtent2D,
max_image_extent: VkExtent2D,
max_image_array_layers: u32,
supported_transforms: VkSurfaceTransformFlagsKHR,
current_transform: VkSurfaceTransformFlagBitsKHR,
supported_composite_alpha: VkCompositeAlphaFlagsKHR,
supported_usage_flags: VkImageUsageFlags,
}

#[repr(C)]
pub struct VkAndroidSurfaceCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkAndroidSurfaceCreateFlagsKHR,
window: ANativeWindow,
}

#[repr(C)]
pub struct VkViSurfaceCreateInfoNN {
s_type: VkStructureType,
p_next: c_void,
flags: VkViSurfaceCreateFlagsNN,
window: c_void,
}

#[repr(C)]
pub struct VkWaylandSurfaceCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkWaylandSurfaceCreateFlagsKHR,
display: wl_display,
surface: wl_surface,
}

#[repr(C)]
pub struct VkWin32SurfaceCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkWin32SurfaceCreateFlagsKHR,
hinstance: HINSTANCE,
hwnd: HWND,
}

#[repr(C)]
pub struct VkXlibSurfaceCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkXlibSurfaceCreateFlagsKHR,
dpy: Display,
window: Window,
}

#[repr(C)]
pub struct VkXcbSurfaceCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkXcbSurfaceCreateFlagsKHR,
connection: xcb_connection_t,
window: xcb_window_t,
}

#[repr(C)]
pub struct VkDirectFBSurfaceCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkDirectFBSurfaceCreateFlagsEXT,
dfb: IDirectFB,
surface: IDirectFBSurface,
}

#[repr(C)]
pub struct VkImagePipeSurfaceCreateInfoFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
flags: VkImagePipeSurfaceCreateFlagsFUCHSIA,
image_pipe_handle: zx_handle_t,
}

#[repr(C)]
pub struct VkStreamDescriptorSurfaceCreateInfoGGP {
s_type: VkStructureType,
p_next: c_void,
flags: VkStreamDescriptorSurfaceCreateFlagsGGP,
stream_descriptor: GgpStreamDescriptor,
}

#[repr(C)]
pub struct VkScreenSurfaceCreateInfoQNX {
s_type: VkStructureType,
p_next: c_void,
flags: VkScreenSurfaceCreateFlagsQNX,
context: _screen_context,
window: _screen_window,
}

#[repr(C)]
pub struct VkSurfaceFormatKHR {
format: VkFormat,
color_space: VkColorSpaceKHR,
}

#[repr(C)]
pub struct VkSwapchainCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkSwapchainCreateFlagsKHR,
surface: VkSurfaceKHR,
min_image_count: u32,
image_format: VkFormat,
image_color_space: VkColorSpaceKHR,
image_extent: VkExtent2D,
image_array_layers: u32,
image_usage: VkImageUsageFlags,
image_sharing_mode: VkSharingMode,
queue_family_index_count: u32,
p_queue_family_indices: u32,
pre_transform: VkSurfaceTransformFlagBitsKHR,
composite_alpha: VkCompositeAlphaFlagBitsKHR,
present_mode: VkPresentModeKHR,
clipped: VkBool32,
#[cfg(vulkan)]
old_swapchain: VkSwapchainKHR,
#[cfg(vulkansc)]
old_swapchain: VkSwapchainKHR,
}

#[repr(C)]
pub struct VkPresentInfoKHR {
s_type: VkStructureType,
p_next: c_void,
wait_semaphore_count: u32,
p_wait_semaphores: VkSemaphore,
swapchain_count: u32,
p_swapchains: VkSwapchainKHR,
p_image_indices: u32,
p_results: VkResult,
}

#[repr(C)]
pub struct VkDebugReportCallbackCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkDebugReportFlagsEXT,
pfn_callback: PFN_vkDebugReportCallbackEXT,
p_user_data: c_void,
}

#[repr(C)]
pub struct VkValidationFlagsEXT {
s_type: VkStructureType,
p_next: c_void,
disabled_validation_check_count: u32,
p_disabled_validation_checks: VkValidationCheckEXT,
}

#[repr(C)]
pub struct VkValidationFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
enabled_validation_feature_count: u32,
p_enabled_validation_features: VkValidationFeatureEnableEXT,
disabled_validation_feature_count: u32,
p_disabled_validation_features: VkValidationFeatureDisableEXT,
}

#[repr(C)]
pub struct VkApplicationParametersEXT {
s_type: VkStructureType,
p_next: c_void,
vendor_id: u32,
device_id: u32,
key: u32,
value: u64,
}

#[repr(C)]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD {
s_type: VkStructureType,
p_next: c_void,
rasterization_order: VkRasterizationOrderAMD,
}

#[repr(C)]
pub struct VkDebugMarkerObjectNameInfoEXT {
s_type: VkStructureType,
p_next: c_void,
object_type: VkDebugReportObjectTypeEXT,
object: u64,
p_object_name: u8,
}

#[repr(C)]
pub struct VkDebugMarkerObjectTagInfoEXT {
s_type: VkStructureType,
p_next: c_void,
object_type: VkDebugReportObjectTypeEXT,
object: u64,
tag_name: u64,
tag_size: usize,
p_tag: c_void,
}

#[repr(C)]
pub struct VkDebugMarkerMarkerInfoEXT {
s_type: VkStructureType,
p_next: c_void,
p_marker_name: u8,
color: f32,
}

#[repr(C)]
pub struct VkDedicatedAllocationImageCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
dedicated_allocation: VkBool32,
}

#[repr(C)]
pub struct VkDedicatedAllocationBufferCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
dedicated_allocation: VkBool32,
}

#[repr(C)]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV {
s_type: VkStructureType,
p_next: c_void,
image: VkImage,
buffer: VkBuffer,
}

#[repr(C)]
pub struct VkExternalImageFormatPropertiesNV {
image_format_properties: VkImageFormatProperties,
external_memory_features: VkExternalMemoryFeatureFlagsNV,
export_from_imported_handle_types: VkExternalMemoryHandleTypeFlagsNV,
compatible_handle_types: VkExternalMemoryHandleTypeFlagsNV,
}

#[repr(C)]
pub struct VkExternalMemoryImageCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
handle_types: VkExternalMemoryHandleTypeFlagsNV,
}

#[repr(C)]
pub struct VkExportMemoryAllocateInfoNV {
s_type: VkStructureType,
p_next: c_void,
handle_types: VkExternalMemoryHandleTypeFlagsNV,
}

#[repr(C)]
pub struct VkImportMemoryWin32HandleInfoNV {
s_type: VkStructureType,
p_next: c_void,
handle_type: VkExternalMemoryHandleTypeFlagsNV,
handle: HANDLE,
}

#[repr(C)]
pub struct VkExportMemoryWin32HandleInfoNV {
s_type: VkStructureType,
p_next: c_void,
p_attributes: SECURITY_ATTRIBUTES,
dw_access: DWORD,
}

#[repr(C)]
pub struct VkExportMemorySciBufInfoNV {
s_type: VkStructureType,
p_next: c_void,
p_attributes: NvSciBufAttrList,
}

#[repr(C)]
pub struct VkImportMemorySciBufInfoNV {
s_type: VkStructureType,
p_next: c_void,
handle_type: VkExternalMemoryHandleTypeFlagBits,
handle: NvSciBufObj,
}

#[repr(C)]
pub struct VkMemoryGetSciBufInfoNV {
s_type: VkStructureType,
p_next: c_void,
memory: VkDeviceMemory,
handle_type: VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkMemorySciBufPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
memory_type_bits: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalMemorySciBufFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
sci_buf_import: VkBool32,
sci_buf_export: VkBool32,
}

#[repr(C)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV {
s_type: VkStructureType,
p_next: c_void,
acquire_count: u32,
p_acquire_syncs: VkDeviceMemory,
p_acquire_keys: u64,
p_acquire_timeout_milliseconds: u32,
release_count: u32,
p_release_syncs: VkDeviceMemory,
p_release_keys: u64,
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
device_generated_commands: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
device_generated_compute: VkBool32,
device_generated_compute_pipelines: VkBool32,
device_generated_compute_capture_replay: VkBool32,
}

#[repr(C)]
pub struct VkDevicePrivateDataCreateInfo {
s_type: VkStructureType,
p_next: c_void,
private_data_slot_request_count: u32,
}

#[repr(C)]
pub struct VkPrivateDataSlotCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkPrivateDataSlotCreateFlags,
}

#[repr(C)]
pub struct VkPhysicalDevicePrivateDataFeatures {
s_type: VkStructureType,
p_next: c_void,
private_data: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
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
pub struct VkPhysicalDeviceMultiDrawPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
max_multi_draw_count: u32,
}

#[repr(C)]
pub struct VkGraphicsShaderGroupCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
stage_count: u32,
p_stages: VkPipelineShaderStageCreateInfo,
p_vertex_input_state: VkPipelineVertexInputStateCreateInfo,
p_tessellation_state: VkPipelineTessellationStateCreateInfo,
}

#[repr(C)]
pub struct VkGraphicsPipelineShaderGroupsCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
group_count: u32,
p_groups: VkGraphicsShaderGroupCreateInfoNV,
pipeline_count: u32,
p_pipelines: VkPipeline,
}

#[repr(C)]
pub struct VkBindShaderGroupIndirectCommandNV {
group_index: u32,
}

#[repr(C)]
pub struct VkBindIndexBufferIndirectCommandNV {
buffer_address: VkDeviceAddress,
size: u32,
index_type: VkIndexType,
}

#[repr(C)]
pub struct VkBindVertexBufferIndirectCommandNV {
buffer_address: VkDeviceAddress,
size: u32,
stride: u32,
}

#[repr(C)]
pub struct VkSetStateFlagsIndirectCommandNV {
data: u32,
}

#[repr(C)]
pub struct VkIndirectCommandsStreamNV {
buffer: VkBuffer,
offset: VkDeviceSize,
}

#[repr(C)]
pub struct VkIndirectCommandsLayoutTokenNV {
s_type: VkStructureType,
p_next: c_void,
token_type: VkIndirectCommandsTokenTypeNV,
stream: u32,
offset: u32,
vertex_binding_unit: u32,
vertex_dynamic_stride: VkBool32,
pushconstant_pipeline_layout: VkPipelineLayout,
pushconstant_shader_stage_flags: VkShaderStageFlags,
pushconstant_offset: u32,
pushconstant_size: u32,
indirect_state_flags: VkIndirectStateFlagsNV,
index_type_count: u32,
p_index_types: VkIndexType,
p_index_type_values: u32,
}

#[repr(C)]
pub struct VkIndirectCommandsLayoutCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
flags: VkIndirectCommandsLayoutUsageFlagsNV,
pipeline_bind_point: VkPipelineBindPoint,
token_count: u32,
p_tokens: VkIndirectCommandsLayoutTokenNV,
stream_count: u32,
p_stream_strides: u32,
}

#[repr(C)]
pub struct VkGeneratedCommandsInfoNV {
s_type: VkStructureType,
p_next: c_void,
pipeline_bind_point: VkPipelineBindPoint,
pipeline: VkPipeline,
indirect_commands_layout: VkIndirectCommandsLayoutNV,
stream_count: u32,
p_streams: VkIndirectCommandsStreamNV,
sequences_count: u32,
preprocess_buffer: VkBuffer,
preprocess_offset: VkDeviceSize,
preprocess_size: VkDeviceSize,
sequences_count_buffer: VkBuffer,
sequences_count_offset: VkDeviceSize,
sequences_index_buffer: VkBuffer,
sequences_index_offset: VkDeviceSize,
}

#[repr(C)]
pub struct VkGeneratedCommandsMemoryRequirementsInfoNV {
s_type: VkStructureType,
p_next: c_void,
pipeline_bind_point: VkPipelineBindPoint,
pipeline: VkPipeline,
indirect_commands_layout: VkIndirectCommandsLayoutNV,
max_sequences_count: u32,
}

#[repr(C)]
pub struct VkPipelineIndirectDeviceAddressInfoNV {
s_type: VkStructureType,
p_next: c_void,
pipeline_bind_point: VkPipelineBindPoint,
pipeline: VkPipeline,
}

#[repr(C)]
pub struct VkBindPipelineIndirectCommandNV {
pipeline_address: VkDeviceAddress,
}

#[repr(C)]
pub struct VkPhysicalDeviceFeatures2 {
s_type: VkStructureType,
p_next: c_void,
features: VkPhysicalDeviceFeatures,
}

#[repr(C)]
pub struct VkPhysicalDeviceProperties2 {
s_type: VkStructureType,
p_next: c_void,
properties: VkPhysicalDeviceProperties,
}

#[repr(C)]
pub struct VkFormatProperties2 {
s_type: VkStructureType,
p_next: c_void,
format_properties: VkFormatProperties,
}

#[repr(C)]
pub struct VkImageFormatProperties2 {
s_type: VkStructureType,
p_next: c_void,
image_format_properties: VkImageFormatProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageFormatInfo2 {
s_type: VkStructureType,
p_next: c_void,
format: VkFormat,
r#type: VkImageType,
tiling: VkImageTiling,
usage: VkImageUsageFlags,
flags: VkImageCreateFlags,
}

#[repr(C)]
pub struct VkQueueFamilyProperties2 {
s_type: VkStructureType,
p_next: c_void,
queue_family_properties: VkQueueFamilyProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties2 {
s_type: VkStructureType,
p_next: c_void,
memory_properties: VkPhysicalDeviceMemoryProperties,
}

#[repr(C)]
pub struct VkSparseImageFormatProperties2 {
s_type: VkStructureType,
p_next: c_void,
properties: VkSparseImageFormatProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2 {
s_type: VkStructureType,
p_next: c_void,
format: VkFormat,
r#type: VkImageType,
samples: VkSampleCountFlagBits,
usage: VkImageUsageFlags,
tiling: VkImageTiling,
}

#[repr(C)]
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
max_push_descriptors: u32,
}

#[repr(C)]
pub struct VkConformanceVersion {
major: u8,
minor: u8,
subminor: u8,
patch: u8,
}

#[repr(C)]
pub struct VkPhysicalDeviceDriverProperties {
s_type: VkStructureType,
p_next: c_void,
driver_id: VkDriverId,
driver_name: u8,
driver_info: u8,
conformance_version: VkConformanceVersion,
}

#[repr(C)]
pub struct VkPresentRegionsKHR {
s_type: VkStructureType,
p_next: c_void,
swapchain_count: u32,
p_regions: VkPresentRegionKHR,
}

#[repr(C)]
pub struct VkPresentRegionKHR {
rectangle_count: u32,
p_rectangles: VkRectLayerKHR,
}

#[repr(C)]
pub struct VkRectLayerKHR {
offset: VkOffset2D,
extent: VkExtent2D,
layer: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVariablePointersFeatures {
s_type: VkStructureType,
p_next: c_void,
variable_pointers_storage_buffer: VkBool32,
variable_pointers: VkBool32,
}

#[repr(C)]
pub struct VkExternalMemoryProperties {
external_memory_features: VkExternalMemoryFeatureFlags,
export_from_imported_handle_types: VkExternalMemoryHandleTypeFlags,
compatible_handle_types: VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalImageFormatInfo {
s_type: VkStructureType,
p_next: c_void,
handle_type: VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkExternalImageFormatProperties {
s_type: VkStructureType,
p_next: c_void,
external_memory_properties: VkExternalMemoryProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalBufferInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkBufferCreateFlags,
usage: VkBufferUsageFlags,
handle_type: VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkExternalBufferProperties {
s_type: VkStructureType,
p_next: c_void,
external_memory_properties: VkExternalMemoryProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceIDProperties {
s_type: VkStructureType,
p_next: c_void,
device_uuid: u8,
driver_uuid: u8,
device_luid: u8,
device_node_mask: u32,
device_luidvalid: VkBool32,
}

#[repr(C)]
pub struct VkExternalMemoryImageCreateInfo {
s_type: VkStructureType,
p_next: c_void,
handle_types: VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct VkExternalMemoryBufferCreateInfo {
s_type: VkStructureType,
p_next: c_void,
handle_types: VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct VkExportMemoryAllocateInfo {
s_type: VkStructureType,
p_next: c_void,
handle_types: VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct VkImportMemoryWin32HandleInfoKHR {
s_type: VkStructureType,
p_next: c_void,
handle_type: VkExternalMemoryHandleTypeFlagBits,
handle: HANDLE,
name: LPCWSTR,
}

#[repr(C)]
pub struct VkExportMemoryWin32HandleInfoKHR {
s_type: VkStructureType,
p_next: c_void,
p_attributes: SECURITY_ATTRIBUTES,
dw_access: DWORD,
name: LPCWSTR,
}

#[repr(C)]
pub struct VkImportMemoryZirconHandleInfoFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
handle_type: VkExternalMemoryHandleTypeFlagBits,
handle: zx_handle_t,
}

#[repr(C)]
pub struct VkMemoryZirconHandlePropertiesFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
memory_type_bits: u32,
}

#[repr(C)]
pub struct VkMemoryGetZirconHandleInfoFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
memory: VkDeviceMemory,
handle_type: VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkMemoryWin32HandlePropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
memory_type_bits: u32,
}

#[repr(C)]
pub struct VkMemoryGetWin32HandleInfoKHR {
s_type: VkStructureType,
p_next: c_void,
memory: VkDeviceMemory,
handle_type: VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkImportMemoryFdInfoKHR {
s_type: VkStructureType,
p_next: c_void,
handle_type: VkExternalMemoryHandleTypeFlagBits,
fd: int,
}

#[repr(C)]
pub struct VkMemoryFdPropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
memory_type_bits: u32,
}

#[repr(C)]
pub struct VkMemoryGetFdInfoKHR {
s_type: VkStructureType,
p_next: c_void,
memory: VkDeviceMemory,
handle_type: VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR {
s_type: VkStructureType,
p_next: c_void,
acquire_count: u32,
p_acquire_syncs: VkDeviceMemory,
p_acquire_keys: u64,
p_acquire_timeouts: u32,
release_count: u32,
p_release_syncs: VkDeviceMemory,
p_release_keys: u64,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalSemaphoreInfo {
s_type: VkStructureType,
p_next: c_void,
handle_type: VkExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkExternalSemaphoreProperties {
s_type: VkStructureType,
p_next: c_void,
export_from_imported_handle_types: VkExternalSemaphoreHandleTypeFlags,
compatible_handle_types: VkExternalSemaphoreHandleTypeFlags,
external_semaphore_features: VkExternalSemaphoreFeatureFlags,
}

#[repr(C)]
pub struct VkExportSemaphoreCreateInfo {
s_type: VkStructureType,
p_next: c_void,
handle_types: VkExternalSemaphoreHandleTypeFlags,
}

#[repr(C)]
pub struct VkImportSemaphoreWin32HandleInfoKHR {
s_type: VkStructureType,
p_next: c_void,
semaphore: VkSemaphore,
flags: VkSemaphoreImportFlags,
handle_type: VkExternalSemaphoreHandleTypeFlagBits,
handle: HANDLE,
name: LPCWSTR,
}

#[repr(C)]
pub struct VkExportSemaphoreWin32HandleInfoKHR {
s_type: VkStructureType,
p_next: c_void,
p_attributes: SECURITY_ATTRIBUTES,
dw_access: DWORD,
name: LPCWSTR,
}

#[repr(C)]
pub struct VkD3D12FenceSubmitInfoKHR {
s_type: VkStructureType,
p_next: c_void,
wait_semaphore_values_count: u32,
p_wait_semaphore_values: u64,
signal_semaphore_values_count: u32,
p_signal_semaphore_values: u64,
}

#[repr(C)]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
s_type: VkStructureType,
p_next: c_void,
semaphore: VkSemaphore,
handle_type: VkExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkImportSemaphoreFdInfoKHR {
s_type: VkStructureType,
p_next: c_void,
semaphore: VkSemaphore,
flags: VkSemaphoreImportFlags,
handle_type: VkExternalSemaphoreHandleTypeFlagBits,
fd: int,
}

#[repr(C)]
pub struct VkSemaphoreGetFdInfoKHR {
s_type: VkStructureType,
p_next: c_void,
semaphore: VkSemaphore,
handle_type: VkExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkImportSemaphoreZirconHandleInfoFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
semaphore: VkSemaphore,
flags: VkSemaphoreImportFlags,
handle_type: VkExternalSemaphoreHandleTypeFlagBits,
zircon_handle: zx_handle_t,
}

#[repr(C)]
pub struct VkSemaphoreGetZirconHandleInfoFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
semaphore: VkSemaphore,
handle_type: VkExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalFenceInfo {
s_type: VkStructureType,
p_next: c_void,
handle_type: VkExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkExternalFenceProperties {
s_type: VkStructureType,
p_next: c_void,
export_from_imported_handle_types: VkExternalFenceHandleTypeFlags,
compatible_handle_types: VkExternalFenceHandleTypeFlags,
external_fence_features: VkExternalFenceFeatureFlags,
}

#[repr(C)]
pub struct VkExportFenceCreateInfo {
s_type: VkStructureType,
p_next: c_void,
handle_types: VkExternalFenceHandleTypeFlags,
}

#[repr(C)]
pub struct VkImportFenceWin32HandleInfoKHR {
s_type: VkStructureType,
p_next: c_void,
fence: VkFence,
flags: VkFenceImportFlags,
handle_type: VkExternalFenceHandleTypeFlagBits,
handle: HANDLE,
name: LPCWSTR,
}

#[repr(C)]
pub struct VkExportFenceWin32HandleInfoKHR {
s_type: VkStructureType,
p_next: c_void,
p_attributes: SECURITY_ATTRIBUTES,
dw_access: DWORD,
name: LPCWSTR,
}

#[repr(C)]
pub struct VkFenceGetWin32HandleInfoKHR {
s_type: VkStructureType,
p_next: c_void,
fence: VkFence,
handle_type: VkExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkImportFenceFdInfoKHR {
s_type: VkStructureType,
p_next: c_void,
fence: VkFence,
flags: VkFenceImportFlags,
handle_type: VkExternalFenceHandleTypeFlagBits,
fd: int,
}

#[repr(C)]
pub struct VkFenceGetFdInfoKHR {
s_type: VkStructureType,
p_next: c_void,
fence: VkFence,
handle_type: VkExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkExportFenceSciSyncInfoNV {
s_type: VkStructureType,
p_next: c_void,
p_attributes: NvSciSyncAttrList,
}

#[repr(C)]
pub struct VkImportFenceSciSyncInfoNV {
s_type: VkStructureType,
p_next: c_void,
fence: VkFence,
handle_type: VkExternalFenceHandleTypeFlagBits,
handle: c_void,
}

#[repr(C)]
pub struct VkFenceGetSciSyncInfoNV {
s_type: VkStructureType,
p_next: c_void,
fence: VkFence,
handle_type: VkExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkExportSemaphoreSciSyncInfoNV {
s_type: VkStructureType,
p_next: c_void,
p_attributes: NvSciSyncAttrList,
}

#[repr(C)]
pub struct VkImportSemaphoreSciSyncInfoNV {
s_type: VkStructureType,
p_next: c_void,
semaphore: VkSemaphore,
handle_type: VkExternalSemaphoreHandleTypeFlagBits,
handle: c_void,
}

#[repr(C)]
pub struct VkSemaphoreGetSciSyncInfoNV {
s_type: VkStructureType,
p_next: c_void,
semaphore: VkSemaphore,
handle_type: VkExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkSciSyncAttributesInfoNV {
s_type: VkStructureType,
p_next: c_void,
client_type: VkSciSyncClientTypeNV,
primitive_type: VkSciSyncPrimitiveTypeNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalSciSyncFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
sci_sync_fence: VkBool32,
sci_sync_semaphore: VkBool32,
sci_sync_import: VkBool32,
sci_sync_export: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalSciSync2FeaturesNV {
s_type: VkStructureType,
p_next: c_void,
sci_sync_fence: VkBool32,
sci_sync_semaphore2: VkBool32,
sci_sync_import: VkBool32,
sci_sync_export: VkBool32,
}

#[repr(C)]
pub struct VkSemaphoreSciSyncPoolCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
handle: NvSciSyncObj,
}

#[repr(C)]
pub struct VkSemaphoreSciSyncCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
semaphore_pool: VkSemaphoreSciSyncPoolNV,
p_fence: NvSciSyncFence,
}

#[repr(C)]
pub struct VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
semaphore_sci_sync_pool_request_count: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewFeatures {
s_type: VkStructureType,
p_next: c_void,
multiview: VkBool32,
multiview_geometry_shader: VkBool32,
multiview_tessellation_shader: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewProperties {
s_type: VkStructureType,
p_next: c_void,
max_multiview_view_count: u32,
max_multiview_instance_index: u32,
}

#[repr(C)]
pub struct VkRenderPassMultiviewCreateInfo {
s_type: VkStructureType,
p_next: c_void,
subpass_count: u32,
p_view_masks: u32,
dependency_count: u32,
p_view_offsets: i32,
correlation_mask_count: u32,
p_correlation_masks: u32,
}

#[repr(C)]
pub struct VkSurfaceCapabilities2EXT {
s_type: VkStructureType,
p_next: c_void,
min_image_count: u32,
max_image_count: u32,
current_extent: VkExtent2D,
min_image_extent: VkExtent2D,
max_image_extent: VkExtent2D,
max_image_array_layers: u32,
supported_transforms: VkSurfaceTransformFlagsKHR,
current_transform: VkSurfaceTransformFlagBitsKHR,
supported_composite_alpha: VkCompositeAlphaFlagsKHR,
supported_usage_flags: VkImageUsageFlags,
supported_surface_counters: VkSurfaceCounterFlagsEXT,
}

#[repr(C)]
pub struct VkDisplayPowerInfoEXT {
s_type: VkStructureType,
p_next: c_void,
power_state: VkDisplayPowerStateEXT,
}

#[repr(C)]
pub struct VkDeviceEventInfoEXT {
s_type: VkStructureType,
p_next: c_void,
device_event: VkDeviceEventTypeEXT,
}

#[repr(C)]
pub struct VkDisplayEventInfoEXT {
s_type: VkStructureType,
p_next: c_void,
display_event: VkDisplayEventTypeEXT,
}

#[repr(C)]
pub struct VkSwapchainCounterCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
surface_counters: VkSurfaceCounterFlagsEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceGroupProperties {
s_type: VkStructureType,
p_next: c_void,
physical_device_count: u32,
physical_devices: VkPhysicalDevice,
subset_allocation: VkBool32,
}

#[repr(C)]
pub struct VkMemoryAllocateFlagsInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkMemoryAllocateFlags,
device_mask: u32,
}

#[repr(C)]
pub struct VkBindBufferMemoryInfo {
s_type: VkStructureType,
p_next: c_void,
buffer: VkBuffer,
memory: VkDeviceMemory,
memory_offset: VkDeviceSize,
}

#[repr(C)]
pub struct VkBindBufferMemoryDeviceGroupInfo {
s_type: VkStructureType,
p_next: c_void,
device_index_count: u32,
p_device_indices: u32,
}

#[repr(C)]
pub struct VkBindImageMemoryInfo {
s_type: VkStructureType,
p_next: c_void,
image: VkImage,
memory: VkDeviceMemory,
memory_offset: VkDeviceSize,
}

#[repr(C)]
pub struct VkBindImageMemoryDeviceGroupInfo {
s_type: VkStructureType,
p_next: c_void,
device_index_count: u32,
p_device_indices: u32,
split_instance_bind_region_count: u32,
p_split_instance_bind_regions: VkRect2D,
}

#[repr(C)]
pub struct VkDeviceGroupRenderPassBeginInfo {
s_type: VkStructureType,
p_next: c_void,
device_mask: u32,
device_render_area_count: u32,
p_device_render_areas: VkRect2D,
}

#[repr(C)]
pub struct VkDeviceGroupCommandBufferBeginInfo {
s_type: VkStructureType,
p_next: c_void,
device_mask: u32,
}

#[repr(C)]
pub struct VkDeviceGroupSubmitInfo {
s_type: VkStructureType,
p_next: c_void,
wait_semaphore_count: u32,
p_wait_semaphore_device_indices: u32,
command_buffer_count: u32,
p_command_buffer_device_masks: u32,
signal_semaphore_count: u32,
p_signal_semaphore_device_indices: u32,
}

#[repr(C)]
pub struct VkDeviceGroupBindSparseInfo {
s_type: VkStructureType,
p_next: c_void,
resource_device_index: u32,
memory_device_index: u32,
}

#[repr(C)]
pub struct VkDeviceGroupPresentCapabilitiesKHR {
s_type: VkStructureType,
p_next: c_void,
present_mask: u32,
modes: VkDeviceGroupPresentModeFlagsKHR,
}

#[repr(C)]
pub struct VkImageSwapchainCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
swapchain: VkSwapchainKHR,
}

#[repr(C)]
pub struct VkBindImageMemorySwapchainInfoKHR {
s_type: VkStructureType,
p_next: c_void,
swapchain: VkSwapchainKHR,
image_index: u32,
}

#[repr(C)]
pub struct VkAcquireNextImageInfoKHR {
s_type: VkStructureType,
p_next: c_void,
swapchain: VkSwapchainKHR,
timeout: u64,
semaphore: VkSemaphore,
fence: VkFence,
device_mask: u32,
}

#[repr(C)]
pub struct VkDeviceGroupPresentInfoKHR {
s_type: VkStructureType,
p_next: c_void,
swapchain_count: u32,
p_device_masks: u32,
mode: VkDeviceGroupPresentModeFlagBitsKHR,
}

#[repr(C)]
pub struct VkDeviceGroupDeviceCreateInfo {
s_type: VkStructureType,
p_next: c_void,
physical_device_count: u32,
p_physical_devices: VkPhysicalDevice,
}

#[repr(C)]
pub struct VkDeviceGroupSwapchainCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
modes: VkDeviceGroupPresentModeFlagsKHR,
}

#[repr(C)]
pub struct VkDescriptorUpdateTemplateEntry {
dst_binding: u32,
dst_array_element: u32,
descriptor_count: u32,
descriptor_type: VkDescriptorType,
offset: usize,
stride: usize,
}

#[repr(C)]
pub struct VkDescriptorUpdateTemplateCreateInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkDescriptorUpdateTemplateCreateFlags,
descriptor_update_entry_count: u32,
p_descriptor_update_entries: VkDescriptorUpdateTemplateEntry,
template_type: VkDescriptorUpdateTemplateType,
descriptor_set_layout: VkDescriptorSetLayout,
pipeline_bind_point: VkPipelineBindPoint,
pipeline_layout: VkPipelineLayout,
set: u32,
}

#[repr(C)]
pub struct VkXYColorEXT {
x: f32,
y: f32,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentIdFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
present_id: VkBool32,
}

#[repr(C)]
pub struct VkPresentIdKHR {
s_type: VkStructureType,
p_next: c_void,
swapchain_count: u32,
p_present_ids: u64,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentWaitFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
present_wait: VkBool32,
}

#[repr(C)]
pub struct VkHdrMetadataEXT {
s_type: VkStructureType,
p_next: c_void,
display_primary_red: VkXYColorEXT,
display_primary_green: VkXYColorEXT,
display_primary_blue: VkXYColorEXT,
white_point: VkXYColorEXT,
max_luminance: f32,
min_luminance: f32,
max_content_light_level: f32,
max_frame_average_light_level: f32,
}

#[repr(C)]
pub struct VkDisplayNativeHdrSurfaceCapabilitiesAMD {
s_type: VkStructureType,
p_next: c_void,
local_dimming_support: VkBool32,
}

#[repr(C)]
pub struct VkSwapchainDisplayNativeHdrCreateInfoAMD {
s_type: VkStructureType,
p_next: c_void,
local_dimming_enable: VkBool32,
}

#[repr(C)]
pub struct VkRefreshCycleDurationGOOGLE {
refresh_duration: u64,
}

#[repr(C)]
pub struct VkPastPresentationTimingGOOGLE {
present_id: u32,
desired_present_time: u64,
actual_present_time: u64,
earliest_present_time: u64,
present_margin: u64,
}

#[repr(C)]
pub struct VkPresentTimesInfoGOOGLE {
s_type: VkStructureType,
p_next: c_void,
swapchain_count: u32,
p_times: VkPresentTimeGOOGLE,
}

#[repr(C)]
pub struct VkPresentTimeGOOGLE {
present_id: u32,
desired_present_time: u64,
}

#[repr(C)]
pub struct VkIOSSurfaceCreateInfoMVK {
s_type: VkStructureType,
p_next: c_void,
flags: VkIOSSurfaceCreateFlagsMVK,
p_view: c_void,
}

#[repr(C)]
pub struct VkMacOSSurfaceCreateInfoMVK {
s_type: VkStructureType,
p_next: c_void,
flags: VkMacOSSurfaceCreateFlagsMVK,
p_view: c_void,
}

#[repr(C)]
pub struct VkMetalSurfaceCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkMetalSurfaceCreateFlagsEXT,
p_layer: CAMetalLayer,
}

#[repr(C)]
pub struct VkViewportWScalingNV {
xcoeff: f32,
ycoeff: f32,
}

#[repr(C)]
pub struct VkPipelineViewportWScalingStateCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
viewport_wscaling_enable: VkBool32,
viewport_count: u32,
p_viewport_wscalings: VkViewportWScalingNV,
}

#[repr(C)]
pub struct VkViewportSwizzleNV {
x: VkViewportCoordinateSwizzleNV,
y: VkViewportCoordinateSwizzleNV,
z: VkViewportCoordinateSwizzleNV,
w: VkViewportCoordinateSwizzleNV,
}

#[repr(C)]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineViewportSwizzleStateCreateFlagsNV,
viewport_count: u32,
p_viewport_swizzles: VkViewportSwizzleNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
max_discard_rectangles: u32,
}

#[repr(C)]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineDiscardRectangleStateCreateFlagsEXT,
discard_rectangle_mode: VkDiscardRectangleModeEXT,
discard_rectangle_count: u32,
p_discard_rectangles: VkRect2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
s_type: VkStructureType,
p_next: c_void,
per_view_position_all_components: VkBool32,
}

#[repr(C)]
pub struct VkInputAttachmentAspectReference {
subpass: u32,
input_attachment_index: u32,
aspect_mask: VkImageAspectFlags,
}

#[repr(C)]
pub struct VkRenderPassInputAttachmentAspectCreateInfo {
s_type: VkStructureType,
p_next: c_void,
aspect_reference_count: u32,
p_aspect_references: VkInputAttachmentAspectReference,
}

#[repr(C)]
pub struct VkPhysicalDeviceSurfaceInfo2KHR {
s_type: VkStructureType,
p_next: c_void,
surface: VkSurfaceKHR,
}

#[repr(C)]
pub struct VkSurfaceCapabilities2KHR {
s_type: VkStructureType,
p_next: c_void,
surface_capabilities: VkSurfaceCapabilitiesKHR,
}

#[repr(C)]
pub struct VkSurfaceFormat2KHR {
s_type: VkStructureType,
p_next: c_void,
surface_format: VkSurfaceFormatKHR,
}

#[repr(C)]
pub struct VkDisplayProperties2KHR {
s_type: VkStructureType,
p_next: c_void,
display_properties: VkDisplayPropertiesKHR,
}

#[repr(C)]
pub struct VkDisplayPlaneProperties2KHR {
s_type: VkStructureType,
p_next: c_void,
display_plane_properties: VkDisplayPlanePropertiesKHR,
}

#[repr(C)]
pub struct VkDisplayModeProperties2KHR {
s_type: VkStructureType,
p_next: c_void,
display_mode_properties: VkDisplayModePropertiesKHR,
}

#[repr(C)]
pub struct VkDisplayPlaneInfo2KHR {
s_type: VkStructureType,
p_next: c_void,
mode: VkDisplayModeKHR,
plane_index: u32,
}

#[repr(C)]
pub struct VkDisplayPlaneCapabilities2KHR {
s_type: VkStructureType,
p_next: c_void,
capabilities: VkDisplayPlaneCapabilitiesKHR,
}

#[repr(C)]
pub struct VkSharedPresentSurfaceCapabilitiesKHR {
s_type: VkStructureType,
p_next: c_void,
shared_present_supported_usage_flags: VkImageUsageFlags,
}

#[repr(C)]
pub struct VkPhysicalDevice16BitStorageFeatures {
s_type: VkStructureType,
p_next: c_void,
storage_buffer16_bit_access: VkBool32,
uniform_and_storage_buffer16_bit_access: VkBool32,
storage_push_constant16: VkBool32,
storage_input_output16: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubgroupProperties {
s_type: VkStructureType,
p_next: c_void,
subgroup_size: u32,
supported_stages: VkShaderStageFlags,
supported_operations: VkSubgroupFeatureFlags,
quad_operations_in_all_stages: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
s_type: VkStructureType,
p_next: c_void,
shader_subgroup_extended_types: VkBool32,
}

#[repr(C)]
pub struct VkBufferMemoryRequirementsInfo2 {
s_type: VkStructureType,
p_next: c_void,
buffer: VkBuffer,
}

#[repr(C)]
pub struct VkDeviceBufferMemoryRequirements {
s_type: VkStructureType,
p_next: c_void,
p_create_info: VkBufferCreateInfo,
}

#[repr(C)]
pub struct VkImageMemoryRequirementsInfo2 {
s_type: VkStructureType,
p_next: c_void,
image: VkImage,
}

#[repr(C)]
pub struct VkImageSparseMemoryRequirementsInfo2 {
s_type: VkStructureType,
p_next: c_void,
image: VkImage,
}

#[repr(C)]
pub struct VkDeviceImageMemoryRequirements {
s_type: VkStructureType,
p_next: c_void,
p_create_info: VkImageCreateInfo,
plane_aspect: VkImageAspectFlagBits,
}

#[repr(C)]
pub struct VkMemoryRequirements2 {
s_type: VkStructureType,
p_next: c_void,
memory_requirements: VkMemoryRequirements,
}

#[repr(C)]
pub struct VkSparseImageMemoryRequirements2 {
s_type: VkStructureType,
p_next: c_void,
memory_requirements: VkSparseImageMemoryRequirements,
}

#[repr(C)]
pub struct VkPhysicalDevicePointClippingProperties {
s_type: VkStructureType,
p_next: c_void,
point_clipping_behavior: VkPointClippingBehavior,
}

#[repr(C)]
pub struct VkMemoryDedicatedRequirements {
s_type: VkStructureType,
p_next: c_void,
prefers_dedicated_allocation: VkBool32,
requires_dedicated_allocation: VkBool32,
}

#[repr(C)]
pub struct VkMemoryDedicatedAllocateInfo {
s_type: VkStructureType,
p_next: c_void,
image: VkImage,
buffer: VkBuffer,
}

#[repr(C)]
pub struct VkImageViewUsageCreateInfo {
s_type: VkStructureType,
p_next: c_void,
usage: VkImageUsageFlags,
}

#[repr(C)]
pub struct VkImageViewSlicedCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
slice_offset: u32,
slice_count: u32,
}

#[repr(C)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfo {
s_type: VkStructureType,
p_next: c_void,
domain_origin: VkTessellationDomainOrigin,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionInfo {
s_type: VkStructureType,
p_next: c_void,
conversion: VkSamplerYcbcrConversion,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionCreateInfo {
s_type: VkStructureType,
p_next: c_void,
format: VkFormat,
ycbcr_model: VkSamplerYcbcrModelConversion,
ycbcr_range: VkSamplerYcbcrRange,
components: VkComponentMapping,
x_chroma_offset: VkChromaLocation,
y_chroma_offset: VkChromaLocation,
chroma_filter: VkFilter,
force_explicit_reconstruction: VkBool32,
}

#[repr(C)]
pub struct VkBindImagePlaneMemoryInfo {
s_type: VkStructureType,
p_next: c_void,
plane_aspect: VkImageAspectFlagBits,
}

#[repr(C)]
pub struct VkImagePlaneMemoryRequirementsInfo {
s_type: VkStructureType,
p_next: c_void,
plane_aspect: VkImageAspectFlagBits,
}

#[repr(C)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures {
s_type: VkStructureType,
p_next: c_void,
sampler_ycbcr_conversion: VkBool32,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionImageFormatProperties {
s_type: VkStructureType,
p_next: c_void,
combined_image_sampler_descriptor_count: u32,
}

#[repr(C)]
pub struct VkTextureLODGatherFormatPropertiesAMD {
s_type: VkStructureType,
p_next: c_void,
supports_texture_gather_lodbias_amd: VkBool32,
}

#[repr(C)]
pub struct VkConditionalRenderingBeginInfoEXT {
s_type: VkStructureType,
p_next: c_void,
buffer: VkBuffer,
offset: VkDeviceSize,
flags: VkConditionalRenderingFlagsEXT,
}

#[repr(C)]
pub struct VkProtectedSubmitInfo {
s_type: VkStructureType,
p_next: c_void,
protected_submit: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceProtectedMemoryFeatures {
s_type: VkStructureType,
p_next: c_void,
protected_memory: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceProtectedMemoryProperties {
s_type: VkStructureType,
p_next: c_void,
protected_no_fault: VkBool32,
}

#[repr(C)]
pub struct VkDeviceQueueInfo2 {
s_type: VkStructureType,
p_next: c_void,
flags: VkDeviceQueueCreateFlags,
queue_family_index: u32,
queue_index: u32,
}

#[repr(C)]
pub struct VkPipelineCoverageToColorStateCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineCoverageToColorStateCreateFlagsNV,
coverage_to_color_enable: VkBool32,
coverage_to_color_location: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties {
s_type: VkStructureType,
p_next: c_void,
filter_minmax_single_component_formats: VkBool32,
filter_minmax_image_component_mapping: VkBool32,
}

#[repr(C)]
pub struct VkSampleLocationEXT {
x: f32,
y: f32,
}

#[repr(C)]
pub struct VkSampleLocationsInfoEXT {
s_type: VkStructureType,
p_next: c_void,
sample_locations_per_pixel: VkSampleCountFlagBits,
sample_location_grid_size: VkExtent2D,
sample_locations_count: u32,
p_sample_locations: VkSampleLocationEXT,
}

#[repr(C)]
pub struct VkAttachmentSampleLocationsEXT {
attachment_index: u32,
sample_locations_info: VkSampleLocationsInfoEXT,
}

#[repr(C)]
pub struct VkSubpassSampleLocationsEXT {
subpass_index: u32,
sample_locations_info: VkSampleLocationsInfoEXT,
}

#[repr(C)]
pub struct VkRenderPassSampleLocationsBeginInfoEXT {
s_type: VkStructureType,
p_next: c_void,
attachment_initial_sample_locations_count: u32,
p_attachment_initial_sample_locations: VkAttachmentSampleLocationsEXT,
post_subpass_sample_locations_count: u32,
p_post_subpass_sample_locations: VkSubpassSampleLocationsEXT,
}

#[repr(C)]
pub struct VkPipelineSampleLocationsStateCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
sample_locations_enable: VkBool32,
sample_locations_info: VkSampleLocationsInfoEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
sample_location_sample_counts: VkSampleCountFlags,
max_sample_location_grid_size: VkExtent2D,
sample_location_coordinate_range: f32,
sample_location_sub_pixel_bits: u32,
variable_sample_locations: VkBool32,
}

#[repr(C)]
pub struct VkMultisamplePropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
max_sample_location_grid_size: VkExtent2D,
}

#[repr(C)]
pub struct VkSamplerReductionModeCreateInfo {
s_type: VkStructureType,
p_next: c_void,
reduction_mode: VkSamplerReductionMode,
}

#[repr(C)]
pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
advanced_blend_coherent_operations: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiDrawFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
multi_draw: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
advanced_blend_max_color_attachments: u32,
advanced_blend_independent_blend: VkBool32,
advanced_blend_non_premultiplied_src_color: VkBool32,
advanced_blend_non_premultiplied_dst_color: VkBool32,
advanced_blend_correlated_overlap: VkBool32,
advanced_blend_all_operations: VkBool32,
}

#[repr(C)]
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
src_premultiplied: VkBool32,
dst_premultiplied: VkBool32,
blend_overlap: VkBlendOverlapEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceInlineUniformBlockFeatures {
s_type: VkStructureType,
p_next: c_void,
inline_uniform_block: VkBool32,
descriptor_binding_inline_uniform_block_update_after_bind: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceInlineUniformBlockProperties {
s_type: VkStructureType,
p_next: c_void,
max_inline_uniform_block_size: u32,
max_per_stage_descriptor_inline_uniform_blocks: u32,
max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
max_descriptor_set_inline_uniform_blocks: u32,
max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
}

#[repr(C)]
pub struct VkWriteDescriptorSetInlineUniformBlock {
s_type: VkStructureType,
p_next: c_void,
data_size: u32,
p_data: c_void,
}

#[repr(C)]
pub struct VkDescriptorPoolInlineUniformBlockCreateInfo {
s_type: VkStructureType,
p_next: c_void,
max_inline_uniform_block_bindings: u32,
}

#[repr(C)]
pub struct VkPipelineCoverageModulationStateCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineCoverageModulationStateCreateFlagsNV,
coverage_modulation_mode: VkCoverageModulationModeNV,
coverage_modulation_table_enable: VkBool32,
coverage_modulation_table_count: u32,
p_coverage_modulation_table: f32,
}

#[repr(C)]
pub struct VkImageFormatListCreateInfo {
s_type: VkStructureType,
p_next: c_void,
view_format_count: u32,
p_view_formats: VkFormat,
}

#[repr(C)]
pub struct VkValidationCacheCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkValidationCacheCreateFlagsEXT,
initial_data_size: usize,
p_initial_data: c_void,
}

#[repr(C)]
pub struct VkShaderModuleValidationCacheCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
validation_cache: VkValidationCacheEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance3Properties {
s_type: VkStructureType,
p_next: c_void,
max_per_set_descriptors: u32,
max_memory_allocation_size: VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance4Features {
s_type: VkStructureType,
p_next: c_void,
maintenance4: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance4Properties {
s_type: VkStructureType,
p_next: c_void,
max_buffer_size: VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance5FeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
maintenance5: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance5PropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
early_fragment_multisample_coverage_after_sample_counting: VkBool32,
early_fragment_sample_mask_test_before_sample_counting: VkBool32,
depth_stencil_swizzle_one_support: VkBool32,
polygon_mode_point_size: VkBool32,
non_strict_single_pixel_wide_lines_use_parallelogram: VkBool32,
non_strict_wide_lines_use_parallelogram: VkBool32,
}

#[repr(C)]
pub struct VkRenderingAreaInfoKHR {
s_type: VkStructureType,
p_next: c_void,
view_mask: u32,
color_attachment_count: u32,
p_color_attachment_formats: VkFormat,
depth_attachment_format: VkFormat,
stencil_attachment_format: VkFormat,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutSupport {
s_type: VkStructureType,
p_next: c_void,
supported: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderDrawParametersFeatures {
s_type: VkStructureType,
p_next: c_void,
shader_draw_parameters: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderFloat16Int8Features {
s_type: VkStructureType,
p_next: c_void,
shader_float16: VkBool32,
shader_int8: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFloatControlsProperties {
s_type: VkStructureType,
p_next: c_void,
denorm_behavior_independence: VkShaderFloatControlsIndependence,
rounding_mode_independence: VkShaderFloatControlsIndependence,
shader_signed_zero_inf_nan_preserve_float16: VkBool32,
shader_signed_zero_inf_nan_preserve_float32: VkBool32,
shader_signed_zero_inf_nan_preserve_float64: VkBool32,
shader_denorm_preserve_float16: VkBool32,
shader_denorm_preserve_float32: VkBool32,
shader_denorm_preserve_float64: VkBool32,
shader_denorm_flush_to_zero_float16: VkBool32,
shader_denorm_flush_to_zero_float32: VkBool32,
shader_denorm_flush_to_zero_float64: VkBool32,
shader_rounding_mode_rtefloat16: VkBool32,
shader_rounding_mode_rtefloat32: VkBool32,
shader_rounding_mode_rtefloat64: VkBool32,
shader_rounding_mode_rtzfloat16: VkBool32,
shader_rounding_mode_rtzfloat32: VkBool32,
shader_rounding_mode_rtzfloat64: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceHostQueryResetFeatures {
s_type: VkStructureType,
p_next: c_void,
host_query_reset: VkBool32,
}

#[repr(C)]
pub struct VkNativeBufferUsage2ANDROID {
consumer: u64,
producer: u64,
}

#[repr(C)]
pub struct VkNativeBufferANDROID {
s_type: VkStructureType,
p_next: c_void,
handle: c_void,
stride: int,
format: int,
usage: int,
usage2: VkNativeBufferUsage2ANDROID,
}

#[repr(C)]
pub struct VkSwapchainImageCreateInfoANDROID {
s_type: VkStructureType,
p_next: c_void,
usage: VkSwapchainImageUsageFlagsANDROID,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentationPropertiesANDROID {
s_type: VkStructureType,
p_next: c_void,
shared_image: VkBool32,
}

#[repr(C)]
pub struct VkShaderResourceUsageAMD {
num_used_vgprs: u32,
num_used_sgprs: u32,
lds_size_per_local_work_group: u32,
lds_usage_size_in_bytes: usize,
scratch_mem_usage_in_bytes: usize,
}

#[repr(C)]
pub struct VkShaderStatisticsInfoAMD {
shader_stage_mask: VkShaderStageFlags,
resource_usage: VkShaderResourceUsageAMD,
num_physical_vgprs: u32,
num_physical_sgprs: u32,
num_available_vgprs: u32,
num_available_sgprs: u32,
compute_work_group_size: u32,
}

#[repr(C)]
pub struct VkDeviceQueueGlobalPriorityCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
global_priority: VkQueueGlobalPriorityKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
global_priority_query: VkBool32,
}

#[repr(C)]
pub struct VkQueueFamilyGlobalPriorityPropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
priority_count: u32,
priorities: VkQueueGlobalPriorityKHR,
}

#[repr(C)]
pub struct VkDebugUtilsObjectNameInfoEXT {
s_type: VkStructureType,
p_next: c_void,
object_type: VkObjectType,
object_handle: u64,
p_object_name: u8,
}

#[repr(C)]
pub struct VkDebugUtilsObjectTagInfoEXT {
s_type: VkStructureType,
p_next: c_void,
object_type: VkObjectType,
object_handle: u64,
tag_name: u64,
tag_size: usize,
p_tag: c_void,
}

#[repr(C)]
pub struct VkDebugUtilsLabelEXT {
s_type: VkStructureType,
p_next: c_void,
p_label_name: u8,
color: f32,
}

#[repr(C)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkDebugUtilsMessengerCreateFlagsEXT,
message_severity: VkDebugUtilsMessageSeverityFlagsEXT,
message_type: VkDebugUtilsMessageTypeFlagsEXT,
pfn_user_callback: PFN_vkDebugUtilsMessengerCallbackEXT,
p_user_data: c_void,
}

#[repr(C)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkDebugUtilsMessengerCallbackDataFlagsEXT,
p_message_id_name: u8,
message_id_number: i32,
p_message: u8,
queue_label_count: u32,
p_queue_labels: VkDebugUtilsLabelEXT,
cmd_buf_label_count: u32,
p_cmd_buf_labels: VkDebugUtilsLabelEXT,
object_count: u32,
p_objects: VkDebugUtilsObjectNameInfoEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceMemoryReportFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
device_memory_report: VkBool32,
}

#[repr(C)]
pub struct VkDeviceDeviceMemoryReportCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkDeviceMemoryReportFlagsEXT,
pfn_user_callback: PFN_vkDeviceMemoryReportCallbackEXT,
p_user_data: c_void,
}

#[repr(C)]
pub struct VkDeviceMemoryReportCallbackDataEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkDeviceMemoryReportFlagsEXT,
r#type: VkDeviceMemoryReportEventTypeEXT,
memory_object_id: u64,
size: VkDeviceSize,
object_type: VkObjectType,
object_handle: u64,
heap_index: u32,
}

#[repr(C)]
pub struct VkImportMemoryHostPointerInfoEXT {
s_type: VkStructureType,
p_next: c_void,
handle_type: VkExternalMemoryHandleTypeFlagBits,
p_host_pointer: c_void,
}

#[repr(C)]
pub struct VkMemoryHostPointerPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
memory_type_bits: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
min_imported_host_pointer_alignment: VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
primitive_overestimation_size: f32,
max_extra_primitive_overestimation_size: f32,
extra_primitive_overestimation_size_granularity: f32,
primitive_underestimation: VkBool32,
conservative_point_and_line_rasterization: VkBool32,
degenerate_triangles_rasterized: VkBool32,
degenerate_lines_rasterized: VkBool32,
fully_covered_fragment_shader_input_variable: VkBool32,
conservative_rasterization_post_depth_coverage: VkBool32,
}

#[repr(C)]
pub struct VkCalibratedTimestampInfoEXT {
s_type: VkStructureType,
p_next: c_void,
time_domain: VkTimeDomainEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderCorePropertiesAMD {
s_type: VkStructureType,
p_next: c_void,
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
pub struct VkPhysicalDeviceShaderCoreProperties2AMD {
s_type: VkStructureType,
p_next: c_void,
shader_core_features: VkShaderCorePropertiesFlagsAMD,
active_compute_unit_count: u32,
}

#[repr(C)]
pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineRasterizationConservativeStateCreateFlagsEXT,
conservative_rasterization_mode: VkConservativeRasterizationModeEXT,
extra_primitive_overestimation_size: f32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingFeatures {
s_type: VkStructureType,
p_next: c_void,
shader_input_attachment_array_dynamic_indexing: VkBool32,
shader_uniform_texel_buffer_array_dynamic_indexing: VkBool32,
shader_storage_texel_buffer_array_dynamic_indexing: VkBool32,
shader_uniform_buffer_array_non_uniform_indexing: VkBool32,
shader_sampled_image_array_non_uniform_indexing: VkBool32,
shader_storage_buffer_array_non_uniform_indexing: VkBool32,
shader_storage_image_array_non_uniform_indexing: VkBool32,
shader_input_attachment_array_non_uniform_indexing: VkBool32,
shader_uniform_texel_buffer_array_non_uniform_indexing: VkBool32,
shader_storage_texel_buffer_array_non_uniform_indexing: VkBool32,
descriptor_binding_uniform_buffer_update_after_bind: VkBool32,
descriptor_binding_sampled_image_update_after_bind: VkBool32,
descriptor_binding_storage_image_update_after_bind: VkBool32,
descriptor_binding_storage_buffer_update_after_bind: VkBool32,
descriptor_binding_uniform_texel_buffer_update_after_bind: VkBool32,
descriptor_binding_storage_texel_buffer_update_after_bind: VkBool32,
descriptor_binding_update_unused_while_pending: VkBool32,
descriptor_binding_partially_bound: VkBool32,
descriptor_binding_variable_descriptor_count: VkBool32,
runtime_descriptor_array: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingProperties {
s_type: VkStructureType,
p_next: c_void,
max_update_after_bind_descriptors_in_all_pools: u32,
shader_uniform_buffer_array_non_uniform_indexing_native: VkBool32,
shader_sampled_image_array_non_uniform_indexing_native: VkBool32,
shader_storage_buffer_array_non_uniform_indexing_native: VkBool32,
shader_storage_image_array_non_uniform_indexing_native: VkBool32,
shader_input_attachment_array_non_uniform_indexing_native: VkBool32,
robust_buffer_access_update_after_bind: VkBool32,
quad_divergent_implicit_lod: VkBool32,
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
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo {
s_type: VkStructureType,
p_next: c_void,
binding_count: u32,
p_binding_flags: VkDescriptorBindingFlags,
}

#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo {
s_type: VkStructureType,
p_next: c_void,
descriptor_set_count: u32,
p_descriptor_counts: u32,
}

#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport {
s_type: VkStructureType,
p_next: c_void,
max_variable_descriptor_count: u32,
}

#[repr(C)]
pub struct VkAttachmentDescription2 {
s_type: VkStructureType,
p_next: c_void,
flags: VkAttachmentDescriptionFlags,
format: VkFormat,
samples: VkSampleCountFlagBits,
load_op: VkAttachmentLoadOp,
store_op: VkAttachmentStoreOp,
stencil_load_op: VkAttachmentLoadOp,
stencil_store_op: VkAttachmentStoreOp,
initial_layout: VkImageLayout,
final_layout: VkImageLayout,
}

#[repr(C)]
pub struct VkAttachmentReference2 {
s_type: VkStructureType,
p_next: c_void,
attachment: u32,
layout: VkImageLayout,
aspect_mask: VkImageAspectFlags,
}

#[repr(C)]
pub struct VkSubpassDescription2 {
s_type: VkStructureType,
p_next: c_void,
flags: VkSubpassDescriptionFlags,
pipeline_bind_point: VkPipelineBindPoint,
view_mask: u32,
input_attachment_count: u32,
p_input_attachments: VkAttachmentReference2,
color_attachment_count: u32,
p_color_attachments: VkAttachmentReference2,
p_resolve_attachments: VkAttachmentReference2,
p_depth_stencil_attachment: VkAttachmentReference2,
preserve_attachment_count: u32,
p_preserve_attachments: u32,
}

#[repr(C)]
pub struct VkSubpassDependency2 {
s_type: VkStructureType,
p_next: c_void,
src_subpass: u32,
dst_subpass: u32,
src_stage_mask: VkPipelineStageFlags,
dst_stage_mask: VkPipelineStageFlags,
src_access_mask: VkAccessFlags,
dst_access_mask: VkAccessFlags,
dependency_flags: VkDependencyFlags,
view_offset: i32,
}

#[repr(C)]
pub struct VkRenderPassCreateInfo2 {
s_type: VkStructureType,
p_next: c_void,
flags: VkRenderPassCreateFlags,
attachment_count: u32,
p_attachments: VkAttachmentDescription2,
subpass_count: u32,
p_subpasses: VkSubpassDescription2,
dependency_count: u32,
p_dependencies: VkSubpassDependency2,
correlated_view_mask_count: u32,
p_correlated_view_masks: u32,
}

#[repr(C)]
pub struct VkSubpassBeginInfo {
s_type: VkStructureType,
p_next: c_void,
contents: VkSubpassContents,
}

#[repr(C)]
pub struct VkSubpassEndInfo {
s_type: VkStructureType,
p_next: c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreFeatures {
s_type: VkStructureType,
p_next: c_void,
timeline_semaphore: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreProperties {
s_type: VkStructureType,
p_next: c_void,
max_timeline_semaphore_value_difference: u64,
}

#[repr(C)]
pub struct VkSemaphoreTypeCreateInfo {
s_type: VkStructureType,
p_next: c_void,
semaphore_type: VkSemaphoreType,
initial_value: u64,
}

#[repr(C)]
pub struct VkTimelineSemaphoreSubmitInfo {
s_type: VkStructureType,
p_next: c_void,
wait_semaphore_value_count: u32,
p_wait_semaphore_values: u64,
signal_semaphore_value_count: u32,
p_signal_semaphore_values: u64,
}

#[repr(C)]
pub struct VkSemaphoreWaitInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkSemaphoreWaitFlags,
semaphore_count: u32,
p_semaphores: VkSemaphore,
p_values: u64,
}

#[repr(C)]
pub struct VkSemaphoreSignalInfo {
s_type: VkStructureType,
p_next: c_void,
semaphore: VkSemaphore,
value: u64,
}

#[repr(C)]
pub struct VkVertexInputBindingDivisorDescriptionEXT {
binding: u32,
divisor: u32,
}

#[repr(C)]
pub struct VkPipelineVertexInputDivisorStateCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
vertex_binding_divisor_count: u32,
p_vertex_binding_divisors: VkVertexInputBindingDivisorDescriptionEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
max_vertex_attrib_divisor: u32,
}

#[repr(C)]
pub struct VkPhysicalDevicePCIBusInfoPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
pci_domain: u32,
pci_bus: u32,
pci_device: u32,
pci_function: u32,
}

#[repr(C)]
pub struct VkImportAndroidHardwareBufferInfoANDROID {
s_type: VkStructureType,
p_next: c_void,
buffer: AHardwareBuffer,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferUsageANDROID {
s_type: VkStructureType,
p_next: c_void,
android_hardware_buffer_usage: u64,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferPropertiesANDROID {
s_type: VkStructureType,
p_next: c_void,
allocation_size: VkDeviceSize,
memory_type_bits: u32,
}

#[repr(C)]
pub struct VkMemoryGetAndroidHardwareBufferInfoANDROID {
s_type: VkStructureType,
p_next: c_void,
memory: VkDeviceMemory,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferFormatPropertiesANDROID {
s_type: VkStructureType,
p_next: c_void,
format: VkFormat,
external_format: u64,
format_features: VkFormatFeatureFlags,
sampler_ycbcr_conversion_components: VkComponentMapping,
suggested_ycbcr_model: VkSamplerYcbcrModelConversion,
suggested_ycbcr_range: VkSamplerYcbcrRange,
suggested_xchroma_offset: VkChromaLocation,
suggested_ychroma_offset: VkChromaLocation,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceConditionalRenderingInfoEXT {
s_type: VkStructureType,
p_next: c_void,
conditional_rendering_enable: VkBool32,
}

#[repr(C)]
pub struct VkExternalFormatANDROID {
s_type: VkStructureType,
p_next: c_void,
external_format: u64,
}

#[repr(C)]
pub struct VkPhysicalDevice8BitStorageFeatures {
s_type: VkStructureType,
p_next: c_void,
storage_buffer8_bit_access: VkBool32,
uniform_and_storage_buffer8_bit_access: VkBool32,
storage_push_constant8: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceConditionalRenderingFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
conditional_rendering: VkBool32,
inherited_conditional_rendering: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures {
s_type: VkStructureType,
p_next: c_void,
vulkan_memory_model: VkBool32,
vulkan_memory_model_device_scope: VkBool32,
vulkan_memory_model_availability_visibility_chains: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicInt64Features {
s_type: VkStructureType,
p_next: c_void,
shader_buffer_int64_atomics: VkBool32,
shader_shared_int64_atomics: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicFloatFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
shader_buffer_float32_atomics: VkBool32,
shader_buffer_float32_atomic_add: VkBool32,
shader_buffer_float64_atomics: VkBool32,
shader_buffer_float64_atomic_add: VkBool32,
shader_shared_float32_atomics: VkBool32,
shader_shared_float32_atomic_add: VkBool32,
shader_shared_float64_atomics: VkBool32,
shader_shared_float64_atomic_add: VkBool32,
shader_image_float32_atomics: VkBool32,
shader_image_float32_atomic_add: VkBool32,
sparse_image_float32_atomics: VkBool32,
sparse_image_float32_atomic_add: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
shader_buffer_float16_atomics: VkBool32,
shader_buffer_float16_atomic_add: VkBool32,
shader_buffer_float16_atomic_min_max: VkBool32,
shader_buffer_float32_atomic_min_max: VkBool32,
shader_buffer_float64_atomic_min_max: VkBool32,
shader_shared_float16_atomics: VkBool32,
shader_shared_float16_atomic_add: VkBool32,
shader_shared_float16_atomic_min_max: VkBool32,
shader_shared_float32_atomic_min_max: VkBool32,
shader_shared_float64_atomic_min_max: VkBool32,
shader_image_float32_atomic_min_max: VkBool32,
sparse_image_float32_atomic_min_max: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
vertex_attribute_instance_rate_divisor: VkBool32,
vertex_attribute_instance_rate_zero_divisor: VkBool32,
}

#[repr(C)]
pub struct VkQueueFamilyCheckpointPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
checkpoint_execution_stage_mask: VkPipelineStageFlags,
}

#[repr(C)]
pub struct VkCheckpointDataNV {
s_type: VkStructureType,
p_next: c_void,
stage: VkPipelineStageFlagBits,
p_checkpoint_marker: c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthStencilResolveProperties {
s_type: VkStructureType,
p_next: c_void,
supported_depth_resolve_modes: VkResolveModeFlags,
supported_stencil_resolve_modes: VkResolveModeFlags,
independent_resolve_none: VkBool32,
independent_resolve: VkBool32,
}

#[repr(C)]
pub struct VkSubpassDescriptionDepthStencilResolve {
s_type: VkStructureType,
p_next: c_void,
depth_resolve_mode: VkResolveModeFlagBits,
stencil_resolve_mode: VkResolveModeFlagBits,
p_depth_stencil_resolve_attachment: VkAttachmentReference2,
}

#[repr(C)]
pub struct VkImageViewASTCDecodeModeEXT {
s_type: VkStructureType,
p_next: c_void,
decode_mode: VkFormat,
}

#[repr(C)]
pub struct VkPhysicalDeviceASTCDecodeFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
decode_mode_shared_exponent: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTransformFeedbackFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
transform_feedback: VkBool32,
geometry_streams: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTransformFeedbackPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
max_transform_feedback_streams: u32,
max_transform_feedback_buffers: u32,
max_transform_feedback_buffer_size: VkDeviceSize,
max_transform_feedback_stream_data_size: u32,
max_transform_feedback_buffer_data_size: u32,
max_transform_feedback_buffer_data_stride: u32,
transform_feedback_queries: VkBool32,
transform_feedback_streams_lines_triangles: VkBool32,
transform_feedback_rasterization_stream_select: VkBool32,
transform_feedback_draw: VkBool32,
}

#[repr(C)]
pub struct VkPipelineRasterizationStateStreamCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineRasterizationStateStreamCreateFlagsEXT,
rasterization_stream: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
representative_fragment_test: VkBool32,
}

#[repr(C)]
pub struct VkPipelineRepresentativeFragmentTestStateCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
representative_fragment_test_enable: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExclusiveScissorFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
exclusive_scissor: VkBool32,
}

#[repr(C)]
pub struct VkPipelineViewportExclusiveScissorStateCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
exclusive_scissor_count: u32,
p_exclusive_scissors: VkRect2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceCornerSampledImageFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
corner_sampled_image: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceComputeShaderDerivativesFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
compute_derivative_group_quads: VkBool32,
compute_derivative_group_linear: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderImageFootprintFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
image_footprint: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
dedicated_allocation_image_aliasing: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
indirect_copy: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
supported_queues: VkQueueFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryDecompressionFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
memory_decompression: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryDecompressionPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
decompression_methods: VkMemoryDecompressionMethodFlagsNV,
max_decompression_indirect_count: u64,
}

#[repr(C)]
pub struct VkShadingRatePaletteNV {
shading_rate_palette_entry_count: u32,
p_shading_rate_palette_entries: VkShadingRatePaletteEntryNV,
}

#[repr(C)]
pub struct VkPipelineViewportShadingRateImageStateCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
shading_rate_image_enable: VkBool32,
viewport_count: u32,
p_shading_rate_palettes: VkShadingRatePaletteNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceShadingRateImageFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
shading_rate_image: VkBool32,
shading_rate_coarse_sample_order: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShadingRateImagePropertiesNV {
s_type: VkStructureType,
p_next: c_void,
shading_rate_texel_size: VkExtent2D,
shading_rate_palette_size: u32,
shading_rate_max_coarse_samples: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceInvocationMaskFeaturesHUAWEI {
s_type: VkStructureType,
p_next: c_void,
invocation_mask: VkBool32,
}

#[repr(C)]
pub struct VkCoarseSampleLocationNV {
pixel_x: u32,
pixel_y: u32,
sample: u32,
}

#[repr(C)]
pub struct VkCoarseSampleOrderCustomNV {
shading_rate: VkShadingRatePaletteEntryNV,
sample_count: u32,
sample_location_count: u32,
p_sample_locations: VkCoarseSampleLocationNV,
}

#[repr(C)]
pub struct VkPipelineViewportCoarseSampleOrderStateCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
sample_order_type: VkCoarseSampleOrderTypeNV,
custom_sample_order_count: u32,
p_custom_sample_orders: VkCoarseSampleOrderCustomNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
task_shader: VkBool32,
mesh_shader: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
max_draw_mesh_tasks_count: u32,
max_task_work_group_invocations: u32,
max_task_work_group_size: u32,
max_task_total_memory_size: u32,
max_task_output_count: u32,
max_mesh_work_group_invocations: u32,
max_mesh_work_group_size: u32,
max_mesh_total_memory_size: u32,
max_mesh_output_vertices: u32,
max_mesh_output_primitives: u32,
max_mesh_multiview_view_count: u32,
mesh_output_per_vertex_granularity: u32,
mesh_output_per_primitive_granularity: u32,
}

#[repr(C)]
pub struct VkDrawMeshTasksIndirectCommandNV {
task_count: u32,
first_task: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
task_shader: VkBool32,
mesh_shader: VkBool32,
multiview_mesh_shader: VkBool32,
primitive_fragment_shading_rate_mesh_shader: VkBool32,
mesh_shader_queries: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
max_task_work_group_total_count: u32,
max_task_work_group_count: u32,
max_task_work_group_invocations: u32,
max_task_work_group_size: u32,
max_task_payload_size: u32,
max_task_shared_memory_size: u32,
max_task_payload_and_shared_memory_size: u32,
max_mesh_work_group_total_count: u32,
max_mesh_work_group_count: u32,
max_mesh_work_group_invocations: u32,
max_mesh_work_group_size: u32,
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
prefers_local_invocation_vertex_output: VkBool32,
prefers_local_invocation_primitive_output: VkBool32,
prefers_compact_vertex_output: VkBool32,
prefers_compact_primitive_output: VkBool32,
}

#[repr(C)]
pub struct VkDrawMeshTasksIndirectCommandEXT {
group_count_x: u32,
group_count_y: u32,
group_count_z: u32,
}

#[repr(C)]
pub struct VkRayTracingShaderGroupCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
r#type: VkRayTracingShaderGroupTypeKHR,
general_shader: u32,
closest_hit_shader: u32,
any_hit_shader: u32,
intersection_shader: u32,
}

#[repr(C)]
pub struct VkRayTracingShaderGroupCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
r#type: VkRayTracingShaderGroupTypeKHR,
general_shader: u32,
closest_hit_shader: u32,
any_hit_shader: u32,
intersection_shader: u32,
p_shader_group_capture_replay_handle: c_void,
}

#[repr(C)]
pub struct VkRayTracingPipelineCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineCreateFlags,
stage_count: u32,
p_stages: VkPipelineShaderStageCreateInfo,
group_count: u32,
p_groups: VkRayTracingShaderGroupCreateInfoNV,
max_recursion_depth: u32,
layout: VkPipelineLayout,
base_pipeline_handle: VkPipeline,
base_pipeline_index: i32,
}

#[repr(C)]
pub struct VkRayTracingPipelineCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineCreateFlags,
stage_count: u32,
p_stages: VkPipelineShaderStageCreateInfo,
group_count: u32,
p_groups: VkRayTracingShaderGroupCreateInfoKHR,
max_pipeline_ray_recursion_depth: u32,
p_library_info: VkPipelineLibraryCreateInfoKHR,
p_library_interface: VkRayTracingPipelineInterfaceCreateInfoKHR,
p_dynamic_state: VkPipelineDynamicStateCreateInfo,
layout: VkPipelineLayout,
base_pipeline_handle: VkPipeline,
base_pipeline_index: i32,
}

#[repr(C)]
pub struct VkGeometryTrianglesNV {
s_type: VkStructureType,
p_next: c_void,
vertex_data: VkBuffer,
vertex_offset: VkDeviceSize,
vertex_count: u32,
vertex_stride: VkDeviceSize,
vertex_format: VkFormat,
index_data: VkBuffer,
index_offset: VkDeviceSize,
index_count: u32,
index_type: VkIndexType,
transform_data: VkBuffer,
transform_offset: VkDeviceSize,
}

#[repr(C)]
pub struct VkGeometryAABBNV {
s_type: VkStructureType,
p_next: c_void,
aabb_data: VkBuffer,
num_aabbs: u32,
stride: u32,
offset: VkDeviceSize,
}

#[repr(C)]
pub struct VkGeometryDataNV {
triangles: VkGeometryTrianglesNV,
aabbs: VkGeometryAABBNV,
}

#[repr(C)]
pub struct VkGeometryNV {
s_type: VkStructureType,
p_next: c_void,
geometry_type: VkGeometryTypeKHR,
geometry: VkGeometryDataNV,
flags: VkGeometryFlagsKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureInfoNV {
s_type: VkStructureType,
p_next: c_void,
r#type: VkAccelerationStructureTypeNV,
flags: VkBuildAccelerationStructureFlagsNV,
instance_count: u32,
geometry_count: u32,
p_geometries: VkGeometryNV,
}

#[repr(C)]
pub struct VkAccelerationStructureCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
compacted_size: VkDeviceSize,
info: VkAccelerationStructureInfoNV,
}

#[repr(C)]
pub struct VkBindAccelerationStructureMemoryInfoNV {
s_type: VkStructureType,
p_next: c_void,
acceleration_structure: VkAccelerationStructureNV,
memory: VkDeviceMemory,
memory_offset: VkDeviceSize,
device_index_count: u32,
p_device_indices: u32,
}

#[repr(C)]
pub struct VkWriteDescriptorSetAccelerationStructureKHR {
s_type: VkStructureType,
p_next: c_void,
acceleration_structure_count: u32,
p_acceleration_structures: VkAccelerationStructureKHR,
}

#[repr(C)]
pub struct VkWriteDescriptorSetAccelerationStructureNV {
s_type: VkStructureType,
p_next: c_void,
acceleration_structure_count: u32,
p_acceleration_structures: VkAccelerationStructureNV,
}

#[repr(C)]
pub struct VkAccelerationStructureMemoryRequirementsInfoNV {
s_type: VkStructureType,
p_next: c_void,
r#type: VkAccelerationStructureMemoryRequirementsTypeNV,
acceleration_structure: VkAccelerationStructureNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceAccelerationStructureFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
acceleration_structure: VkBool32,
acceleration_structure_capture_replay: VkBool32,
acceleration_structure_indirect_build: VkBool32,
acceleration_structure_host_commands: VkBool32,
descriptor_binding_acceleration_structure_update_after_bind: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
ray_tracing_pipeline: VkBool32,
ray_tracing_pipeline_shader_group_handle_capture_replay: VkBool32,
ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: VkBool32,
ray_tracing_pipeline_trace_rays_indirect: VkBool32,
ray_traversal_primitive_culling: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayQueryFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
ray_query: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceAccelerationStructurePropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
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
pub struct VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
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
pub struct VkPhysicalDeviceRayTracingPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
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
pub struct VkStridedDeviceAddressRegionKHR {
device_address: VkDeviceAddress,
stride: VkDeviceSize,
size: VkDeviceSize,
}

#[repr(C)]
pub struct VkTraceRaysIndirectCommandKHR {
width: u32,
height: u32,
depth: u32,
}

#[repr(C)]
pub struct VkTraceRaysIndirectCommand2KHR {
raygen_shader_record_address: VkDeviceAddress,
raygen_shader_record_size: VkDeviceSize,
miss_shader_binding_table_address: VkDeviceAddress,
miss_shader_binding_table_size: VkDeviceSize,
miss_shader_binding_table_stride: VkDeviceSize,
hit_shader_binding_table_address: VkDeviceAddress,
hit_shader_binding_table_size: VkDeviceSize,
hit_shader_binding_table_stride: VkDeviceSize,
callable_shader_binding_table_address: VkDeviceAddress,
callable_shader_binding_table_size: VkDeviceSize,
callable_shader_binding_table_stride: VkDeviceSize,
width: u32,
height: u32,
depth: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
ray_tracing_maintenance1: VkBool32,
ray_tracing_pipeline_trace_rays_indirect2: VkBool32,
}

#[repr(C)]
pub struct VkDrmFormatModifierPropertiesListEXT {
s_type: VkStructureType,
p_next: c_void,
drm_format_modifier_count: u32,
p_drm_format_modifier_properties: VkDrmFormatModifierPropertiesEXT,
}

#[repr(C)]
pub struct VkDrmFormatModifierPropertiesEXT {
drm_format_modifier: u64,
drm_format_modifier_plane_count: u32,
drm_format_modifier_tiling_features: VkFormatFeatureFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageDrmFormatModifierInfoEXT {
s_type: VkStructureType,
p_next: c_void,
drm_format_modifier: u64,
sharing_mode: VkSharingMode,
queue_family_index_count: u32,
p_queue_family_indices: u32,
}

#[repr(C)]
pub struct VkImageDrmFormatModifierListCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
drm_format_modifier_count: u32,
p_drm_format_modifiers: u64,
}

#[repr(C)]
pub struct VkImageDrmFormatModifierExplicitCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
drm_format_modifier: u64,
drm_format_modifier_plane_count: u32,
p_plane_layouts: VkSubresourceLayout,
}

#[repr(C)]
pub struct VkImageDrmFormatModifierPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
drm_format_modifier: u64,
}

#[repr(C)]
pub struct VkImageStencilUsageCreateInfo {
s_type: VkStructureType,
p_next: c_void,
stencil_usage: VkImageUsageFlags,
}

#[repr(C)]
pub struct VkDeviceMemoryOverallocationCreateInfoAMD {
s_type: VkStructureType,
p_next: c_void,
overallocation_behavior: VkMemoryOverallocationBehaviorAMD,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
fragment_density_map: VkBool32,
fragment_density_map_dynamic: VkBool32,
fragment_density_map_non_subsampled_images: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMap2FeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
fragment_density_map_deferred: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
s_type: VkStructureType,
p_next: c_void,
fragment_density_map_offset: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
min_fragment_density_texel_size: VkExtent2D,
max_fragment_density_texel_size: VkExtent2D,
fragment_density_invocations: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMap2PropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
subsampled_loads: VkBool32,
subsampled_coarse_reconstruction_early_access: VkBool32,
max_subsampled_array_layers: u32,
max_descriptor_set_subsampled_samplers: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
s_type: VkStructureType,
p_next: c_void,
fragment_density_offset_granularity: VkExtent2D,
}

#[repr(C)]
pub struct VkRenderPassFragmentDensityMapCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
fragment_density_map_attachment: VkAttachmentReference,
}

#[repr(C)]
pub struct VkSubpassFragmentDensityMapOffsetEndInfoQCOM {
s_type: VkStructureType,
p_next: c_void,
fragment_density_offset_count: u32,
p_fragment_density_offsets: VkOffset2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceScalarBlockLayoutFeatures {
s_type: VkStructureType,
p_next: c_void,
scalar_block_layout: VkBool32,
}

#[repr(C)]
pub struct VkSurfaceProtectedCapabilitiesKHR {
s_type: VkStructureType,
p_next: c_void,
supports_protected: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures {
s_type: VkStructureType,
p_next: c_void,
uniform_buffer_standard_layout: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthClipEnableFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
depth_clip_enable: VkBool32,
}

#[repr(C)]
pub struct VkPipelineRasterizationDepthClipStateCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineRasterizationDepthClipStateCreateFlagsEXT,
depth_clip_enable: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryBudgetPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
heap_budget: VkDeviceSize,
heap_usage: VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryPriorityFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
memory_priority: VkBool32,
}

#[repr(C)]
pub struct VkMemoryPriorityAllocateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
priority: f32,
}

#[repr(C)]
pub struct VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
pageable_device_local_memory: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures {
s_type: VkStructureType,
p_next: c_void,
buffer_device_address: VkBool32,
buffer_device_address_capture_replay: VkBool32,
buffer_device_address_multi_device: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
buffer_device_address: VkBool32,
buffer_device_address_capture_replay: VkBool32,
buffer_device_address_multi_device: VkBool32,
}

#[repr(C)]
pub struct VkBufferDeviceAddressInfo {
s_type: VkStructureType,
p_next: c_void,
buffer: VkBuffer,
}

#[repr(C)]
pub struct VkBufferOpaqueCaptureAddressCreateInfo {
s_type: VkStructureType,
p_next: c_void,
opaque_capture_address: u64,
}

#[repr(C)]
pub struct VkBufferDeviceAddressCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
device_address: VkDeviceAddress,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageViewImageFormatInfoEXT {
s_type: VkStructureType,
p_next: c_void,
image_view_type: VkImageViewType,
}

#[repr(C)]
pub struct VkFilterCubicImageViewImageFormatPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
filter_cubic: VkBool32,
filter_cubic_minmax: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImagelessFramebufferFeatures {
s_type: VkStructureType,
p_next: c_void,
imageless_framebuffer: VkBool32,
}

#[repr(C)]
pub struct VkFramebufferAttachmentsCreateInfo {
s_type: VkStructureType,
p_next: c_void,
attachment_image_info_count: u32,
p_attachment_image_infos: VkFramebufferAttachmentImageInfo,
}

#[repr(C)]
pub struct VkFramebufferAttachmentImageInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkImageCreateFlags,
usage: VkImageUsageFlags,
width: u32,
height: u32,
layer_count: u32,
view_format_count: u32,
p_view_formats: VkFormat,
}

#[repr(C)]
pub struct VkRenderPassAttachmentBeginInfo {
s_type: VkStructureType,
p_next: c_void,
attachment_count: u32,
p_attachments: VkImageView,
}

#[repr(C)]
pub struct VkPhysicalDeviceTextureCompressionASTCHDRFeatures {
s_type: VkStructureType,
p_next: c_void,
texture_compression_astc_hdr: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
cooperative_matrix: VkBool32,
cooperative_matrix_robust_buffer_access: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
cooperative_matrix_supported_stages: VkShaderStageFlags,
}

#[repr(C)]
pub struct VkCooperativeMatrixPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
_msize: u32,
_nsize: u32,
_ksize: u32,
_atype: VkComponentTypeNV,
_btype: VkComponentTypeNV,
_ctype: VkComponentTypeNV,
_dtype: VkComponentTypeNV,
scope: VkScopeNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceYcbcrImageArraysFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
ycbcr_image_arrays: VkBool32,
}

#[repr(C)]
pub struct VkImageViewHandleInfoNVX {
s_type: VkStructureType,
p_next: c_void,
image_view: VkImageView,
descriptor_type: VkDescriptorType,
sampler: VkSampler,
}

#[repr(C)]
pub struct VkImageViewAddressPropertiesNVX {
s_type: VkStructureType,
p_next: c_void,
device_address: VkDeviceAddress,
size: VkDeviceSize,
}

#[repr(C)]
pub struct VkPresentFrameTokenGGP {
s_type: VkStructureType,
p_next: c_void,
frame_token: GgpFrameToken,
}

#[repr(C)]
pub struct VkPipelineCreationFeedback {
flags: VkPipelineCreationFeedbackFlags,
duration: u64,
}

#[repr(C)]
pub struct VkPipelineCreationFeedbackCreateInfo {
s_type: VkStructureType,
p_next: c_void,
p_pipeline_creation_feedback: VkPipelineCreationFeedback,
pipeline_stage_creation_feedback_count: u32,
p_pipeline_stage_creation_feedbacks: VkPipelineCreationFeedback,
}

#[repr(C)]
pub struct VkSurfaceFullScreenExclusiveInfoEXT {
s_type: VkStructureType,
p_next: c_void,
full_screen_exclusive: VkFullScreenExclusiveEXT,
}

#[repr(C)]
pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
s_type: VkStructureType,
p_next: c_void,
hmonitor: HMONITOR,
}

#[repr(C)]
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
s_type: VkStructureType,
p_next: c_void,
full_screen_exclusive_supported: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentBarrierFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
present_barrier: VkBool32,
}

#[repr(C)]
pub struct VkSurfaceCapabilitiesPresentBarrierNV {
s_type: VkStructureType,
p_next: c_void,
present_barrier_supported: VkBool32,
}

#[repr(C)]
pub struct VkSwapchainPresentBarrierCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
present_barrier_enable: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePerformanceQueryFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
performance_counter_query_pools: VkBool32,
performance_counter_multiple_query_pools: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePerformanceQueryPropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
allow_command_buffer_query_copies: VkBool32,
}

#[repr(C)]
pub struct VkPerformanceCounterKHR {
s_type: VkStructureType,
p_next: c_void,
unit: VkPerformanceCounterUnitKHR,
scope: VkPerformanceCounterScopeKHR,
storage: VkPerformanceCounterStorageKHR,
uuid: u8,
}

#[repr(C)]
pub struct VkPerformanceCounterDescriptionKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkPerformanceCounterDescriptionFlagsKHR,
name: u8,
category: u8,
description: u8,
}

#[repr(C)]
pub struct VkQueryPoolPerformanceCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
queue_family_index: u32,
counter_index_count: u32,
p_counter_indices: u32,
}

#[repr(C)]
pub struct VkAcquireProfilingLockInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkAcquireProfilingLockFlagsKHR,
timeout: u64,
}

#[repr(C)]
pub struct VkPerformanceQuerySubmitInfoKHR {
s_type: VkStructureType,
p_next: c_void,
counter_pass_index: u32,
}

#[repr(C)]
pub struct VkPerformanceQueryReservationInfoKHR {
s_type: VkStructureType,
p_next: c_void,
max_performance_queries_per_pool: u32,
}

#[repr(C)]
pub struct VkHeadlessSurfaceCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkHeadlessSurfaceCreateFlagsEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceCoverageReductionModeFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
coverage_reduction_mode: VkBool32,
}

#[repr(C)]
pub struct VkPipelineCoverageReductionStateCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineCoverageReductionStateCreateFlagsNV,
coverage_reduction_mode: VkCoverageReductionModeNV,
}

#[repr(C)]
pub struct VkFramebufferMixedSamplesCombinationNV {
s_type: VkStructureType,
p_next: c_void,
coverage_reduction_mode: VkCoverageReductionModeNV,
rasterization_samples: VkSampleCountFlagBits,
depth_stencil_samples: VkSampleCountFlags,
color_samples: VkSampleCountFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
s_type: VkStructureType,
p_next: c_void,
shader_integer_functions2: VkBool32,
}

#[repr(C)]
pub struct VkPerformanceValueINTEL {
r#type: VkPerformanceValueTypeINTEL,
data: VkPerformanceValueDataINTEL,
}

#[repr(C)]
pub struct VkInitializePerformanceApiInfoINTEL {
s_type: VkStructureType,
p_next: c_void,
p_user_data: c_void,
}

#[repr(C)]
pub struct VkQueryPoolPerformanceQueryCreateInfoINTEL {
s_type: VkStructureType,
p_next: c_void,
performance_counters_sampling: VkQueryPoolSamplingModeINTEL,
}

#[repr(C)]
pub struct VkPerformanceMarkerInfoINTEL {
s_type: VkStructureType,
p_next: c_void,
marker: u64,
}

#[repr(C)]
pub struct VkPerformanceStreamMarkerInfoINTEL {
s_type: VkStructureType,
p_next: c_void,
marker: u32,
}

#[repr(C)]
pub struct VkPerformanceOverrideInfoINTEL {
s_type: VkStructureType,
p_next: c_void,
r#type: VkPerformanceOverrideTypeINTEL,
enable: VkBool32,
parameter: u64,
}

#[repr(C)]
pub struct VkPerformanceConfigurationAcquireInfoINTEL {
s_type: VkStructureType,
p_next: c_void,
r#type: VkPerformanceConfigurationTypeINTEL,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderClockFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
shader_subgroup_clock: VkBool32,
shader_device_clock: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceIndexTypeUint8FeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
index_type_uint8: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSMBuiltinsPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
shader_smcount: u32,
shader_warps_per_sm: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSMBuiltinsFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
shader_smbuiltins: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
fragment_shader_sample_interlock: VkBool32,
fragment_shader_pixel_interlock: VkBool32,
fragment_shader_shading_rate_interlock: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures {
s_type: VkStructureType,
p_next: c_void,
separate_depth_stencil_layouts: VkBool32,
}

#[repr(C)]
pub struct VkAttachmentReferenceStencilLayout {
s_type: VkStructureType,
p_next: c_void,
stencil_layout: VkImageLayout,
}

#[repr(C)]
pub struct VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
primitive_topology_list_restart: VkBool32,
primitive_topology_patch_list_restart: VkBool32,
}

#[repr(C)]
pub struct VkAttachmentDescriptionStencilLayout {
s_type: VkStructureType,
p_next: c_void,
stencil_initial_layout: VkImageLayout,
stencil_final_layout: VkImageLayout,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
pipeline_executable_info: VkBool32,
}

#[repr(C)]
pub struct VkPipelineInfoKHR {
s_type: VkStructureType,
p_next: c_void,
pipeline: VkPipeline,
}

#[repr(C)]
pub struct VkPipelineExecutablePropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
stages: VkShaderStageFlags,
name: u8,
description: u8,
subgroup_size: u32,
}

#[repr(C)]
pub struct VkPipelineExecutableInfoKHR {
s_type: VkStructureType,
p_next: c_void,
pipeline: VkPipeline,
executable_index: u32,
}

#[repr(C)]
pub struct VkPipelineExecutableStatisticKHR {
s_type: VkStructureType,
p_next: c_void,
name: u8,
description: u8,
format: VkPipelineExecutableStatisticFormatKHR,
value: VkPipelineExecutableStatisticValueKHR,
}

#[repr(C)]
pub struct VkPipelineExecutableInternalRepresentationKHR {
s_type: VkStructureType,
p_next: c_void,
name: u8,
description: u8,
is_text: VkBool32,
data_size: usize,
p_data: c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures {
s_type: VkStructureType,
p_next: c_void,
shader_demote_to_helper_invocation: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
texel_buffer_alignment: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTexelBufferAlignmentProperties {
s_type: VkStructureType,
p_next: c_void,
storage_texel_buffer_offset_alignment_bytes: VkDeviceSize,
storage_texel_buffer_offset_single_texel_alignment: VkBool32,
uniform_texel_buffer_offset_alignment_bytes: VkDeviceSize,
uniform_texel_buffer_offset_single_texel_alignment: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubgroupSizeControlFeatures {
s_type: VkStructureType,
p_next: c_void,
subgroup_size_control: VkBool32,
compute_full_subgroups: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubgroupSizeControlProperties {
s_type: VkStructureType,
p_next: c_void,
min_subgroup_size: u32,
max_subgroup_size: u32,
max_compute_workgroup_subgroups: u32,
required_subgroup_size_stages: VkShaderStageFlags,
}

#[repr(C)]
pub struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
s_type: VkStructureType,
p_next: c_void,
required_subgroup_size: u32,
}

#[repr(C)]
pub struct VkSubpassShadingPipelineCreateInfoHUAWEI {
s_type: VkStructureType,
p_next: c_void,
render_pass: VkRenderPass,
subpass: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubpassShadingPropertiesHUAWEI {
s_type: VkStructureType,
p_next: c_void,
max_subpass_shading_workgroup_size_aspect_ratio: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI {
s_type: VkStructureType,
p_next: c_void,
max_work_group_count: u32,
max_work_group_size: u32,
max_output_cluster_count: u32,
indirect_buffer_offset_alignment: VkDeviceSize,
}

#[repr(C)]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfo {
s_type: VkStructureType,
p_next: c_void,
opaque_capture_address: u64,
}

#[repr(C)]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfo {
s_type: VkStructureType,
p_next: c_void,
memory: VkDeviceMemory,
}

#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
rectangular_lines: VkBool32,
bresenham_lines: VkBool32,
smooth_lines: VkBool32,
stippled_rectangular_lines: VkBool32,
stippled_bresenham_lines: VkBool32,
stippled_smooth_lines: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
line_sub_pixel_precision_bits: u32,
}

#[repr(C)]
pub struct VkPipelineRasterizationLineStateCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
line_rasterization_mode: VkLineRasterizationModeEXT,
stippled_line_enable: VkBool32,
line_stipple_factor: u32,
line_stipple_pattern: u16,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineCreationCacheControlFeatures {
s_type: VkStructureType,
p_next: c_void,
pipeline_creation_cache_control: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan11Features {
s_type: VkStructureType,
p_next: c_void,
storage_buffer16_bit_access: VkBool32,
uniform_and_storage_buffer16_bit_access: VkBool32,
storage_push_constant16: VkBool32,
storage_input_output16: VkBool32,
multiview: VkBool32,
multiview_geometry_shader: VkBool32,
multiview_tessellation_shader: VkBool32,
variable_pointers_storage_buffer: VkBool32,
variable_pointers: VkBool32,
protected_memory: VkBool32,
sampler_ycbcr_conversion: VkBool32,
shader_draw_parameters: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan11Properties {
s_type: VkStructureType,
p_next: c_void,
device_uuid: u8,
driver_uuid: u8,
device_luid: u8,
device_node_mask: u32,
device_luidvalid: VkBool32,
subgroup_size: u32,
subgroup_supported_stages: VkShaderStageFlags,
subgroup_supported_operations: VkSubgroupFeatureFlags,
subgroup_quad_operations_in_all_stages: VkBool32,
point_clipping_behavior: VkPointClippingBehavior,
max_multiview_view_count: u32,
max_multiview_instance_index: u32,
protected_no_fault: VkBool32,
max_per_set_descriptors: u32,
max_memory_allocation_size: VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan12Features {
s_type: VkStructureType,
p_next: c_void,
sampler_mirror_clamp_to_edge: VkBool32,
draw_indirect_count: VkBool32,
storage_buffer8_bit_access: VkBool32,
uniform_and_storage_buffer8_bit_access: VkBool32,
storage_push_constant8: VkBool32,
shader_buffer_int64_atomics: VkBool32,
shader_shared_int64_atomics: VkBool32,
shader_float16: VkBool32,
shader_int8: VkBool32,
descriptor_indexing: VkBool32,
shader_input_attachment_array_dynamic_indexing: VkBool32,
shader_uniform_texel_buffer_array_dynamic_indexing: VkBool32,
shader_storage_texel_buffer_array_dynamic_indexing: VkBool32,
shader_uniform_buffer_array_non_uniform_indexing: VkBool32,
shader_sampled_image_array_non_uniform_indexing: VkBool32,
shader_storage_buffer_array_non_uniform_indexing: VkBool32,
shader_storage_image_array_non_uniform_indexing: VkBool32,
shader_input_attachment_array_non_uniform_indexing: VkBool32,
shader_uniform_texel_buffer_array_non_uniform_indexing: VkBool32,
shader_storage_texel_buffer_array_non_uniform_indexing: VkBool32,
descriptor_binding_uniform_buffer_update_after_bind: VkBool32,
descriptor_binding_sampled_image_update_after_bind: VkBool32,
descriptor_binding_storage_image_update_after_bind: VkBool32,
descriptor_binding_storage_buffer_update_after_bind: VkBool32,
descriptor_binding_uniform_texel_buffer_update_after_bind: VkBool32,
descriptor_binding_storage_texel_buffer_update_after_bind: VkBool32,
descriptor_binding_update_unused_while_pending: VkBool32,
descriptor_binding_partially_bound: VkBool32,
descriptor_binding_variable_descriptor_count: VkBool32,
runtime_descriptor_array: VkBool32,
sampler_filter_minmax: VkBool32,
scalar_block_layout: VkBool32,
imageless_framebuffer: VkBool32,
uniform_buffer_standard_layout: VkBool32,
shader_subgroup_extended_types: VkBool32,
separate_depth_stencil_layouts: VkBool32,
host_query_reset: VkBool32,
timeline_semaphore: VkBool32,
buffer_device_address: VkBool32,
buffer_device_address_capture_replay: VkBool32,
buffer_device_address_multi_device: VkBool32,
vulkan_memory_model: VkBool32,
vulkan_memory_model_device_scope: VkBool32,
vulkan_memory_model_availability_visibility_chains: VkBool32,
shader_output_viewport_index: VkBool32,
shader_output_layer: VkBool32,
subgroup_broadcast_dynamic_id: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan12Properties {
s_type: VkStructureType,
p_next: c_void,
driver_id: VkDriverId,
driver_name: u8,
driver_info: u8,
conformance_version: VkConformanceVersion,
denorm_behavior_independence: VkShaderFloatControlsIndependence,
rounding_mode_independence: VkShaderFloatControlsIndependence,
shader_signed_zero_inf_nan_preserve_float16: VkBool32,
shader_signed_zero_inf_nan_preserve_float32: VkBool32,
shader_signed_zero_inf_nan_preserve_float64: VkBool32,
shader_denorm_preserve_float16: VkBool32,
shader_denorm_preserve_float32: VkBool32,
shader_denorm_preserve_float64: VkBool32,
shader_denorm_flush_to_zero_float16: VkBool32,
shader_denorm_flush_to_zero_float32: VkBool32,
shader_denorm_flush_to_zero_float64: VkBool32,
shader_rounding_mode_rtefloat16: VkBool32,
shader_rounding_mode_rtefloat32: VkBool32,
shader_rounding_mode_rtefloat64: VkBool32,
shader_rounding_mode_rtzfloat16: VkBool32,
shader_rounding_mode_rtzfloat32: VkBool32,
shader_rounding_mode_rtzfloat64: VkBool32,
max_update_after_bind_descriptors_in_all_pools: u32,
shader_uniform_buffer_array_non_uniform_indexing_native: VkBool32,
shader_sampled_image_array_non_uniform_indexing_native: VkBool32,
shader_storage_buffer_array_non_uniform_indexing_native: VkBool32,
shader_storage_image_array_non_uniform_indexing_native: VkBool32,
shader_input_attachment_array_non_uniform_indexing_native: VkBool32,
robust_buffer_access_update_after_bind: VkBool32,
quad_divergent_implicit_lod: VkBool32,
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
supported_depth_resolve_modes: VkResolveModeFlags,
supported_stencil_resolve_modes: VkResolveModeFlags,
independent_resolve_none: VkBool32,
independent_resolve: VkBool32,
filter_minmax_single_component_formats: VkBool32,
filter_minmax_image_component_mapping: VkBool32,
max_timeline_semaphore_value_difference: u64,
framebuffer_integer_color_sample_counts: VkSampleCountFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan13Features {
s_type: VkStructureType,
p_next: c_void,
robust_image_access: VkBool32,
inline_uniform_block: VkBool32,
descriptor_binding_inline_uniform_block_update_after_bind: VkBool32,
pipeline_creation_cache_control: VkBool32,
private_data: VkBool32,
shader_demote_to_helper_invocation: VkBool32,
shader_terminate_invocation: VkBool32,
subgroup_size_control: VkBool32,
compute_full_subgroups: VkBool32,
synchronization2: VkBool32,
texture_compression_astc_hdr: VkBool32,
shader_zero_initialize_workgroup_memory: VkBool32,
dynamic_rendering: VkBool32,
shader_integer_dot_product: VkBool32,
maintenance4: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan13Properties {
s_type: VkStructureType,
p_next: c_void,
min_subgroup_size: u32,
max_subgroup_size: u32,
max_compute_workgroup_subgroups: u32,
required_subgroup_size_stages: VkShaderStageFlags,
max_inline_uniform_block_size: u32,
max_per_stage_descriptor_inline_uniform_blocks: u32,
max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
max_descriptor_set_inline_uniform_blocks: u32,
max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
max_inline_uniform_total_size: u32,
integer_dot_product8_bit_unsigned_accelerated: VkBool32,
integer_dot_product8_bit_signed_accelerated: VkBool32,
integer_dot_product8_bit_mixed_signedness_accelerated: VkBool32,
integer_dot_product4x8_bit_packed_unsigned_accelerated: VkBool32,
integer_dot_product4x8_bit_packed_signed_accelerated: VkBool32,
integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: VkBool32,
integer_dot_product16_bit_unsigned_accelerated: VkBool32,
integer_dot_product16_bit_signed_accelerated: VkBool32,
integer_dot_product16_bit_mixed_signedness_accelerated: VkBool32,
integer_dot_product32_bit_unsigned_accelerated: VkBool32,
integer_dot_product32_bit_signed_accelerated: VkBool32,
integer_dot_product32_bit_mixed_signedness_accelerated: VkBool32,
integer_dot_product64_bit_unsigned_accelerated: VkBool32,
integer_dot_product64_bit_signed_accelerated: VkBool32,
integer_dot_product64_bit_mixed_signedness_accelerated: VkBool32,
integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: VkBool32,
integer_dot_product_accumulating_saturating8_bit_signed_accelerated: VkBool32,
integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: VkBool32,
integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: VkBool32,
integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: VkBool32,
integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: VkBool32,
integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: VkBool32,
integer_dot_product_accumulating_saturating16_bit_signed_accelerated: VkBool32,
integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: VkBool32,
integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: VkBool32,
integer_dot_product_accumulating_saturating32_bit_signed_accelerated: VkBool32,
integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: VkBool32,
integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: VkBool32,
integer_dot_product_accumulating_saturating64_bit_signed_accelerated: VkBool32,
integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: VkBool32,
storage_texel_buffer_offset_alignment_bytes: VkDeviceSize,
storage_texel_buffer_offset_single_texel_alignment: VkBool32,
uniform_texel_buffer_offset_alignment_bytes: VkDeviceSize,
uniform_texel_buffer_offset_single_texel_alignment: VkBool32,
max_buffer_size: VkDeviceSize,
}

#[repr(C)]
pub struct VkPipelineCompilerControlCreateInfoAMD {
s_type: VkStructureType,
p_next: c_void,
compiler_control_flags: VkPipelineCompilerControlFlagsAMD,
}

#[repr(C)]
pub struct VkPhysicalDeviceCoherentMemoryFeaturesAMD {
s_type: VkStructureType,
p_next: c_void,
device_coherent_memory: VkBool32,
}

#[repr(C)]
pub struct VkFaultData {
s_type: VkStructureType,
p_next: c_void,
fault_level: VkFaultLevel,
fault_type: VkFaultType,
}

#[repr(C)]
pub struct VkFaultCallbackInfo {
s_type: VkStructureType,
p_next: c_void,
fault_count: u32,
p_faults: VkFaultData,
pfn_fault_callback: PFN_vkFaultCallbackFunction,
}

#[repr(C)]
pub struct VkPhysicalDeviceToolProperties {
s_type: VkStructureType,
p_next: c_void,
name: u8,
version: u8,
purposes: VkToolPurposeFlags,
description: u8,
layer: u8,
}

#[repr(C)]
pub struct VkSamplerCustomBorderColorCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
custom_border_color: VkClearColorValue,
format: VkFormat,
}

#[repr(C)]
pub struct VkPhysicalDeviceCustomBorderColorPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
max_custom_border_color_samplers: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCustomBorderColorFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
custom_border_colors: VkBool32,
custom_border_color_without_format: VkBool32,
}

#[repr(C)]
pub struct VkSamplerBorderColorComponentMappingCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
components: VkComponentMapping,
srgb: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceBorderColorSwizzleFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
border_color_swizzle: VkBool32,
border_color_swizzle_from_image: VkBool32,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryTrianglesDataKHR {
s_type: VkStructureType,
p_next: c_void,
vertex_format: VkFormat,
vertex_data: VkDeviceOrHostAddressConstKHR,
vertex_stride: VkDeviceSize,
max_vertex: u32,
index_type: VkIndexType,
index_data: VkDeviceOrHostAddressConstKHR,
transform_data: VkDeviceOrHostAddressConstKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryAabbsDataKHR {
s_type: VkStructureType,
p_next: c_void,
data: VkDeviceOrHostAddressConstKHR,
stride: VkDeviceSize,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryInstancesDataKHR {
s_type: VkStructureType,
p_next: c_void,
array_of_pointers: VkBool32,
data: VkDeviceOrHostAddressConstKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryKHR {
s_type: VkStructureType,
p_next: c_void,
geometry_type: VkGeometryTypeKHR,
geometry: VkAccelerationStructureGeometryDataKHR,
flags: VkGeometryFlagsKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureBuildGeometryInfoKHR {
s_type: VkStructureType,
p_next: c_void,
r#type: VkAccelerationStructureTypeKHR,
flags: VkBuildAccelerationStructureFlagsKHR,
mode: VkBuildAccelerationStructureModeKHR,
src_acceleration_structure: VkAccelerationStructureKHR,
dst_acceleration_structure: VkAccelerationStructureKHR,
geometry_count: u32,
p_geometries: VkAccelerationStructureGeometryKHR,
pp_geometries: VkAccelerationStructureGeometryKHR,
scratch_data: VkDeviceOrHostAddressKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureBuildRangeInfoKHR {
primitive_count: u32,
primitive_offset: u32,
first_vertex: u32,
transform_offset: u32,
}

#[repr(C)]
pub struct VkAccelerationStructureCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
create_flags: VkAccelerationStructureCreateFlagsKHR,
buffer: VkBuffer,
offset: VkDeviceSize,
size: VkDeviceSize,
r#type: VkAccelerationStructureTypeKHR,
device_address: VkDeviceAddress,
}

#[repr(C)]
pub struct VkAabbPositionsKHR {
min_x: f32,
min_y: f32,
min_z: f32,
max_x: f32,
max_y: f32,
max_z: f32,
}

#[repr(C)]
pub struct VkTransformMatrixKHR {
matrix: f32,
}

#[repr(C)]
pub struct VkAccelerationStructureInstanceKHR {
transform: VkTransformMatrixKHR,
instance_custom_index: u32,
mask: u32,
instance_shader_binding_table_record_offset: u32,
flags: VkGeometryInstanceFlagsKHR,
acceleration_structure_reference: u64,
}

#[repr(C)]
pub struct VkAccelerationStructureDeviceAddressInfoKHR {
s_type: VkStructureType,
p_next: c_void,
acceleration_structure: VkAccelerationStructureKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureVersionInfoKHR {
s_type: VkStructureType,
p_next: c_void,
p_version_data: u8,
}

#[repr(C)]
pub struct VkCopyAccelerationStructureInfoKHR {
s_type: VkStructureType,
p_next: c_void,
src: VkAccelerationStructureKHR,
dst: VkAccelerationStructureKHR,
mode: VkCopyAccelerationStructureModeKHR,
}

#[repr(C)]
pub struct VkCopyAccelerationStructureToMemoryInfoKHR {
s_type: VkStructureType,
p_next: c_void,
src: VkAccelerationStructureKHR,
dst: VkDeviceOrHostAddressKHR,
mode: VkCopyAccelerationStructureModeKHR,
}

#[repr(C)]
pub struct VkCopyMemoryToAccelerationStructureInfoKHR {
s_type: VkStructureType,
p_next: c_void,
src: VkDeviceOrHostAddressConstKHR,
dst: VkAccelerationStructureKHR,
mode: VkCopyAccelerationStructureModeKHR,
}

#[repr(C)]
pub struct VkRayTracingPipelineInterfaceCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
max_pipeline_ray_payload_size: u32,
max_pipeline_ray_hit_attribute_size: u32,
}

#[repr(C)]
pub struct VkPipelineLibraryCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
library_count: u32,
p_libraries: VkPipeline,
}

#[repr(C)]
pub struct VkRefreshObjectKHR {
object_type: VkObjectType,
object_handle: u64,
flags: VkRefreshObjectFlagsKHR,
}

#[repr(C)]
pub struct VkRefreshObjectListKHR {
s_type: VkStructureType,
p_next: c_void,
object_count: u32,
p_objects: VkRefreshObjectKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicStateFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
extended_dynamic_state: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState2FeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
extended_dynamic_state2: VkBool32,
extended_dynamic_state2_logic_op: VkBool32,
extended_dynamic_state2_patch_control_points: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState3FeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
extended_dynamic_state3_tessellation_domain_origin: VkBool32,
extended_dynamic_state3_depth_clamp_enable: VkBool32,
extended_dynamic_state3_polygon_mode: VkBool32,
extended_dynamic_state3_rasterization_samples: VkBool32,
extended_dynamic_state3_sample_mask: VkBool32,
extended_dynamic_state3_alpha_to_coverage_enable: VkBool32,
extended_dynamic_state3_alpha_to_one_enable: VkBool32,
extended_dynamic_state3_logic_op_enable: VkBool32,
extended_dynamic_state3_color_blend_enable: VkBool32,
extended_dynamic_state3_color_blend_equation: VkBool32,
extended_dynamic_state3_color_write_mask: VkBool32,
extended_dynamic_state3_rasterization_stream: VkBool32,
extended_dynamic_state3_conservative_rasterization_mode: VkBool32,
extended_dynamic_state3_extra_primitive_overestimation_size: VkBool32,
extended_dynamic_state3_depth_clip_enable: VkBool32,
extended_dynamic_state3_sample_locations_enable: VkBool32,
extended_dynamic_state3_color_blend_advanced: VkBool32,
extended_dynamic_state3_provoking_vertex_mode: VkBool32,
extended_dynamic_state3_line_rasterization_mode: VkBool32,
extended_dynamic_state3_line_stipple_enable: VkBool32,
extended_dynamic_state3_depth_clip_negative_one_to_one: VkBool32,
extended_dynamic_state3_viewport_wscaling_enable: VkBool32,
extended_dynamic_state3_viewport_swizzle: VkBool32,
extended_dynamic_state3_coverage_to_color_enable: VkBool32,
extended_dynamic_state3_coverage_to_color_location: VkBool32,
extended_dynamic_state3_coverage_modulation_mode: VkBool32,
extended_dynamic_state3_coverage_modulation_table_enable: VkBool32,
extended_dynamic_state3_coverage_modulation_table: VkBool32,
extended_dynamic_state3_coverage_reduction_mode: VkBool32,
extended_dynamic_state3_representative_fragment_test_enable: VkBool32,
extended_dynamic_state3_shading_rate_image_enable: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState3PropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
dynamic_primitive_topology_unrestricted: VkBool32,
}

#[repr(C)]
pub struct VkColorBlendEquationEXT {
src_color_blend_factor: VkBlendFactor,
dst_color_blend_factor: VkBlendFactor,
color_blend_op: VkBlendOp,
src_alpha_blend_factor: VkBlendFactor,
dst_alpha_blend_factor: VkBlendFactor,
alpha_blend_op: VkBlendOp,
}

#[repr(C)]
pub struct VkColorBlendAdvancedEXT {
advanced_blend_op: VkBlendOp,
src_premultiplied: VkBool32,
dst_premultiplied: VkBool32,
blend_overlap: VkBlendOverlapEXT,
clamp_results: VkBool32,
}

#[repr(C)]
pub struct VkRenderPassTransformBeginInfoQCOM {
s_type: VkStructureType,
p_next: c_void,
transform: VkSurfaceTransformFlagBitsKHR,
}

#[repr(C)]
pub struct VkCopyCommandTransformInfoQCOM {
s_type: VkStructureType,
p_next: c_void,
transform: VkSurfaceTransformFlagBitsKHR,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
s_type: VkStructureType,
p_next: c_void,
transform: VkSurfaceTransformFlagBitsKHR,
render_area: VkRect2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceDiagnosticsConfigFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
diagnostics_config: VkBool32,
}

#[repr(C)]
pub struct VkDeviceDiagnosticsConfigCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
flags: VkDeviceDiagnosticsConfigFlagsNV,
}

#[repr(C)]
pub struct VkPipelineOfflineCreateInfo {
s_type: VkStructureType,
p_next: c_void,
pipeline_identifier: u8,
match_control: VkPipelineMatchControl,
pool_entry_size: VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
s_type: VkStructureType,
p_next: c_void,
shader_zero_initialize_workgroup_memory: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
shader_subgroup_uniform_control_flow: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRobustness2FeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
robust_buffer_access2: VkBool32,
robust_image_access2: VkBool32,
null_descriptor: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRobustness2PropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
robust_storage_buffer_access_size_alignment: VkDeviceSize,
robust_uniform_buffer_access_size_alignment: VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageRobustnessFeatures {
s_type: VkStructureType,
p_next: c_void,
robust_image_access: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
workgroup_memory_explicit_layout: VkBool32,
workgroup_memory_explicit_layout_scalar_block_layout: VkBool32,
workgroup_memory_explicit_layout8_bit_access: VkBool32,
workgroup_memory_explicit_layout16_bit_access: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePortabilitySubsetFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
ant_alpha_color_blend_factors: VkBool32,
events: VkBool32,
image_view_format_reinterpretation: VkBool32,
image_view_format_swizzle: VkBool32,
image_view2_don3_dimage: VkBool32,
multisample_array_image: VkBool32,
mutable_comparison_samplers: VkBool32,
point_polygons: VkBool32,
sampler_mip_lod_bias: VkBool32,
separate_stencil_mask_ref: VkBool32,
shader_sample_rate_interpolation_functions: VkBool32,
tessellation_isolines: VkBool32,
tessellation_point_mode: VkBool32,
triangle_fans: VkBool32,
vertex_attribute_access_beyond_stride: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePortabilitySubsetPropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
min_vertex_input_binding_stride_alignment: u32,
}

#[repr(C)]
pub struct VkPhysicalDevice4444FormatsFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
format_a4_r4_g4_b4: VkBool32,
format_a4_b4_g4_r4: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubpassShadingFeaturesHUAWEI {
s_type: VkStructureType,
p_next: c_void,
subpass_shading: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI {
s_type: VkStructureType,
p_next: c_void,
clusterculling_shader: VkBool32,
multiview_cluster_culling_shader: VkBool32,
}

#[repr(C)]
pub struct VkBufferCopy2 {
s_type: VkStructureType,
p_next: c_void,
src_offset: VkDeviceSize,
dst_offset: VkDeviceSize,
size: VkDeviceSize,
}

#[repr(C)]
pub struct VkImageCopy2 {
s_type: VkStructureType,
p_next: c_void,
src_subresource: VkImageSubresourceLayers,
src_offset: VkOffset3D,
dst_subresource: VkImageSubresourceLayers,
dst_offset: VkOffset3D,
extent: VkExtent3D,
}

#[repr(C)]
pub struct VkImageBlit2 {
s_type: VkStructureType,
p_next: c_void,
src_subresource: VkImageSubresourceLayers,
src_offsets: VkOffset3D,
dst_subresource: VkImageSubresourceLayers,
dst_offsets: VkOffset3D,
}

#[repr(C)]
pub struct VkBufferImageCopy2 {
s_type: VkStructureType,
p_next: c_void,
buffer_offset: VkDeviceSize,
buffer_row_length: u32,
buffer_image_height: u32,
image_subresource: VkImageSubresourceLayers,
image_offset: VkOffset3D,
image_extent: VkExtent3D,
}

#[repr(C)]
pub struct VkImageResolve2 {
s_type: VkStructureType,
p_next: c_void,
src_subresource: VkImageSubresourceLayers,
src_offset: VkOffset3D,
dst_subresource: VkImageSubresourceLayers,
dst_offset: VkOffset3D,
extent: VkExtent3D,
}

#[repr(C)]
pub struct VkCopyBufferInfo2 {
s_type: VkStructureType,
p_next: c_void,
src_buffer: VkBuffer,
dst_buffer: VkBuffer,
region_count: u32,
p_regions: VkBufferCopy2,
}

#[repr(C)]
pub struct VkCopyImageInfo2 {
s_type: VkStructureType,
p_next: c_void,
src_image: VkImage,
src_image_layout: VkImageLayout,
dst_image: VkImage,
dst_image_layout: VkImageLayout,
region_count: u32,
p_regions: VkImageCopy2,
}

#[repr(C)]
pub struct VkBlitImageInfo2 {
s_type: VkStructureType,
p_next: c_void,
src_image: VkImage,
src_image_layout: VkImageLayout,
dst_image: VkImage,
dst_image_layout: VkImageLayout,
region_count: u32,
p_regions: VkImageBlit2,
filter: VkFilter,
}

#[repr(C)]
pub struct VkCopyBufferToImageInfo2 {
s_type: VkStructureType,
p_next: c_void,
src_buffer: VkBuffer,
dst_image: VkImage,
dst_image_layout: VkImageLayout,
region_count: u32,
p_regions: VkBufferImageCopy2,
}

#[repr(C)]
pub struct VkCopyImageToBufferInfo2 {
s_type: VkStructureType,
p_next: c_void,
src_image: VkImage,
src_image_layout: VkImageLayout,
dst_buffer: VkBuffer,
region_count: u32,
p_regions: VkBufferImageCopy2,
}

#[repr(C)]
pub struct VkResolveImageInfo2 {
s_type: VkStructureType,
p_next: c_void,
src_image: VkImage,
src_image_layout: VkImageLayout,
dst_image: VkImage,
dst_image_layout: VkImageLayout,
region_count: u32,
p_regions: VkImageResolve2,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
shader_image_int64_atomics: VkBool32,
sparse_image_int64_atomics: VkBool32,
}

#[repr(C)]
pub struct VkFragmentShadingRateAttachmentInfoKHR {
s_type: VkStructureType,
p_next: c_void,
p_fragment_shading_rate_attachment: VkAttachmentReference2,
shading_rate_attachment_texel_size: VkExtent2D,
}

#[repr(C)]
pub struct VkPipelineFragmentShadingRateStateCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
fragment_size: VkExtent2D,
combiner_ops: VkFragmentShadingRateCombinerOpKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
pipeline_fragment_shading_rate: VkBool32,
primitive_fragment_shading_rate: VkBool32,
attachment_fragment_shading_rate: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRatePropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
min_fragment_shading_rate_attachment_texel_size: VkExtent2D,
max_fragment_shading_rate_attachment_texel_size: VkExtent2D,
max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
primitive_fragment_shading_rate_with_multiple_viewports: VkBool32,
layered_shading_rate_attachments: VkBool32,
fragment_shading_rate_non_trivial_combiner_ops: VkBool32,
max_fragment_size: VkExtent2D,
max_fragment_size_aspect_ratio: u32,
max_fragment_shading_rate_coverage_samples: u32,
max_fragment_shading_rate_rasterization_samples: VkSampleCountFlagBits,
fragment_shading_rate_with_shader_depth_stencil_writes: VkBool32,
fragment_shading_rate_with_sample_mask: VkBool32,
fragment_shading_rate_with_shader_sample_mask: VkBool32,
fragment_shading_rate_with_conservative_rasterization: VkBool32,
fragment_shading_rate_with_fragment_shader_interlock: VkBool32,
fragment_shading_rate_with_custom_sample_locations: VkBool32,
fragment_shading_rate_strict_multiply_combiner: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateKHR {
s_type: VkStructureType,
p_next: c_void,
sample_counts: VkSampleCountFlags,
fragment_size: VkExtent2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderTerminateInvocationFeatures {
s_type: VkStructureType,
p_next: c_void,
shader_terminate_invocation: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
fragment_shading_rate_enums: VkBool32,
supersample_fragment_shading_rates: VkBool32,
no_invocation_fragment_shading_rates: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
max_fragment_shading_rate_invocation_count: VkSampleCountFlagBits,
}

#[repr(C)]
pub struct VkPipelineFragmentShadingRateEnumStateCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
shading_rate_type: VkFragmentShadingRateTypeNV,
shading_rate: VkFragmentShadingRateNV,
combiner_ops: VkFragmentShadingRateCombinerOpKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureBuildSizesInfoKHR {
s_type: VkStructureType,
p_next: c_void,
acceleration_structure_size: VkDeviceSize,
update_scratch_size: VkDeviceSize,
build_scratch_size: VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceImage2DViewOf3DFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
image2_dview_of3_d: VkBool32,
sampler2_dview_of3_d: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
image_sliced_view_of3_d: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
attachment_feedback_loop_dynamic_state: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
mutable_descriptor_type: VkBool32,
}

#[repr(C)]
pub struct VkMutableDescriptorTypeListEXT {
descriptor_type_count: u32,
p_descriptor_types: VkDescriptorType,
}

#[repr(C)]
pub struct VkMutableDescriptorTypeCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
mutable_descriptor_type_list_count: u32,
p_mutable_descriptor_type_lists: VkMutableDescriptorTypeListEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthClipControlFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
depth_clip_control: VkBool32,
}

#[repr(C)]
pub struct VkPipelineViewportDepthClipControlCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
negative_one_to_one: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
vertex_input_dynamic_state: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryRDMAFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
external_memory_rdma: VkBool32,
}

#[repr(C)]
pub struct VkVertexInputBindingDescription2EXT {
s_type: VkStructureType,
p_next: c_void,
binding: u32,
stride: u32,
input_rate: VkVertexInputRate,
divisor: u32,
}

#[repr(C)]
pub struct VkVertexInputAttributeDescription2EXT {
s_type: VkStructureType,
p_next: c_void,
location: u32,
binding: u32,
format: VkFormat,
offset: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceColorWriteEnableFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
color_write_enable: VkBool32,
}

#[repr(C)]
pub struct VkPipelineColorWriteCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
attachment_count: u32,
p_color_write_enables: VkBool32,
}

#[repr(C)]
pub struct VkMemoryBarrier2 {
s_type: VkStructureType,
p_next: c_void,
src_stage_mask: VkPipelineStageFlags2,
src_access_mask: VkAccessFlags2,
dst_stage_mask: VkPipelineStageFlags2,
dst_access_mask: VkAccessFlags2,
}

#[repr(C)]
pub struct VkImageMemoryBarrier2 {
s_type: VkStructureType,
p_next: c_void,
src_stage_mask: VkPipelineStageFlags2,
src_access_mask: VkAccessFlags2,
dst_stage_mask: VkPipelineStageFlags2,
dst_access_mask: VkAccessFlags2,
old_layout: VkImageLayout,
new_layout: VkImageLayout,
src_queue_family_index: u32,
dst_queue_family_index: u32,
image: VkImage,
subresource_range: VkImageSubresourceRange,
}

#[repr(C)]
pub struct VkBufferMemoryBarrier2 {
s_type: VkStructureType,
p_next: c_void,
src_stage_mask: VkPipelineStageFlags2,
src_access_mask: VkAccessFlags2,
dst_stage_mask: VkPipelineStageFlags2,
dst_access_mask: VkAccessFlags2,
src_queue_family_index: u32,
dst_queue_family_index: u32,
buffer: VkBuffer,
offset: VkDeviceSize,
size: VkDeviceSize,
}

#[repr(C)]
pub struct VkDependencyInfo {
s_type: VkStructureType,
p_next: c_void,
dependency_flags: VkDependencyFlags,
memory_barrier_count: u32,
p_memory_barriers: VkMemoryBarrier2,
buffer_memory_barrier_count: u32,
p_buffer_memory_barriers: VkBufferMemoryBarrier2,
image_memory_barrier_count: u32,
p_image_memory_barriers: VkImageMemoryBarrier2,
}

#[repr(C)]
pub struct VkSemaphoreSubmitInfo {
s_type: VkStructureType,
p_next: c_void,
semaphore: VkSemaphore,
value: u64,
stage_mask: VkPipelineStageFlags2,
device_index: u32,
}

#[repr(C)]
pub struct VkCommandBufferSubmitInfo {
s_type: VkStructureType,
p_next: c_void,
command_buffer: VkCommandBuffer,
device_mask: u32,
}

#[repr(C)]
pub struct VkSubmitInfo2 {
s_type: VkStructureType,
p_next: c_void,
flags: VkSubmitFlags,
wait_semaphore_info_count: u32,
p_wait_semaphore_infos: VkSemaphoreSubmitInfo,
command_buffer_info_count: u32,
p_command_buffer_infos: VkCommandBufferSubmitInfo,
signal_semaphore_info_count: u32,
p_signal_semaphore_infos: VkSemaphoreSubmitInfo,
}

#[repr(C)]
pub struct VkQueueFamilyCheckpointProperties2NV {
s_type: VkStructureType,
p_next: c_void,
checkpoint_execution_stage_mask: VkPipelineStageFlags2,
}

#[repr(C)]
pub struct VkCheckpointData2NV {
s_type: VkStructureType,
p_next: c_void,
stage: VkPipelineStageFlags2,
p_checkpoint_marker: c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceSynchronization2Features {
s_type: VkStructureType,
p_next: c_void,
synchronization2: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceHostImageCopyFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
host_image_copy: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceHostImageCopyPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
copy_src_layout_count: u32,
p_copy_src_layouts: VkImageLayout,
copy_dst_layout_count: u32,
p_copy_dst_layouts: VkImageLayout,
optimal_tiling_layout_uuid: u8,
identical_memory_type_requirements: VkBool32,
}

#[repr(C)]
pub struct VkMemoryToImageCopyEXT {
s_type: VkStructureType,
p_next: c_void,
p_host_pointer: c_void,
memory_row_length: u32,
memory_image_height: u32,
image_subresource: VkImageSubresourceLayers,
image_offset: VkOffset3D,
image_extent: VkExtent3D,
}

#[repr(C)]
pub struct VkImageToMemoryCopyEXT {
s_type: VkStructureType,
p_next: c_void,
p_host_pointer: c_void,
memory_row_length: u32,
memory_image_height: u32,
image_subresource: VkImageSubresourceLayers,
image_offset: VkOffset3D,
image_extent: VkExtent3D,
}

#[repr(C)]
pub struct VkCopyMemoryToImageInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkHostImageCopyFlagsEXT,
dst_image: VkImage,
dst_image_layout: VkImageLayout,
region_count: u32,
p_regions: VkMemoryToImageCopyEXT,
}

#[repr(C)]
pub struct VkCopyImageToMemoryInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkHostImageCopyFlagsEXT,
src_image: VkImage,
src_image_layout: VkImageLayout,
region_count: u32,
p_regions: VkImageToMemoryCopyEXT,
}

#[repr(C)]
pub struct VkCopyImageToImageInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkHostImageCopyFlagsEXT,
src_image: VkImage,
src_image_layout: VkImageLayout,
dst_image: VkImage,
dst_image_layout: VkImageLayout,
region_count: u32,
p_regions: VkImageCopy2,
}

#[repr(C)]
pub struct VkHostImageLayoutTransitionInfoEXT {
s_type: VkStructureType,
p_next: c_void,
image: VkImage,
old_layout: VkImageLayout,
new_layout: VkImageLayout,
subresource_range: VkImageSubresourceRange,
}

#[repr(C)]
pub struct VkSubresourceHostMemcpySizeEXT {
s_type: VkStructureType,
p_next: c_void,
size: VkDeviceSize,
}

#[repr(C)]
pub struct VkHostImageCopyDevicePerformanceQueryEXT {
s_type: VkStructureType,
p_next: c_void,
optimal_device_access: VkBool32,
identical_memory_layout: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkanSC10Properties {
s_type: VkStructureType,
p_next: c_void,
device_no_dynamic_host_allocations: VkBool32,
device_destroy_frees_memory: VkBool32,
command_pool_multiple_command_buffers_recording: VkBool32,
command_pool_reset_command_buffer: VkBool32,
command_buffer_simultaneous_use: VkBool32,
secondary_command_buffer_null_or_imageless_framebuffer: VkBool32,
recycle_descriptor_set_memory: VkBool32,
recycle_pipeline_memory: VkBool32,
max_render_pass_subpasses: u32,
max_render_pass_dependencies: u32,
max_subpass_input_attachments: u32,
max_subpass_preserve_attachments: u32,
max_framebuffer_attachments: u32,
max_descriptor_set_layout_bindings: u32,
max_query_fault_count: u32,
max_callback_fault_count: u32,
max_command_pool_command_buffers: u32,
max_command_buffer_size: VkDeviceSize,
}

#[repr(C)]
pub struct VkPipelinePoolSize {
s_type: VkStructureType,
p_next: c_void,
pool_entry_size: VkDeviceSize,
pool_entry_count: u32,
}

#[repr(C)]
pub struct VkDeviceObjectReservationCreateInfo {
s_type: VkStructureType,
p_next: c_void,
pipeline_cache_create_info_count: u32,
p_pipeline_cache_create_infos: VkPipelineCacheCreateInfo,
pipeline_pool_size_count: u32,
p_pipeline_pool_sizes: VkPipelinePoolSize,
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
pub struct VkCommandPoolMemoryReservationCreateInfo {
s_type: VkStructureType,
p_next: c_void,
command_pool_reserved_size: VkDeviceSize,
command_pool_max_command_buffers: u32,
}

#[repr(C)]
pub struct VkCommandPoolMemoryConsumption {
s_type: VkStructureType,
p_next: c_void,
command_pool_allocated: VkDeviceSize,
command_pool_reserved_size: VkDeviceSize,
command_buffer_allocated: VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkanSC10Features {
s_type: VkStructureType,
p_next: c_void,
shader_atomic_instructions: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
primitives_generated_query: VkBool32,
primitives_generated_query_with_rasterizer_discard: VkBool32,
primitives_generated_query_with_non_zero_streams: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceLegacyDitheringFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
legacy_dithering: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
multisampled_render_to_single_sampled: VkBool32,
}

#[repr(C)]
pub struct VkSubpassResolvePerformanceQueryEXT {
s_type: VkStructureType,
p_next: c_void,
optimal: VkBool32,
}

#[repr(C)]
pub struct VkMultisampledRenderToSingleSampledInfoEXT {
s_type: VkStructureType,
p_next: c_void,
multisampled_render_to_single_sampled_enable: VkBool32,
rasterization_samples: VkSampleCountFlagBits,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineProtectedAccessFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
pipeline_protected_access: VkBool32,
}

#[repr(C)]
pub struct VkQueueFamilyVideoPropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
video_codec_operations: VkVideoCodecOperationFlagsKHR,
}

#[repr(C)]
pub struct VkQueueFamilyQueryResultStatusPropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
query_result_status_support: VkBool32,
}

#[repr(C)]
pub struct VkVideoProfileListInfoKHR {
s_type: VkStructureType,
p_next: c_void,
profile_count: u32,
p_profiles: VkVideoProfileInfoKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceVideoFormatInfoKHR {
s_type: VkStructureType,
p_next: c_void,
image_usage: VkImageUsageFlags,
}

#[repr(C)]
pub struct VkVideoFormatPropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
format: VkFormat,
component_mapping: VkComponentMapping,
image_create_flags: VkImageCreateFlags,
image_type: VkImageType,
image_tiling: VkImageTiling,
image_usage_flags: VkImageUsageFlags,
}

#[repr(C)]
pub struct VkVideoProfileInfoKHR {
s_type: VkStructureType,
p_next: c_void,
video_codec_operation: VkVideoCodecOperationFlagBitsKHR,
chroma_subsampling: VkVideoChromaSubsamplingFlagsKHR,
luma_bit_depth: VkVideoComponentBitDepthFlagsKHR,
chroma_bit_depth: VkVideoComponentBitDepthFlagsKHR,
}

#[repr(C)]
pub struct VkVideoCapabilitiesKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkVideoCapabilityFlagsKHR,
min_bitstream_buffer_offset_alignment: VkDeviceSize,
min_bitstream_buffer_size_alignment: VkDeviceSize,
picture_access_granularity: VkExtent2D,
min_coded_extent: VkExtent2D,
max_coded_extent: VkExtent2D,
max_dpb_slots: u32,
max_active_reference_pictures: u32,
std_header_version: VkExtensionProperties,
}

#[repr(C)]
pub struct VkVideoSessionMemoryRequirementsKHR {
s_type: VkStructureType,
p_next: c_void,
memory_bind_index: u32,
memory_requirements: VkMemoryRequirements,
}

#[repr(C)]
pub struct VkBindVideoSessionMemoryInfoKHR {
s_type: VkStructureType,
p_next: c_void,
memory_bind_index: u32,
memory: VkDeviceMemory,
memory_offset: VkDeviceSize,
memory_size: VkDeviceSize,
}

#[repr(C)]
pub struct VkVideoPictureResourceInfoKHR {
s_type: VkStructureType,
p_next: c_void,
coded_offset: VkOffset2D,
coded_extent: VkExtent2D,
base_array_layer: u32,
image_view_binding: VkImageView,
}

#[repr(C)]
pub struct VkVideoReferenceSlotInfoKHR {
s_type: VkStructureType,
p_next: c_void,
slot_index: i32,
p_picture_resource: VkVideoPictureResourceInfoKHR,
}

#[repr(C)]
pub struct VkVideoDecodeCapabilitiesKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkVideoDecodeCapabilityFlagsKHR,
}

#[repr(C)]
pub struct VkVideoDecodeUsageInfoKHR {
s_type: VkStructureType,
p_next: c_void,
video_usage_hints: VkVideoDecodeUsageFlagsKHR,
}

#[repr(C)]
pub struct VkVideoDecodeInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkVideoDecodeFlagsKHR,
src_buffer: VkBuffer,
src_buffer_offset: VkDeviceSize,
src_buffer_range: VkDeviceSize,
dst_picture_resource: VkVideoPictureResourceInfoKHR,
p_setup_reference_slot: VkVideoReferenceSlotInfoKHR,
reference_slot_count: u32,
p_reference_slots: VkVideoReferenceSlotInfoKHR,
}

#[repr(C)]
pub struct VkVideoDecodeH264ProfileInfoKHR {
s_type: VkStructureType,
p_next: c_void,
std_profile_idc: StdVideoH264ProfileIdc,
picture_layout: VkVideoDecodeH264PictureLayoutFlagBitsKHR,
}

#[repr(C)]
pub struct VkVideoDecodeH264CapabilitiesKHR {
s_type: VkStructureType,
p_next: c_void,
max_level_idc: StdVideoH264LevelIdc,
field_offset_granularity: VkOffset2D,
}

#[repr(C)]
pub struct VkVideoDecodeH264SessionParametersAddInfoKHR {
s_type: VkStructureType,
p_next: c_void,
std_spscount: u32,
p_std_spss: StdVideoH264SequenceParameterSet,
std_ppscount: u32,
p_std_ppss: StdVideoH264PictureParameterSet,
}

#[repr(C)]
pub struct VkVideoDecodeH264SessionParametersCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
max_std_spscount: u32,
max_std_ppscount: u32,
p_parameters_add_info: VkVideoDecodeH264SessionParametersAddInfoKHR,
}

#[repr(C)]
pub struct VkVideoDecodeH264PictureInfoKHR {
s_type: VkStructureType,
p_next: c_void,
p_std_picture_info: StdVideoDecodeH264PictureInfo,
slice_count: u32,
p_slice_offsets: u32,
}

#[repr(C)]
pub struct VkVideoDecodeH264DpbSlotInfoKHR {
s_type: VkStructureType,
p_next: c_void,
p_std_reference_info: StdVideoDecodeH264ReferenceInfo,
}

#[repr(C)]
pub struct VkVideoDecodeH265ProfileInfoKHR {
s_type: VkStructureType,
p_next: c_void,
std_profile_idc: StdVideoH265ProfileIdc,
}

#[repr(C)]
pub struct VkVideoDecodeH265CapabilitiesKHR {
s_type: VkStructureType,
p_next: c_void,
max_level_idc: StdVideoH265LevelIdc,
}

#[repr(C)]
pub struct VkVideoDecodeH265SessionParametersAddInfoKHR {
s_type: VkStructureType,
p_next: c_void,
std_vpscount: u32,
p_std_vpss: StdVideoH265VideoParameterSet,
std_spscount: u32,
p_std_spss: StdVideoH265SequenceParameterSet,
std_ppscount: u32,
p_std_ppss: StdVideoH265PictureParameterSet,
}

#[repr(C)]
pub struct VkVideoDecodeH265SessionParametersCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
max_std_vpscount: u32,
max_std_spscount: u32,
max_std_ppscount: u32,
p_parameters_add_info: VkVideoDecodeH265SessionParametersAddInfoKHR,
}

#[repr(C)]
pub struct VkVideoDecodeH265PictureInfoKHR {
s_type: VkStructureType,
p_next: c_void,
p_std_picture_info: StdVideoDecodeH265PictureInfo,
slice_segment_count: u32,
p_slice_segment_offsets: u32,
}

#[repr(C)]
pub struct VkVideoDecodeH265DpbSlotInfoKHR {
s_type: VkStructureType,
p_next: c_void,
p_std_reference_info: StdVideoDecodeH265ReferenceInfo,
}

#[repr(C)]
pub struct VkVideoSessionCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
queue_family_index: u32,
flags: VkVideoSessionCreateFlagsKHR,
p_video_profile: VkVideoProfileInfoKHR,
picture_format: VkFormat,
max_coded_extent: VkExtent2D,
reference_picture_format: VkFormat,
max_dpb_slots: u32,
max_active_reference_pictures: u32,
p_std_header_version: VkExtensionProperties,
}

#[repr(C)]
pub struct VkVideoSessionParametersCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkVideoSessionParametersCreateFlagsKHR,
video_session_parameters_template: VkVideoSessionParametersKHR,
video_session: VkVideoSessionKHR,
}

#[repr(C)]
pub struct VkVideoSessionParametersUpdateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
update_sequence_count: u32,
}

#[repr(C)]
pub struct VkVideoEncodeSessionParametersGetInfoKHR {
s_type: VkStructureType,
p_next: c_void,
video_session_parameters: VkVideoSessionParametersKHR,
}

#[repr(C)]
pub struct VkVideoEncodeSessionParametersFeedbackInfoKHR {
s_type: VkStructureType,
p_next: c_void,
has_overrides: VkBool32,
}

#[repr(C)]
pub struct VkVideoBeginCodingInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkVideoBeginCodingFlagsKHR,
video_session: VkVideoSessionKHR,
video_session_parameters: VkVideoSessionParametersKHR,
reference_slot_count: u32,
p_reference_slots: VkVideoReferenceSlotInfoKHR,
}

#[repr(C)]
pub struct VkVideoEndCodingInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkVideoEndCodingFlagsKHR,
}

#[repr(C)]
pub struct VkVideoCodingControlInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkVideoCodingControlFlagsKHR,
}

#[repr(C)]
pub struct VkVideoEncodeUsageInfoKHR {
s_type: VkStructureType,
p_next: c_void,
video_usage_hints: VkVideoEncodeUsageFlagsKHR,
video_content_hints: VkVideoEncodeContentFlagsKHR,
tuning_mode: VkVideoEncodeTuningModeKHR,
}

#[repr(C)]
pub struct VkVideoEncodeInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkVideoEncodeFlagsKHR,
dst_buffer: VkBuffer,
dst_buffer_offset: VkDeviceSize,
dst_buffer_range: VkDeviceSize,
src_picture_resource: VkVideoPictureResourceInfoKHR,
p_setup_reference_slot: VkVideoReferenceSlotInfoKHR,
reference_slot_count: u32,
p_reference_slots: VkVideoReferenceSlotInfoKHR,
preceding_externally_encoded_bytes: u32,
}

#[repr(C)]
pub struct VkQueryPoolVideoEncodeFeedbackCreateInfoKHR {
s_type: VkStructureType,
p_next: c_void,
encode_feedback_flags: VkVideoEncodeFeedbackFlagsKHR,
}

#[repr(C)]
pub struct VkVideoEncodeQualityLevelInfoKHR {
s_type: VkStructureType,
p_next: c_void,
quality_level: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR {
s_type: VkStructureType,
p_next: c_void,
p_video_profile: VkVideoProfileInfoKHR,
quality_level: u32,
}

#[repr(C)]
pub struct VkVideoEncodeQualityLevelPropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
preferred_rate_control_mode: VkVideoEncodeRateControlModeFlagBitsKHR,
preferred_rate_control_layer_count: u32,
}

#[repr(C)]
pub struct VkVideoEncodeRateControlInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkVideoEncodeRateControlFlagsKHR,
rate_control_mode: VkVideoEncodeRateControlModeFlagBitsKHR,
layer_count: u32,
p_layers: VkVideoEncodeRateControlLayerInfoKHR,
virtual_buffer_size_in_ms: u32,
initial_virtual_buffer_size_in_ms: u32,
}

#[repr(C)]
pub struct VkVideoEncodeRateControlLayerInfoKHR {
s_type: VkStructureType,
p_next: c_void,
average_bitrate: u64,
max_bitrate: u64,
frame_rate_numerator: u32,
frame_rate_denominator: u32,
}

#[repr(C)]
pub struct VkVideoEncodeCapabilitiesKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkVideoEncodeCapabilityFlagsKHR,
rate_control_modes: VkVideoEncodeRateControlModeFlagsKHR,
max_rate_control_layers: u32,
max_bitrate: u64,
max_quality_levels: u32,
encode_input_picture_granularity: VkExtent2D,
supported_encode_feedback_flags: VkVideoEncodeFeedbackFlagsKHR,
}

#[repr(C)]
pub struct VkVideoEncodeH264CapabilitiesEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkVideoEncodeH264CapabilityFlagsEXT,
max_level_idc: StdVideoH264LevelIdc,
max_slice_count: u32,
max_ppicture_l0_reference_count: u32,
max_bpicture_l0_reference_count: u32,
max_l1_reference_count: u32,
max_temporal_layer_count: u32,
expect_dyadic_temporal_layer_pattern: VkBool32,
min_qp: i32,
max_qp: i32,
prefers_gop_remaining_frames: VkBool32,
requires_gop_remaining_frames: VkBool32,
std_syntax_flags: VkVideoEncodeH264StdFlagsEXT,
}

#[repr(C)]
pub struct VkVideoEncodeH264QualityLevelPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
preferred_rate_control_flags: VkVideoEncodeH264RateControlFlagsEXT,
preferred_gop_frame_count: u32,
preferred_idr_period: u32,
preferred_consecutive_bframe_count: u32,
preferred_temporal_layer_count: u32,
preferred_constant_qp: VkVideoEncodeH264QpEXT,
preferred_max_l0_reference_count: u32,
preferred_max_l1_reference_count: u32,
preferred_std_entropy_coding_mode_flag: VkBool32,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
use_max_level_idc: VkBool32,
max_level_idc: StdVideoH264LevelIdc,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersAddInfoEXT {
s_type: VkStructureType,
p_next: c_void,
std_spscount: u32,
p_std_spss: StdVideoH264SequenceParameterSet,
std_ppscount: u32,
p_std_ppss: StdVideoH264PictureParameterSet,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
max_std_spscount: u32,
max_std_ppscount: u32,
p_parameters_add_info: VkVideoEncodeH264SessionParametersAddInfoEXT,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersGetInfoEXT {
s_type: VkStructureType,
p_next: c_void,
write_std_sps: VkBool32,
write_std_pps: VkBool32,
std_spsid: u32,
std_ppsid: u32,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersFeedbackInfoEXT {
s_type: VkStructureType,
p_next: c_void,
has_std_spsoverrides: VkBool32,
has_std_ppsoverrides: VkBool32,
}

#[repr(C)]
pub struct VkVideoEncodeH264DpbSlotInfoEXT {
s_type: VkStructureType,
p_next: c_void,
p_std_reference_info: StdVideoEncodeH264ReferenceInfo,
}

#[repr(C)]
pub struct VkVideoEncodeH264PictureInfoEXT {
s_type: VkStructureType,
p_next: c_void,
nalu_slice_entry_count: u32,
p_nalu_slice_entries: VkVideoEncodeH264NaluSliceInfoEXT,
p_std_picture_info: StdVideoEncodeH264PictureInfo,
generate_prefix_nalu: VkBool32,
}

#[repr(C)]
pub struct VkVideoEncodeH264ProfileInfoEXT {
s_type: VkStructureType,
p_next: c_void,
std_profile_idc: StdVideoH264ProfileIdc,
}

#[repr(C)]
pub struct VkVideoEncodeH264NaluSliceInfoEXT {
s_type: VkStructureType,
p_next: c_void,
ant_qp: i32,
p_std_slice_header: StdVideoEncodeH264SliceHeader,
}

#[repr(C)]
pub struct VkVideoEncodeH264RateControlInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkVideoEncodeH264RateControlFlagsEXT,
gop_frame_count: u32,
idr_period: u32,
consecutive_bframe_count: u32,
temporal_layer_count: u32,
}

#[repr(C)]
pub struct VkVideoEncodeH264QpEXT {
qp_i: i32,
qp_p: i32,
qp_b: i32,
}

#[repr(C)]
pub struct VkVideoEncodeH264FrameSizeEXT {
frame_isize: u32,
frame_psize: u32,
frame_bsize: u32,
}

#[repr(C)]
pub struct VkVideoEncodeH264GopRemainingFrameInfoEXT {
s_type: VkStructureType,
p_next: c_void,
use_gop_remaining_frames: VkBool32,
gop_remaining_i: u32,
gop_remaining_p: u32,
gop_remaining_b: u32,
}

#[repr(C)]
pub struct VkVideoEncodeH264RateControlLayerInfoEXT {
s_type: VkStructureType,
p_next: c_void,
use_min_qp: VkBool32,
min_qp: VkVideoEncodeH264QpEXT,
use_max_qp: VkBool32,
max_qp: VkVideoEncodeH264QpEXT,
use_max_frame_size: VkBool32,
max_frame_size: VkVideoEncodeH264FrameSizeEXT,
}

#[repr(C)]
pub struct VkVideoEncodeH265CapabilitiesEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkVideoEncodeH265CapabilityFlagsEXT,
max_level_idc: StdVideoH265LevelIdc,
max_slice_segment_count: u32,
max_tiles: VkExtent2D,
ctb_sizes: VkVideoEncodeH265CtbSizeFlagsEXT,
transform_block_sizes: VkVideoEncodeH265TransformBlockSizeFlagsEXT,
max_ppicture_l0_reference_count: u32,
max_bpicture_l0_reference_count: u32,
max_l1_reference_count: u32,
max_sub_layer_count: u32,
expect_dyadic_temporal_sub_layer_pattern: VkBool32,
min_qp: i32,
max_qp: i32,
prefers_gop_remaining_frames: VkBool32,
requires_gop_remaining_frames: VkBool32,
std_syntax_flags: VkVideoEncodeH265StdFlagsEXT,
}

#[repr(C)]
pub struct VkVideoEncodeH265QualityLevelPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
preferred_rate_control_flags: VkVideoEncodeH265RateControlFlagsEXT,
preferred_gop_frame_count: u32,
preferred_idr_period: u32,
preferred_consecutive_bframe_count: u32,
preferred_sub_layer_count: u32,
preferred_constant_qp: VkVideoEncodeH265QpEXT,
preferred_max_l0_reference_count: u32,
preferred_max_l1_reference_count: u32,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
use_max_level_idc: VkBool32,
max_level_idc: StdVideoH265LevelIdc,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersAddInfoEXT {
s_type: VkStructureType,
p_next: c_void,
std_vpscount: u32,
p_std_vpss: StdVideoH265VideoParameterSet,
std_spscount: u32,
p_std_spss: StdVideoH265SequenceParameterSet,
std_ppscount: u32,
p_std_ppss: StdVideoH265PictureParameterSet,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
max_std_vpscount: u32,
max_std_spscount: u32,
max_std_ppscount: u32,
p_parameters_add_info: VkVideoEncodeH265SessionParametersAddInfoEXT,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersGetInfoEXT {
s_type: VkStructureType,
p_next: c_void,
write_std_vps: VkBool32,
write_std_sps: VkBool32,
write_std_pps: VkBool32,
std_vpsid: u32,
std_spsid: u32,
std_ppsid: u32,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersFeedbackInfoEXT {
s_type: VkStructureType,
p_next: c_void,
has_std_vpsoverrides: VkBool32,
has_std_spsoverrides: VkBool32,
has_std_ppsoverrides: VkBool32,
}

#[repr(C)]
pub struct VkVideoEncodeH265PictureInfoEXT {
s_type: VkStructureType,
p_next: c_void,
nalu_slice_segment_entry_count: u32,
p_nalu_slice_segment_entries: VkVideoEncodeH265NaluSliceSegmentInfoEXT,
p_std_picture_info: StdVideoEncodeH265PictureInfo,
}

#[repr(C)]
pub struct VkVideoEncodeH265NaluSliceSegmentInfoEXT {
s_type: VkStructureType,
p_next: c_void,
ant_qp: i32,
p_std_slice_segment_header: StdVideoEncodeH265SliceSegmentHeader,
}

#[repr(C)]
pub struct VkVideoEncodeH265RateControlInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkVideoEncodeH265RateControlFlagsEXT,
gop_frame_count: u32,
idr_period: u32,
consecutive_bframe_count: u32,
sub_layer_count: u32,
}

#[repr(C)]
pub struct VkVideoEncodeH265QpEXT {
qp_i: i32,
qp_p: i32,
qp_b: i32,
}

#[repr(C)]
pub struct VkVideoEncodeH265FrameSizeEXT {
frame_isize: u32,
frame_psize: u32,
frame_bsize: u32,
}

#[repr(C)]
pub struct VkVideoEncodeH265GopRemainingFrameInfoEXT {
s_type: VkStructureType,
p_next: c_void,
use_gop_remaining_frames: VkBool32,
gop_remaining_i: u32,
gop_remaining_p: u32,
gop_remaining_b: u32,
}

#[repr(C)]
pub struct VkVideoEncodeH265RateControlLayerInfoEXT {
s_type: VkStructureType,
p_next: c_void,
use_min_qp: VkBool32,
min_qp: VkVideoEncodeH265QpEXT,
use_max_qp: VkBool32,
max_qp: VkVideoEncodeH265QpEXT,
use_max_frame_size: VkBool32,
max_frame_size: VkVideoEncodeH265FrameSizeEXT,
}

#[repr(C)]
pub struct VkVideoEncodeH265ProfileInfoEXT {
s_type: VkStructureType,
p_next: c_void,
std_profile_idc: StdVideoH265ProfileIdc,
}

#[repr(C)]
pub struct VkVideoEncodeH265DpbSlotInfoEXT {
s_type: VkStructureType,
p_next: c_void,
p_std_reference_info: StdVideoEncodeH265ReferenceInfo,
}

#[repr(C)]
pub struct VkPhysicalDeviceInheritedViewportScissorFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
inherited_viewport_scissor2_d: VkBool32,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceViewportScissorInfoNV {
s_type: VkStructureType,
p_next: c_void,
viewport_scissor2_d: VkBool32,
viewport_depth_count: u32,
p_viewport_depths: VkViewport,
}

#[repr(C)]
pub struct VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
ycbcr2plane444_formats: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceProvokingVertexFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
provoking_vertex_last: VkBool32,
transform_feedback_preserves_provoking_vertex: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceProvokingVertexPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
provoking_vertex_mode_per_pipeline: VkBool32,
transform_feedback_preserves_triangle_fan_provoking_vertex: VkBool32,
}

#[repr(C)]
pub struct VkPipelineRasterizationProvokingVertexStateCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
provoking_vertex_mode: VkProvokingVertexModeEXT,
}

#[repr(C)]
pub struct VkCuModuleCreateInfoNVX {
s_type: VkStructureType,
p_next: c_void,
data_size: usize,
p_data: c_void,
}

#[repr(C)]
pub struct VkCuFunctionCreateInfoNVX {
s_type: VkStructureType,
p_next: c_void,
module: VkCuModuleNVX,
p_name: u8,
}

#[repr(C)]
pub struct VkCuLaunchInfoNVX {
s_type: VkStructureType,
p_next: c_void,
function: VkCuFunctionNVX,
grid_dim_x: u32,
grid_dim_y: u32,
grid_dim_z: u32,
block_dim_x: u32,
block_dim_y: u32,
block_dim_z: u32,
shared_mem_bytes: u32,
param_count: usize,
p_params: c_void,
extra_count: usize,
p_extras: c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
descriptor_buffer: VkBool32,
descriptor_buffer_capture_replay: VkBool32,
descriptor_buffer_image_layout_ignored: VkBool32,
descriptor_buffer_push_descriptors: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
combined_image_sampler_descriptor_single_array: VkBool32,
bufferless_push_descriptors: VkBool32,
allow_sampler_image_view_post_submit_creation: VkBool32,
descriptor_buffer_offset_alignment: VkDeviceSize,
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
max_sampler_descriptor_buffer_range: VkDeviceSize,
max_resource_descriptor_buffer_range: VkDeviceSize,
sampler_descriptor_buffer_address_space_size: VkDeviceSize,
resource_descriptor_buffer_address_space_size: VkDeviceSize,
descriptor_buffer_address_space_size: VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
combined_image_sampler_density_map_descriptor_size: usize,
}

#[repr(C)]
pub struct VkDescriptorAddressInfoEXT {
s_type: VkStructureType,
p_next: c_void,
address: VkDeviceAddress,
range: VkDeviceSize,
format: VkFormat,
}

#[repr(C)]
pub struct VkDescriptorBufferBindingInfoEXT {
s_type: VkStructureType,
p_next: c_void,
address: VkDeviceAddress,
usage: VkBufferUsageFlags,
}

#[repr(C)]
pub struct VkDescriptorBufferBindingPushDescriptorBufferHandleEXT {
s_type: VkStructureType,
p_next: c_void,
buffer: VkBuffer,
}

#[repr(C)]
pub struct VkDescriptorGetInfoEXT {
s_type: VkStructureType,
p_next: c_void,
r#type: VkDescriptorType,
data: VkDescriptorDataEXT,
}

#[repr(C)]
pub struct VkBufferCaptureDescriptorDataInfoEXT {
s_type: VkStructureType,
p_next: c_void,
buffer: VkBuffer,
}

#[repr(C)]
pub struct VkImageCaptureDescriptorDataInfoEXT {
s_type: VkStructureType,
p_next: c_void,
image: VkImage,
}

#[repr(C)]
pub struct VkImageViewCaptureDescriptorDataInfoEXT {
s_type: VkStructureType,
p_next: c_void,
image_view: VkImageView,
}

#[repr(C)]
pub struct VkSamplerCaptureDescriptorDataInfoEXT {
s_type: VkStructureType,
p_next: c_void,
sampler: VkSampler,
}

#[repr(C)]
pub struct VkAccelerationStructureCaptureDescriptorDataInfoEXT {
s_type: VkStructureType,
p_next: c_void,
acceleration_structure: VkAccelerationStructureKHR,
acceleration_structure_nv: VkAccelerationStructureNV,
}

#[repr(C)]
pub struct VkOpaqueCaptureDescriptorDataCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
opaque_capture_descriptor_data: c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerDotProductFeatures {
s_type: VkStructureType,
p_next: c_void,
shader_integer_dot_product: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerDotProductProperties {
s_type: VkStructureType,
p_next: c_void,
integer_dot_product8_bit_unsigned_accelerated: VkBool32,
integer_dot_product8_bit_signed_accelerated: VkBool32,
integer_dot_product8_bit_mixed_signedness_accelerated: VkBool32,
integer_dot_product4x8_bit_packed_unsigned_accelerated: VkBool32,
integer_dot_product4x8_bit_packed_signed_accelerated: VkBool32,
integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: VkBool32,
integer_dot_product16_bit_unsigned_accelerated: VkBool32,
integer_dot_product16_bit_signed_accelerated: VkBool32,
integer_dot_product16_bit_mixed_signedness_accelerated: VkBool32,
integer_dot_product32_bit_unsigned_accelerated: VkBool32,
integer_dot_product32_bit_signed_accelerated: VkBool32,
integer_dot_product32_bit_mixed_signedness_accelerated: VkBool32,
integer_dot_product64_bit_unsigned_accelerated: VkBool32,
integer_dot_product64_bit_signed_accelerated: VkBool32,
integer_dot_product64_bit_mixed_signedness_accelerated: VkBool32,
integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: VkBool32,
integer_dot_product_accumulating_saturating8_bit_signed_accelerated: VkBool32,
integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: VkBool32,
integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: VkBool32,
integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: VkBool32,
integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: VkBool32,
integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: VkBool32,
integer_dot_product_accumulating_saturating16_bit_signed_accelerated: VkBool32,
integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: VkBool32,
integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: VkBool32,
integer_dot_product_accumulating_saturating32_bit_signed_accelerated: VkBool32,
integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: VkBool32,
integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: VkBool32,
integer_dot_product_accumulating_saturating64_bit_signed_accelerated: VkBool32,
integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDrmPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
has_primary: VkBool32,
has_render: VkBool32,
primary_major: i64,
primary_minor: i64,
render_major: i64,
render_minor: i64,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
fragment_shader_barycentric: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
tri_strip_vertex_order_independent_of_provoking_vertex: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingMotionBlurFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
ray_tracing_motion_blur: VkBool32,
ray_tracing_motion_blur_pipeline_trace_rays_indirect: VkBool32,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryMotionTrianglesDataNV {
s_type: VkStructureType,
p_next: c_void,
vertex_data: VkDeviceOrHostAddressConstKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureMotionInfoNV {
s_type: VkStructureType,
p_next: c_void,
max_instances: u32,
flags: VkAccelerationStructureMotionInfoFlagsNV,
}

#[repr(C)]
pub struct VkSRTDataNV {
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
pub struct VkAccelerationStructureSRTMotionInstanceNV {
transform_t0: VkSRTDataNV,
transform_t1: VkSRTDataNV,
instance_custom_index: u32,
mask: u32,
instance_shader_binding_table_record_offset: u32,
flags: VkGeometryInstanceFlagsKHR,
acceleration_structure_reference: u64,
}

#[repr(C)]
pub struct VkAccelerationStructureMatrixMotionInstanceNV {
transform_t0: VkTransformMatrixKHR,
transform_t1: VkTransformMatrixKHR,
instance_custom_index: u32,
mask: u32,
instance_shader_binding_table_record_offset: u32,
flags: VkGeometryInstanceFlagsKHR,
acceleration_structure_reference: u64,
}

#[repr(C)]
pub struct VkAccelerationStructureMotionInstanceNV {
r#type: VkAccelerationStructureMotionInstanceTypeNV,
flags: VkAccelerationStructureMotionInstanceFlagsNV,
data: VkAccelerationStructureMotionInstanceDataNV,
}

#[repr(C)]
pub struct VkMemoryGetRemoteAddressInfoNV {
s_type: VkStructureType,
p_next: c_void,
memory: VkDeviceMemory,
handle_type: VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkImportMemoryBufferCollectionFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
collection: VkBufferCollectionFUCHSIA,
index: u32,
}

#[repr(C)]
pub struct VkBufferCollectionImageCreateInfoFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
collection: VkBufferCollectionFUCHSIA,
index: u32,
}

#[repr(C)]
pub struct VkBufferCollectionBufferCreateInfoFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
collection: VkBufferCollectionFUCHSIA,
index: u32,
}

#[repr(C)]
pub struct VkBufferCollectionCreateInfoFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
collection_token: zx_handle_t,
}

#[repr(C)]
pub struct VkBufferCollectionPropertiesFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
memory_type_bits: u32,
buffer_count: u32,
create_info_index: u32,
sysmem_pixel_format: u64,
format_features: VkFormatFeatureFlags,
sysmem_color_space_index: VkSysmemColorSpaceFUCHSIA,
sampler_ycbcr_conversion_components: VkComponentMapping,
suggested_ycbcr_model: VkSamplerYcbcrModelConversion,
suggested_ycbcr_range: VkSamplerYcbcrRange,
suggested_xchroma_offset: VkChromaLocation,
suggested_ychroma_offset: VkChromaLocation,
}

#[repr(C)]
pub struct VkBufferConstraintsInfoFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
create_info: VkBufferCreateInfo,
required_format_features: VkFormatFeatureFlags,
buffer_collection_constraints: VkBufferCollectionConstraintsInfoFUCHSIA,
}

#[repr(C)]
pub struct VkSysmemColorSpaceFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
color_space: u32,
}

#[repr(C)]
pub struct VkImageFormatConstraintsInfoFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
image_create_info: VkImageCreateInfo,
required_format_features: VkFormatFeatureFlags,
flags: VkImageFormatConstraintsFlagsFUCHSIA,
sysmem_pixel_format: u64,
color_space_count: u32,
p_color_spaces: VkSysmemColorSpaceFUCHSIA,
}

#[repr(C)]
pub struct VkImageConstraintsInfoFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
format_constraints_count: u32,
p_format_constraints: VkImageFormatConstraintsInfoFUCHSIA,
buffer_collection_constraints: VkBufferCollectionConstraintsInfoFUCHSIA,
flags: VkImageConstraintsInfoFlagsFUCHSIA,
}

#[repr(C)]
pub struct VkBufferCollectionConstraintsInfoFUCHSIA {
s_type: VkStructureType,
p_next: c_void,
min_buffer_count: u32,
max_buffer_count: u32,
min_buffer_count_for_camping: u32,
min_buffer_count_for_dedicated_slack: u32,
min_buffer_count_for_shared_slack: u32,
}

#[repr(C)]
pub struct VkCudaModuleCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
data_size: usize,
p_data: c_void,
}

#[repr(C)]
pub struct VkCudaFunctionCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
module: VkCudaModuleNV,
p_name: u8,
}

#[repr(C)]
pub struct VkCudaLaunchInfoNV {
s_type: VkStructureType,
p_next: c_void,
function: VkCudaFunctionNV,
grid_dim_x: u32,
grid_dim_y: u32,
grid_dim_z: u32,
block_dim_x: u32,
block_dim_y: u32,
block_dim_z: u32,
shared_mem_bytes: u32,
param_count: usize,
p_params: c_void,
extra_count: usize,
p_extras: c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
format_rgba10x6_without_ycb_cr_sampler: VkBool32,
}

#[repr(C)]
pub struct VkFormatProperties3 {
s_type: VkStructureType,
p_next: c_void,
linear_tiling_features: VkFormatFeatureFlags2,
optimal_tiling_features: VkFormatFeatureFlags2,
buffer_features: VkFormatFeatureFlags2,
}

#[repr(C)]
pub struct VkDrmFormatModifierPropertiesList2EXT {
s_type: VkStructureType,
p_next: c_void,
drm_format_modifier_count: u32,
p_drm_format_modifier_properties: VkDrmFormatModifierProperties2EXT,
}

#[repr(C)]
pub struct VkDrmFormatModifierProperties2EXT {
drm_format_modifier: u64,
drm_format_modifier_plane_count: u32,
drm_format_modifier_tiling_features: VkFormatFeatureFlags2,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferFormatProperties2ANDROID {
s_type: VkStructureType,
p_next: c_void,
format: VkFormat,
external_format: u64,
format_features: VkFormatFeatureFlags2,
sampler_ycbcr_conversion_components: VkComponentMapping,
suggested_ycbcr_model: VkSamplerYcbcrModelConversion,
suggested_ycbcr_range: VkSamplerYcbcrRange,
suggested_xchroma_offset: VkChromaLocation,
suggested_ychroma_offset: VkChromaLocation,
}

#[repr(C)]
pub struct VkPipelineRenderingCreateInfo {
s_type: VkStructureType,
p_next: c_void,
view_mask: u32,
color_attachment_count: u32,
p_color_attachment_formats: VkFormat,
depth_attachment_format: VkFormat,
stencil_attachment_format: VkFormat,
}

#[repr(C)]
pub struct VkRenderingInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkRenderingFlags,
render_area: VkRect2D,
layer_count: u32,
view_mask: u32,
color_attachment_count: u32,
p_color_attachments: VkRenderingAttachmentInfo,
p_depth_attachment: VkRenderingAttachmentInfo,
p_stencil_attachment: VkRenderingAttachmentInfo,
}

#[repr(C)]
pub struct VkRenderingAttachmentInfo {
s_type: VkStructureType,
p_next: c_void,
image_view: VkImageView,
image_layout: VkImageLayout,
resolve_mode: VkResolveModeFlagBits,
resolve_image_view: VkImageView,
resolve_image_layout: VkImageLayout,
load_op: VkAttachmentLoadOp,
store_op: VkAttachmentStoreOp,
clear_value: VkClearValue,
}

#[repr(C)]
pub struct VkRenderingFragmentShadingRateAttachmentInfoKHR {
s_type: VkStructureType,
p_next: c_void,
image_view: VkImageView,
image_layout: VkImageLayout,
shading_rate_attachment_texel_size: VkExtent2D,
}

#[repr(C)]
pub struct VkRenderingFragmentDensityMapAttachmentInfoEXT {
s_type: VkStructureType,
p_next: c_void,
image_view: VkImageView,
image_layout: VkImageLayout,
}

#[repr(C)]
pub struct VkPhysicalDeviceDynamicRenderingFeatures {
s_type: VkStructureType,
p_next: c_void,
dynamic_rendering: VkBool32,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceRenderingInfo {
s_type: VkStructureType,
p_next: c_void,
flags: VkRenderingFlags,
view_mask: u32,
#[cfg(vulkan)]
color_attachment_count: u32,
#[cfg(vulkansc)]
color_attachment_count: u32,
p_color_attachment_formats: VkFormat,
depth_attachment_format: VkFormat,
stencil_attachment_format: VkFormat,
rasterization_samples: VkSampleCountFlagBits,
}

#[repr(C)]
pub struct VkAttachmentSampleCountInfoAMD {
s_type: VkStructureType,
p_next: c_void,
color_attachment_count: u32,
p_color_attachment_samples: VkSampleCountFlagBits,
depth_stencil_attachment_samples: VkSampleCountFlagBits,
}

#[repr(C)]
pub struct VkMultiviewPerViewAttributesInfoNVX {
s_type: VkStructureType,
p_next: c_void,
per_view_attributes: VkBool32,
per_view_attributes_position_xonly: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageViewMinLodFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
min_lod: VkBool32,
}

#[repr(C)]
pub struct VkImageViewMinLodCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
min_lod: f32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
rasterization_order_color_attachment_access: VkBool32,
rasterization_order_depth_attachment_access: VkBool32,
rasterization_order_stencil_attachment_access: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceLinearColorAttachmentFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
linear_color_attachment: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
graphics_pipeline_library: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
graphics_pipeline_library_fast_linking: VkBool32,
graphics_pipeline_library_independent_interpolation_decoration: VkBool32,
}

#[repr(C)]
pub struct VkGraphicsPipelineLibraryCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkGraphicsPipelineLibraryFlagsEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
s_type: VkStructureType,
p_next: c_void,
descriptor_set_host_mapping: VkBool32,
}

#[repr(C)]
pub struct VkDescriptorSetBindingReferenceVALVE {
s_type: VkStructureType,
p_next: c_void,
descriptor_set_layout: VkDescriptorSetLayout,
binding: u32,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutHostMappingInfoVALVE {
s_type: VkStructureType,
p_next: c_void,
descriptor_offset: usize,
descriptor_size: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceNestedCommandBufferFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
nested_command_buffer: VkBool32,
nested_command_buffer_rendering: VkBool32,
nested_command_buffer_simultaneous_use: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceNestedCommandBufferPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
max_command_buffer_nesting_level: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
shader_module_identifier: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
shader_module_identifier_algorithm_uuid: u8,
}

#[repr(C)]
pub struct VkPipelineShaderStageModuleIdentifierCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
identifier_size: u32,
p_identifier: u8,
}

#[repr(C)]
pub struct VkShaderModuleIdentifierEXT {
s_type: VkStructureType,
p_next: c_void,
identifier_size: u32,
identifier: u8,
}

#[repr(C)]
pub struct VkImageCompressionControlEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkImageCompressionFlagsEXT,
compression_control_plane_count: u32,
p_fixed_rate_flags: VkImageCompressionFixedRateFlagsEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageCompressionControlFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
image_compression_control: VkBool32,
}

#[repr(C)]
pub struct VkImageCompressionPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
image_compression_flags: VkImageCompressionFlagsEXT,
image_compression_fixed_rate_flags: VkImageCompressionFixedRateFlagsEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
image_compression_control_swapchain: VkBool32,
}

#[repr(C)]
pub struct VkImageSubresource2KHR {
s_type: VkStructureType,
p_next: c_void,
image_subresource: VkImageSubresource,
}

#[repr(C)]
pub struct VkSubresourceLayout2KHR {
s_type: VkStructureType,
p_next: c_void,
subresource_layout: VkSubresourceLayout,
}

#[repr(C)]
pub struct VkRenderPassCreationControlEXT {
s_type: VkStructureType,
p_next: c_void,
disallow_merging: VkBool32,
}

#[repr(C)]
pub struct VkRenderPassCreationFeedbackInfoEXT {
post_merge_subpass_count: u32,
}

#[repr(C)]
pub struct VkRenderPassCreationFeedbackCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
p_render_pass_feedback: VkRenderPassCreationFeedbackInfoEXT,
}

#[repr(C)]
pub struct VkRenderPassSubpassFeedbackInfoEXT {
subpass_merge_status: VkSubpassMergeStatusEXT,
description: u8,
post_merge_index: u32,
}

#[repr(C)]
pub struct VkRenderPassSubpassFeedbackCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
p_subpass_feedback: VkRenderPassSubpassFeedbackInfoEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
subpass_merge_feedback: VkBool32,
}

#[repr(C)]
pub struct VkMicromapBuildInfoEXT {
s_type: VkStructureType,
p_next: c_void,
r#type: VkMicromapTypeEXT,
flags: VkBuildMicromapFlagsEXT,
mode: VkBuildMicromapModeEXT,
dst_micromap: VkMicromapEXT,
usage_counts_count: u32,
p_usage_counts: VkMicromapUsageEXT,
pp_usage_counts: VkMicromapUsageEXT,
data: VkDeviceOrHostAddressConstKHR,
scratch_data: VkDeviceOrHostAddressKHR,
triangle_array: VkDeviceOrHostAddressConstKHR,
triangle_array_stride: VkDeviceSize,
}

#[repr(C)]
pub struct VkMicromapCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
create_flags: VkMicromapCreateFlagsEXT,
buffer: VkBuffer,
offset: VkDeviceSize,
size: VkDeviceSize,
r#type: VkMicromapTypeEXT,
device_address: VkDeviceAddress,
}

#[repr(C)]
pub struct VkMicromapVersionInfoEXT {
s_type: VkStructureType,
p_next: c_void,
p_version_data: u8,
}

#[repr(C)]
pub struct VkCopyMicromapInfoEXT {
s_type: VkStructureType,
p_next: c_void,
src: VkMicromapEXT,
dst: VkMicromapEXT,
mode: VkCopyMicromapModeEXT,
}

#[repr(C)]
pub struct VkCopyMicromapToMemoryInfoEXT {
s_type: VkStructureType,
p_next: c_void,
src: VkMicromapEXT,
dst: VkDeviceOrHostAddressKHR,
mode: VkCopyMicromapModeEXT,
}

#[repr(C)]
pub struct VkCopyMemoryToMicromapInfoEXT {
s_type: VkStructureType,
p_next: c_void,
src: VkDeviceOrHostAddressConstKHR,
dst: VkMicromapEXT,
mode: VkCopyMicromapModeEXT,
}

#[repr(C)]
pub struct VkMicromapBuildSizesInfoEXT {
s_type: VkStructureType,
p_next: c_void,
micromap_size: VkDeviceSize,
build_scratch_size: VkDeviceSize,
discardable: VkBool32,
}

#[repr(C)]
pub struct VkMicromapUsageEXT {
count: u32,
subdivision_level: u32,
format: u32,
}

#[repr(C)]
pub struct VkMicromapTriangleEXT {
data_offset: u32,
subdivision_level: u16,
format: u16,
}

#[repr(C)]
pub struct VkPhysicalDeviceOpacityMicromapFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
micromap: VkBool32,
micromap_capture_replay: VkBool32,
micromap_host_commands: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceOpacityMicromapPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
max_opacity2_state_subdivision_level: u32,
max_opacity4_state_subdivision_level: u32,
}

#[repr(C)]
pub struct VkAccelerationStructureTrianglesOpacityMicromapEXT {
s_type: VkStructureType,
p_next: c_void,
index_type: VkIndexType,
index_buffer: VkDeviceOrHostAddressConstKHR,
index_stride: VkDeviceSize,
base_triangle: u32,
usage_counts_count: u32,
p_usage_counts: VkMicromapUsageEXT,
pp_usage_counts: VkMicromapUsageEXT,
micromap: VkMicromapEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceDisplacementMicromapFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
displacement_micromap: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDisplacementMicromapPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
max_displacement_micromap_subdivision_level: u32,
}

#[repr(C)]
pub struct VkAccelerationStructureTrianglesDisplacementMicromapNV {
s_type: VkStructureType,
p_next: c_void,
displacement_bias_and_scale_format: VkFormat,
displacement_vector_format: VkFormat,
displacement_bias_and_scale_buffer: VkDeviceOrHostAddressConstKHR,
displacement_bias_and_scale_stride: VkDeviceSize,
displacement_vector_buffer: VkDeviceOrHostAddressConstKHR,
displacement_vector_stride: VkDeviceSize,
displaced_micromap_primitive_flags: VkDeviceOrHostAddressConstKHR,
displaced_micromap_primitive_flags_stride: VkDeviceSize,
index_type: VkIndexType,
index_buffer: VkDeviceOrHostAddressConstKHR,
index_stride: VkDeviceSize,
base_triangle: u32,
usage_counts_count: u32,
p_usage_counts: VkMicromapUsageEXT,
pp_usage_counts: VkMicromapUsageEXT,
micromap: VkMicromapEXT,
}

#[repr(C)]
pub struct VkPipelinePropertiesIdentifierEXT {
s_type: VkStructureType,
p_next: c_void,
pipeline_identifier: u8,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelinePropertiesFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
pipeline_properties_identifier: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {
s_type: VkStructureType,
p_next: c_void,
shader_early_and_late_fragment_tests: VkBool32,
}

#[repr(C)]
pub struct VkExternalMemoryAcquireUnmodifiedEXT {
s_type: VkStructureType,
p_next: c_void,
acquire_unmodified_memory: VkBool32,
}

#[repr(C)]
pub struct VkExportMetalObjectCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
export_object_type: VkExportMetalObjectTypeFlagBitsEXT,
}

#[repr(C)]
pub struct VkExportMetalObjectsInfoEXT {
s_type: VkStructureType,
p_next: c_void,
}

#[repr(C)]
pub struct VkExportMetalDeviceInfoEXT {
s_type: VkStructureType,
p_next: c_void,
mtl_device: MTLDevice_id,
}

#[repr(C)]
pub struct VkExportMetalCommandQueueInfoEXT {
s_type: VkStructureType,
p_next: c_void,
queue: VkQueue,
mtl_command_queue: MTLCommandQueue_id,
}

#[repr(C)]
pub struct VkExportMetalBufferInfoEXT {
s_type: VkStructureType,
p_next: c_void,
memory: VkDeviceMemory,
mtl_buffer: MTLBuffer_id,
}

#[repr(C)]
pub struct VkImportMetalBufferInfoEXT {
s_type: VkStructureType,
p_next: c_void,
mtl_buffer: MTLBuffer_id,
}

#[repr(C)]
pub struct VkExportMetalTextureInfoEXT {
s_type: VkStructureType,
p_next: c_void,
image: VkImage,
image_view: VkImageView,
buffer_view: VkBufferView,
plane: VkImageAspectFlagBits,
mtl_texture: MTLTexture_id,
}

#[repr(C)]
pub struct VkImportMetalTextureInfoEXT {
s_type: VkStructureType,
p_next: c_void,
plane: VkImageAspectFlagBits,
mtl_texture: MTLTexture_id,
}

#[repr(C)]
pub struct VkExportMetalIOSurfaceInfoEXT {
s_type: VkStructureType,
p_next: c_void,
image: VkImage,
io_surface: IOSurfaceRef,
}

#[repr(C)]
pub struct VkImportMetalIOSurfaceInfoEXT {
s_type: VkStructureType,
p_next: c_void,
io_surface: IOSurfaceRef,
}

#[repr(C)]
pub struct VkExportMetalSharedEventInfoEXT {
s_type: VkStructureType,
p_next: c_void,
semaphore: VkSemaphore,
event: VkEvent,
mtl_shared_event: MTLSharedEvent_id,
}

#[repr(C)]
pub struct VkImportMetalSharedEventInfoEXT {
s_type: VkStructureType,
p_next: c_void,
mtl_shared_event: MTLSharedEvent_id,
}

#[repr(C)]
pub struct VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
non_seamless_cube_map: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineRobustnessFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
pipeline_robustness: VkBool32,
}

#[repr(C)]
pub struct VkPipelineRobustnessCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
storage_buffers: VkPipelineRobustnessBufferBehaviorEXT,
uniform_buffers: VkPipelineRobustnessBufferBehaviorEXT,
vertex_inputs: VkPipelineRobustnessBufferBehaviorEXT,
images: VkPipelineRobustnessImageBehaviorEXT,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineRobustnessPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
default_robustness_storage_buffers: VkPipelineRobustnessBufferBehaviorEXT,
default_robustness_uniform_buffers: VkPipelineRobustnessBufferBehaviorEXT,
default_robustness_vertex_inputs: VkPipelineRobustnessBufferBehaviorEXT,
default_robustness_images: VkPipelineRobustnessImageBehaviorEXT,
}

#[repr(C)]
pub struct VkImageViewSampleWeightCreateInfoQCOM {
s_type: VkStructureType,
p_next: c_void,
filter_center: VkOffset2D,
filter_size: VkExtent2D,
num_phases: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageProcessingFeaturesQCOM {
s_type: VkStructureType,
p_next: c_void,
texture_sample_weighted: VkBool32,
texture_box_filter: VkBool32,
texture_block_match: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageProcessingPropertiesQCOM {
s_type: VkStructureType,
p_next: c_void,
max_weight_filter_phases: u32,
max_weight_filter_dimension: VkExtent2D,
max_block_match_region: VkExtent2D,
max_box_filter_block_size: VkExtent2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceTilePropertiesFeaturesQCOM {
s_type: VkStructureType,
p_next: c_void,
tile_properties: VkBool32,
}

#[repr(C)]
pub struct VkTilePropertiesQCOM {
s_type: VkStructureType,
p_next: c_void,
tile_size: VkExtent3D,
apron_size: VkExtent2D,
origin: VkOffset2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceAmigoProfilingFeaturesSEC {
s_type: VkStructureType,
p_next: c_void,
amigo_profiling: VkBool32,
}

#[repr(C)]
pub struct VkAmigoProfilingSubmitInfoSEC {
s_type: VkStructureType,
p_next: c_void,
first_draw_timestamp: u64,
swap_buffer_timestamp: u64,
}

#[repr(C)]
pub struct VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
attachment_feedback_loop_layout: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthClampZeroOneFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
depth_clamp_zero_one: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceAddressBindingReportFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
report_address_binding: VkBool32,
}

#[repr(C)]
pub struct VkDeviceAddressBindingCallbackDataEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkDeviceAddressBindingFlagsEXT,
base_address: VkDeviceAddress,
size: VkDeviceSize,
binding_type: VkDeviceAddressBindingTypeEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceOpticalFlowFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
optical_flow: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceOpticalFlowPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
supported_output_grid_sizes: VkOpticalFlowGridSizeFlagsNV,
supported_hint_grid_sizes: VkOpticalFlowGridSizeFlagsNV,
hint_supported: VkBool32,
cost_supported: VkBool32,
bidirectional_flow_supported: VkBool32,
global_flow_supported: VkBool32,
min_width: u32,
min_height: u32,
max_width: u32,
max_height: u32,
max_num_regions_of_interest: u32,
}

#[repr(C)]
pub struct VkOpticalFlowImageFormatInfoNV {
s_type: VkStructureType,
p_next: c_void,
usage: VkOpticalFlowUsageFlagsNV,
}

#[repr(C)]
pub struct VkOpticalFlowImageFormatPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
format: VkFormat,
}

#[repr(C)]
pub struct VkOpticalFlowSessionCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
width: u32,
height: u32,
image_format: VkFormat,
flow_vector_format: VkFormat,
cost_format: VkFormat,
output_grid_size: VkOpticalFlowGridSizeFlagsNV,
hint_grid_size: VkOpticalFlowGridSizeFlagsNV,
performance_level: VkOpticalFlowPerformanceLevelNV,
flags: VkOpticalFlowSessionCreateFlagsNV,
}

#[repr(C)]
pub struct VkOpticalFlowSessionCreatePrivateDataInfoNV {
s_type: VkStructureType,
p_next: c_void,
id: u32,
size: u32,
p_private_data: c_void,
}

#[repr(C)]
pub struct VkOpticalFlowExecuteInfoNV {
s_type: VkStructureType,
p_next: c_void,
flags: VkOpticalFlowExecuteFlagsNV,
region_count: u32,
p_regions: VkRect2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceFaultFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
device_fault: VkBool32,
device_fault_vendor_binary: VkBool32,
}

#[repr(C)]
pub struct VkDeviceFaultAddressInfoEXT {
address_type: VkDeviceFaultAddressTypeEXT,
reported_address: VkDeviceAddress,
address_precision: VkDeviceSize,
}

#[repr(C)]
pub struct VkDeviceFaultVendorInfoEXT {
description: u8,
vendor_fault_code: u64,
vendor_fault_data: u64,
}

#[repr(C)]
pub struct VkDeviceFaultCountsEXT {
s_type: VkStructureType,
p_next: c_void,
address_info_count: u32,
vendor_info_count: u32,
vendor_binary_size: VkDeviceSize,
}

#[repr(C)]
pub struct VkDeviceFaultInfoEXT {
s_type: VkStructureType,
p_next: c_void,
description: u8,
p_address_infos: VkDeviceFaultAddressInfoEXT,
p_vendor_infos: VkDeviceFaultVendorInfoEXT,
p_vendor_binary_data: c_void,
}

#[repr(C)]
pub struct VkDeviceFaultVendorBinaryHeaderVersionOneEXT {
header_size: u32,
header_version: VkDeviceFaultVendorBinaryHeaderVersionEXT,
vendor_id: u32,
device_id: u32,
driver_version: u32,
pipeline_cache_uuid: u8,
application_name_offset: u32,
application_version: u32,
engine_name_offset: u32,
engine_version: u32,
api_version: u32,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
pipeline_library_group_handles: VkBool32,
}

#[repr(C)]
pub struct VkDepthBiasInfoEXT {
s_type: VkStructureType,
p_next: c_void,
depth_bias_constant_factor: f32,
depth_bias_clamp: f32,
depth_bias_slope_factor: f32,
}

#[repr(C)]
pub struct VkDepthBiasRepresentationInfoEXT {
s_type: VkStructureType,
p_next: c_void,
depth_bias_representation: VkDepthBiasRepresentationEXT,
depth_bias_exact: VkBool32,
}

#[repr(C)]
pub struct VkDecompressMemoryRegionNV {
src_address: VkDeviceAddress,
dst_address: VkDeviceAddress,
compressed_size: VkDeviceSize,
decompressed_size: VkDeviceSize,
decompression_method: VkMemoryDecompressionMethodFlagsNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM {
s_type: VkStructureType,
p_next: c_void,
shader_core_mask: u64,
shader_core_count: u32,
shader_warps_per_core: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM {
s_type: VkStructureType,
p_next: c_void,
shader_core_builtins: VkBool32,
}

#[repr(C)]
pub struct VkFrameBoundaryEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkFrameBoundaryFlagsEXT,
frame_id: u64,
image_count: u32,
p_images: VkImage,
buffer_count: u32,
p_buffers: VkBuffer,
tag_name: u64,
tag_size: usize,
p_tag: c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceFrameBoundaryFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
frame_boundary: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
dynamic_rendering_unused_attachments: VkBool32,
}

#[repr(C)]
pub struct VkSurfacePresentModeEXT {
s_type: VkStructureType,
p_next: c_void,
present_mode: VkPresentModeKHR,
}

#[repr(C)]
pub struct VkSurfacePresentScalingCapabilitiesEXT {
s_type: VkStructureType,
p_next: c_void,
supported_present_scaling: VkPresentScalingFlagsEXT,
supported_present_gravity_x: VkPresentGravityFlagsEXT,
supported_present_gravity_y: VkPresentGravityFlagsEXT,
min_scaled_image_extent: VkExtent2D,
max_scaled_image_extent: VkExtent2D,
}

#[repr(C)]
pub struct VkSurfacePresentModeCompatibilityEXT {
s_type: VkStructureType,
p_next: c_void,
present_mode_count: u32,
p_present_modes: VkPresentModeKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
swapchain_maintenance1: VkBool32,
}

#[repr(C)]
pub struct VkSwapchainPresentFenceInfoEXT {
s_type: VkStructureType,
p_next: c_void,
swapchain_count: u32,
p_fences: VkFence,
}

#[repr(C)]
pub struct VkSwapchainPresentModesCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
present_mode_count: u32,
p_present_modes: VkPresentModeKHR,
}

#[repr(C)]
pub struct VkSwapchainPresentModeInfoEXT {
s_type: VkStructureType,
p_next: c_void,
swapchain_count: u32,
p_present_modes: VkPresentModeKHR,
}

#[repr(C)]
pub struct VkSwapchainPresentScalingCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
scaling_behavior: VkPresentScalingFlagsEXT,
present_gravity_x: VkPresentGravityFlagsEXT,
present_gravity_y: VkPresentGravityFlagsEXT,
}

#[repr(C)]
pub struct VkReleaseSwapchainImagesInfoEXT {
s_type: VkStructureType,
p_next: c_void,
swapchain: VkSwapchainKHR,
image_index_count: u32,
p_image_indices: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthBiasControlFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
depth_bias_control: VkBool32,
least_representable_value_force_unorm_representation: VkBool32,
float_representation: VkBool32,
depth_bias_exact: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
ray_tracing_invocation_reorder: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
ray_tracing_invocation_reorder_reordering_hint: VkRayTracingInvocationReorderModeNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
extended_sparse_address_space: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV {
s_type: VkStructureType,
p_next: c_void,
extended_sparse_address_space_size: VkDeviceSize,
extended_sparse_image_usage_flags: VkImageUsageFlags,
extended_sparse_buffer_usage_flags: VkBufferUsageFlags,
}

#[repr(C)]
pub struct VkDirectDriverLoadingInfoLUNARG {
s_type: VkStructureType,
p_next: c_void,
flags: VkDirectDriverLoadingFlagsLUNARG,
pfn_get_instance_proc_addr: PFN_vkGetInstanceProcAddrLUNARG,
}

#[repr(C)]
pub struct VkDirectDriverLoadingListLUNARG {
s_type: VkStructureType,
p_next: c_void,
mode: VkDirectDriverLoadingModeLUNARG,
driver_count: u32,
p_drivers: VkDirectDriverLoadingInfoLUNARG,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM {
s_type: VkStructureType,
p_next: c_void,
multiview_per_view_viewports: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
ray_tracing_position_fetch: VkBool32,
}

#[repr(C)]
pub struct VkDeviceImageSubresourceInfoKHR {
s_type: VkStructureType,
p_next: c_void,
p_create_info: VkImageCreateInfo,
p_subresource: VkImageSubresource2KHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderCorePropertiesARM {
s_type: VkStructureType,
p_next: c_void,
pixel_rate: u32,
texel_rate: u32,
fma_rate: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM {
s_type: VkStructureType,
p_next: c_void,
multiview_per_view_render_areas: VkBool32,
}

#[repr(C)]
pub struct VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM {
s_type: VkStructureType,
p_next: c_void,
per_view_render_area_count: u32,
p_per_view_render_areas: VkRect2D,
}

#[repr(C)]
pub struct VkQueryLowLatencySupportNV {
s_type: VkStructureType,
p_next: c_void,
p_queried_low_latency_data: c_void,
}

#[repr(C)]
pub struct VkMemoryMapInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkMemoryMapFlags,
memory: VkDeviceMemory,
offset: VkDeviceSize,
size: VkDeviceSize,
}

#[repr(C)]
pub struct VkMemoryUnmapInfoKHR {
s_type: VkStructureType,
p_next: c_void,
flags: VkMemoryUnmapFlagsKHR,
memory: VkDeviceMemory,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderObjectFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
shader_object: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderObjectPropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
shader_binary_uuid: u8,
shader_binary_version: u32,
}

#[repr(C)]
pub struct VkShaderCreateInfoEXT {
s_type: VkStructureType,
p_next: c_void,
flags: VkShaderCreateFlagsEXT,
stage: VkShaderStageFlagBits,
next_stage: VkShaderStageFlags,
code_type: VkShaderCodeTypeEXT,
code_size: usize,
p_code: c_void,
p_name: u8,
set_layout_count: u32,
p_set_layouts: VkDescriptorSetLayout,
push_constant_range_count: u32,
p_push_constant_ranges: VkPushConstantRange,
p_specialization_info: VkSpecializationInfo,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderTileImageFeaturesEXT {
s_type: VkStructureType,
p_next: c_void,
shader_tile_image_color_read_access: VkBool32,
shader_tile_image_depth_read_access: VkBool32,
shader_tile_image_stencil_read_access: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderTileImagePropertiesEXT {
s_type: VkStructureType,
p_next: c_void,
shader_tile_image_coherent_read_accelerated: VkBool32,
shader_tile_image_read_sample_from_pixel_rate_invocation: VkBool32,
shader_tile_image_read_from_helper_invocation: VkBool32,
}

#[repr(C)]
pub struct VkImportScreenBufferInfoQNX {
s_type: VkStructureType,
p_next: c_void,
buffer: _screen_buffer,
}

#[repr(C)]
pub struct VkScreenBufferPropertiesQNX {
s_type: VkStructureType,
p_next: c_void,
allocation_size: VkDeviceSize,
memory_type_bits: u32,
}

#[repr(C)]
pub struct VkScreenBufferFormatPropertiesQNX {
s_type: VkStructureType,
p_next: c_void,
format: VkFormat,
external_format: u64,
screen_usage: u64,
format_features: VkFormatFeatureFlags,
sampler_ycbcr_conversion_components: VkComponentMapping,
suggested_ycbcr_model: VkSamplerYcbcrModelConversion,
suggested_ycbcr_range: VkSamplerYcbcrRange,
suggested_xchroma_offset: VkChromaLocation,
suggested_ychroma_offset: VkChromaLocation,
}

#[repr(C)]
pub struct VkExternalFormatQNX {
s_type: VkStructureType,
p_next: c_void,
external_format: u64,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX {
s_type: VkStructureType,
p_next: c_void,
screen_buffer_import: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixFeaturesKHR {
s_type: VkStructureType,
p_next: c_void,
cooperative_matrix: VkBool32,
cooperative_matrix_robust_buffer_access: VkBool32,
}

#[repr(C)]
pub struct VkCooperativeMatrixPropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
_msize: u32,
_nsize: u32,
_ksize: u32,
_atype: VkComponentTypeKHR,
_btype: VkComponentTypeKHR,
_ctype: VkComponentTypeKHR,
_result_type: VkComponentTypeKHR,
saturating_accumulation: VkBool32,
scope: VkScopeKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixPropertiesKHR {
s_type: VkStructureType,
p_next: c_void,
cooperative_matrix_supported_stages: VkShaderStageFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderEnqueuePropertiesAMDX {
s_type: VkStructureType,
p_next: c_void,
max_execution_graph_depth: u32,
max_execution_graph_shader_output_nodes: u32,
max_execution_graph_shader_payload_size: u32,
max_execution_graph_shader_payload_count: u32,
execution_graph_dispatch_address_alignment: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderEnqueueFeaturesAMDX {
s_type: VkStructureType,
p_next: c_void,
shader_enqueue: VkBool32,
}

#[repr(C)]
pub struct VkExecutionGraphPipelineCreateInfoAMDX {
s_type: VkStructureType,
p_next: c_void,
flags: VkPipelineCreateFlags,
stage_count: u32,
p_stages: VkPipelineShaderStageCreateInfo,
p_library_info: VkPipelineLibraryCreateInfoKHR,
layout: VkPipelineLayout,
base_pipeline_handle: VkPipeline,
base_pipeline_index: i32,
}

#[repr(C)]
pub struct VkPipelineShaderStageNodeCreateInfoAMDX {
s_type: VkStructureType,
p_next: c_void,
p_name: u8,
index: u32,
}

#[repr(C)]
pub struct VkExecutionGraphPipelineScratchSizeAMDX {
s_type: VkStructureType,
p_next: c_void,
size: VkDeviceSize,
}

#[repr(C)]
pub struct VkDispatchGraphInfoAMDX {
node_index: u32,
payload_count: u32,
payloads: VkDeviceOrHostAddressConstAMDX,
payload_stride: u64,
}

#[repr(C)]
pub struct VkDispatchGraphCountInfoAMDX {
count: u32,
infos: VkDeviceOrHostAddressConstAMDX,
stride: u64,
}

#[repr(C)]
pub struct VkPhysicalDeviceCubicClampFeaturesQCOM {
s_type: VkStructureType,
p_next: c_void,
cubic_range_clamp: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceYcbcrDegammaFeaturesQCOM {
s_type: VkStructureType,
p_next: c_void,
ycbcr_degamma: VkBool32,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM {
s_type: VkStructureType,
p_next: c_void,
enable_ydegamma: VkBool32,
enable_cb_cr_degamma: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCubicWeightsFeaturesQCOM {
s_type: VkStructureType,
p_next: c_void,
selectable_cubic_weights: VkBool32,
}

#[repr(C)]
pub struct VkSamplerCubicWeightsCreateInfoQCOM {
s_type: VkStructureType,
p_next: c_void,
cubic_weights: VkCubicFilterWeightsQCOM,
}

#[repr(C)]
pub struct VkBlitImageCubicWeightsInfoQCOM {
s_type: VkStructureType,
p_next: c_void,
cubic_weights: VkCubicFilterWeightsQCOM,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageProcessing2FeaturesQCOM {
s_type: VkStructureType,
p_next: c_void,
texture_block_match2: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageProcessing2PropertiesQCOM {
s_type: VkStructureType,
p_next: c_void,
max_block_match_window: VkExtent2D,
}

#[repr(C)]
pub struct VkSamplerBlockMatchWindowCreateInfoQCOM {
s_type: VkStructureType,
p_next: c_void,
window_extent: VkExtent2D,
window_compare_mode: VkBlockMatchWindowCompareModeQCOM,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
descriptor_pool_overallocation: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceLayeredDriverPropertiesMSFT {
s_type: VkStructureType,
p_next: c_void,
underlying_api: VkLayeredDriverUnderlyingApiMSFT,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalFormatResolveFeaturesANDROID {
s_type: VkStructureType,
p_next: c_void,
external_format_resolve: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalFormatResolvePropertiesANDROID {
s_type: VkStructureType,
p_next: c_void,
null_color_attachment_with_external_format_resolve: VkBool32,
external_format_resolve_chroma_offset_x: VkChromaLocation,
external_format_resolve_chroma_offset_y: VkChromaLocation,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferFormatResolvePropertiesANDROID {
s_type: VkStructureType,
p_next: c_void,
color_attachment_format: VkFormat,
}

#[repr(C)]
pub struct VkLatencySleepModeInfoNV {
s_type: VkStructureType,
p_next: c_void,
low_latency_mode: VkBool32,
low_latency_boost: VkBool32,
minimum_interval_us: u32,
}

#[repr(C)]
pub struct VkLatencySleepInfoNV {
s_type: VkStructureType,
p_next: c_void,
signal_semaphore: VkSemaphore,
value: u64,
}

#[repr(C)]
pub struct VkSetLatencyMarkerInfoNV {
s_type: VkStructureType,
p_next: c_void,
present_id: u64,
marker: VkLatencyMarkerNV,
}

#[repr(C)]
pub struct VkGetLatencyMarkerInfoNV {
s_type: VkStructureType,
p_next: c_void,
p_timings: VkLatencyTimingsFrameReportNV,
}

#[repr(C)]
pub struct VkLatencyTimingsFrameReportNV {
s_type: VkStructureType,
p_next: c_void,
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
pub struct VkOutOfBandQueueTypeInfoNV {
s_type: VkStructureType,
p_next: c_void,
queue_type: VkOutOfBandQueueTypeNV,
}

#[repr(C)]
pub struct VkLatencySubmissionPresentIdNV {
s_type: VkStructureType,
p_next: c_void,
present_id: u64,
}

#[repr(C)]
pub struct VkSwapchainLatencyCreateInfoNV {
s_type: VkStructureType,
p_next: c_void,
latency_mode_enable: VkBool32,
}

#[repr(C)]
pub struct VkLatencySurfaceCapabilitiesNV {
s_type: VkStructureType,
p_next: c_void,
present_mode_count: u32,
p_present_modes: VkPresentModeKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceCudaKernelLaunchFeaturesNV {
s_type: VkStructureType,
p_next: c_void,
cuda_kernel_launch_features: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCudaKernelLaunchPropertiesNV {
s_type: VkStructureType,
p_next: c_void,
compute_capability_minor: u32,
compute_capability_major: u32,
}

#[repr(C)]
pub struct VkDeviceQueueShaderCoreControlCreateInfoARM {
s_type: VkStructureType,
p_next: c_void,
shader_core_count: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSchedulingControlsFeaturesARM {
s_type: VkStructureType,
p_next: c_void,
scheduling_controls: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSchedulingControlsPropertiesARM {
s_type: VkStructureType,
p_next: c_void,
scheduling_controls_flags: VkPhysicalDeviceSchedulingControlsFlagsARM,
}

#[repr(C)]
pub struct VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG {
s_type: VkStructureType,
p_next: c_void,
relaxed_line_rasterization: VkBool32,
}

