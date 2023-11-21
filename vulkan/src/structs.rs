#[repr(C)]
pub struct VkBaseOutStructure {
s_type: bool,
p_next: bool,
}

#[repr(C)]
pub struct VkBaseInStructure {
s_type: bool,
p_next: bool,
}

#[repr(C)]
pub struct VkOffset2D {
x: bool,
y: bool,
}

#[repr(C)]
pub struct VkOffset3D {
x: bool,
y: bool,
z: bool,
}

#[repr(C)]
pub struct VkExtent2D {
width: bool,
height: bool,
}

#[repr(C)]
pub struct VkExtent3D {
width: bool,
height: bool,
depth: bool,
}

#[repr(C)]
pub struct VkViewport {
x: bool,
y: bool,
width: bool,
height: bool,
min_depth: bool,
max_depth: bool,
}

#[repr(C)]
pub struct VkRect2D {
offset: bool,
extent: bool,
}

#[repr(C)]
pub struct VkClearRect {
rect: bool,
base_array_layer: bool,
layer_count: bool,
}

#[repr(C)]
pub struct VkComponentMapping {
r: bool,
g: bool,
b: bool,
a: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceProperties {
api_version: bool,
driver_version: bool,
vendor_id: bool,
device_id: bool,
device_type: bool,
device_name: bool,
pipeline_cache_uuid: bool,
limits: bool,
sparse_properties: bool,
}

#[repr(C)]
pub struct VkExtensionProperties {
extension_name: bool,
spec_version: bool,
}

#[repr(C)]
pub struct VkLayerProperties {
layer_name: bool,
spec_version: bool,
implementation_version: bool,
description: bool,
}

#[repr(C)]
pub struct VkApplicationInfo {
s_type: bool,
p_next: bool,
p_application_name: bool,
application_version: bool,
p_engine_name: bool,
engine_version: bool,
api_version: bool,
}

#[repr(C)]
pub struct VkAllocationCallbacks {
p_user_data: bool,
pfn_allocation: bool,
pfn_reallocation: bool,
pfn_free: bool,
pfn_internal_allocation: bool,
pfn_internal_free: bool,
}

#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
queue_family_index: bool,
queue_count: bool,
p_queue_priorities: bool,
}

#[repr(C)]
pub struct VkDeviceCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
queue_create_info_count: bool,
p_queue_create_infos: bool,
#[deprecated(note = "Ignored")]
enabled_layer_count: bool,
#[deprecated(note = "Ignored")]
pp_enabled_layer_names: bool,
enabled_extension_count: bool,
pp_enabled_extension_names: bool,
p_enabled_features: bool,
}

#[repr(C)]
pub struct VkInstanceCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
p_application_info: bool,
enabled_layer_count: bool,
pp_enabled_layer_names: bool,
enabled_extension_count: bool,
pp_enabled_extension_names: bool,
}

#[repr(C)]
pub struct VkQueueFamilyProperties {
queue_flags: bool,
queue_count: bool,
timestamp_valid_bits: bool,
min_image_transfer_granularity: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
memory_type_count: bool,
memory_types: bool,
memory_heap_count: bool,
memory_heaps: bool,
}

#[repr(C)]
pub struct VkMemoryAllocateInfo {
s_type: bool,
p_next: bool,
allocation_size: bool,
memory_type_index: bool,
}

#[repr(C)]
pub struct VkMemoryRequirements {
size: bool,
alignment: bool,
memory_type_bits: bool,
}

#[repr(C)]
pub struct VkSparseImageFormatProperties {
aspect_mask: bool,
image_granularity: bool,
flags: bool,
}

#[repr(C)]
pub struct VkSparseImageMemoryRequirements {
format_properties: bool,
image_mip_tail_first_lod: bool,
image_mip_tail_size: bool,
image_mip_tail_offset: bool,
image_mip_tail_stride: bool,
}

#[repr(C)]
pub struct VkMemoryType {
property_flags: bool,
heap_index: bool,
}

#[repr(C)]
pub struct VkMemoryHeap {
size: bool,
flags: bool,
}

#[repr(C)]
pub struct VkMappedMemoryRange {
s_type: bool,
p_next: bool,
memory: bool,
offset: bool,
size: bool,
}

#[repr(C)]
pub struct VkFormatProperties {
linear_tiling_features: bool,
optimal_tiling_features: bool,
buffer_features: bool,
}

#[repr(C)]
pub struct VkImageFormatProperties {
max_extent: bool,
max_mip_levels: bool,
max_array_layers: bool,
sample_counts: bool,
max_resource_size: bool,
}

#[repr(C)]
pub struct VkDescriptorBufferInfo {
buffer: bool,
offset: bool,
range: bool,
}

#[repr(C)]
pub struct VkDescriptorImageInfo {
sampler: bool,
image_view: bool,
image_layout: bool,
}

#[repr(C)]
pub struct VkWriteDescriptorSet {
s_type: bool,
p_next: bool,
dst_set: bool,
dst_binding: bool,
dst_array_element: bool,
descriptor_count: bool,
descriptor_type: bool,
p_image_info: bool,
p_buffer_info: bool,
p_texel_buffer_view: bool,
}

#[repr(C)]
pub struct VkCopyDescriptorSet {
s_type: bool,
p_next: bool,
src_set: bool,
src_binding: bool,
src_array_element: bool,
dst_set: bool,
dst_binding: bool,
dst_array_element: bool,
descriptor_count: bool,
}

#[repr(C)]
pub struct VkBufferUsageFlags2CreateInfoKHR {
s_type: bool,
p_next: bool,
usage: bool,
}

#[repr(C)]
pub struct VkBufferCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
size: bool,
usage: bool,
sharing_mode: bool,
queue_family_index_count: bool,
p_queue_family_indices: bool,
}

#[repr(C)]
pub struct VkBufferViewCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
buffer: bool,
format: bool,
offset: bool,
range: bool,
}

#[repr(C)]
pub struct VkImageSubresource {
aspect_mask: bool,
mip_level: bool,
array_layer: bool,
}

#[repr(C)]
pub struct VkImageSubresourceLayers {
aspect_mask: bool,
mip_level: bool,
base_array_layer: bool,
layer_count: bool,
}

#[repr(C)]
pub struct VkImageSubresourceRange {
aspect_mask: bool,
base_mip_level: bool,
level_count: bool,
base_array_layer: bool,
layer_count: bool,
}

#[repr(C)]
pub struct VkMemoryBarrier {
s_type: bool,
p_next: bool,
src_access_mask: bool,
dst_access_mask: bool,
}

#[repr(C)]
pub struct VkBufferMemoryBarrier {
s_type: bool,
p_next: bool,
src_access_mask: bool,
dst_access_mask: bool,
src_queue_family_index: bool,
dst_queue_family_index: bool,
buffer: bool,
offset: bool,
size: bool,
}

#[repr(C)]
pub struct VkImageMemoryBarrier {
s_type: bool,
p_next: bool,
src_access_mask: bool,
dst_access_mask: bool,
old_layout: bool,
new_layout: bool,
src_queue_family_index: bool,
dst_queue_family_index: bool,
image: bool,
subresource_range: bool,
}

#[repr(C)]
pub struct VkImageCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
image_type: bool,
format: bool,
extent: bool,
mip_levels: bool,
array_layers: bool,
samples: bool,
tiling: bool,
usage: bool,
sharing_mode: bool,
queue_family_index_count: bool,
p_queue_family_indices: bool,
initial_layout: bool,
}

#[repr(C)]
pub struct VkSubresourceLayout {
offset: bool,
size: bool,
row_pitch: bool,
array_pitch: bool,
depth_pitch: bool,
}

#[repr(C)]
pub struct VkImageViewCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
image: bool,
view_type: bool,
format: bool,
components: bool,
subresource_range: bool,
}

#[repr(C)]
pub struct VkBufferCopy {
src_offset: bool,
dst_offset: bool,
size: bool,
}

#[repr(C)]
pub struct VkSparseMemoryBind {
resource_offset: bool,
size: bool,
memory: bool,
memory_offset: bool,
flags: bool,
}

#[repr(C)]
pub struct VkSparseImageMemoryBind {
subresource: bool,
offset: bool,
extent: bool,
memory: bool,
memory_offset: bool,
flags: bool,
}

#[repr(C)]
pub struct VkSparseBufferMemoryBindInfo {
buffer: bool,
bind_count: bool,
p_binds: bool,
}

#[repr(C)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
image: bool,
bind_count: bool,
p_binds: bool,
}

#[repr(C)]
pub struct VkSparseImageMemoryBindInfo {
image: bool,
bind_count: bool,
p_binds: bool,
}

#[repr(C)]
pub struct VkBindSparseInfo {
s_type: bool,
p_next: bool,
wait_semaphore_count: bool,
p_wait_semaphores: bool,
buffer_bind_count: bool,
p_buffer_binds: bool,
image_opaque_bind_count: bool,
p_image_opaque_binds: bool,
image_bind_count: bool,
p_image_binds: bool,
signal_semaphore_count: bool,
p_signal_semaphores: bool,
}

#[repr(C)]
pub struct VkImageCopy {
src_subresource: bool,
src_offset: bool,
dst_subresource: bool,
dst_offset: bool,
extent: bool,
}

#[repr(C)]
pub struct VkImageBlit {
src_subresource: bool,
src_offsets: bool,
dst_subresource: bool,
dst_offsets: bool,
}

#[repr(C)]
pub struct VkBufferImageCopy {
buffer_offset: bool,
buffer_row_length: bool,
buffer_image_height: bool,
image_subresource: bool,
image_offset: bool,
image_extent: bool,
}

#[repr(C)]
pub struct VkCopyMemoryIndirectCommandNV {
src_address: bool,
dst_address: bool,
size: bool,
}

#[repr(C)]
pub struct VkCopyMemoryToImageIndirectCommandNV {
src_address: bool,
buffer_row_length: bool,
buffer_image_height: bool,
image_subresource: bool,
image_offset: bool,
image_extent: bool,
}

#[repr(C)]
pub struct VkImageResolve {
src_subresource: bool,
src_offset: bool,
dst_subresource: bool,
dst_offset: bool,
extent: bool,
}

#[repr(C)]
pub struct VkShaderModuleCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
code_size: bool,
p_code: bool,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutBinding {
binding: bool,
descriptor_type: bool,
descriptor_count: bool,
stage_flags: bool,
p_immutable_samplers: bool,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
binding_count: bool,
p_bindings: bool,
}

#[repr(C)]
pub struct VkDescriptorPoolSize {
r#type: bool,
descriptor_count: bool,
}

#[repr(C)]
pub struct VkDescriptorPoolCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
max_sets: bool,
pool_size_count: bool,
p_pool_sizes: bool,
}

#[repr(C)]
pub struct VkDescriptorSetAllocateInfo {
s_type: bool,
p_next: bool,
descriptor_pool: bool,
descriptor_set_count: bool,
p_set_layouts: bool,
}

#[repr(C)]
pub struct VkSpecializationMapEntry {
ant_id: bool,
offset: bool,
size: bool,
}

#[repr(C)]
pub struct VkSpecializationInfo {
map_entry_count: bool,
p_map_entries: bool,
data_size: bool,
p_data: bool,
}

#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
stage: bool,
module: bool,
#[cfg(vulkan)]
p_name: bool,
#[cfg(vulkansc)]
p_name: bool,
p_specialization_info: bool,
}

#[repr(C)]
pub struct VkComputePipelineCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
stage: bool,
layout: bool,
base_pipeline_handle: bool,
base_pipeline_index: bool,
}

#[repr(C)]
pub struct VkComputePipelineIndirectBufferInfoNV {
s_type: bool,
p_next: bool,
device_address: bool,
size: bool,
pipeline_device_address_capture_replay: bool,
}

#[repr(C)]
pub struct VkPipelineCreateFlags2CreateInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
}

#[repr(C)]
pub struct VkVertexInputBindingDescription {
binding: bool,
stride: bool,
input_rate: bool,
}

#[repr(C)]
pub struct VkVertexInputAttributeDescription {
location: bool,
binding: bool,
format: bool,
offset: bool,
}

#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
vertex_binding_description_count: bool,
p_vertex_binding_descriptions: bool,
vertex_attribute_description_count: bool,
p_vertex_attribute_descriptions: bool,
}

#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
topology: bool,
primitive_restart_enable: bool,
}

#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
patch_control_points: bool,
}

#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
viewport_count: bool,
p_viewports: bool,
scissor_count: bool,
p_scissors: bool,
}

#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
depth_clamp_enable: bool,
rasterizer_discard_enable: bool,
polygon_mode: bool,
cull_mode: bool,
front_face: bool,
depth_bias_enable: bool,
depth_bias_constant_factor: bool,
depth_bias_clamp: bool,
depth_bias_slope_factor: bool,
line_width: bool,
}

#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
rasterization_samples: bool,
sample_shading_enable: bool,
min_sample_shading: bool,
p_sample_mask: bool,
alpha_to_coverage_enable: bool,
alpha_to_one_enable: bool,
}

#[repr(C)]
pub struct VkPipelineColorBlendAttachmentState {
blend_enable: bool,
src_color_blend_factor: bool,
dst_color_blend_factor: bool,
color_blend_op: bool,
src_alpha_blend_factor: bool,
dst_alpha_blend_factor: bool,
alpha_blend_op: bool,
color_write_mask: bool,
}

#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
logic_op_enable: bool,
logic_op: bool,
attachment_count: bool,
p_attachments: bool,
blend_constants: bool,
}

#[repr(C)]
pub struct VkPipelineDynamicStateCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
dynamic_state_count: bool,
p_dynamic_states: bool,
}

#[repr(C)]
pub struct VkStencilOpState {
fail_op: bool,
pass_op: bool,
depth_fail_op: bool,
compare_op: bool,
compare_mask: bool,
write_mask: bool,
reference: bool,
}

#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
depth_test_enable: bool,
depth_write_enable: bool,
depth_compare_op: bool,
depth_bounds_test_enable: bool,
stencil_test_enable: bool,
front: bool,
back: bool,
min_depth_bounds: bool,
max_depth_bounds: bool,
}

#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
stage_count: bool,
#[cfg(vulkan)]
p_stages: bool,
#[cfg(vulkansc)]
p_stages: bool,
p_vertex_input_state: bool,
p_input_assembly_state: bool,
p_tessellation_state: bool,
p_viewport_state: bool,
p_rasterization_state: bool,
p_multisample_state: bool,
p_depth_stencil_state: bool,
p_color_blend_state: bool,
p_dynamic_state: bool,
layout: bool,
render_pass: bool,
subpass: bool,
base_pipeline_handle: bool,
base_pipeline_index: bool,
}

#[repr(C)]
pub struct VkPipelineCacheCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
#[cfg(vulkan)]
initial_data_size: bool,
#[cfg(vulkansc)]
initial_data_size: bool,
p_initial_data: bool,
}

#[repr(C)]
pub struct VkPipelineCacheHeaderVersionOne {
header_size: bool,
header_version: bool,
vendor_id: bool,
device_id: bool,
pipeline_cache_uuid: bool,
}

#[repr(C)]
pub struct VkPipelineCacheStageValidationIndexEntry {
code_size: bool,
code_offset: bool,
}

#[repr(C)]
pub struct VkPipelineCacheSafetyCriticalIndexEntry {
pipeline_identifier: bool,
pipeline_memory_size: bool,
json_size: bool,
json_offset: bool,
stage_index_count: bool,
stage_index_stride: bool,
stage_index_offset: bool,
}

#[repr(C)]
pub struct VkPipelineCacheHeaderVersionSafetyCriticalOne {
header_version_one: bool,
validation_version: bool,
implementation_data: bool,
pipeline_index_count: bool,
pipeline_index_stride: bool,
pipeline_index_offset: bool,
}

#[repr(C)]
pub struct VkPushConstantRange {
stage_flags: bool,
offset: bool,
size: bool,
}

#[repr(C)]
pub struct VkPipelineLayoutCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
set_layout_count: bool,
p_set_layouts: bool,
push_constant_range_count: bool,
p_push_constant_ranges: bool,
}

#[repr(C)]
pub struct VkSamplerCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
mag_filter: bool,
min_filter: bool,
mipmap_mode: bool,
address_mode_u: bool,
address_mode_v: bool,
address_mode_w: bool,
mip_lod_bias: bool,
anisotropy_enable: bool,
max_anisotropy: bool,
compare_enable: bool,
compare_op: bool,
min_lod: bool,
max_lod: bool,
border_color: bool,
unnormalized_coordinates: bool,
}

#[repr(C)]
pub struct VkCommandPoolCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
queue_family_index: bool,
}

#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
s_type: bool,
p_next: bool,
command_pool: bool,
level: bool,
command_buffer_count: bool,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceInfo {
s_type: bool,
p_next: bool,
render_pass: bool,
subpass: bool,
framebuffer: bool,
occlusion_query_enable: bool,
query_flags: bool,
pipeline_statistics: bool,
}

#[repr(C)]
pub struct VkCommandBufferBeginInfo {
s_type: bool,
p_next: bool,
flags: bool,
p_inheritance_info: bool,
}

#[repr(C)]
pub struct VkRenderPassBeginInfo {
s_type: bool,
p_next: bool,
render_pass: bool,
framebuffer: bool,
render_area: bool,
clear_value_count: bool,
p_clear_values: bool,
}

#[repr(C)]
pub struct VkClearDepthStencilValue {
depth: bool,
stencil: bool,
}

#[repr(C)]
pub struct VkClearAttachment {
aspect_mask: bool,
color_attachment: bool,
clear_value: bool,
}

#[repr(C)]
pub struct VkAttachmentDescription {
flags: bool,
format: bool,
samples: bool,
load_op: bool,
store_op: bool,
stencil_load_op: bool,
stencil_store_op: bool,
initial_layout: bool,
final_layout: bool,
}

#[repr(C)]
pub struct VkAttachmentReference {
attachment: bool,
layout: bool,
}

#[repr(C)]
pub struct VkSubpassDescription {
flags: bool,
pipeline_bind_point: bool,
input_attachment_count: bool,
p_input_attachments: bool,
color_attachment_count: bool,
p_color_attachments: bool,
p_resolve_attachments: bool,
p_depth_stencil_attachment: bool,
preserve_attachment_count: bool,
p_preserve_attachments: bool,
}

#[repr(C)]
pub struct VkSubpassDependency {
src_subpass: bool,
dst_subpass: bool,
src_stage_mask: bool,
dst_stage_mask: bool,
src_access_mask: bool,
dst_access_mask: bool,
dependency_flags: bool,
}

#[repr(C)]
pub struct VkRenderPassCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
attachment_count: bool,
p_attachments: bool,
subpass_count: bool,
p_subpasses: bool,
dependency_count: bool,
p_dependencies: bool,
}

#[repr(C)]
pub struct VkEventCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
}

#[repr(C)]
pub struct VkFenceCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFeatures {
robust_buffer_access: bool,
full_draw_index_uint32: bool,
image_cube_array: bool,
independent_blend: bool,
geometry_shader: bool,
tessellation_shader: bool,
sample_rate_shading: bool,
dual_src_blend: bool,
logic_op: bool,
multi_draw_indirect: bool,
draw_indirect_first_instance: bool,
depth_clamp: bool,
depth_bias_clamp: bool,
fill_mode_non_solid: bool,
depth_bounds: bool,
wide_lines: bool,
large_points: bool,
alpha_to_one: bool,
multi_viewport: bool,
sampler_anisotropy: bool,
texture_compression_etc2: bool,
texture_compression_astc_ldr: bool,
texture_compression_bc: bool,
occlusion_query_precise: bool,
pipeline_statistics_query: bool,
vertex_pipeline_stores_and_atomics: bool,
fragment_stores_and_atomics: bool,
shader_tessellation_and_geometry_point_size: bool,
shader_image_gather_extended: bool,
shader_storage_image_extended_formats: bool,
shader_storage_image_multisample: bool,
shader_storage_image_read_without_format: bool,
shader_storage_image_write_without_format: bool,
shader_uniform_buffer_array_dynamic_indexing: bool,
shader_sampled_image_array_dynamic_indexing: bool,
shader_storage_buffer_array_dynamic_indexing: bool,
shader_storage_image_array_dynamic_indexing: bool,
shader_clip_distance: bool,
shader_cull_distance: bool,
shader_float64: bool,
shader_int64: bool,
shader_int16: bool,
shader_resource_residency: bool,
shader_resource_min_lod: bool,
sparse_binding: bool,
sparse_residency_buffer: bool,
sparse_residency_image2_d: bool,
sparse_residency_image3_d: bool,
sparse_residency2_samples: bool,
sparse_residency4_samples: bool,
sparse_residency8_samples: bool,
sparse_residency16_samples: bool,
sparse_residency_aliased: bool,
variable_multisample_rate: bool,
inherited_queries: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
residency_standard2_dblock_shape: bool,
residency_standard2_dmultisample_block_shape: bool,
residency_standard3_dblock_shape: bool,
residency_aligned_mip_size: bool,
residency_non_resident_strict: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceLimits {
max_image_dimension1_d: bool,
max_image_dimension2_d: bool,
max_image_dimension3_d: bool,
max_image_dimension_cube: bool,
max_image_array_layers: bool,
max_texel_buffer_elements: bool,
max_uniform_buffer_range: bool,
max_storage_buffer_range: bool,
max_push_constants_size: bool,
max_memory_allocation_count: bool,
max_sampler_allocation_count: bool,
buffer_image_granularity: bool,
sparse_address_space_size: bool,
max_bound_descriptor_sets: bool,
max_per_stage_descriptor_samplers: bool,
max_per_stage_descriptor_uniform_buffers: bool,
max_per_stage_descriptor_storage_buffers: bool,
max_per_stage_descriptor_sampled_images: bool,
max_per_stage_descriptor_storage_images: bool,
max_per_stage_descriptor_input_attachments: bool,
max_per_stage_resources: bool,
max_descriptor_set_samplers: bool,
max_descriptor_set_uniform_buffers: bool,
max_descriptor_set_uniform_buffers_dynamic: bool,
max_descriptor_set_storage_buffers: bool,
max_descriptor_set_storage_buffers_dynamic: bool,
max_descriptor_set_sampled_images: bool,
max_descriptor_set_storage_images: bool,
max_descriptor_set_input_attachments: bool,
max_vertex_input_attributes: bool,
max_vertex_input_bindings: bool,
max_vertex_input_attribute_offset: bool,
max_vertex_input_binding_stride: bool,
max_vertex_output_components: bool,
max_tessellation_generation_level: bool,
max_tessellation_patch_size: bool,
max_tessellation_control_per_vertex_input_components: bool,
max_tessellation_control_per_vertex_output_components: bool,
max_tessellation_control_per_patch_output_components: bool,
max_tessellation_control_total_output_components: bool,
max_tessellation_evaluation_input_components: bool,
max_tessellation_evaluation_output_components: bool,
max_geometry_shader_invocations: bool,
max_geometry_input_components: bool,
max_geometry_output_components: bool,
max_geometry_output_vertices: bool,
max_geometry_total_output_components: bool,
max_fragment_input_components: bool,
max_fragment_output_attachments: bool,
max_fragment_dual_src_attachments: bool,
max_fragment_combined_output_resources: bool,
max_compute_shared_memory_size: bool,
max_compute_work_group_count: bool,
max_compute_work_group_invocations: bool,
max_compute_work_group_size: bool,
sub_pixel_precision_bits: bool,
sub_texel_precision_bits: bool,
mipmap_precision_bits: bool,
max_draw_indexed_index_value: bool,
max_draw_indirect_count: bool,
max_sampler_lod_bias: bool,
max_sampler_anisotropy: bool,
max_viewports: bool,
max_viewport_dimensions: bool,
viewport_bounds_range: bool,
viewport_sub_pixel_bits: bool,
min_memory_map_alignment: bool,
min_texel_buffer_offset_alignment: bool,
min_uniform_buffer_offset_alignment: bool,
min_storage_buffer_offset_alignment: bool,
min_texel_offset: bool,
max_texel_offset: bool,
min_texel_gather_offset: bool,
max_texel_gather_offset: bool,
min_interpolation_offset: bool,
max_interpolation_offset: bool,
sub_pixel_interpolation_offset_bits: bool,
max_framebuffer_width: bool,
max_framebuffer_height: bool,
max_framebuffer_layers: bool,
framebuffer_color_sample_counts: bool,
framebuffer_depth_sample_counts: bool,
framebuffer_stencil_sample_counts: bool,
framebuffer_no_attachments_sample_counts: bool,
max_color_attachments: bool,
sampled_image_color_sample_counts: bool,
sampled_image_integer_sample_counts: bool,
sampled_image_depth_sample_counts: bool,
sampled_image_stencil_sample_counts: bool,
storage_image_sample_counts: bool,
max_sample_mask_words: bool,
timestamp_compute_and_graphics: bool,
timestamp_period: bool,
max_clip_distances: bool,
max_cull_distances: bool,
max_combined_clip_and_cull_distances: bool,
discrete_queue_priorities: bool,
point_size_range: bool,
line_width_range: bool,
point_size_granularity: bool,
line_width_granularity: bool,
strict_lines: bool,
standard_sample_locations: bool,
optimal_buffer_copy_offset_alignment: bool,
optimal_buffer_copy_row_pitch_alignment: bool,
non_coherent_atom_size: bool,
}

#[repr(C)]
pub struct VkSemaphoreCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
}

#[repr(C)]
pub struct VkQueryPoolCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
query_type: bool,
query_count: bool,
pipeline_statistics: bool,
}

#[repr(C)]
pub struct VkFramebufferCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
render_pass: bool,
attachment_count: bool,
p_attachments: bool,
width: bool,
height: bool,
layers: bool,
}

#[repr(C)]
pub struct VkDrawIndirectCommand {
vertex_count: bool,
instance_count: bool,
first_vertex: bool,
first_instance: bool,
}

#[repr(C)]
pub struct VkDrawIndexedIndirectCommand {
index_count: bool,
instance_count: bool,
first_index: bool,
vertex_offset: bool,
first_instance: bool,
}

#[repr(C)]
pub struct VkDispatchIndirectCommand {
x: bool,
y: bool,
z: bool,
}

#[repr(C)]
pub struct VkMultiDrawInfoEXT {
first_vertex: bool,
vertex_count: bool,
}

#[repr(C)]
pub struct VkMultiDrawIndexedInfoEXT {
first_index: bool,
index_count: bool,
vertex_offset: bool,
}

#[repr(C)]
pub struct VkSubmitInfo {
s_type: bool,
p_next: bool,
wait_semaphore_count: bool,
p_wait_semaphores: bool,
p_wait_dst_stage_mask: bool,
command_buffer_count: bool,
p_command_buffers: bool,
signal_semaphore_count: bool,
p_signal_semaphores: bool,
}

#[repr(C)]
pub struct VkDisplayPropertiesKHR {
display: bool,
display_name: bool,
physical_dimensions: bool,
physical_resolution: bool,
supported_transforms: bool,
plane_reorder_possible: bool,
persistent_content: bool,
}

#[repr(C)]
pub struct VkDisplayPlanePropertiesKHR {
current_display: bool,
current_stack_index: bool,
}

#[repr(C)]
pub struct VkDisplayModeParametersKHR {
visible_region: bool,
refresh_rate: bool,
}

#[repr(C)]
pub struct VkDisplayModePropertiesKHR {
display_mode: bool,
parameters: bool,
}

#[repr(C)]
pub struct VkDisplayModeCreateInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
parameters: bool,
}

#[repr(C)]
pub struct VkDisplayPlaneCapabilitiesKHR {
supported_alpha: bool,
min_src_position: bool,
max_src_position: bool,
min_src_extent: bool,
max_src_extent: bool,
min_dst_position: bool,
max_dst_position: bool,
min_dst_extent: bool,
max_dst_extent: bool,
}

#[repr(C)]
pub struct VkDisplaySurfaceCreateInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
display_mode: bool,
plane_index: bool,
plane_stack_index: bool,
transform: bool,
global_alpha: bool,
alpha_mode: bool,
image_extent: bool,
}

#[repr(C)]
pub struct VkDisplayPresentInfoKHR {
s_type: bool,
p_next: bool,
src_rect: bool,
dst_rect: bool,
persistent: bool,
}

#[repr(C)]
pub struct VkSurfaceCapabilitiesKHR {
min_image_count: bool,
max_image_count: bool,
current_extent: bool,
min_image_extent: bool,
max_image_extent: bool,
max_image_array_layers: bool,
supported_transforms: bool,
current_transform: bool,
supported_composite_alpha: bool,
supported_usage_flags: bool,
}

#[repr(C)]
pub struct VkAndroidSurfaceCreateInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
window: bool,
}

#[repr(C)]
pub struct VkViSurfaceCreateInfoNN {
s_type: bool,
p_next: bool,
flags: bool,
window: bool,
}

#[repr(C)]
pub struct VkWaylandSurfaceCreateInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
display: bool,
surface: bool,
}

#[repr(C)]
pub struct VkWin32SurfaceCreateInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
hinstance: bool,
hwnd: bool,
}

#[repr(C)]
pub struct VkXlibSurfaceCreateInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
dpy: bool,
window: bool,
}

#[repr(C)]
pub struct VkXcbSurfaceCreateInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
connection: bool,
window: bool,
}

#[repr(C)]
pub struct VkDirectFBSurfaceCreateInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
dfb: bool,
surface: bool,
}

#[repr(C)]
pub struct VkImagePipeSurfaceCreateInfoFUCHSIA {
s_type: bool,
p_next: bool,
flags: bool,
image_pipe_handle: bool,
}

#[repr(C)]
pub struct VkStreamDescriptorSurfaceCreateInfoGGP {
s_type: bool,
p_next: bool,
flags: bool,
stream_descriptor: bool,
}

#[repr(C)]
pub struct VkScreenSurfaceCreateInfoQNX {
s_type: bool,
p_next: bool,
flags: bool,
context: bool,
window: bool,
}

#[repr(C)]
pub struct VkSurfaceFormatKHR {
format: bool,
color_space: bool,
}

#[repr(C)]
pub struct VkSwapchainCreateInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
surface: bool,
min_image_count: bool,
image_format: bool,
image_color_space: bool,
image_extent: bool,
image_array_layers: bool,
image_usage: bool,
image_sharing_mode: bool,
queue_family_index_count: bool,
p_queue_family_indices: bool,
pre_transform: bool,
composite_alpha: bool,
present_mode: bool,
clipped: bool,
#[cfg(vulkan)]
old_swapchain: bool,
#[cfg(vulkansc)]
old_swapchain: bool,
}

#[repr(C)]
pub struct VkPresentInfoKHR {
s_type: bool,
p_next: bool,
wait_semaphore_count: bool,
p_wait_semaphores: bool,
swapchain_count: bool,
p_swapchains: bool,
p_image_indices: bool,
p_results: bool,
}

#[repr(C)]
pub struct VkDebugReportCallbackCreateInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
pfn_callback: bool,
p_user_data: bool,
}

#[repr(C)]
pub struct VkValidationFlagsEXT {
s_type: bool,
p_next: bool,
disabled_validation_check_count: bool,
p_disabled_validation_checks: bool,
}

#[repr(C)]
pub struct VkValidationFeaturesEXT {
s_type: bool,
p_next: bool,
enabled_validation_feature_count: bool,
p_enabled_validation_features: bool,
disabled_validation_feature_count: bool,
p_disabled_validation_features: bool,
}

#[repr(C)]
pub struct VkApplicationParametersEXT {
s_type: bool,
p_next: bool,
vendor_id: bool,
device_id: bool,
key: bool,
value: bool,
}

#[repr(C)]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD {
s_type: bool,
p_next: bool,
rasterization_order: bool,
}

#[repr(C)]
pub struct VkDebugMarkerObjectNameInfoEXT {
s_type: bool,
p_next: bool,
object_type: bool,
object: bool,
p_object_name: bool,
}

#[repr(C)]
pub struct VkDebugMarkerObjectTagInfoEXT {
s_type: bool,
p_next: bool,
object_type: bool,
object: bool,
tag_name: bool,
tag_size: bool,
p_tag: bool,
}

#[repr(C)]
pub struct VkDebugMarkerMarkerInfoEXT {
s_type: bool,
p_next: bool,
p_marker_name: bool,
color: bool,
}

#[repr(C)]
pub struct VkDedicatedAllocationImageCreateInfoNV {
s_type: bool,
p_next: bool,
dedicated_allocation: bool,
}

#[repr(C)]
pub struct VkDedicatedAllocationBufferCreateInfoNV {
s_type: bool,
p_next: bool,
dedicated_allocation: bool,
}

#[repr(C)]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV {
s_type: bool,
p_next: bool,
image: bool,
buffer: bool,
}

#[repr(C)]
pub struct VkExternalImageFormatPropertiesNV {
image_format_properties: bool,
external_memory_features: bool,
export_from_imported_handle_types: bool,
compatible_handle_types: bool,
}

#[repr(C)]
pub struct VkExternalMemoryImageCreateInfoNV {
s_type: bool,
p_next: bool,
handle_types: bool,
}

#[repr(C)]
pub struct VkExportMemoryAllocateInfoNV {
s_type: bool,
p_next: bool,
handle_types: bool,
}

#[repr(C)]
pub struct VkImportMemoryWin32HandleInfoNV {
s_type: bool,
p_next: bool,
handle_type: bool,
handle: bool,
}

#[repr(C)]
pub struct VkExportMemoryWin32HandleInfoNV {
s_type: bool,
p_next: bool,
p_attributes: bool,
dw_access: bool,
}

#[repr(C)]
pub struct VkExportMemorySciBufInfoNV {
s_type: bool,
p_next: bool,
p_attributes: bool,
}

#[repr(C)]
pub struct VkImportMemorySciBufInfoNV {
s_type: bool,
p_next: bool,
handle_type: bool,
handle: bool,
}

#[repr(C)]
pub struct VkMemoryGetSciBufInfoNV {
s_type: bool,
p_next: bool,
memory: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkMemorySciBufPropertiesNV {
s_type: bool,
p_next: bool,
memory_type_bits: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalMemorySciBufFeaturesNV {
s_type: bool,
p_next: bool,
sci_buf_import: bool,
sci_buf_export: bool,
}

#[repr(C)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV {
s_type: bool,
p_next: bool,
acquire_count: bool,
p_acquire_syncs: bool,
p_acquire_keys: bool,
p_acquire_timeout_milliseconds: bool,
release_count: bool,
p_release_syncs: bool,
p_release_keys: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
s_type: bool,
p_next: bool,
device_generated_commands: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV {
s_type: bool,
p_next: bool,
device_generated_compute: bool,
device_generated_compute_pipelines: bool,
device_generated_compute_capture_replay: bool,
}

#[repr(C)]
pub struct VkDevicePrivateDataCreateInfo {
s_type: bool,
p_next: bool,
private_data_slot_request_count: bool,
}

#[repr(C)]
pub struct VkPrivateDataSlotCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePrivateDataFeatures {
s_type: bool,
p_next: bool,
private_data: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
s_type: bool,
p_next: bool,
max_graphics_shader_group_count: bool,
max_indirect_sequence_count: bool,
max_indirect_commands_token_count: bool,
max_indirect_commands_stream_count: bool,
max_indirect_commands_token_offset: bool,
max_indirect_commands_stream_stride: bool,
min_sequences_count_buffer_offset_alignment: bool,
min_sequences_index_buffer_offset_alignment: bool,
min_indirect_commands_buffer_offset_alignment: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiDrawPropertiesEXT {
s_type: bool,
p_next: bool,
max_multi_draw_count: bool,
}

#[repr(C)]
pub struct VkGraphicsShaderGroupCreateInfoNV {
s_type: bool,
p_next: bool,
stage_count: bool,
p_stages: bool,
p_vertex_input_state: bool,
p_tessellation_state: bool,
}

#[repr(C)]
pub struct VkGraphicsPipelineShaderGroupsCreateInfoNV {
s_type: bool,
p_next: bool,
group_count: bool,
p_groups: bool,
pipeline_count: bool,
p_pipelines: bool,
}

#[repr(C)]
pub struct VkBindShaderGroupIndirectCommandNV {
group_index: bool,
}

#[repr(C)]
pub struct VkBindIndexBufferIndirectCommandNV {
buffer_address: bool,
size: bool,
index_type: bool,
}

#[repr(C)]
pub struct VkBindVertexBufferIndirectCommandNV {
buffer_address: bool,
size: bool,
stride: bool,
}

#[repr(C)]
pub struct VkSetStateFlagsIndirectCommandNV {
data: bool,
}

#[repr(C)]
pub struct VkIndirectCommandsStreamNV {
buffer: bool,
offset: bool,
}

#[repr(C)]
pub struct VkIndirectCommandsLayoutTokenNV {
s_type: bool,
p_next: bool,
token_type: bool,
stream: bool,
offset: bool,
vertex_binding_unit: bool,
vertex_dynamic_stride: bool,
pushconstant_pipeline_layout: bool,
pushconstant_shader_stage_flags: bool,
pushconstant_offset: bool,
pushconstant_size: bool,
indirect_state_flags: bool,
index_type_count: bool,
p_index_types: bool,
p_index_type_values: bool,
}

#[repr(C)]
pub struct VkIndirectCommandsLayoutCreateInfoNV {
s_type: bool,
p_next: bool,
flags: bool,
pipeline_bind_point: bool,
token_count: bool,
p_tokens: bool,
stream_count: bool,
p_stream_strides: bool,
}

#[repr(C)]
pub struct VkGeneratedCommandsInfoNV {
s_type: bool,
p_next: bool,
pipeline_bind_point: bool,
pipeline: bool,
indirect_commands_layout: bool,
stream_count: bool,
p_streams: bool,
sequences_count: bool,
preprocess_buffer: bool,
preprocess_offset: bool,
preprocess_size: bool,
sequences_count_buffer: bool,
sequences_count_offset: bool,
sequences_index_buffer: bool,
sequences_index_offset: bool,
}

#[repr(C)]
pub struct VkGeneratedCommandsMemoryRequirementsInfoNV {
s_type: bool,
p_next: bool,
pipeline_bind_point: bool,
pipeline: bool,
indirect_commands_layout: bool,
max_sequences_count: bool,
}

#[repr(C)]
pub struct VkPipelineIndirectDeviceAddressInfoNV {
s_type: bool,
p_next: bool,
pipeline_bind_point: bool,
pipeline: bool,
}

#[repr(C)]
pub struct VkBindPipelineIndirectCommandNV {
pipeline_address: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFeatures2 {
s_type: bool,
p_next: bool,
features: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceProperties2 {
s_type: bool,
p_next: bool,
properties: bool,
}

#[repr(C)]
pub struct VkFormatProperties2 {
s_type: bool,
p_next: bool,
format_properties: bool,
}

#[repr(C)]
pub struct VkImageFormatProperties2 {
s_type: bool,
p_next: bool,
image_format_properties: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageFormatInfo2 {
s_type: bool,
p_next: bool,
format: bool,
r#type: bool,
tiling: bool,
usage: bool,
flags: bool,
}

#[repr(C)]
pub struct VkQueueFamilyProperties2 {
s_type: bool,
p_next: bool,
queue_family_properties: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties2 {
s_type: bool,
p_next: bool,
memory_properties: bool,
}

#[repr(C)]
pub struct VkSparseImageFormatProperties2 {
s_type: bool,
p_next: bool,
properties: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2 {
s_type: bool,
p_next: bool,
format: bool,
r#type: bool,
samples: bool,
usage: bool,
tiling: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR {
s_type: bool,
p_next: bool,
max_push_descriptors: bool,
}

#[repr(C)]
pub struct VkConformanceVersion {
major: bool,
minor: bool,
subminor: bool,
patch: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDriverProperties {
s_type: bool,
p_next: bool,
driver_id: bool,
driver_name: bool,
driver_info: bool,
conformance_version: bool,
}

#[repr(C)]
pub struct VkPresentRegionsKHR {
s_type: bool,
p_next: bool,
swapchain_count: bool,
p_regions: bool,
}

#[repr(C)]
pub struct VkPresentRegionKHR {
rectangle_count: bool,
p_rectangles: bool,
}

#[repr(C)]
pub struct VkRectLayerKHR {
offset: bool,
extent: bool,
layer: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVariablePointersFeatures {
s_type: bool,
p_next: bool,
variable_pointers_storage_buffer: bool,
variable_pointers: bool,
}

#[repr(C)]
pub struct VkExternalMemoryProperties {
external_memory_features: bool,
export_from_imported_handle_types: bool,
compatible_handle_types: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalImageFormatInfo {
s_type: bool,
p_next: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkExternalImageFormatProperties {
s_type: bool,
p_next: bool,
external_memory_properties: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalBufferInfo {
s_type: bool,
p_next: bool,
flags: bool,
usage: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkExternalBufferProperties {
s_type: bool,
p_next: bool,
external_memory_properties: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceIDProperties {
s_type: bool,
p_next: bool,
device_uuid: bool,
driver_uuid: bool,
device_luid: bool,
device_node_mask: bool,
device_luidvalid: bool,
}

#[repr(C)]
pub struct VkExternalMemoryImageCreateInfo {
s_type: bool,
p_next: bool,
handle_types: bool,
}

#[repr(C)]
pub struct VkExternalMemoryBufferCreateInfo {
s_type: bool,
p_next: bool,
handle_types: bool,
}

#[repr(C)]
pub struct VkExportMemoryAllocateInfo {
s_type: bool,
p_next: bool,
handle_types: bool,
}

#[repr(C)]
pub struct VkImportMemoryWin32HandleInfoKHR {
s_type: bool,
p_next: bool,
handle_type: bool,
handle: bool,
name: bool,
}

#[repr(C)]
pub struct VkExportMemoryWin32HandleInfoKHR {
s_type: bool,
p_next: bool,
p_attributes: bool,
dw_access: bool,
name: bool,
}

#[repr(C)]
pub struct VkImportMemoryZirconHandleInfoFUCHSIA {
s_type: bool,
p_next: bool,
handle_type: bool,
handle: bool,
}

#[repr(C)]
pub struct VkMemoryZirconHandlePropertiesFUCHSIA {
s_type: bool,
p_next: bool,
memory_type_bits: bool,
}

#[repr(C)]
pub struct VkMemoryGetZirconHandleInfoFUCHSIA {
s_type: bool,
p_next: bool,
memory: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkMemoryWin32HandlePropertiesKHR {
s_type: bool,
p_next: bool,
memory_type_bits: bool,
}

#[repr(C)]
pub struct VkMemoryGetWin32HandleInfoKHR {
s_type: bool,
p_next: bool,
memory: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkImportMemoryFdInfoKHR {
s_type: bool,
p_next: bool,
handle_type: bool,
fd: bool,
}

#[repr(C)]
pub struct VkMemoryFdPropertiesKHR {
s_type: bool,
p_next: bool,
memory_type_bits: bool,
}

#[repr(C)]
pub struct VkMemoryGetFdInfoKHR {
s_type: bool,
p_next: bool,
memory: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR {
s_type: bool,
p_next: bool,
acquire_count: bool,
p_acquire_syncs: bool,
p_acquire_keys: bool,
p_acquire_timeouts: bool,
release_count: bool,
p_release_syncs: bool,
p_release_keys: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalSemaphoreInfo {
s_type: bool,
p_next: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkExternalSemaphoreProperties {
s_type: bool,
p_next: bool,
export_from_imported_handle_types: bool,
compatible_handle_types: bool,
external_semaphore_features: bool,
}

#[repr(C)]
pub struct VkExportSemaphoreCreateInfo {
s_type: bool,
p_next: bool,
handle_types: bool,
}

#[repr(C)]
pub struct VkImportSemaphoreWin32HandleInfoKHR {
s_type: bool,
p_next: bool,
semaphore: bool,
flags: bool,
handle_type: bool,
handle: bool,
name: bool,
}

#[repr(C)]
pub struct VkExportSemaphoreWin32HandleInfoKHR {
s_type: bool,
p_next: bool,
p_attributes: bool,
dw_access: bool,
name: bool,
}

#[repr(C)]
pub struct VkD3D12FenceSubmitInfoKHR {
s_type: bool,
p_next: bool,
wait_semaphore_values_count: bool,
p_wait_semaphore_values: bool,
signal_semaphore_values_count: bool,
p_signal_semaphore_values: bool,
}

#[repr(C)]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
s_type: bool,
p_next: bool,
semaphore: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkImportSemaphoreFdInfoKHR {
s_type: bool,
p_next: bool,
semaphore: bool,
flags: bool,
handle_type: bool,
fd: bool,
}

#[repr(C)]
pub struct VkSemaphoreGetFdInfoKHR {
s_type: bool,
p_next: bool,
semaphore: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkImportSemaphoreZirconHandleInfoFUCHSIA {
s_type: bool,
p_next: bool,
semaphore: bool,
flags: bool,
handle_type: bool,
zircon_handle: bool,
}

#[repr(C)]
pub struct VkSemaphoreGetZirconHandleInfoFUCHSIA {
s_type: bool,
p_next: bool,
semaphore: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalFenceInfo {
s_type: bool,
p_next: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkExternalFenceProperties {
s_type: bool,
p_next: bool,
export_from_imported_handle_types: bool,
compatible_handle_types: bool,
external_fence_features: bool,
}

#[repr(C)]
pub struct VkExportFenceCreateInfo {
s_type: bool,
p_next: bool,
handle_types: bool,
}

#[repr(C)]
pub struct VkImportFenceWin32HandleInfoKHR {
s_type: bool,
p_next: bool,
fence: bool,
flags: bool,
handle_type: bool,
handle: bool,
name: bool,
}

#[repr(C)]
pub struct VkExportFenceWin32HandleInfoKHR {
s_type: bool,
p_next: bool,
p_attributes: bool,
dw_access: bool,
name: bool,
}

#[repr(C)]
pub struct VkFenceGetWin32HandleInfoKHR {
s_type: bool,
p_next: bool,
fence: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkImportFenceFdInfoKHR {
s_type: bool,
p_next: bool,
fence: bool,
flags: bool,
handle_type: bool,
fd: bool,
}

#[repr(C)]
pub struct VkFenceGetFdInfoKHR {
s_type: bool,
p_next: bool,
fence: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkExportFenceSciSyncInfoNV {
s_type: bool,
p_next: bool,
p_attributes: bool,
}

#[repr(C)]
pub struct VkImportFenceSciSyncInfoNV {
s_type: bool,
p_next: bool,
fence: bool,
handle_type: bool,
handle: bool,
}

#[repr(C)]
pub struct VkFenceGetSciSyncInfoNV {
s_type: bool,
p_next: bool,
fence: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkExportSemaphoreSciSyncInfoNV {
s_type: bool,
p_next: bool,
p_attributes: bool,
}

#[repr(C)]
pub struct VkImportSemaphoreSciSyncInfoNV {
s_type: bool,
p_next: bool,
semaphore: bool,
handle_type: bool,
handle: bool,
}

#[repr(C)]
pub struct VkSemaphoreGetSciSyncInfoNV {
s_type: bool,
p_next: bool,
semaphore: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkSciSyncAttributesInfoNV {
s_type: bool,
p_next: bool,
client_type: bool,
primitive_type: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalSciSyncFeaturesNV {
s_type: bool,
p_next: bool,
sci_sync_fence: bool,
sci_sync_semaphore: bool,
sci_sync_import: bool,
sci_sync_export: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalSciSync2FeaturesNV {
s_type: bool,
p_next: bool,
sci_sync_fence: bool,
sci_sync_semaphore2: bool,
sci_sync_import: bool,
sci_sync_export: bool,
}

#[repr(C)]
pub struct VkSemaphoreSciSyncPoolCreateInfoNV {
s_type: bool,
p_next: bool,
handle: bool,
}

#[repr(C)]
pub struct VkSemaphoreSciSyncCreateInfoNV {
s_type: bool,
p_next: bool,
semaphore_pool: bool,
p_fence: bool,
}

#[repr(C)]
pub struct VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV {
s_type: bool,
p_next: bool,
semaphore_sci_sync_pool_request_count: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewFeatures {
s_type: bool,
p_next: bool,
multiview: bool,
multiview_geometry_shader: bool,
multiview_tessellation_shader: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewProperties {
s_type: bool,
p_next: bool,
max_multiview_view_count: bool,
max_multiview_instance_index: bool,
}

#[repr(C)]
pub struct VkRenderPassMultiviewCreateInfo {
s_type: bool,
p_next: bool,
subpass_count: bool,
p_view_masks: bool,
dependency_count: bool,
p_view_offsets: bool,
correlation_mask_count: bool,
p_correlation_masks: bool,
}

#[repr(C)]
pub struct VkSurfaceCapabilities2EXT {
s_type: bool,
p_next: bool,
min_image_count: bool,
max_image_count: bool,
current_extent: bool,
min_image_extent: bool,
max_image_extent: bool,
max_image_array_layers: bool,
supported_transforms: bool,
current_transform: bool,
supported_composite_alpha: bool,
supported_usage_flags: bool,
supported_surface_counters: bool,
}

#[repr(C)]
pub struct VkDisplayPowerInfoEXT {
s_type: bool,
p_next: bool,
power_state: bool,
}

#[repr(C)]
pub struct VkDeviceEventInfoEXT {
s_type: bool,
p_next: bool,
device_event: bool,
}

#[repr(C)]
pub struct VkDisplayEventInfoEXT {
s_type: bool,
p_next: bool,
display_event: bool,
}

#[repr(C)]
pub struct VkSwapchainCounterCreateInfoEXT {
s_type: bool,
p_next: bool,
surface_counters: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceGroupProperties {
s_type: bool,
p_next: bool,
physical_device_count: bool,
physical_devices: bool,
subset_allocation: bool,
}

#[repr(C)]
pub struct VkMemoryAllocateFlagsInfo {
s_type: bool,
p_next: bool,
flags: bool,
device_mask: bool,
}

#[repr(C)]
pub struct VkBindBufferMemoryInfo {
s_type: bool,
p_next: bool,
buffer: bool,
memory: bool,
memory_offset: bool,
}

#[repr(C)]
pub struct VkBindBufferMemoryDeviceGroupInfo {
s_type: bool,
p_next: bool,
device_index_count: bool,
p_device_indices: bool,
}

#[repr(C)]
pub struct VkBindImageMemoryInfo {
s_type: bool,
p_next: bool,
image: bool,
memory: bool,
memory_offset: bool,
}

#[repr(C)]
pub struct VkBindImageMemoryDeviceGroupInfo {
s_type: bool,
p_next: bool,
device_index_count: bool,
p_device_indices: bool,
split_instance_bind_region_count: bool,
p_split_instance_bind_regions: bool,
}

#[repr(C)]
pub struct VkDeviceGroupRenderPassBeginInfo {
s_type: bool,
p_next: bool,
device_mask: bool,
device_render_area_count: bool,
p_device_render_areas: bool,
}

#[repr(C)]
pub struct VkDeviceGroupCommandBufferBeginInfo {
s_type: bool,
p_next: bool,
device_mask: bool,
}

#[repr(C)]
pub struct VkDeviceGroupSubmitInfo {
s_type: bool,
p_next: bool,
wait_semaphore_count: bool,
p_wait_semaphore_device_indices: bool,
command_buffer_count: bool,
p_command_buffer_device_masks: bool,
signal_semaphore_count: bool,
p_signal_semaphore_device_indices: bool,
}

#[repr(C)]
pub struct VkDeviceGroupBindSparseInfo {
s_type: bool,
p_next: bool,
resource_device_index: bool,
memory_device_index: bool,
}

#[repr(C)]
pub struct VkDeviceGroupPresentCapabilitiesKHR {
s_type: bool,
p_next: bool,
present_mask: bool,
modes: bool,
}

#[repr(C)]
pub struct VkImageSwapchainCreateInfoKHR {
s_type: bool,
p_next: bool,
swapchain: bool,
}

#[repr(C)]
pub struct VkBindImageMemorySwapchainInfoKHR {
s_type: bool,
p_next: bool,
swapchain: bool,
image_index: bool,
}

#[repr(C)]
pub struct VkAcquireNextImageInfoKHR {
s_type: bool,
p_next: bool,
swapchain: bool,
timeout: bool,
semaphore: bool,
fence: bool,
device_mask: bool,
}

#[repr(C)]
pub struct VkDeviceGroupPresentInfoKHR {
s_type: bool,
p_next: bool,
swapchain_count: bool,
p_device_masks: bool,
mode: bool,
}

#[repr(C)]
pub struct VkDeviceGroupDeviceCreateInfo {
s_type: bool,
p_next: bool,
physical_device_count: bool,
p_physical_devices: bool,
}

#[repr(C)]
pub struct VkDeviceGroupSwapchainCreateInfoKHR {
s_type: bool,
p_next: bool,
modes: bool,
}

#[repr(C)]
pub struct VkDescriptorUpdateTemplateEntry {
dst_binding: bool,
dst_array_element: bool,
descriptor_count: bool,
descriptor_type: bool,
offset: bool,
stride: bool,
}

#[repr(C)]
pub struct VkDescriptorUpdateTemplateCreateInfo {
s_type: bool,
p_next: bool,
flags: bool,
descriptor_update_entry_count: bool,
p_descriptor_update_entries: bool,
template_type: bool,
descriptor_set_layout: bool,
pipeline_bind_point: bool,
pipeline_layout: bool,
set: bool,
}

#[repr(C)]
pub struct VkXYColorEXT {
x: bool,
y: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentIdFeaturesKHR {
s_type: bool,
p_next: bool,
present_id: bool,
}

#[repr(C)]
pub struct VkPresentIdKHR {
s_type: bool,
p_next: bool,
swapchain_count: bool,
p_present_ids: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentWaitFeaturesKHR {
s_type: bool,
p_next: bool,
present_wait: bool,
}

#[repr(C)]
pub struct VkHdrMetadataEXT {
s_type: bool,
p_next: bool,
display_primary_red: bool,
display_primary_green: bool,
display_primary_blue: bool,
white_point: bool,
max_luminance: bool,
min_luminance: bool,
max_content_light_level: bool,
max_frame_average_light_level: bool,
}

#[repr(C)]
pub struct VkDisplayNativeHdrSurfaceCapabilitiesAMD {
s_type: bool,
p_next: bool,
local_dimming_support: bool,
}

#[repr(C)]
pub struct VkSwapchainDisplayNativeHdrCreateInfoAMD {
s_type: bool,
p_next: bool,
local_dimming_enable: bool,
}

#[repr(C)]
pub struct VkRefreshCycleDurationGOOGLE {
refresh_duration: bool,
}

#[repr(C)]
pub struct VkPastPresentationTimingGOOGLE {
present_id: bool,
desired_present_time: bool,
actual_present_time: bool,
earliest_present_time: bool,
present_margin: bool,
}

#[repr(C)]
pub struct VkPresentTimesInfoGOOGLE {
s_type: bool,
p_next: bool,
swapchain_count: bool,
p_times: bool,
}

#[repr(C)]
pub struct VkPresentTimeGOOGLE {
present_id: bool,
desired_present_time: bool,
}

#[repr(C)]
pub struct VkIOSSurfaceCreateInfoMVK {
s_type: bool,
p_next: bool,
flags: bool,
p_view: bool,
}

#[repr(C)]
pub struct VkMacOSSurfaceCreateInfoMVK {
s_type: bool,
p_next: bool,
flags: bool,
p_view: bool,
}

#[repr(C)]
pub struct VkMetalSurfaceCreateInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
p_layer: bool,
}

#[repr(C)]
pub struct VkViewportWScalingNV {
xcoeff: bool,
ycoeff: bool,
}

#[repr(C)]
pub struct VkPipelineViewportWScalingStateCreateInfoNV {
s_type: bool,
p_next: bool,
viewport_wscaling_enable: bool,
viewport_count: bool,
p_viewport_wscalings: bool,
}

#[repr(C)]
pub struct VkViewportSwizzleNV {
x: bool,
y: bool,
z: bool,
w: bool,
}

#[repr(C)]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV {
s_type: bool,
p_next: bool,
flags: bool,
viewport_count: bool,
p_viewport_swizzles: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
s_type: bool,
p_next: bool,
max_discard_rectangles: bool,
}

#[repr(C)]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
discard_rectangle_mode: bool,
discard_rectangle_count: bool,
p_discard_rectangles: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
s_type: bool,
p_next: bool,
per_view_position_all_components: bool,
}

#[repr(C)]
pub struct VkInputAttachmentAspectReference {
subpass: bool,
input_attachment_index: bool,
aspect_mask: bool,
}

#[repr(C)]
pub struct VkRenderPassInputAttachmentAspectCreateInfo {
s_type: bool,
p_next: bool,
aspect_reference_count: bool,
p_aspect_references: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSurfaceInfo2KHR {
s_type: bool,
p_next: bool,
surface: bool,
}

#[repr(C)]
pub struct VkSurfaceCapabilities2KHR {
s_type: bool,
p_next: bool,
surface_capabilities: bool,
}

#[repr(C)]
pub struct VkSurfaceFormat2KHR {
s_type: bool,
p_next: bool,
surface_format: bool,
}

#[repr(C)]
pub struct VkDisplayProperties2KHR {
s_type: bool,
p_next: bool,
display_properties: bool,
}

#[repr(C)]
pub struct VkDisplayPlaneProperties2KHR {
s_type: bool,
p_next: bool,
display_plane_properties: bool,
}

#[repr(C)]
pub struct VkDisplayModeProperties2KHR {
s_type: bool,
p_next: bool,
display_mode_properties: bool,
}

#[repr(C)]
pub struct VkDisplayPlaneInfo2KHR {
s_type: bool,
p_next: bool,
mode: bool,
plane_index: bool,
}

#[repr(C)]
pub struct VkDisplayPlaneCapabilities2KHR {
s_type: bool,
p_next: bool,
capabilities: bool,
}

#[repr(C)]
pub struct VkSharedPresentSurfaceCapabilitiesKHR {
s_type: bool,
p_next: bool,
shared_present_supported_usage_flags: bool,
}

#[repr(C)]
pub struct VkPhysicalDevice16BitStorageFeatures {
s_type: bool,
p_next: bool,
storage_buffer16_bit_access: bool,
uniform_and_storage_buffer16_bit_access: bool,
storage_push_constant16: bool,
storage_input_output16: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubgroupProperties {
s_type: bool,
p_next: bool,
subgroup_size: bool,
supported_stages: bool,
supported_operations: bool,
quad_operations_in_all_stages: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
s_type: bool,
p_next: bool,
shader_subgroup_extended_types: bool,
}

#[repr(C)]
pub struct VkBufferMemoryRequirementsInfo2 {
s_type: bool,
p_next: bool,
buffer: bool,
}

#[repr(C)]
pub struct VkDeviceBufferMemoryRequirements {
s_type: bool,
p_next: bool,
p_create_info: bool,
}

#[repr(C)]
pub struct VkImageMemoryRequirementsInfo2 {
s_type: bool,
p_next: bool,
image: bool,
}

#[repr(C)]
pub struct VkImageSparseMemoryRequirementsInfo2 {
s_type: bool,
p_next: bool,
image: bool,
}

#[repr(C)]
pub struct VkDeviceImageMemoryRequirements {
s_type: bool,
p_next: bool,
p_create_info: bool,
plane_aspect: bool,
}

#[repr(C)]
pub struct VkMemoryRequirements2 {
s_type: bool,
p_next: bool,
memory_requirements: bool,
}

#[repr(C)]
pub struct VkSparseImageMemoryRequirements2 {
s_type: bool,
p_next: bool,
memory_requirements: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePointClippingProperties {
s_type: bool,
p_next: bool,
point_clipping_behavior: bool,
}

#[repr(C)]
pub struct VkMemoryDedicatedRequirements {
s_type: bool,
p_next: bool,
prefers_dedicated_allocation: bool,
requires_dedicated_allocation: bool,
}

#[repr(C)]
pub struct VkMemoryDedicatedAllocateInfo {
s_type: bool,
p_next: bool,
image: bool,
buffer: bool,
}

#[repr(C)]
pub struct VkImageViewUsageCreateInfo {
s_type: bool,
p_next: bool,
usage: bool,
}

#[repr(C)]
pub struct VkImageViewSlicedCreateInfoEXT {
s_type: bool,
p_next: bool,
slice_offset: bool,
slice_count: bool,
}

#[repr(C)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfo {
s_type: bool,
p_next: bool,
domain_origin: bool,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionInfo {
s_type: bool,
p_next: bool,
conversion: bool,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionCreateInfo {
s_type: bool,
p_next: bool,
format: bool,
ycbcr_model: bool,
ycbcr_range: bool,
components: bool,
x_chroma_offset: bool,
y_chroma_offset: bool,
chroma_filter: bool,
force_explicit_reconstruction: bool,
}

#[repr(C)]
pub struct VkBindImagePlaneMemoryInfo {
s_type: bool,
p_next: bool,
plane_aspect: bool,
}

#[repr(C)]
pub struct VkImagePlaneMemoryRequirementsInfo {
s_type: bool,
p_next: bool,
plane_aspect: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures {
s_type: bool,
p_next: bool,
sampler_ycbcr_conversion: bool,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionImageFormatProperties {
s_type: bool,
p_next: bool,
combined_image_sampler_descriptor_count: bool,
}

#[repr(C)]
pub struct VkTextureLODGatherFormatPropertiesAMD {
s_type: bool,
p_next: bool,
supports_texture_gather_lodbias_amd: bool,
}

#[repr(C)]
pub struct VkConditionalRenderingBeginInfoEXT {
s_type: bool,
p_next: bool,
buffer: bool,
offset: bool,
flags: bool,
}

#[repr(C)]
pub struct VkProtectedSubmitInfo {
s_type: bool,
p_next: bool,
protected_submit: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceProtectedMemoryFeatures {
s_type: bool,
p_next: bool,
protected_memory: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceProtectedMemoryProperties {
s_type: bool,
p_next: bool,
protected_no_fault: bool,
}

#[repr(C)]
pub struct VkDeviceQueueInfo2 {
s_type: bool,
p_next: bool,
flags: bool,
queue_family_index: bool,
queue_index: bool,
}

#[repr(C)]
pub struct VkPipelineCoverageToColorStateCreateInfoNV {
s_type: bool,
p_next: bool,
flags: bool,
coverage_to_color_enable: bool,
coverage_to_color_location: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties {
s_type: bool,
p_next: bool,
filter_minmax_single_component_formats: bool,
filter_minmax_image_component_mapping: bool,
}

#[repr(C)]
pub struct VkSampleLocationEXT {
x: bool,
y: bool,
}

#[repr(C)]
pub struct VkSampleLocationsInfoEXT {
s_type: bool,
p_next: bool,
sample_locations_per_pixel: bool,
sample_location_grid_size: bool,
sample_locations_count: bool,
p_sample_locations: bool,
}

#[repr(C)]
pub struct VkAttachmentSampleLocationsEXT {
attachment_index: bool,
sample_locations_info: bool,
}

#[repr(C)]
pub struct VkSubpassSampleLocationsEXT {
subpass_index: bool,
sample_locations_info: bool,
}

#[repr(C)]
pub struct VkRenderPassSampleLocationsBeginInfoEXT {
s_type: bool,
p_next: bool,
attachment_initial_sample_locations_count: bool,
p_attachment_initial_sample_locations: bool,
post_subpass_sample_locations_count: bool,
p_post_subpass_sample_locations: bool,
}

#[repr(C)]
pub struct VkPipelineSampleLocationsStateCreateInfoEXT {
s_type: bool,
p_next: bool,
sample_locations_enable: bool,
sample_locations_info: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT {
s_type: bool,
p_next: bool,
sample_location_sample_counts: bool,
max_sample_location_grid_size: bool,
sample_location_coordinate_range: bool,
sample_location_sub_pixel_bits: bool,
variable_sample_locations: bool,
}

#[repr(C)]
pub struct VkMultisamplePropertiesEXT {
s_type: bool,
p_next: bool,
max_sample_location_grid_size: bool,
}

#[repr(C)]
pub struct VkSamplerReductionModeCreateInfo {
s_type: bool,
p_next: bool,
reduction_mode: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
s_type: bool,
p_next: bool,
advanced_blend_coherent_operations: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiDrawFeaturesEXT {
s_type: bool,
p_next: bool,
multi_draw: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
s_type: bool,
p_next: bool,
advanced_blend_max_color_attachments: bool,
advanced_blend_independent_blend: bool,
advanced_blend_non_premultiplied_src_color: bool,
advanced_blend_non_premultiplied_dst_color: bool,
advanced_blend_correlated_overlap: bool,
advanced_blend_all_operations: bool,
}

#[repr(C)]
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
s_type: bool,
p_next: bool,
src_premultiplied: bool,
dst_premultiplied: bool,
blend_overlap: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceInlineUniformBlockFeatures {
s_type: bool,
p_next: bool,
inline_uniform_block: bool,
descriptor_binding_inline_uniform_block_update_after_bind: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceInlineUniformBlockProperties {
s_type: bool,
p_next: bool,
max_inline_uniform_block_size: bool,
max_per_stage_descriptor_inline_uniform_blocks: bool,
max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: bool,
max_descriptor_set_inline_uniform_blocks: bool,
max_descriptor_set_update_after_bind_inline_uniform_blocks: bool,
}

#[repr(C)]
pub struct VkWriteDescriptorSetInlineUniformBlock {
s_type: bool,
p_next: bool,
data_size: bool,
p_data: bool,
}

#[repr(C)]
pub struct VkDescriptorPoolInlineUniformBlockCreateInfo {
s_type: bool,
p_next: bool,
max_inline_uniform_block_bindings: bool,
}

#[repr(C)]
pub struct VkPipelineCoverageModulationStateCreateInfoNV {
s_type: bool,
p_next: bool,
flags: bool,
coverage_modulation_mode: bool,
coverage_modulation_table_enable: bool,
coverage_modulation_table_count: bool,
p_coverage_modulation_table: bool,
}

#[repr(C)]
pub struct VkImageFormatListCreateInfo {
s_type: bool,
p_next: bool,
view_format_count: bool,
p_view_formats: bool,
}

#[repr(C)]
pub struct VkValidationCacheCreateInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
initial_data_size: bool,
p_initial_data: bool,
}

#[repr(C)]
pub struct VkShaderModuleValidationCacheCreateInfoEXT {
s_type: bool,
p_next: bool,
validation_cache: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance3Properties {
s_type: bool,
p_next: bool,
max_per_set_descriptors: bool,
max_memory_allocation_size: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance4Features {
s_type: bool,
p_next: bool,
maintenance4: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance4Properties {
s_type: bool,
p_next: bool,
max_buffer_size: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance5FeaturesKHR {
s_type: bool,
p_next: bool,
maintenance5: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance5PropertiesKHR {
s_type: bool,
p_next: bool,
early_fragment_multisample_coverage_after_sample_counting: bool,
early_fragment_sample_mask_test_before_sample_counting: bool,
depth_stencil_swizzle_one_support: bool,
polygon_mode_point_size: bool,
non_strict_single_pixel_wide_lines_use_parallelogram: bool,
non_strict_wide_lines_use_parallelogram: bool,
}

#[repr(C)]
pub struct VkRenderingAreaInfoKHR {
s_type: bool,
p_next: bool,
view_mask: bool,
color_attachment_count: bool,
p_color_attachment_formats: bool,
depth_attachment_format: bool,
stencil_attachment_format: bool,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutSupport {
s_type: bool,
p_next: bool,
supported: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderDrawParametersFeatures {
s_type: bool,
p_next: bool,
shader_draw_parameters: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderFloat16Int8Features {
s_type: bool,
p_next: bool,
shader_float16: bool,
shader_int8: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFloatControlsProperties {
s_type: bool,
p_next: bool,
denorm_behavior_independence: bool,
rounding_mode_independence: bool,
shader_signed_zero_inf_nan_preserve_float16: bool,
shader_signed_zero_inf_nan_preserve_float32: bool,
shader_signed_zero_inf_nan_preserve_float64: bool,
shader_denorm_preserve_float16: bool,
shader_denorm_preserve_float32: bool,
shader_denorm_preserve_float64: bool,
shader_denorm_flush_to_zero_float16: bool,
shader_denorm_flush_to_zero_float32: bool,
shader_denorm_flush_to_zero_float64: bool,
shader_rounding_mode_rtefloat16: bool,
shader_rounding_mode_rtefloat32: bool,
shader_rounding_mode_rtefloat64: bool,
shader_rounding_mode_rtzfloat16: bool,
shader_rounding_mode_rtzfloat32: bool,
shader_rounding_mode_rtzfloat64: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceHostQueryResetFeatures {
s_type: bool,
p_next: bool,
host_query_reset: bool,
}

#[repr(C)]
pub struct VkNativeBufferUsage2ANDROID {
consumer: bool,
producer: bool,
}

#[repr(C)]
pub struct VkNativeBufferANDROID {
s_type: bool,
p_next: bool,
handle: bool,
stride: bool,
format: bool,
usage: bool,
usage2: bool,
}

#[repr(C)]
pub struct VkSwapchainImageCreateInfoANDROID {
s_type: bool,
p_next: bool,
usage: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentationPropertiesANDROID {
s_type: bool,
p_next: bool,
shared_image: bool,
}

#[repr(C)]
pub struct VkShaderResourceUsageAMD {
num_used_vgprs: bool,
num_used_sgprs: bool,
lds_size_per_local_work_group: bool,
lds_usage_size_in_bytes: bool,
scratch_mem_usage_in_bytes: bool,
}

#[repr(C)]
pub struct VkShaderStatisticsInfoAMD {
shader_stage_mask: bool,
resource_usage: bool,
num_physical_vgprs: bool,
num_physical_sgprs: bool,
num_available_vgprs: bool,
num_available_sgprs: bool,
compute_work_group_size: bool,
}

#[repr(C)]
pub struct VkDeviceQueueGlobalPriorityCreateInfoKHR {
s_type: bool,
p_next: bool,
global_priority: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR {
s_type: bool,
p_next: bool,
global_priority_query: bool,
}

#[repr(C)]
pub struct VkQueueFamilyGlobalPriorityPropertiesKHR {
s_type: bool,
p_next: bool,
priority_count: bool,
priorities: bool,
}

#[repr(C)]
pub struct VkDebugUtilsObjectNameInfoEXT {
s_type: bool,
p_next: bool,
object_type: bool,
object_handle: bool,
p_object_name: bool,
}

#[repr(C)]
pub struct VkDebugUtilsObjectTagInfoEXT {
s_type: bool,
p_next: bool,
object_type: bool,
object_handle: bool,
tag_name: bool,
tag_size: bool,
p_tag: bool,
}

#[repr(C)]
pub struct VkDebugUtilsLabelEXT {
s_type: bool,
p_next: bool,
p_label_name: bool,
color: bool,
}

#[repr(C)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
message_severity: bool,
message_type: bool,
pfn_user_callback: bool,
p_user_data: bool,
}

#[repr(C)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
s_type: bool,
p_next: bool,
flags: bool,
p_message_id_name: bool,
message_id_number: bool,
p_message: bool,
queue_label_count: bool,
p_queue_labels: bool,
cmd_buf_label_count: bool,
p_cmd_buf_labels: bool,
object_count: bool,
p_objects: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceMemoryReportFeaturesEXT {
s_type: bool,
p_next: bool,
device_memory_report: bool,
}

#[repr(C)]
pub struct VkDeviceDeviceMemoryReportCreateInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
pfn_user_callback: bool,
p_user_data: bool,
}

#[repr(C)]
pub struct VkDeviceMemoryReportCallbackDataEXT {
s_type: bool,
p_next: bool,
flags: bool,
r#type: bool,
memory_object_id: bool,
size: bool,
object_type: bool,
object_handle: bool,
heap_index: bool,
}

#[repr(C)]
pub struct VkImportMemoryHostPointerInfoEXT {
s_type: bool,
p_next: bool,
handle_type: bool,
p_host_pointer: bool,
}

#[repr(C)]
pub struct VkMemoryHostPointerPropertiesEXT {
s_type: bool,
p_next: bool,
memory_type_bits: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
s_type: bool,
p_next: bool,
min_imported_host_pointer_alignment: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
s_type: bool,
p_next: bool,
primitive_overestimation_size: bool,
max_extra_primitive_overestimation_size: bool,
extra_primitive_overestimation_size_granularity: bool,
primitive_underestimation: bool,
conservative_point_and_line_rasterization: bool,
degenerate_triangles_rasterized: bool,
degenerate_lines_rasterized: bool,
fully_covered_fragment_shader_input_variable: bool,
conservative_rasterization_post_depth_coverage: bool,
}

#[repr(C)]
pub struct VkCalibratedTimestampInfoEXT {
s_type: bool,
p_next: bool,
time_domain: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderCorePropertiesAMD {
s_type: bool,
p_next: bool,
shader_engine_count: bool,
shader_arrays_per_engine_count: bool,
compute_units_per_shader_array: bool,
simd_per_compute_unit: bool,
wavefronts_per_simd: bool,
wavefront_size: bool,
sgprs_per_simd: bool,
min_sgpr_allocation: bool,
max_sgpr_allocation: bool,
sgpr_allocation_granularity: bool,
vgprs_per_simd: bool,
min_vgpr_allocation: bool,
max_vgpr_allocation: bool,
vgpr_allocation_granularity: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreProperties2AMD {
s_type: bool,
p_next: bool,
shader_core_features: bool,
active_compute_unit_count: bool,
}

#[repr(C)]
pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
conservative_rasterization_mode: bool,
extra_primitive_overestimation_size: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingFeatures {
s_type: bool,
p_next: bool,
shader_input_attachment_array_dynamic_indexing: bool,
shader_uniform_texel_buffer_array_dynamic_indexing: bool,
shader_storage_texel_buffer_array_dynamic_indexing: bool,
shader_uniform_buffer_array_non_uniform_indexing: bool,
shader_sampled_image_array_non_uniform_indexing: bool,
shader_storage_buffer_array_non_uniform_indexing: bool,
shader_storage_image_array_non_uniform_indexing: bool,
shader_input_attachment_array_non_uniform_indexing: bool,
shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
shader_storage_texel_buffer_array_non_uniform_indexing: bool,
descriptor_binding_uniform_buffer_update_after_bind: bool,
descriptor_binding_sampled_image_update_after_bind: bool,
descriptor_binding_storage_image_update_after_bind: bool,
descriptor_binding_storage_buffer_update_after_bind: bool,
descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
descriptor_binding_storage_texel_buffer_update_after_bind: bool,
descriptor_binding_update_unused_while_pending: bool,
descriptor_binding_partially_bound: bool,
descriptor_binding_variable_descriptor_count: bool,
runtime_descriptor_array: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingProperties {
s_type: bool,
p_next: bool,
max_update_after_bind_descriptors_in_all_pools: bool,
shader_uniform_buffer_array_non_uniform_indexing_native: bool,
shader_sampled_image_array_non_uniform_indexing_native: bool,
shader_storage_buffer_array_non_uniform_indexing_native: bool,
shader_storage_image_array_non_uniform_indexing_native: bool,
shader_input_attachment_array_non_uniform_indexing_native: bool,
robust_buffer_access_update_after_bind: bool,
quad_divergent_implicit_lod: bool,
max_per_stage_descriptor_update_after_bind_samplers: bool,
max_per_stage_descriptor_update_after_bind_uniform_buffers: bool,
max_per_stage_descriptor_update_after_bind_storage_buffers: bool,
max_per_stage_descriptor_update_after_bind_sampled_images: bool,
max_per_stage_descriptor_update_after_bind_storage_images: bool,
max_per_stage_descriptor_update_after_bind_input_attachments: bool,
max_per_stage_update_after_bind_resources: bool,
max_descriptor_set_update_after_bind_samplers: bool,
max_descriptor_set_update_after_bind_uniform_buffers: bool,
max_descriptor_set_update_after_bind_uniform_buffers_dynamic: bool,
max_descriptor_set_update_after_bind_storage_buffers: bool,
max_descriptor_set_update_after_bind_storage_buffers_dynamic: bool,
max_descriptor_set_update_after_bind_sampled_images: bool,
max_descriptor_set_update_after_bind_storage_images: bool,
max_descriptor_set_update_after_bind_input_attachments: bool,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo {
s_type: bool,
p_next: bool,
binding_count: bool,
p_binding_flags: bool,
}

#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo {
s_type: bool,
p_next: bool,
descriptor_set_count: bool,
p_descriptor_counts: bool,
}

#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport {
s_type: bool,
p_next: bool,
max_variable_descriptor_count: bool,
}

#[repr(C)]
pub struct VkAttachmentDescription2 {
s_type: bool,
p_next: bool,
flags: bool,
format: bool,
samples: bool,
load_op: bool,
store_op: bool,
stencil_load_op: bool,
stencil_store_op: bool,
initial_layout: bool,
final_layout: bool,
}

#[repr(C)]
pub struct VkAttachmentReference2 {
s_type: bool,
p_next: bool,
attachment: bool,
layout: bool,
aspect_mask: bool,
}

#[repr(C)]
pub struct VkSubpassDescription2 {
s_type: bool,
p_next: bool,
flags: bool,
pipeline_bind_point: bool,
view_mask: bool,
input_attachment_count: bool,
p_input_attachments: bool,
color_attachment_count: bool,
p_color_attachments: bool,
p_resolve_attachments: bool,
p_depth_stencil_attachment: bool,
preserve_attachment_count: bool,
p_preserve_attachments: bool,
}

#[repr(C)]
pub struct VkSubpassDependency2 {
s_type: bool,
p_next: bool,
src_subpass: bool,
dst_subpass: bool,
src_stage_mask: bool,
dst_stage_mask: bool,
src_access_mask: bool,
dst_access_mask: bool,
dependency_flags: bool,
view_offset: bool,
}

#[repr(C)]
pub struct VkRenderPassCreateInfo2 {
s_type: bool,
p_next: bool,
flags: bool,
attachment_count: bool,
p_attachments: bool,
subpass_count: bool,
p_subpasses: bool,
dependency_count: bool,
p_dependencies: bool,
correlated_view_mask_count: bool,
p_correlated_view_masks: bool,
}

#[repr(C)]
pub struct VkSubpassBeginInfo {
s_type: bool,
p_next: bool,
contents: bool,
}

#[repr(C)]
pub struct VkSubpassEndInfo {
s_type: bool,
p_next: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreFeatures {
s_type: bool,
p_next: bool,
timeline_semaphore: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreProperties {
s_type: bool,
p_next: bool,
max_timeline_semaphore_value_difference: bool,
}

#[repr(C)]
pub struct VkSemaphoreTypeCreateInfo {
s_type: bool,
p_next: bool,
semaphore_type: bool,
initial_value: bool,
}

#[repr(C)]
pub struct VkTimelineSemaphoreSubmitInfo {
s_type: bool,
p_next: bool,
wait_semaphore_value_count: bool,
p_wait_semaphore_values: bool,
signal_semaphore_value_count: bool,
p_signal_semaphore_values: bool,
}

#[repr(C)]
pub struct VkSemaphoreWaitInfo {
s_type: bool,
p_next: bool,
flags: bool,
semaphore_count: bool,
p_semaphores: bool,
p_values: bool,
}

#[repr(C)]
pub struct VkSemaphoreSignalInfo {
s_type: bool,
p_next: bool,
semaphore: bool,
value: bool,
}

#[repr(C)]
pub struct VkVertexInputBindingDivisorDescriptionEXT {
binding: bool,
divisor: bool,
}

#[repr(C)]
pub struct VkPipelineVertexInputDivisorStateCreateInfoEXT {
s_type: bool,
p_next: bool,
vertex_binding_divisor_count: bool,
p_vertex_binding_divisors: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
s_type: bool,
p_next: bool,
max_vertex_attrib_divisor: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePCIBusInfoPropertiesEXT {
s_type: bool,
p_next: bool,
pci_domain: bool,
pci_bus: bool,
pci_device: bool,
pci_function: bool,
}

#[repr(C)]
pub struct VkImportAndroidHardwareBufferInfoANDROID {
s_type: bool,
p_next: bool,
buffer: bool,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferUsageANDROID {
s_type: bool,
p_next: bool,
android_hardware_buffer_usage: bool,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferPropertiesANDROID {
s_type: bool,
p_next: bool,
allocation_size: bool,
memory_type_bits: bool,
}

#[repr(C)]
pub struct VkMemoryGetAndroidHardwareBufferInfoANDROID {
s_type: bool,
p_next: bool,
memory: bool,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferFormatPropertiesANDROID {
s_type: bool,
p_next: bool,
format: bool,
external_format: bool,
format_features: bool,
sampler_ycbcr_conversion_components: bool,
suggested_ycbcr_model: bool,
suggested_ycbcr_range: bool,
suggested_xchroma_offset: bool,
suggested_ychroma_offset: bool,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceConditionalRenderingInfoEXT {
s_type: bool,
p_next: bool,
conditional_rendering_enable: bool,
}

#[repr(C)]
pub struct VkExternalFormatANDROID {
s_type: bool,
p_next: bool,
external_format: bool,
}

#[repr(C)]
pub struct VkPhysicalDevice8BitStorageFeatures {
s_type: bool,
p_next: bool,
storage_buffer8_bit_access: bool,
uniform_and_storage_buffer8_bit_access: bool,
storage_push_constant8: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceConditionalRenderingFeaturesEXT {
s_type: bool,
p_next: bool,
conditional_rendering: bool,
inherited_conditional_rendering: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures {
s_type: bool,
p_next: bool,
vulkan_memory_model: bool,
vulkan_memory_model_device_scope: bool,
vulkan_memory_model_availability_visibility_chains: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicInt64Features {
s_type: bool,
p_next: bool,
shader_buffer_int64_atomics: bool,
shader_shared_int64_atomics: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicFloatFeaturesEXT {
s_type: bool,
p_next: bool,
shader_buffer_float32_atomics: bool,
shader_buffer_float32_atomic_add: bool,
shader_buffer_float64_atomics: bool,
shader_buffer_float64_atomic_add: bool,
shader_shared_float32_atomics: bool,
shader_shared_float32_atomic_add: bool,
shader_shared_float64_atomics: bool,
shader_shared_float64_atomic_add: bool,
shader_image_float32_atomics: bool,
shader_image_float32_atomic_add: bool,
sparse_image_float32_atomics: bool,
sparse_image_float32_atomic_add: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT {
s_type: bool,
p_next: bool,
shader_buffer_float16_atomics: bool,
shader_buffer_float16_atomic_add: bool,
shader_buffer_float16_atomic_min_max: bool,
shader_buffer_float32_atomic_min_max: bool,
shader_buffer_float64_atomic_min_max: bool,
shader_shared_float16_atomics: bool,
shader_shared_float16_atomic_add: bool,
shader_shared_float16_atomic_min_max: bool,
shader_shared_float32_atomic_min_max: bool,
shader_shared_float64_atomic_min_max: bool,
shader_image_float32_atomic_min_max: bool,
sparse_image_float32_atomic_min_max: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT {
s_type: bool,
p_next: bool,
vertex_attribute_instance_rate_divisor: bool,
vertex_attribute_instance_rate_zero_divisor: bool,
}

#[repr(C)]
pub struct VkQueueFamilyCheckpointPropertiesNV {
s_type: bool,
p_next: bool,
checkpoint_execution_stage_mask: bool,
}

#[repr(C)]
pub struct VkCheckpointDataNV {
s_type: bool,
p_next: bool,
stage: bool,
p_checkpoint_marker: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthStencilResolveProperties {
s_type: bool,
p_next: bool,
supported_depth_resolve_modes: bool,
supported_stencil_resolve_modes: bool,
independent_resolve_none: bool,
independent_resolve: bool,
}

#[repr(C)]
pub struct VkSubpassDescriptionDepthStencilResolve {
s_type: bool,
p_next: bool,
depth_resolve_mode: bool,
stencil_resolve_mode: bool,
p_depth_stencil_resolve_attachment: bool,
}

#[repr(C)]
pub struct VkImageViewASTCDecodeModeEXT {
s_type: bool,
p_next: bool,
decode_mode: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceASTCDecodeFeaturesEXT {
s_type: bool,
p_next: bool,
decode_mode_shared_exponent: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceTransformFeedbackFeaturesEXT {
s_type: bool,
p_next: bool,
transform_feedback: bool,
geometry_streams: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceTransformFeedbackPropertiesEXT {
s_type: bool,
p_next: bool,
max_transform_feedback_streams: bool,
max_transform_feedback_buffers: bool,
max_transform_feedback_buffer_size: bool,
max_transform_feedback_stream_data_size: bool,
max_transform_feedback_buffer_data_size: bool,
max_transform_feedback_buffer_data_stride: bool,
transform_feedback_queries: bool,
transform_feedback_streams_lines_triangles: bool,
transform_feedback_rasterization_stream_select: bool,
transform_feedback_draw: bool,
}

#[repr(C)]
pub struct VkPipelineRasterizationStateStreamCreateInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
rasterization_stream: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV {
s_type: bool,
p_next: bool,
representative_fragment_test: bool,
}

#[repr(C)]
pub struct VkPipelineRepresentativeFragmentTestStateCreateInfoNV {
s_type: bool,
p_next: bool,
representative_fragment_test_enable: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExclusiveScissorFeaturesNV {
s_type: bool,
p_next: bool,
exclusive_scissor: bool,
}

#[repr(C)]
pub struct VkPipelineViewportExclusiveScissorStateCreateInfoNV {
s_type: bool,
p_next: bool,
exclusive_scissor_count: bool,
p_exclusive_scissors: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCornerSampledImageFeaturesNV {
s_type: bool,
p_next: bool,
corner_sampled_image: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceComputeShaderDerivativesFeaturesNV {
s_type: bool,
p_next: bool,
compute_derivative_group_quads: bool,
compute_derivative_group_linear: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderImageFootprintFeaturesNV {
s_type: bool,
p_next: bool,
image_footprint: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
s_type: bool,
p_next: bool,
dedicated_allocation_image_aliasing: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectFeaturesNV {
s_type: bool,
p_next: bool,
indirect_copy: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectPropertiesNV {
s_type: bool,
p_next: bool,
supported_queues: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryDecompressionFeaturesNV {
s_type: bool,
p_next: bool,
memory_decompression: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryDecompressionPropertiesNV {
s_type: bool,
p_next: bool,
decompression_methods: bool,
max_decompression_indirect_count: bool,
}

#[repr(C)]
pub struct VkShadingRatePaletteNV {
shading_rate_palette_entry_count: bool,
p_shading_rate_palette_entries: bool,
}

#[repr(C)]
pub struct VkPipelineViewportShadingRateImageStateCreateInfoNV {
s_type: bool,
p_next: bool,
shading_rate_image_enable: bool,
viewport_count: bool,
p_shading_rate_palettes: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShadingRateImageFeaturesNV {
s_type: bool,
p_next: bool,
shading_rate_image: bool,
shading_rate_coarse_sample_order: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShadingRateImagePropertiesNV {
s_type: bool,
p_next: bool,
shading_rate_texel_size: bool,
shading_rate_palette_size: bool,
shading_rate_max_coarse_samples: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceInvocationMaskFeaturesHUAWEI {
s_type: bool,
p_next: bool,
invocation_mask: bool,
}

#[repr(C)]
pub struct VkCoarseSampleLocationNV {
pixel_x: bool,
pixel_y: bool,
sample: bool,
}

#[repr(C)]
pub struct VkCoarseSampleOrderCustomNV {
shading_rate: bool,
sample_count: bool,
sample_location_count: bool,
p_sample_locations: bool,
}

#[repr(C)]
pub struct VkPipelineViewportCoarseSampleOrderStateCreateInfoNV {
s_type: bool,
p_next: bool,
sample_order_type: bool,
custom_sample_order_count: bool,
p_custom_sample_orders: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderFeaturesNV {
s_type: bool,
p_next: bool,
task_shader: bool,
mesh_shader: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderPropertiesNV {
s_type: bool,
p_next: bool,
max_draw_mesh_tasks_count: bool,
max_task_work_group_invocations: bool,
max_task_work_group_size: bool,
max_task_total_memory_size: bool,
max_task_output_count: bool,
max_mesh_work_group_invocations: bool,
max_mesh_work_group_size: bool,
max_mesh_total_memory_size: bool,
max_mesh_output_vertices: bool,
max_mesh_output_primitives: bool,
max_mesh_multiview_view_count: bool,
mesh_output_per_vertex_granularity: bool,
mesh_output_per_primitive_granularity: bool,
}

#[repr(C)]
pub struct VkDrawMeshTasksIndirectCommandNV {
task_count: bool,
first_task: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderFeaturesEXT {
s_type: bool,
p_next: bool,
task_shader: bool,
mesh_shader: bool,
multiview_mesh_shader: bool,
primitive_fragment_shading_rate_mesh_shader: bool,
mesh_shader_queries: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderPropertiesEXT {
s_type: bool,
p_next: bool,
max_task_work_group_total_count: bool,
max_task_work_group_count: bool,
max_task_work_group_invocations: bool,
max_task_work_group_size: bool,
max_task_payload_size: bool,
max_task_shared_memory_size: bool,
max_task_payload_and_shared_memory_size: bool,
max_mesh_work_group_total_count: bool,
max_mesh_work_group_count: bool,
max_mesh_work_group_invocations: bool,
max_mesh_work_group_size: bool,
max_mesh_shared_memory_size: bool,
max_mesh_payload_and_shared_memory_size: bool,
max_mesh_output_memory_size: bool,
max_mesh_payload_and_output_memory_size: bool,
max_mesh_output_components: bool,
max_mesh_output_vertices: bool,
max_mesh_output_primitives: bool,
max_mesh_output_layers: bool,
max_mesh_multiview_view_count: bool,
mesh_output_per_vertex_granularity: bool,
mesh_output_per_primitive_granularity: bool,
max_preferred_task_work_group_invocations: bool,
max_preferred_mesh_work_group_invocations: bool,
prefers_local_invocation_vertex_output: bool,
prefers_local_invocation_primitive_output: bool,
prefers_compact_vertex_output: bool,
prefers_compact_primitive_output: bool,
}

#[repr(C)]
pub struct VkDrawMeshTasksIndirectCommandEXT {
group_count_x: bool,
group_count_y: bool,
group_count_z: bool,
}

#[repr(C)]
pub struct VkRayTracingShaderGroupCreateInfoNV {
s_type: bool,
p_next: bool,
r#type: bool,
general_shader: bool,
closest_hit_shader: bool,
any_hit_shader: bool,
intersection_shader: bool,
}

#[repr(C)]
pub struct VkRayTracingShaderGroupCreateInfoKHR {
s_type: bool,
p_next: bool,
r#type: bool,
general_shader: bool,
closest_hit_shader: bool,
any_hit_shader: bool,
intersection_shader: bool,
p_shader_group_capture_replay_handle: bool,
}

#[repr(C)]
pub struct VkRayTracingPipelineCreateInfoNV {
s_type: bool,
p_next: bool,
flags: bool,
stage_count: bool,
p_stages: bool,
group_count: bool,
p_groups: bool,
max_recursion_depth: bool,
layout: bool,
base_pipeline_handle: bool,
base_pipeline_index: bool,
}

#[repr(C)]
pub struct VkRayTracingPipelineCreateInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
stage_count: bool,
p_stages: bool,
group_count: bool,
p_groups: bool,
max_pipeline_ray_recursion_depth: bool,
p_library_info: bool,
p_library_interface: bool,
p_dynamic_state: bool,
layout: bool,
base_pipeline_handle: bool,
base_pipeline_index: bool,
}

#[repr(C)]
pub struct VkGeometryTrianglesNV {
s_type: bool,
p_next: bool,
vertex_data: bool,
vertex_offset: bool,
vertex_count: bool,
vertex_stride: bool,
vertex_format: bool,
index_data: bool,
index_offset: bool,
index_count: bool,
index_type: bool,
transform_data: bool,
transform_offset: bool,
}

#[repr(C)]
pub struct VkGeometryAABBNV {
s_type: bool,
p_next: bool,
aabb_data: bool,
num_aabbs: bool,
stride: bool,
offset: bool,
}

#[repr(C)]
pub struct VkGeometryDataNV {
triangles: bool,
aabbs: bool,
}

#[repr(C)]
pub struct VkGeometryNV {
s_type: bool,
p_next: bool,
geometry_type: bool,
geometry: bool,
flags: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureInfoNV {
s_type: bool,
p_next: bool,
r#type: bool,
flags: bool,
instance_count: bool,
geometry_count: bool,
p_geometries: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureCreateInfoNV {
s_type: bool,
p_next: bool,
compacted_size: bool,
info: bool,
}

#[repr(C)]
pub struct VkBindAccelerationStructureMemoryInfoNV {
s_type: bool,
p_next: bool,
acceleration_structure: bool,
memory: bool,
memory_offset: bool,
device_index_count: bool,
p_device_indices: bool,
}

#[repr(C)]
pub struct VkWriteDescriptorSetAccelerationStructureKHR {
s_type: bool,
p_next: bool,
acceleration_structure_count: bool,
p_acceleration_structures: bool,
}

#[repr(C)]
pub struct VkWriteDescriptorSetAccelerationStructureNV {
s_type: bool,
p_next: bool,
acceleration_structure_count: bool,
p_acceleration_structures: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureMemoryRequirementsInfoNV {
s_type: bool,
p_next: bool,
r#type: bool,
acceleration_structure: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceAccelerationStructureFeaturesKHR {
s_type: bool,
p_next: bool,
acceleration_structure: bool,
acceleration_structure_capture_replay: bool,
acceleration_structure_indirect_build: bool,
acceleration_structure_host_commands: bool,
descriptor_binding_acceleration_structure_update_after_bind: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
s_type: bool,
p_next: bool,
ray_tracing_pipeline: bool,
ray_tracing_pipeline_shader_group_handle_capture_replay: bool,
ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: bool,
ray_tracing_pipeline_trace_rays_indirect: bool,
ray_traversal_primitive_culling: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayQueryFeaturesKHR {
s_type: bool,
p_next: bool,
ray_query: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceAccelerationStructurePropertiesKHR {
s_type: bool,
p_next: bool,
max_geometry_count: bool,
max_instance_count: bool,
max_primitive_count: bool,
max_per_stage_descriptor_acceleration_structures: bool,
max_per_stage_descriptor_update_after_bind_acceleration_structures: bool,
max_descriptor_set_acceleration_structures: bool,
max_descriptor_set_update_after_bind_acceleration_structures: bool,
min_acceleration_structure_scratch_offset_alignment: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
s_type: bool,
p_next: bool,
shader_group_handle_size: bool,
max_ray_recursion_depth: bool,
max_shader_group_stride: bool,
shader_group_base_alignment: bool,
shader_group_handle_capture_replay_size: bool,
max_ray_dispatch_invocation_count: bool,
shader_group_handle_alignment: bool,
max_ray_hit_attribute_size: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPropertiesNV {
s_type: bool,
p_next: bool,
shader_group_handle_size: bool,
max_recursion_depth: bool,
max_shader_group_stride: bool,
shader_group_base_alignment: bool,
max_geometry_count: bool,
max_instance_count: bool,
max_triangle_count: bool,
max_descriptor_set_acceleration_structures: bool,
}

#[repr(C)]
pub struct VkStridedDeviceAddressRegionKHR {
device_address: bool,
stride: bool,
size: bool,
}

#[repr(C)]
pub struct VkTraceRaysIndirectCommandKHR {
width: bool,
height: bool,
depth: bool,
}

#[repr(C)]
pub struct VkTraceRaysIndirectCommand2KHR {
raygen_shader_record_address: bool,
raygen_shader_record_size: bool,
miss_shader_binding_table_address: bool,
miss_shader_binding_table_size: bool,
miss_shader_binding_table_stride: bool,
hit_shader_binding_table_address: bool,
hit_shader_binding_table_size: bool,
hit_shader_binding_table_stride: bool,
callable_shader_binding_table_address: bool,
callable_shader_binding_table_size: bool,
callable_shader_binding_table_stride: bool,
width: bool,
height: bool,
depth: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR {
s_type: bool,
p_next: bool,
ray_tracing_maintenance1: bool,
ray_tracing_pipeline_trace_rays_indirect2: bool,
}

#[repr(C)]
pub struct VkDrmFormatModifierPropertiesListEXT {
s_type: bool,
p_next: bool,
drm_format_modifier_count: bool,
p_drm_format_modifier_properties: bool,
}

#[repr(C)]
pub struct VkDrmFormatModifierPropertiesEXT {
drm_format_modifier: bool,
drm_format_modifier_plane_count: bool,
drm_format_modifier_tiling_features: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageDrmFormatModifierInfoEXT {
s_type: bool,
p_next: bool,
drm_format_modifier: bool,
sharing_mode: bool,
queue_family_index_count: bool,
p_queue_family_indices: bool,
}

#[repr(C)]
pub struct VkImageDrmFormatModifierListCreateInfoEXT {
s_type: bool,
p_next: bool,
drm_format_modifier_count: bool,
p_drm_format_modifiers: bool,
}

#[repr(C)]
pub struct VkImageDrmFormatModifierExplicitCreateInfoEXT {
s_type: bool,
p_next: bool,
drm_format_modifier: bool,
drm_format_modifier_plane_count: bool,
p_plane_layouts: bool,
}

#[repr(C)]
pub struct VkImageDrmFormatModifierPropertiesEXT {
s_type: bool,
p_next: bool,
drm_format_modifier: bool,
}

#[repr(C)]
pub struct VkImageStencilUsageCreateInfo {
s_type: bool,
p_next: bool,
stencil_usage: bool,
}

#[repr(C)]
pub struct VkDeviceMemoryOverallocationCreateInfoAMD {
s_type: bool,
p_next: bool,
overallocation_behavior: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapFeaturesEXT {
s_type: bool,
p_next: bool,
fragment_density_map: bool,
fragment_density_map_dynamic: bool,
fragment_density_map_non_subsampled_images: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMap2FeaturesEXT {
s_type: bool,
p_next: bool,
fragment_density_map_deferred: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
s_type: bool,
p_next: bool,
fragment_density_map_offset: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapPropertiesEXT {
s_type: bool,
p_next: bool,
min_fragment_density_texel_size: bool,
max_fragment_density_texel_size: bool,
fragment_density_invocations: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMap2PropertiesEXT {
s_type: bool,
p_next: bool,
subsampled_loads: bool,
subsampled_coarse_reconstruction_early_access: bool,
max_subsampled_array_layers: bool,
max_descriptor_set_subsampled_samplers: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
s_type: bool,
p_next: bool,
fragment_density_offset_granularity: bool,
}

#[repr(C)]
pub struct VkRenderPassFragmentDensityMapCreateInfoEXT {
s_type: bool,
p_next: bool,
fragment_density_map_attachment: bool,
}

#[repr(C)]
pub struct VkSubpassFragmentDensityMapOffsetEndInfoQCOM {
s_type: bool,
p_next: bool,
fragment_density_offset_count: bool,
p_fragment_density_offsets: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceScalarBlockLayoutFeatures {
s_type: bool,
p_next: bool,
scalar_block_layout: bool,
}

#[repr(C)]
pub struct VkSurfaceProtectedCapabilitiesKHR {
s_type: bool,
p_next: bool,
supports_protected: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures {
s_type: bool,
p_next: bool,
uniform_buffer_standard_layout: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthClipEnableFeaturesEXT {
s_type: bool,
p_next: bool,
depth_clip_enable: bool,
}

#[repr(C)]
pub struct VkPipelineRasterizationDepthClipStateCreateInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
depth_clip_enable: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryBudgetPropertiesEXT {
s_type: bool,
p_next: bool,
heap_budget: bool,
heap_usage: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryPriorityFeaturesEXT {
s_type: bool,
p_next: bool,
memory_priority: bool,
}

#[repr(C)]
pub struct VkMemoryPriorityAllocateInfoEXT {
s_type: bool,
p_next: bool,
priority: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
s_type: bool,
p_next: bool,
pageable_device_local_memory: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures {
s_type: bool,
p_next: bool,
buffer_device_address: bool,
buffer_device_address_capture_replay: bool,
buffer_device_address_multi_device: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesEXT {
s_type: bool,
p_next: bool,
buffer_device_address: bool,
buffer_device_address_capture_replay: bool,
buffer_device_address_multi_device: bool,
}

#[repr(C)]
pub struct VkBufferDeviceAddressInfo {
s_type: bool,
p_next: bool,
buffer: bool,
}

#[repr(C)]
pub struct VkBufferOpaqueCaptureAddressCreateInfo {
s_type: bool,
p_next: bool,
opaque_capture_address: bool,
}

#[repr(C)]
pub struct VkBufferDeviceAddressCreateInfoEXT {
s_type: bool,
p_next: bool,
device_address: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageViewImageFormatInfoEXT {
s_type: bool,
p_next: bool,
image_view_type: bool,
}

#[repr(C)]
pub struct VkFilterCubicImageViewImageFormatPropertiesEXT {
s_type: bool,
p_next: bool,
filter_cubic: bool,
filter_cubic_minmax: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceImagelessFramebufferFeatures {
s_type: bool,
p_next: bool,
imageless_framebuffer: bool,
}

#[repr(C)]
pub struct VkFramebufferAttachmentsCreateInfo {
s_type: bool,
p_next: bool,
attachment_image_info_count: bool,
p_attachment_image_infos: bool,
}

#[repr(C)]
pub struct VkFramebufferAttachmentImageInfo {
s_type: bool,
p_next: bool,
flags: bool,
usage: bool,
width: bool,
height: bool,
layer_count: bool,
view_format_count: bool,
p_view_formats: bool,
}

#[repr(C)]
pub struct VkRenderPassAttachmentBeginInfo {
s_type: bool,
p_next: bool,
attachment_count: bool,
p_attachments: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceTextureCompressionASTCHDRFeatures {
s_type: bool,
p_next: bool,
texture_compression_astc_hdr: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixFeaturesNV {
s_type: bool,
p_next: bool,
cooperative_matrix: bool,
cooperative_matrix_robust_buffer_access: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixPropertiesNV {
s_type: bool,
p_next: bool,
cooperative_matrix_supported_stages: bool,
}

#[repr(C)]
pub struct VkCooperativeMatrixPropertiesNV {
s_type: bool,
p_next: bool,
_msize: bool,
_nsize: bool,
_ksize: bool,
_atype: bool,
_btype: bool,
_ctype: bool,
_dtype: bool,
scope: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceYcbcrImageArraysFeaturesEXT {
s_type: bool,
p_next: bool,
ycbcr_image_arrays: bool,
}

#[repr(C)]
pub struct VkImageViewHandleInfoNVX {
s_type: bool,
p_next: bool,
image_view: bool,
descriptor_type: bool,
sampler: bool,
}

#[repr(C)]
pub struct VkImageViewAddressPropertiesNVX {
s_type: bool,
p_next: bool,
device_address: bool,
size: bool,
}

#[repr(C)]
pub struct VkPresentFrameTokenGGP {
s_type: bool,
p_next: bool,
frame_token: bool,
}

#[repr(C)]
pub struct VkPipelineCreationFeedback {
flags: bool,
duration: bool,
}

#[repr(C)]
pub struct VkPipelineCreationFeedbackCreateInfo {
s_type: bool,
p_next: bool,
p_pipeline_creation_feedback: bool,
pipeline_stage_creation_feedback_count: bool,
p_pipeline_stage_creation_feedbacks: bool,
}

#[repr(C)]
pub struct VkSurfaceFullScreenExclusiveInfoEXT {
s_type: bool,
p_next: bool,
full_screen_exclusive: bool,
}

#[repr(C)]
pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
s_type: bool,
p_next: bool,
hmonitor: bool,
}

#[repr(C)]
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
s_type: bool,
p_next: bool,
full_screen_exclusive_supported: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentBarrierFeaturesNV {
s_type: bool,
p_next: bool,
present_barrier: bool,
}

#[repr(C)]
pub struct VkSurfaceCapabilitiesPresentBarrierNV {
s_type: bool,
p_next: bool,
present_barrier_supported: bool,
}

#[repr(C)]
pub struct VkSwapchainPresentBarrierCreateInfoNV {
s_type: bool,
p_next: bool,
present_barrier_enable: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePerformanceQueryFeaturesKHR {
s_type: bool,
p_next: bool,
performance_counter_query_pools: bool,
performance_counter_multiple_query_pools: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePerformanceQueryPropertiesKHR {
s_type: bool,
p_next: bool,
allow_command_buffer_query_copies: bool,
}

#[repr(C)]
pub struct VkPerformanceCounterKHR {
s_type: bool,
p_next: bool,
unit: bool,
scope: bool,
storage: bool,
uuid: bool,
}

#[repr(C)]
pub struct VkPerformanceCounterDescriptionKHR {
s_type: bool,
p_next: bool,
flags: bool,
name: bool,
category: bool,
description: bool,
}

#[repr(C)]
pub struct VkQueryPoolPerformanceCreateInfoKHR {
s_type: bool,
p_next: bool,
queue_family_index: bool,
counter_index_count: bool,
p_counter_indices: bool,
}

#[repr(C)]
pub struct VkAcquireProfilingLockInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
timeout: bool,
}

#[repr(C)]
pub struct VkPerformanceQuerySubmitInfoKHR {
s_type: bool,
p_next: bool,
counter_pass_index: bool,
}

#[repr(C)]
pub struct VkPerformanceQueryReservationInfoKHR {
s_type: bool,
p_next: bool,
max_performance_queries_per_pool: bool,
}

#[repr(C)]
pub struct VkHeadlessSurfaceCreateInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCoverageReductionModeFeaturesNV {
s_type: bool,
p_next: bool,
coverage_reduction_mode: bool,
}

#[repr(C)]
pub struct VkPipelineCoverageReductionStateCreateInfoNV {
s_type: bool,
p_next: bool,
flags: bool,
coverage_reduction_mode: bool,
}

#[repr(C)]
pub struct VkFramebufferMixedSamplesCombinationNV {
s_type: bool,
p_next: bool,
coverage_reduction_mode: bool,
rasterization_samples: bool,
depth_stencil_samples: bool,
color_samples: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
s_type: bool,
p_next: bool,
shader_integer_functions2: bool,
}

#[repr(C)]
pub struct VkPerformanceValueINTEL {
r#type: bool,
data: bool,
}

#[repr(C)]
pub struct VkInitializePerformanceApiInfoINTEL {
s_type: bool,
p_next: bool,
p_user_data: bool,
}

#[repr(C)]
pub struct VkQueryPoolPerformanceQueryCreateInfoINTEL {
s_type: bool,
p_next: bool,
performance_counters_sampling: bool,
}

#[repr(C)]
pub struct VkPerformanceMarkerInfoINTEL {
s_type: bool,
p_next: bool,
marker: bool,
}

#[repr(C)]
pub struct VkPerformanceStreamMarkerInfoINTEL {
s_type: bool,
p_next: bool,
marker: bool,
}

#[repr(C)]
pub struct VkPerformanceOverrideInfoINTEL {
s_type: bool,
p_next: bool,
r#type: bool,
enable: bool,
parameter: bool,
}

#[repr(C)]
pub struct VkPerformanceConfigurationAcquireInfoINTEL {
s_type: bool,
p_next: bool,
r#type: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderClockFeaturesKHR {
s_type: bool,
p_next: bool,
shader_subgroup_clock: bool,
shader_device_clock: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceIndexTypeUint8FeaturesEXT {
s_type: bool,
p_next: bool,
index_type_uint8: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSMBuiltinsPropertiesNV {
s_type: bool,
p_next: bool,
shader_smcount: bool,
shader_warps_per_sm: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSMBuiltinsFeaturesNV {
s_type: bool,
p_next: bool,
shader_smbuiltins: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT {
s_type: bool,
p_next: bool,
fragment_shader_sample_interlock: bool,
fragment_shader_pixel_interlock: bool,
fragment_shader_shading_rate_interlock: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures {
s_type: bool,
p_next: bool,
separate_depth_stencil_layouts: bool,
}

#[repr(C)]
pub struct VkAttachmentReferenceStencilLayout {
s_type: bool,
p_next: bool,
stencil_layout: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
s_type: bool,
p_next: bool,
primitive_topology_list_restart: bool,
primitive_topology_patch_list_restart: bool,
}

#[repr(C)]
pub struct VkAttachmentDescriptionStencilLayout {
s_type: bool,
p_next: bool,
stencil_initial_layout: bool,
stencil_final_layout: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
s_type: bool,
p_next: bool,
pipeline_executable_info: bool,
}

#[repr(C)]
pub struct VkPipelineInfoKHR {
s_type: bool,
p_next: bool,
pipeline: bool,
}

#[repr(C)]
pub struct VkPipelineExecutablePropertiesKHR {
s_type: bool,
p_next: bool,
stages: bool,
name: bool,
description: bool,
subgroup_size: bool,
}

#[repr(C)]
pub struct VkPipelineExecutableInfoKHR {
s_type: bool,
p_next: bool,
pipeline: bool,
executable_index: bool,
}

#[repr(C)]
pub struct VkPipelineExecutableStatisticKHR {
s_type: bool,
p_next: bool,
name: bool,
description: bool,
format: bool,
value: bool,
}

#[repr(C)]
pub struct VkPipelineExecutableInternalRepresentationKHR {
s_type: bool,
p_next: bool,
name: bool,
description: bool,
is_text: bool,
data_size: bool,
p_data: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures {
s_type: bool,
p_next: bool,
shader_demote_to_helper_invocation: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT {
s_type: bool,
p_next: bool,
texel_buffer_alignment: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceTexelBufferAlignmentProperties {
s_type: bool,
p_next: bool,
storage_texel_buffer_offset_alignment_bytes: bool,
storage_texel_buffer_offset_single_texel_alignment: bool,
uniform_texel_buffer_offset_alignment_bytes: bool,
uniform_texel_buffer_offset_single_texel_alignment: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubgroupSizeControlFeatures {
s_type: bool,
p_next: bool,
subgroup_size_control: bool,
compute_full_subgroups: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubgroupSizeControlProperties {
s_type: bool,
p_next: bool,
min_subgroup_size: bool,
max_subgroup_size: bool,
max_compute_workgroup_subgroups: bool,
required_subgroup_size_stages: bool,
}

#[repr(C)]
pub struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
s_type: bool,
p_next: bool,
required_subgroup_size: bool,
}

#[repr(C)]
pub struct VkSubpassShadingPipelineCreateInfoHUAWEI {
s_type: bool,
p_next: bool,
render_pass: bool,
subpass: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubpassShadingPropertiesHUAWEI {
s_type: bool,
p_next: bool,
max_subpass_shading_workgroup_size_aspect_ratio: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI {
s_type: bool,
p_next: bool,
max_work_group_count: bool,
max_work_group_size: bool,
max_output_cluster_count: bool,
indirect_buffer_offset_alignment: bool,
}

#[repr(C)]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfo {
s_type: bool,
p_next: bool,
opaque_capture_address: bool,
}

#[repr(C)]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfo {
s_type: bool,
p_next: bool,
memory: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationFeaturesEXT {
s_type: bool,
p_next: bool,
rectangular_lines: bool,
bresenham_lines: bool,
smooth_lines: bool,
stippled_rectangular_lines: bool,
stippled_bresenham_lines: bool,
stippled_smooth_lines: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationPropertiesEXT {
s_type: bool,
p_next: bool,
line_sub_pixel_precision_bits: bool,
}

#[repr(C)]
pub struct VkPipelineRasterizationLineStateCreateInfoEXT {
s_type: bool,
p_next: bool,
line_rasterization_mode: bool,
stippled_line_enable: bool,
line_stipple_factor: bool,
line_stipple_pattern: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineCreationCacheControlFeatures {
s_type: bool,
p_next: bool,
pipeline_creation_cache_control: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan11Features {
s_type: bool,
p_next: bool,
storage_buffer16_bit_access: bool,
uniform_and_storage_buffer16_bit_access: bool,
storage_push_constant16: bool,
storage_input_output16: bool,
multiview: bool,
multiview_geometry_shader: bool,
multiview_tessellation_shader: bool,
variable_pointers_storage_buffer: bool,
variable_pointers: bool,
protected_memory: bool,
sampler_ycbcr_conversion: bool,
shader_draw_parameters: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan11Properties {
s_type: bool,
p_next: bool,
device_uuid: bool,
driver_uuid: bool,
device_luid: bool,
device_node_mask: bool,
device_luidvalid: bool,
subgroup_size: bool,
subgroup_supported_stages: bool,
subgroup_supported_operations: bool,
subgroup_quad_operations_in_all_stages: bool,
point_clipping_behavior: bool,
max_multiview_view_count: bool,
max_multiview_instance_index: bool,
protected_no_fault: bool,
max_per_set_descriptors: bool,
max_memory_allocation_size: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan12Features {
s_type: bool,
p_next: bool,
sampler_mirror_clamp_to_edge: bool,
draw_indirect_count: bool,
storage_buffer8_bit_access: bool,
uniform_and_storage_buffer8_bit_access: bool,
storage_push_constant8: bool,
shader_buffer_int64_atomics: bool,
shader_shared_int64_atomics: bool,
shader_float16: bool,
shader_int8: bool,
descriptor_indexing: bool,
shader_input_attachment_array_dynamic_indexing: bool,
shader_uniform_texel_buffer_array_dynamic_indexing: bool,
shader_storage_texel_buffer_array_dynamic_indexing: bool,
shader_uniform_buffer_array_non_uniform_indexing: bool,
shader_sampled_image_array_non_uniform_indexing: bool,
shader_storage_buffer_array_non_uniform_indexing: bool,
shader_storage_image_array_non_uniform_indexing: bool,
shader_input_attachment_array_non_uniform_indexing: bool,
shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
shader_storage_texel_buffer_array_non_uniform_indexing: bool,
descriptor_binding_uniform_buffer_update_after_bind: bool,
descriptor_binding_sampled_image_update_after_bind: bool,
descriptor_binding_storage_image_update_after_bind: bool,
descriptor_binding_storage_buffer_update_after_bind: bool,
descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
descriptor_binding_storage_texel_buffer_update_after_bind: bool,
descriptor_binding_update_unused_while_pending: bool,
descriptor_binding_partially_bound: bool,
descriptor_binding_variable_descriptor_count: bool,
runtime_descriptor_array: bool,
sampler_filter_minmax: bool,
scalar_block_layout: bool,
imageless_framebuffer: bool,
uniform_buffer_standard_layout: bool,
shader_subgroup_extended_types: bool,
separate_depth_stencil_layouts: bool,
host_query_reset: bool,
timeline_semaphore: bool,
buffer_device_address: bool,
buffer_device_address_capture_replay: bool,
buffer_device_address_multi_device: bool,
vulkan_memory_model: bool,
vulkan_memory_model_device_scope: bool,
vulkan_memory_model_availability_visibility_chains: bool,
shader_output_viewport_index: bool,
shader_output_layer: bool,
subgroup_broadcast_dynamic_id: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan12Properties {
s_type: bool,
p_next: bool,
driver_id: bool,
driver_name: bool,
driver_info: bool,
conformance_version: bool,
denorm_behavior_independence: bool,
rounding_mode_independence: bool,
shader_signed_zero_inf_nan_preserve_float16: bool,
shader_signed_zero_inf_nan_preserve_float32: bool,
shader_signed_zero_inf_nan_preserve_float64: bool,
shader_denorm_preserve_float16: bool,
shader_denorm_preserve_float32: bool,
shader_denorm_preserve_float64: bool,
shader_denorm_flush_to_zero_float16: bool,
shader_denorm_flush_to_zero_float32: bool,
shader_denorm_flush_to_zero_float64: bool,
shader_rounding_mode_rtefloat16: bool,
shader_rounding_mode_rtefloat32: bool,
shader_rounding_mode_rtefloat64: bool,
shader_rounding_mode_rtzfloat16: bool,
shader_rounding_mode_rtzfloat32: bool,
shader_rounding_mode_rtzfloat64: bool,
max_update_after_bind_descriptors_in_all_pools: bool,
shader_uniform_buffer_array_non_uniform_indexing_native: bool,
shader_sampled_image_array_non_uniform_indexing_native: bool,
shader_storage_buffer_array_non_uniform_indexing_native: bool,
shader_storage_image_array_non_uniform_indexing_native: bool,
shader_input_attachment_array_non_uniform_indexing_native: bool,
robust_buffer_access_update_after_bind: bool,
quad_divergent_implicit_lod: bool,
max_per_stage_descriptor_update_after_bind_samplers: bool,
max_per_stage_descriptor_update_after_bind_uniform_buffers: bool,
max_per_stage_descriptor_update_after_bind_storage_buffers: bool,
max_per_stage_descriptor_update_after_bind_sampled_images: bool,
max_per_stage_descriptor_update_after_bind_storage_images: bool,
max_per_stage_descriptor_update_after_bind_input_attachments: bool,
max_per_stage_update_after_bind_resources: bool,
max_descriptor_set_update_after_bind_samplers: bool,
max_descriptor_set_update_after_bind_uniform_buffers: bool,
max_descriptor_set_update_after_bind_uniform_buffers_dynamic: bool,
max_descriptor_set_update_after_bind_storage_buffers: bool,
max_descriptor_set_update_after_bind_storage_buffers_dynamic: bool,
max_descriptor_set_update_after_bind_sampled_images: bool,
max_descriptor_set_update_after_bind_storage_images: bool,
max_descriptor_set_update_after_bind_input_attachments: bool,
supported_depth_resolve_modes: bool,
supported_stencil_resolve_modes: bool,
independent_resolve_none: bool,
independent_resolve: bool,
filter_minmax_single_component_formats: bool,
filter_minmax_image_component_mapping: bool,
max_timeline_semaphore_value_difference: bool,
framebuffer_integer_color_sample_counts: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan13Features {
s_type: bool,
p_next: bool,
robust_image_access: bool,
inline_uniform_block: bool,
descriptor_binding_inline_uniform_block_update_after_bind: bool,
pipeline_creation_cache_control: bool,
private_data: bool,
shader_demote_to_helper_invocation: bool,
shader_terminate_invocation: bool,
subgroup_size_control: bool,
compute_full_subgroups: bool,
synchronization2: bool,
texture_compression_astc_hdr: bool,
shader_zero_initialize_workgroup_memory: bool,
dynamic_rendering: bool,
shader_integer_dot_product: bool,
maintenance4: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan13Properties {
s_type: bool,
p_next: bool,
min_subgroup_size: bool,
max_subgroup_size: bool,
max_compute_workgroup_subgroups: bool,
required_subgroup_size_stages: bool,
max_inline_uniform_block_size: bool,
max_per_stage_descriptor_inline_uniform_blocks: bool,
max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: bool,
max_descriptor_set_inline_uniform_blocks: bool,
max_descriptor_set_update_after_bind_inline_uniform_blocks: bool,
max_inline_uniform_total_size: bool,
integer_dot_product8_bit_unsigned_accelerated: bool,
integer_dot_product8_bit_signed_accelerated: bool,
integer_dot_product8_bit_mixed_signedness_accelerated: bool,
integer_dot_product4x8_bit_packed_unsigned_accelerated: bool,
integer_dot_product4x8_bit_packed_signed_accelerated: bool,
integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: bool,
integer_dot_product16_bit_unsigned_accelerated: bool,
integer_dot_product16_bit_signed_accelerated: bool,
integer_dot_product16_bit_mixed_signedness_accelerated: bool,
integer_dot_product32_bit_unsigned_accelerated: bool,
integer_dot_product32_bit_signed_accelerated: bool,
integer_dot_product32_bit_mixed_signedness_accelerated: bool,
integer_dot_product64_bit_unsigned_accelerated: bool,
integer_dot_product64_bit_signed_accelerated: bool,
integer_dot_product64_bit_mixed_signedness_accelerated: bool,
integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: bool,
integer_dot_product_accumulating_saturating8_bit_signed_accelerated: bool,
integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: bool,
integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: bool,
integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: bool,
integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: bool,
integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: bool,
integer_dot_product_accumulating_saturating16_bit_signed_accelerated: bool,
integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: bool,
integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: bool,
integer_dot_product_accumulating_saturating32_bit_signed_accelerated: bool,
integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: bool,
integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: bool,
integer_dot_product_accumulating_saturating64_bit_signed_accelerated: bool,
integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: bool,
storage_texel_buffer_offset_alignment_bytes: bool,
storage_texel_buffer_offset_single_texel_alignment: bool,
uniform_texel_buffer_offset_alignment_bytes: bool,
uniform_texel_buffer_offset_single_texel_alignment: bool,
max_buffer_size: bool,
}

#[repr(C)]
pub struct VkPipelineCompilerControlCreateInfoAMD {
s_type: bool,
p_next: bool,
compiler_control_flags: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCoherentMemoryFeaturesAMD {
s_type: bool,
p_next: bool,
device_coherent_memory: bool,
}

#[repr(C)]
pub struct VkFaultData {
s_type: bool,
p_next: bool,
fault_level: bool,
fault_type: bool,
}

#[repr(C)]
pub struct VkFaultCallbackInfo {
s_type: bool,
p_next: bool,
fault_count: bool,
p_faults: bool,
pfn_fault_callback: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceToolProperties {
s_type: bool,
p_next: bool,
name: bool,
version: bool,
purposes: bool,
description: bool,
layer: bool,
}

#[repr(C)]
pub struct VkSamplerCustomBorderColorCreateInfoEXT {
s_type: bool,
p_next: bool,
custom_border_color: bool,
format: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCustomBorderColorPropertiesEXT {
s_type: bool,
p_next: bool,
max_custom_border_color_samplers: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCustomBorderColorFeaturesEXT {
s_type: bool,
p_next: bool,
custom_border_colors: bool,
custom_border_color_without_format: bool,
}

#[repr(C)]
pub struct VkSamplerBorderColorComponentMappingCreateInfoEXT {
s_type: bool,
p_next: bool,
components: bool,
srgb: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceBorderColorSwizzleFeaturesEXT {
s_type: bool,
p_next: bool,
border_color_swizzle: bool,
border_color_swizzle_from_image: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryTrianglesDataKHR {
s_type: bool,
p_next: bool,
vertex_format: bool,
vertex_data: bool,
vertex_stride: bool,
max_vertex: bool,
index_type: bool,
index_data: bool,
transform_data: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryAabbsDataKHR {
s_type: bool,
p_next: bool,
data: bool,
stride: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryInstancesDataKHR {
s_type: bool,
p_next: bool,
array_of_pointers: bool,
data: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryKHR {
s_type: bool,
p_next: bool,
geometry_type: bool,
geometry: bool,
flags: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureBuildGeometryInfoKHR {
s_type: bool,
p_next: bool,
r#type: bool,
flags: bool,
mode: bool,
src_acceleration_structure: bool,
dst_acceleration_structure: bool,
geometry_count: bool,
p_geometries: bool,
pp_geometries: bool,
scratch_data: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureBuildRangeInfoKHR {
primitive_count: bool,
primitive_offset: bool,
first_vertex: bool,
transform_offset: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureCreateInfoKHR {
s_type: bool,
p_next: bool,
create_flags: bool,
buffer: bool,
offset: bool,
size: bool,
r#type: bool,
device_address: bool,
}

#[repr(C)]
pub struct VkAabbPositionsKHR {
min_x: bool,
min_y: bool,
min_z: bool,
max_x: bool,
max_y: bool,
max_z: bool,
}

#[repr(C)]
pub struct VkTransformMatrixKHR {
matrix: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureInstanceKHR {
transform: bool,
instance_custom_index: bool,
mask: bool,
instance_shader_binding_table_record_offset: bool,
flags: bool,
acceleration_structure_reference: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureDeviceAddressInfoKHR {
s_type: bool,
p_next: bool,
acceleration_structure: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureVersionInfoKHR {
s_type: bool,
p_next: bool,
p_version_data: bool,
}

#[repr(C)]
pub struct VkCopyAccelerationStructureInfoKHR {
s_type: bool,
p_next: bool,
src: bool,
dst: bool,
mode: bool,
}

#[repr(C)]
pub struct VkCopyAccelerationStructureToMemoryInfoKHR {
s_type: bool,
p_next: bool,
src: bool,
dst: bool,
mode: bool,
}

#[repr(C)]
pub struct VkCopyMemoryToAccelerationStructureInfoKHR {
s_type: bool,
p_next: bool,
src: bool,
dst: bool,
mode: bool,
}

#[repr(C)]
pub struct VkRayTracingPipelineInterfaceCreateInfoKHR {
s_type: bool,
p_next: bool,
max_pipeline_ray_payload_size: bool,
max_pipeline_ray_hit_attribute_size: bool,
}

#[repr(C)]
pub struct VkPipelineLibraryCreateInfoKHR {
s_type: bool,
p_next: bool,
library_count: bool,
p_libraries: bool,
}

#[repr(C)]
pub struct VkRefreshObjectKHR {
object_type: bool,
object_handle: bool,
flags: bool,
}

#[repr(C)]
pub struct VkRefreshObjectListKHR {
s_type: bool,
p_next: bool,
object_count: bool,
p_objects: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicStateFeaturesEXT {
s_type: bool,
p_next: bool,
extended_dynamic_state: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState2FeaturesEXT {
s_type: bool,
p_next: bool,
extended_dynamic_state2: bool,
extended_dynamic_state2_logic_op: bool,
extended_dynamic_state2_patch_control_points: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState3FeaturesEXT {
s_type: bool,
p_next: bool,
extended_dynamic_state3_tessellation_domain_origin: bool,
extended_dynamic_state3_depth_clamp_enable: bool,
extended_dynamic_state3_polygon_mode: bool,
extended_dynamic_state3_rasterization_samples: bool,
extended_dynamic_state3_sample_mask: bool,
extended_dynamic_state3_alpha_to_coverage_enable: bool,
extended_dynamic_state3_alpha_to_one_enable: bool,
extended_dynamic_state3_logic_op_enable: bool,
extended_dynamic_state3_color_blend_enable: bool,
extended_dynamic_state3_color_blend_equation: bool,
extended_dynamic_state3_color_write_mask: bool,
extended_dynamic_state3_rasterization_stream: bool,
extended_dynamic_state3_conservative_rasterization_mode: bool,
extended_dynamic_state3_extra_primitive_overestimation_size: bool,
extended_dynamic_state3_depth_clip_enable: bool,
extended_dynamic_state3_sample_locations_enable: bool,
extended_dynamic_state3_color_blend_advanced: bool,
extended_dynamic_state3_provoking_vertex_mode: bool,
extended_dynamic_state3_line_rasterization_mode: bool,
extended_dynamic_state3_line_stipple_enable: bool,
extended_dynamic_state3_depth_clip_negative_one_to_one: bool,
extended_dynamic_state3_viewport_wscaling_enable: bool,
extended_dynamic_state3_viewport_swizzle: bool,
extended_dynamic_state3_coverage_to_color_enable: bool,
extended_dynamic_state3_coverage_to_color_location: bool,
extended_dynamic_state3_coverage_modulation_mode: bool,
extended_dynamic_state3_coverage_modulation_table_enable: bool,
extended_dynamic_state3_coverage_modulation_table: bool,
extended_dynamic_state3_coverage_reduction_mode: bool,
extended_dynamic_state3_representative_fragment_test_enable: bool,
extended_dynamic_state3_shading_rate_image_enable: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState3PropertiesEXT {
s_type: bool,
p_next: bool,
dynamic_primitive_topology_unrestricted: bool,
}

#[repr(C)]
pub struct VkColorBlendEquationEXT {
src_color_blend_factor: bool,
dst_color_blend_factor: bool,
color_blend_op: bool,
src_alpha_blend_factor: bool,
dst_alpha_blend_factor: bool,
alpha_blend_op: bool,
}

#[repr(C)]
pub struct VkColorBlendAdvancedEXT {
advanced_blend_op: bool,
src_premultiplied: bool,
dst_premultiplied: bool,
blend_overlap: bool,
clamp_results: bool,
}

#[repr(C)]
pub struct VkRenderPassTransformBeginInfoQCOM {
s_type: bool,
p_next: bool,
transform: bool,
}

#[repr(C)]
pub struct VkCopyCommandTransformInfoQCOM {
s_type: bool,
p_next: bool,
transform: bool,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
s_type: bool,
p_next: bool,
transform: bool,
render_area: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDiagnosticsConfigFeaturesNV {
s_type: bool,
p_next: bool,
diagnostics_config: bool,
}

#[repr(C)]
pub struct VkDeviceDiagnosticsConfigCreateInfoNV {
s_type: bool,
p_next: bool,
flags: bool,
}

#[repr(C)]
pub struct VkPipelineOfflineCreateInfo {
s_type: bool,
p_next: bool,
pipeline_identifier: bool,
match_control: bool,
pool_entry_size: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
s_type: bool,
p_next: bool,
shader_zero_initialize_workgroup_memory: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
s_type: bool,
p_next: bool,
shader_subgroup_uniform_control_flow: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRobustness2FeaturesEXT {
s_type: bool,
p_next: bool,
robust_buffer_access2: bool,
robust_image_access2: bool,
null_descriptor: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRobustness2PropertiesEXT {
s_type: bool,
p_next: bool,
robust_storage_buffer_access_size_alignment: bool,
robust_uniform_buffer_access_size_alignment: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageRobustnessFeatures {
s_type: bool,
p_next: bool,
robust_image_access: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
s_type: bool,
p_next: bool,
workgroup_memory_explicit_layout: bool,
workgroup_memory_explicit_layout_scalar_block_layout: bool,
workgroup_memory_explicit_layout8_bit_access: bool,
workgroup_memory_explicit_layout16_bit_access: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePortabilitySubsetFeaturesKHR {
s_type: bool,
p_next: bool,
ant_alpha_color_blend_factors: bool,
events: bool,
image_view_format_reinterpretation: bool,
image_view_format_swizzle: bool,
image_view2_don3_dimage: bool,
multisample_array_image: bool,
mutable_comparison_samplers: bool,
point_polygons: bool,
sampler_mip_lod_bias: bool,
separate_stencil_mask_ref: bool,
shader_sample_rate_interpolation_functions: bool,
tessellation_isolines: bool,
tessellation_point_mode: bool,
triangle_fans: bool,
vertex_attribute_access_beyond_stride: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePortabilitySubsetPropertiesKHR {
s_type: bool,
p_next: bool,
min_vertex_input_binding_stride_alignment: bool,
}

#[repr(C)]
pub struct VkPhysicalDevice4444FormatsFeaturesEXT {
s_type: bool,
p_next: bool,
format_a4_r4_g4_b4: bool,
format_a4_b4_g4_r4: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubpassShadingFeaturesHUAWEI {
s_type: bool,
p_next: bool,
subpass_shading: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI {
s_type: bool,
p_next: bool,
clusterculling_shader: bool,
multiview_cluster_culling_shader: bool,
}

#[repr(C)]
pub struct VkBufferCopy2 {
s_type: bool,
p_next: bool,
src_offset: bool,
dst_offset: bool,
size: bool,
}

#[repr(C)]
pub struct VkImageCopy2 {
s_type: bool,
p_next: bool,
src_subresource: bool,
src_offset: bool,
dst_subresource: bool,
dst_offset: bool,
extent: bool,
}

#[repr(C)]
pub struct VkImageBlit2 {
s_type: bool,
p_next: bool,
src_subresource: bool,
src_offsets: bool,
dst_subresource: bool,
dst_offsets: bool,
}

#[repr(C)]
pub struct VkBufferImageCopy2 {
s_type: bool,
p_next: bool,
buffer_offset: bool,
buffer_row_length: bool,
buffer_image_height: bool,
image_subresource: bool,
image_offset: bool,
image_extent: bool,
}

#[repr(C)]
pub struct VkImageResolve2 {
s_type: bool,
p_next: bool,
src_subresource: bool,
src_offset: bool,
dst_subresource: bool,
dst_offset: bool,
extent: bool,
}

#[repr(C)]
pub struct VkCopyBufferInfo2 {
s_type: bool,
p_next: bool,
src_buffer: bool,
dst_buffer: bool,
region_count: bool,
p_regions: bool,
}

#[repr(C)]
pub struct VkCopyImageInfo2 {
s_type: bool,
p_next: bool,
src_image: bool,
src_image_layout: bool,
dst_image: bool,
dst_image_layout: bool,
region_count: bool,
p_regions: bool,
}

#[repr(C)]
pub struct VkBlitImageInfo2 {
s_type: bool,
p_next: bool,
src_image: bool,
src_image_layout: bool,
dst_image: bool,
dst_image_layout: bool,
region_count: bool,
p_regions: bool,
filter: bool,
}

#[repr(C)]
pub struct VkCopyBufferToImageInfo2 {
s_type: bool,
p_next: bool,
src_buffer: bool,
dst_image: bool,
dst_image_layout: bool,
region_count: bool,
p_regions: bool,
}

#[repr(C)]
pub struct VkCopyImageToBufferInfo2 {
s_type: bool,
p_next: bool,
src_image: bool,
src_image_layout: bool,
dst_buffer: bool,
region_count: bool,
p_regions: bool,
}

#[repr(C)]
pub struct VkResolveImageInfo2 {
s_type: bool,
p_next: bool,
src_image: bool,
src_image_layout: bool,
dst_image: bool,
dst_image_layout: bool,
region_count: bool,
p_regions: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
s_type: bool,
p_next: bool,
shader_image_int64_atomics: bool,
sparse_image_int64_atomics: bool,
}

#[repr(C)]
pub struct VkFragmentShadingRateAttachmentInfoKHR {
s_type: bool,
p_next: bool,
p_fragment_shading_rate_attachment: bool,
shading_rate_attachment_texel_size: bool,
}

#[repr(C)]
pub struct VkPipelineFragmentShadingRateStateCreateInfoKHR {
s_type: bool,
p_next: bool,
fragment_size: bool,
combiner_ops: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateFeaturesKHR {
s_type: bool,
p_next: bool,
pipeline_fragment_shading_rate: bool,
primitive_fragment_shading_rate: bool,
attachment_fragment_shading_rate: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRatePropertiesKHR {
s_type: bool,
p_next: bool,
min_fragment_shading_rate_attachment_texel_size: bool,
max_fragment_shading_rate_attachment_texel_size: bool,
max_fragment_shading_rate_attachment_texel_size_aspect_ratio: bool,
primitive_fragment_shading_rate_with_multiple_viewports: bool,
layered_shading_rate_attachments: bool,
fragment_shading_rate_non_trivial_combiner_ops: bool,
max_fragment_size: bool,
max_fragment_size_aspect_ratio: bool,
max_fragment_shading_rate_coverage_samples: bool,
max_fragment_shading_rate_rasterization_samples: bool,
fragment_shading_rate_with_shader_depth_stencil_writes: bool,
fragment_shading_rate_with_sample_mask: bool,
fragment_shading_rate_with_shader_sample_mask: bool,
fragment_shading_rate_with_conservative_rasterization: bool,
fragment_shading_rate_with_fragment_shader_interlock: bool,
fragment_shading_rate_with_custom_sample_locations: bool,
fragment_shading_rate_strict_multiply_combiner: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateKHR {
s_type: bool,
p_next: bool,
sample_counts: bool,
fragment_size: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderTerminateInvocationFeatures {
s_type: bool,
p_next: bool,
shader_terminate_invocation: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
s_type: bool,
p_next: bool,
fragment_shading_rate_enums: bool,
supersample_fragment_shading_rates: bool,
no_invocation_fragment_shading_rates: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
s_type: bool,
p_next: bool,
max_fragment_shading_rate_invocation_count: bool,
}

#[repr(C)]
pub struct VkPipelineFragmentShadingRateEnumStateCreateInfoNV {
s_type: bool,
p_next: bool,
shading_rate_type: bool,
shading_rate: bool,
combiner_ops: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureBuildSizesInfoKHR {
s_type: bool,
p_next: bool,
acceleration_structure_size: bool,
update_scratch_size: bool,
build_scratch_size: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceImage2DViewOf3DFeaturesEXT {
s_type: bool,
p_next: bool,
image2_dview_of3_d: bool,
sampler2_dview_of3_d: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT {
s_type: bool,
p_next: bool,
image_sliced_view_of3_d: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT {
s_type: bool,
p_next: bool,
attachment_feedback_loop_dynamic_state: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT {
s_type: bool,
p_next: bool,
mutable_descriptor_type: bool,
}

#[repr(C)]
pub struct VkMutableDescriptorTypeListEXT {
descriptor_type_count: bool,
p_descriptor_types: bool,
}

#[repr(C)]
pub struct VkMutableDescriptorTypeCreateInfoEXT {
s_type: bool,
p_next: bool,
mutable_descriptor_type_list_count: bool,
p_mutable_descriptor_type_lists: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthClipControlFeaturesEXT {
s_type: bool,
p_next: bool,
depth_clip_control: bool,
}

#[repr(C)]
pub struct VkPipelineViewportDepthClipControlCreateInfoEXT {
s_type: bool,
p_next: bool,
negative_one_to_one: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT {
s_type: bool,
p_next: bool,
vertex_input_dynamic_state: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryRDMAFeaturesNV {
s_type: bool,
p_next: bool,
external_memory_rdma: bool,
}

#[repr(C)]
pub struct VkVertexInputBindingDescription2EXT {
s_type: bool,
p_next: bool,
binding: bool,
stride: bool,
input_rate: bool,
divisor: bool,
}

#[repr(C)]
pub struct VkVertexInputAttributeDescription2EXT {
s_type: bool,
p_next: bool,
location: bool,
binding: bool,
format: bool,
offset: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceColorWriteEnableFeaturesEXT {
s_type: bool,
p_next: bool,
color_write_enable: bool,
}

#[repr(C)]
pub struct VkPipelineColorWriteCreateInfoEXT {
s_type: bool,
p_next: bool,
attachment_count: bool,
p_color_write_enables: bool,
}

#[repr(C)]
pub struct VkMemoryBarrier2 {
s_type: bool,
p_next: bool,
src_stage_mask: bool,
src_access_mask: bool,
dst_stage_mask: bool,
dst_access_mask: bool,
}

#[repr(C)]
pub struct VkImageMemoryBarrier2 {
s_type: bool,
p_next: bool,
src_stage_mask: bool,
src_access_mask: bool,
dst_stage_mask: bool,
dst_access_mask: bool,
old_layout: bool,
new_layout: bool,
src_queue_family_index: bool,
dst_queue_family_index: bool,
image: bool,
subresource_range: bool,
}

#[repr(C)]
pub struct VkBufferMemoryBarrier2 {
s_type: bool,
p_next: bool,
src_stage_mask: bool,
src_access_mask: bool,
dst_stage_mask: bool,
dst_access_mask: bool,
src_queue_family_index: bool,
dst_queue_family_index: bool,
buffer: bool,
offset: bool,
size: bool,
}

#[repr(C)]
pub struct VkDependencyInfo {
s_type: bool,
p_next: bool,
dependency_flags: bool,
memory_barrier_count: bool,
p_memory_barriers: bool,
buffer_memory_barrier_count: bool,
p_buffer_memory_barriers: bool,
image_memory_barrier_count: bool,
p_image_memory_barriers: bool,
}

#[repr(C)]
pub struct VkSemaphoreSubmitInfo {
s_type: bool,
p_next: bool,
semaphore: bool,
value: bool,
stage_mask: bool,
device_index: bool,
}

#[repr(C)]
pub struct VkCommandBufferSubmitInfo {
s_type: bool,
p_next: bool,
command_buffer: bool,
device_mask: bool,
}

#[repr(C)]
pub struct VkSubmitInfo2 {
s_type: bool,
p_next: bool,
flags: bool,
wait_semaphore_info_count: bool,
p_wait_semaphore_infos: bool,
command_buffer_info_count: bool,
p_command_buffer_infos: bool,
signal_semaphore_info_count: bool,
p_signal_semaphore_infos: bool,
}

#[repr(C)]
pub struct VkQueueFamilyCheckpointProperties2NV {
s_type: bool,
p_next: bool,
checkpoint_execution_stage_mask: bool,
}

#[repr(C)]
pub struct VkCheckpointData2NV {
s_type: bool,
p_next: bool,
stage: bool,
p_checkpoint_marker: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSynchronization2Features {
s_type: bool,
p_next: bool,
synchronization2: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceHostImageCopyFeaturesEXT {
s_type: bool,
p_next: bool,
host_image_copy: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceHostImageCopyPropertiesEXT {
s_type: bool,
p_next: bool,
copy_src_layout_count: bool,
p_copy_src_layouts: bool,
copy_dst_layout_count: bool,
p_copy_dst_layouts: bool,
optimal_tiling_layout_uuid: bool,
identical_memory_type_requirements: bool,
}

#[repr(C)]
pub struct VkMemoryToImageCopyEXT {
s_type: bool,
p_next: bool,
p_host_pointer: bool,
memory_row_length: bool,
memory_image_height: bool,
image_subresource: bool,
image_offset: bool,
image_extent: bool,
}

#[repr(C)]
pub struct VkImageToMemoryCopyEXT {
s_type: bool,
p_next: bool,
p_host_pointer: bool,
memory_row_length: bool,
memory_image_height: bool,
image_subresource: bool,
image_offset: bool,
image_extent: bool,
}

#[repr(C)]
pub struct VkCopyMemoryToImageInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
dst_image: bool,
dst_image_layout: bool,
region_count: bool,
p_regions: bool,
}

#[repr(C)]
pub struct VkCopyImageToMemoryInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
src_image: bool,
src_image_layout: bool,
region_count: bool,
p_regions: bool,
}

#[repr(C)]
pub struct VkCopyImageToImageInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
src_image: bool,
src_image_layout: bool,
dst_image: bool,
dst_image_layout: bool,
region_count: bool,
p_regions: bool,
}

#[repr(C)]
pub struct VkHostImageLayoutTransitionInfoEXT {
s_type: bool,
p_next: bool,
image: bool,
old_layout: bool,
new_layout: bool,
subresource_range: bool,
}

#[repr(C)]
pub struct VkSubresourceHostMemcpySizeEXT {
s_type: bool,
p_next: bool,
size: bool,
}

#[repr(C)]
pub struct VkHostImageCopyDevicePerformanceQueryEXT {
s_type: bool,
p_next: bool,
optimal_device_access: bool,
identical_memory_layout: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkanSC10Properties {
s_type: bool,
p_next: bool,
device_no_dynamic_host_allocations: bool,
device_destroy_frees_memory: bool,
command_pool_multiple_command_buffers_recording: bool,
command_pool_reset_command_buffer: bool,
command_buffer_simultaneous_use: bool,
secondary_command_buffer_null_or_imageless_framebuffer: bool,
recycle_descriptor_set_memory: bool,
recycle_pipeline_memory: bool,
max_render_pass_subpasses: bool,
max_render_pass_dependencies: bool,
max_subpass_input_attachments: bool,
max_subpass_preserve_attachments: bool,
max_framebuffer_attachments: bool,
max_descriptor_set_layout_bindings: bool,
max_query_fault_count: bool,
max_callback_fault_count: bool,
max_command_pool_command_buffers: bool,
max_command_buffer_size: bool,
}

#[repr(C)]
pub struct VkPipelinePoolSize {
s_type: bool,
p_next: bool,
pool_entry_size: bool,
pool_entry_count: bool,
}

#[repr(C)]
pub struct VkDeviceObjectReservationCreateInfo {
s_type: bool,
p_next: bool,
pipeline_cache_create_info_count: bool,
p_pipeline_cache_create_infos: bool,
pipeline_pool_size_count: bool,
p_pipeline_pool_sizes: bool,
semaphore_request_count: bool,
command_buffer_request_count: bool,
fence_request_count: bool,
device_memory_request_count: bool,
buffer_request_count: bool,
image_request_count: bool,
event_request_count: bool,
query_pool_request_count: bool,
buffer_view_request_count: bool,
image_view_request_count: bool,
layered_image_view_request_count: bool,
pipeline_cache_request_count: bool,
pipeline_layout_request_count: bool,
render_pass_request_count: bool,
graphics_pipeline_request_count: bool,
compute_pipeline_request_count: bool,
descriptor_set_layout_request_count: bool,
sampler_request_count: bool,
descriptor_pool_request_count: bool,
descriptor_set_request_count: bool,
framebuffer_request_count: bool,
command_pool_request_count: bool,
sampler_ycbcr_conversion_request_count: bool,
surface_request_count: bool,
swapchain_request_count: bool,
display_mode_request_count: bool,
subpass_description_request_count: bool,
attachment_description_request_count: bool,
descriptor_set_layout_binding_request_count: bool,
descriptor_set_layout_binding_limit: bool,
max_image_view_mip_levels: bool,
max_image_view_array_layers: bool,
max_layered_image_view_mip_levels: bool,
max_occlusion_queries_per_pool: bool,
max_pipeline_statistics_queries_per_pool: bool,
max_timestamp_queries_per_pool: bool,
max_immutable_samplers_per_descriptor_set_layout: bool,
}

#[repr(C)]
pub struct VkCommandPoolMemoryReservationCreateInfo {
s_type: bool,
p_next: bool,
command_pool_reserved_size: bool,
command_pool_max_command_buffers: bool,
}

#[repr(C)]
pub struct VkCommandPoolMemoryConsumption {
s_type: bool,
p_next: bool,
command_pool_allocated: bool,
command_pool_reserved_size: bool,
command_buffer_allocated: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkanSC10Features {
s_type: bool,
p_next: bool,
shader_atomic_instructions: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
s_type: bool,
p_next: bool,
primitives_generated_query: bool,
primitives_generated_query_with_rasterizer_discard: bool,
primitives_generated_query_with_non_zero_streams: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceLegacyDitheringFeaturesEXT {
s_type: bool,
p_next: bool,
legacy_dithering: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
s_type: bool,
p_next: bool,
multisampled_render_to_single_sampled: bool,
}

#[repr(C)]
pub struct VkSubpassResolvePerformanceQueryEXT {
s_type: bool,
p_next: bool,
optimal: bool,
}

#[repr(C)]
pub struct VkMultisampledRenderToSingleSampledInfoEXT {
s_type: bool,
p_next: bool,
multisampled_render_to_single_sampled_enable: bool,
rasterization_samples: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineProtectedAccessFeaturesEXT {
s_type: bool,
p_next: bool,
pipeline_protected_access: bool,
}

#[repr(C)]
pub struct VkQueueFamilyVideoPropertiesKHR {
s_type: bool,
p_next: bool,
video_codec_operations: bool,
}

#[repr(C)]
pub struct VkQueueFamilyQueryResultStatusPropertiesKHR {
s_type: bool,
p_next: bool,
query_result_status_support: bool,
}

#[repr(C)]
pub struct VkVideoProfileListInfoKHR {
s_type: bool,
p_next: bool,
profile_count: bool,
p_profiles: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVideoFormatInfoKHR {
s_type: bool,
p_next: bool,
image_usage: bool,
}

#[repr(C)]
pub struct VkVideoFormatPropertiesKHR {
s_type: bool,
p_next: bool,
format: bool,
component_mapping: bool,
image_create_flags: bool,
image_type: bool,
image_tiling: bool,
image_usage_flags: bool,
}

#[repr(C)]
pub struct VkVideoProfileInfoKHR {
s_type: bool,
p_next: bool,
video_codec_operation: bool,
chroma_subsampling: bool,
luma_bit_depth: bool,
chroma_bit_depth: bool,
}

#[repr(C)]
pub struct VkVideoCapabilitiesKHR {
s_type: bool,
p_next: bool,
flags: bool,
min_bitstream_buffer_offset_alignment: bool,
min_bitstream_buffer_size_alignment: bool,
picture_access_granularity: bool,
min_coded_extent: bool,
max_coded_extent: bool,
max_dpb_slots: bool,
max_active_reference_pictures: bool,
std_header_version: bool,
}

#[repr(C)]
pub struct VkVideoSessionMemoryRequirementsKHR {
s_type: bool,
p_next: bool,
memory_bind_index: bool,
memory_requirements: bool,
}

#[repr(C)]
pub struct VkBindVideoSessionMemoryInfoKHR {
s_type: bool,
p_next: bool,
memory_bind_index: bool,
memory: bool,
memory_offset: bool,
memory_size: bool,
}

#[repr(C)]
pub struct VkVideoPictureResourceInfoKHR {
s_type: bool,
p_next: bool,
coded_offset: bool,
coded_extent: bool,
base_array_layer: bool,
image_view_binding: bool,
}

#[repr(C)]
pub struct VkVideoReferenceSlotInfoKHR {
s_type: bool,
p_next: bool,
slot_index: bool,
p_picture_resource: bool,
}

#[repr(C)]
pub struct VkVideoDecodeCapabilitiesKHR {
s_type: bool,
p_next: bool,
flags: bool,
}

#[repr(C)]
pub struct VkVideoDecodeUsageInfoKHR {
s_type: bool,
p_next: bool,
video_usage_hints: bool,
}

#[repr(C)]
pub struct VkVideoDecodeInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
src_buffer: bool,
src_buffer_offset: bool,
src_buffer_range: bool,
dst_picture_resource: bool,
p_setup_reference_slot: bool,
reference_slot_count: bool,
p_reference_slots: bool,
}

#[repr(C)]
pub struct VkVideoDecodeH264ProfileInfoKHR {
s_type: bool,
p_next: bool,
std_profile_idc: bool,
picture_layout: bool,
}

#[repr(C)]
pub struct VkVideoDecodeH264CapabilitiesKHR {
s_type: bool,
p_next: bool,
max_level_idc: bool,
field_offset_granularity: bool,
}

#[repr(C)]
pub struct VkVideoDecodeH264SessionParametersAddInfoKHR {
s_type: bool,
p_next: bool,
std_spscount: bool,
p_std_spss: bool,
std_ppscount: bool,
p_std_ppss: bool,
}

#[repr(C)]
pub struct VkVideoDecodeH264SessionParametersCreateInfoKHR {
s_type: bool,
p_next: bool,
max_std_spscount: bool,
max_std_ppscount: bool,
p_parameters_add_info: bool,
}

#[repr(C)]
pub struct VkVideoDecodeH264PictureInfoKHR {
s_type: bool,
p_next: bool,
p_std_picture_info: bool,
slice_count: bool,
p_slice_offsets: bool,
}

#[repr(C)]
pub struct VkVideoDecodeH264DpbSlotInfoKHR {
s_type: bool,
p_next: bool,
p_std_reference_info: bool,
}

#[repr(C)]
pub struct VkVideoDecodeH265ProfileInfoKHR {
s_type: bool,
p_next: bool,
std_profile_idc: bool,
}

#[repr(C)]
pub struct VkVideoDecodeH265CapabilitiesKHR {
s_type: bool,
p_next: bool,
max_level_idc: bool,
}

#[repr(C)]
pub struct VkVideoDecodeH265SessionParametersAddInfoKHR {
s_type: bool,
p_next: bool,
std_vpscount: bool,
p_std_vpss: bool,
std_spscount: bool,
p_std_spss: bool,
std_ppscount: bool,
p_std_ppss: bool,
}

#[repr(C)]
pub struct VkVideoDecodeH265SessionParametersCreateInfoKHR {
s_type: bool,
p_next: bool,
max_std_vpscount: bool,
max_std_spscount: bool,
max_std_ppscount: bool,
p_parameters_add_info: bool,
}

#[repr(C)]
pub struct VkVideoDecodeH265PictureInfoKHR {
s_type: bool,
p_next: bool,
p_std_picture_info: bool,
slice_segment_count: bool,
p_slice_segment_offsets: bool,
}

#[repr(C)]
pub struct VkVideoDecodeH265DpbSlotInfoKHR {
s_type: bool,
p_next: bool,
p_std_reference_info: bool,
}

#[repr(C)]
pub struct VkVideoSessionCreateInfoKHR {
s_type: bool,
p_next: bool,
queue_family_index: bool,
flags: bool,
p_video_profile: bool,
picture_format: bool,
max_coded_extent: bool,
reference_picture_format: bool,
max_dpb_slots: bool,
max_active_reference_pictures: bool,
p_std_header_version: bool,
}

#[repr(C)]
pub struct VkVideoSessionParametersCreateInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
video_session_parameters_template: bool,
video_session: bool,
}

#[repr(C)]
pub struct VkVideoSessionParametersUpdateInfoKHR {
s_type: bool,
p_next: bool,
update_sequence_count: bool,
}

#[repr(C)]
pub struct VkVideoEncodeSessionParametersGetInfoKHR {
s_type: bool,
p_next: bool,
video_session_parameters: bool,
}

#[repr(C)]
pub struct VkVideoEncodeSessionParametersFeedbackInfoKHR {
s_type: bool,
p_next: bool,
has_overrides: bool,
}

#[repr(C)]
pub struct VkVideoBeginCodingInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
video_session: bool,
video_session_parameters: bool,
reference_slot_count: bool,
p_reference_slots: bool,
}

#[repr(C)]
pub struct VkVideoEndCodingInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
}

#[repr(C)]
pub struct VkVideoCodingControlInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
}

#[repr(C)]
pub struct VkVideoEncodeUsageInfoKHR {
s_type: bool,
p_next: bool,
video_usage_hints: bool,
video_content_hints: bool,
tuning_mode: bool,
}

#[repr(C)]
pub struct VkVideoEncodeInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
dst_buffer: bool,
dst_buffer_offset: bool,
dst_buffer_range: bool,
src_picture_resource: bool,
p_setup_reference_slot: bool,
reference_slot_count: bool,
p_reference_slots: bool,
preceding_externally_encoded_bytes: bool,
}

#[repr(C)]
pub struct VkQueryPoolVideoEncodeFeedbackCreateInfoKHR {
s_type: bool,
p_next: bool,
encode_feedback_flags: bool,
}

#[repr(C)]
pub struct VkVideoEncodeQualityLevelInfoKHR {
s_type: bool,
p_next: bool,
quality_level: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR {
s_type: bool,
p_next: bool,
p_video_profile: bool,
quality_level: bool,
}

#[repr(C)]
pub struct VkVideoEncodeQualityLevelPropertiesKHR {
s_type: bool,
p_next: bool,
preferred_rate_control_mode: bool,
preferred_rate_control_layer_count: bool,
}

#[repr(C)]
pub struct VkVideoEncodeRateControlInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
rate_control_mode: bool,
layer_count: bool,
p_layers: bool,
virtual_buffer_size_in_ms: bool,
initial_virtual_buffer_size_in_ms: bool,
}

#[repr(C)]
pub struct VkVideoEncodeRateControlLayerInfoKHR {
s_type: bool,
p_next: bool,
average_bitrate: bool,
max_bitrate: bool,
frame_rate_numerator: bool,
frame_rate_denominator: bool,
}

#[repr(C)]
pub struct VkVideoEncodeCapabilitiesKHR {
s_type: bool,
p_next: bool,
flags: bool,
rate_control_modes: bool,
max_rate_control_layers: bool,
max_bitrate: bool,
max_quality_levels: bool,
encode_input_picture_granularity: bool,
supported_encode_feedback_flags: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264CapabilitiesEXT {
s_type: bool,
p_next: bool,
flags: bool,
max_level_idc: bool,
max_slice_count: bool,
max_ppicture_l0_reference_count: bool,
max_bpicture_l0_reference_count: bool,
max_l1_reference_count: bool,
max_temporal_layer_count: bool,
expect_dyadic_temporal_layer_pattern: bool,
min_qp: bool,
max_qp: bool,
prefers_gop_remaining_frames: bool,
requires_gop_remaining_frames: bool,
std_syntax_flags: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264QualityLevelPropertiesEXT {
s_type: bool,
p_next: bool,
preferred_rate_control_flags: bool,
preferred_gop_frame_count: bool,
preferred_idr_period: bool,
preferred_consecutive_bframe_count: bool,
preferred_temporal_layer_count: bool,
preferred_constant_qp: bool,
preferred_max_l0_reference_count: bool,
preferred_max_l1_reference_count: bool,
preferred_std_entropy_coding_mode_flag: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionCreateInfoEXT {
s_type: bool,
p_next: bool,
use_max_level_idc: bool,
max_level_idc: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersAddInfoEXT {
s_type: bool,
p_next: bool,
std_spscount: bool,
p_std_spss: bool,
std_ppscount: bool,
p_std_ppss: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersCreateInfoEXT {
s_type: bool,
p_next: bool,
max_std_spscount: bool,
max_std_ppscount: bool,
p_parameters_add_info: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersGetInfoEXT {
s_type: bool,
p_next: bool,
write_std_sps: bool,
write_std_pps: bool,
std_spsid: bool,
std_ppsid: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersFeedbackInfoEXT {
s_type: bool,
p_next: bool,
has_std_spsoverrides: bool,
has_std_ppsoverrides: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264DpbSlotInfoEXT {
s_type: bool,
p_next: bool,
p_std_reference_info: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264PictureInfoEXT {
s_type: bool,
p_next: bool,
nalu_slice_entry_count: bool,
p_nalu_slice_entries: bool,
p_std_picture_info: bool,
generate_prefix_nalu: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264ProfileInfoEXT {
s_type: bool,
p_next: bool,
std_profile_idc: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264NaluSliceInfoEXT {
s_type: bool,
p_next: bool,
ant_qp: bool,
p_std_slice_header: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264RateControlInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
gop_frame_count: bool,
idr_period: bool,
consecutive_bframe_count: bool,
temporal_layer_count: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264QpEXT {
qp_i: bool,
qp_p: bool,
qp_b: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264FrameSizeEXT {
frame_isize: bool,
frame_psize: bool,
frame_bsize: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264GopRemainingFrameInfoEXT {
s_type: bool,
p_next: bool,
use_gop_remaining_frames: bool,
gop_remaining_i: bool,
gop_remaining_p: bool,
gop_remaining_b: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH264RateControlLayerInfoEXT {
s_type: bool,
p_next: bool,
use_min_qp: bool,
min_qp: bool,
use_max_qp: bool,
max_qp: bool,
use_max_frame_size: bool,
max_frame_size: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265CapabilitiesEXT {
s_type: bool,
p_next: bool,
flags: bool,
max_level_idc: bool,
max_slice_segment_count: bool,
max_tiles: bool,
ctb_sizes: bool,
transform_block_sizes: bool,
max_ppicture_l0_reference_count: bool,
max_bpicture_l0_reference_count: bool,
max_l1_reference_count: bool,
max_sub_layer_count: bool,
expect_dyadic_temporal_sub_layer_pattern: bool,
min_qp: bool,
max_qp: bool,
prefers_gop_remaining_frames: bool,
requires_gop_remaining_frames: bool,
std_syntax_flags: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265QualityLevelPropertiesEXT {
s_type: bool,
p_next: bool,
preferred_rate_control_flags: bool,
preferred_gop_frame_count: bool,
preferred_idr_period: bool,
preferred_consecutive_bframe_count: bool,
preferred_sub_layer_count: bool,
preferred_constant_qp: bool,
preferred_max_l0_reference_count: bool,
preferred_max_l1_reference_count: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionCreateInfoEXT {
s_type: bool,
p_next: bool,
use_max_level_idc: bool,
max_level_idc: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersAddInfoEXT {
s_type: bool,
p_next: bool,
std_vpscount: bool,
p_std_vpss: bool,
std_spscount: bool,
p_std_spss: bool,
std_ppscount: bool,
p_std_ppss: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersCreateInfoEXT {
s_type: bool,
p_next: bool,
max_std_vpscount: bool,
max_std_spscount: bool,
max_std_ppscount: bool,
p_parameters_add_info: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersGetInfoEXT {
s_type: bool,
p_next: bool,
write_std_vps: bool,
write_std_sps: bool,
write_std_pps: bool,
std_vpsid: bool,
std_spsid: bool,
std_ppsid: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersFeedbackInfoEXT {
s_type: bool,
p_next: bool,
has_std_vpsoverrides: bool,
has_std_spsoverrides: bool,
has_std_ppsoverrides: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265PictureInfoEXT {
s_type: bool,
p_next: bool,
nalu_slice_segment_entry_count: bool,
p_nalu_slice_segment_entries: bool,
p_std_picture_info: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265NaluSliceSegmentInfoEXT {
s_type: bool,
p_next: bool,
ant_qp: bool,
p_std_slice_segment_header: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265RateControlInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
gop_frame_count: bool,
idr_period: bool,
consecutive_bframe_count: bool,
sub_layer_count: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265QpEXT {
qp_i: bool,
qp_p: bool,
qp_b: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265FrameSizeEXT {
frame_isize: bool,
frame_psize: bool,
frame_bsize: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265GopRemainingFrameInfoEXT {
s_type: bool,
p_next: bool,
use_gop_remaining_frames: bool,
gop_remaining_i: bool,
gop_remaining_p: bool,
gop_remaining_b: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265RateControlLayerInfoEXT {
s_type: bool,
p_next: bool,
use_min_qp: bool,
min_qp: bool,
use_max_qp: bool,
max_qp: bool,
use_max_frame_size: bool,
max_frame_size: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265ProfileInfoEXT {
s_type: bool,
p_next: bool,
std_profile_idc: bool,
}

#[repr(C)]
pub struct VkVideoEncodeH265DpbSlotInfoEXT {
s_type: bool,
p_next: bool,
p_std_reference_info: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceInheritedViewportScissorFeaturesNV {
s_type: bool,
p_next: bool,
inherited_viewport_scissor2_d: bool,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceViewportScissorInfoNV {
s_type: bool,
p_next: bool,
viewport_scissor2_d: bool,
viewport_depth_count: bool,
p_viewport_depths: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
s_type: bool,
p_next: bool,
ycbcr2plane444_formats: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceProvokingVertexFeaturesEXT {
s_type: bool,
p_next: bool,
provoking_vertex_last: bool,
transform_feedback_preserves_provoking_vertex: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceProvokingVertexPropertiesEXT {
s_type: bool,
p_next: bool,
provoking_vertex_mode_per_pipeline: bool,
transform_feedback_preserves_triangle_fan_provoking_vertex: bool,
}

#[repr(C)]
pub struct VkPipelineRasterizationProvokingVertexStateCreateInfoEXT {
s_type: bool,
p_next: bool,
provoking_vertex_mode: bool,
}

#[repr(C)]
pub struct VkCuModuleCreateInfoNVX {
s_type: bool,
p_next: bool,
data_size: bool,
p_data: bool,
}

#[repr(C)]
pub struct VkCuFunctionCreateInfoNVX {
s_type: bool,
p_next: bool,
module: bool,
p_name: bool,
}

#[repr(C)]
pub struct VkCuLaunchInfoNVX {
s_type: bool,
p_next: bool,
function: bool,
grid_dim_x: bool,
grid_dim_y: bool,
grid_dim_z: bool,
block_dim_x: bool,
block_dim_y: bool,
block_dim_z: bool,
shared_mem_bytes: bool,
param_count: bool,
p_params: bool,
extra_count: bool,
p_extras: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferFeaturesEXT {
s_type: bool,
p_next: bool,
descriptor_buffer: bool,
descriptor_buffer_capture_replay: bool,
descriptor_buffer_image_layout_ignored: bool,
descriptor_buffer_push_descriptors: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferPropertiesEXT {
s_type: bool,
p_next: bool,
combined_image_sampler_descriptor_single_array: bool,
bufferless_push_descriptors: bool,
allow_sampler_image_view_post_submit_creation: bool,
descriptor_buffer_offset_alignment: bool,
max_descriptor_buffer_bindings: bool,
max_resource_descriptor_buffer_bindings: bool,
max_sampler_descriptor_buffer_bindings: bool,
max_embedded_immutable_sampler_bindings: bool,
max_embedded_immutable_samplers: bool,
buffer_capture_replay_descriptor_data_size: bool,
image_capture_replay_descriptor_data_size: bool,
image_view_capture_replay_descriptor_data_size: bool,
sampler_capture_replay_descriptor_data_size: bool,
acceleration_structure_capture_replay_descriptor_data_size: bool,
sampler_descriptor_size: bool,
combined_image_sampler_descriptor_size: bool,
sampled_image_descriptor_size: bool,
storage_image_descriptor_size: bool,
uniform_texel_buffer_descriptor_size: bool,
robust_uniform_texel_buffer_descriptor_size: bool,
storage_texel_buffer_descriptor_size: bool,
robust_storage_texel_buffer_descriptor_size: bool,
uniform_buffer_descriptor_size: bool,
robust_uniform_buffer_descriptor_size: bool,
storage_buffer_descriptor_size: bool,
robust_storage_buffer_descriptor_size: bool,
input_attachment_descriptor_size: bool,
acceleration_structure_descriptor_size: bool,
max_sampler_descriptor_buffer_range: bool,
max_resource_descriptor_buffer_range: bool,
sampler_descriptor_buffer_address_space_size: bool,
resource_descriptor_buffer_address_space_size: bool,
descriptor_buffer_address_space_size: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {
s_type: bool,
p_next: bool,
combined_image_sampler_density_map_descriptor_size: bool,
}

#[repr(C)]
pub struct VkDescriptorAddressInfoEXT {
s_type: bool,
p_next: bool,
address: bool,
range: bool,
format: bool,
}

#[repr(C)]
pub struct VkDescriptorBufferBindingInfoEXT {
s_type: bool,
p_next: bool,
address: bool,
usage: bool,
}

#[repr(C)]
pub struct VkDescriptorBufferBindingPushDescriptorBufferHandleEXT {
s_type: bool,
p_next: bool,
buffer: bool,
}

#[repr(C)]
pub struct VkDescriptorGetInfoEXT {
s_type: bool,
p_next: bool,
r#type: bool,
data: bool,
}

#[repr(C)]
pub struct VkBufferCaptureDescriptorDataInfoEXT {
s_type: bool,
p_next: bool,
buffer: bool,
}

#[repr(C)]
pub struct VkImageCaptureDescriptorDataInfoEXT {
s_type: bool,
p_next: bool,
image: bool,
}

#[repr(C)]
pub struct VkImageViewCaptureDescriptorDataInfoEXT {
s_type: bool,
p_next: bool,
image_view: bool,
}

#[repr(C)]
pub struct VkSamplerCaptureDescriptorDataInfoEXT {
s_type: bool,
p_next: bool,
sampler: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureCaptureDescriptorDataInfoEXT {
s_type: bool,
p_next: bool,
acceleration_structure: bool,
acceleration_structure_nv: bool,
}

#[repr(C)]
pub struct VkOpaqueCaptureDescriptorDataCreateInfoEXT {
s_type: bool,
p_next: bool,
opaque_capture_descriptor_data: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerDotProductFeatures {
s_type: bool,
p_next: bool,
shader_integer_dot_product: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerDotProductProperties {
s_type: bool,
p_next: bool,
integer_dot_product8_bit_unsigned_accelerated: bool,
integer_dot_product8_bit_signed_accelerated: bool,
integer_dot_product8_bit_mixed_signedness_accelerated: bool,
integer_dot_product4x8_bit_packed_unsigned_accelerated: bool,
integer_dot_product4x8_bit_packed_signed_accelerated: bool,
integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: bool,
integer_dot_product16_bit_unsigned_accelerated: bool,
integer_dot_product16_bit_signed_accelerated: bool,
integer_dot_product16_bit_mixed_signedness_accelerated: bool,
integer_dot_product32_bit_unsigned_accelerated: bool,
integer_dot_product32_bit_signed_accelerated: bool,
integer_dot_product32_bit_mixed_signedness_accelerated: bool,
integer_dot_product64_bit_unsigned_accelerated: bool,
integer_dot_product64_bit_signed_accelerated: bool,
integer_dot_product64_bit_mixed_signedness_accelerated: bool,
integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: bool,
integer_dot_product_accumulating_saturating8_bit_signed_accelerated: bool,
integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: bool,
integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: bool,
integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: bool,
integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: bool,
integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: bool,
integer_dot_product_accumulating_saturating16_bit_signed_accelerated: bool,
integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: bool,
integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: bool,
integer_dot_product_accumulating_saturating32_bit_signed_accelerated: bool,
integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: bool,
integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: bool,
integer_dot_product_accumulating_saturating64_bit_signed_accelerated: bool,
integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDrmPropertiesEXT {
s_type: bool,
p_next: bool,
has_primary: bool,
has_render: bool,
primary_major: bool,
primary_minor: bool,
render_major: bool,
render_minor: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
s_type: bool,
p_next: bool,
fragment_shader_barycentric: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
s_type: bool,
p_next: bool,
tri_strip_vertex_order_independent_of_provoking_vertex: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingMotionBlurFeaturesNV {
s_type: bool,
p_next: bool,
ray_tracing_motion_blur: bool,
ray_tracing_motion_blur_pipeline_trace_rays_indirect: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryMotionTrianglesDataNV {
s_type: bool,
p_next: bool,
vertex_data: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureMotionInfoNV {
s_type: bool,
p_next: bool,
max_instances: bool,
flags: bool,
}

#[repr(C)]
pub struct VkSRTDataNV {
sx: bool,
a: bool,
b: bool,
pvx: bool,
sy: bool,
c: bool,
pvy: bool,
sz: bool,
pvz: bool,
qx: bool,
qy: bool,
qz: bool,
qw: bool,
tx: bool,
ty: bool,
tz: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureSRTMotionInstanceNV {
transform_t0: bool,
transform_t1: bool,
instance_custom_index: bool,
mask: bool,
instance_shader_binding_table_record_offset: bool,
flags: bool,
acceleration_structure_reference: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureMatrixMotionInstanceNV {
transform_t0: bool,
transform_t1: bool,
instance_custom_index: bool,
mask: bool,
instance_shader_binding_table_record_offset: bool,
flags: bool,
acceleration_structure_reference: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureMotionInstanceNV {
r#type: bool,
flags: bool,
data: bool,
}

#[repr(C)]
pub struct VkMemoryGetRemoteAddressInfoNV {
s_type: bool,
p_next: bool,
memory: bool,
handle_type: bool,
}

#[repr(C)]
pub struct VkImportMemoryBufferCollectionFUCHSIA {
s_type: bool,
p_next: bool,
collection: bool,
index: bool,
}

#[repr(C)]
pub struct VkBufferCollectionImageCreateInfoFUCHSIA {
s_type: bool,
p_next: bool,
collection: bool,
index: bool,
}

#[repr(C)]
pub struct VkBufferCollectionBufferCreateInfoFUCHSIA {
s_type: bool,
p_next: bool,
collection: bool,
index: bool,
}

#[repr(C)]
pub struct VkBufferCollectionCreateInfoFUCHSIA {
s_type: bool,
p_next: bool,
collection_token: bool,
}

#[repr(C)]
pub struct VkBufferCollectionPropertiesFUCHSIA {
s_type: bool,
p_next: bool,
memory_type_bits: bool,
buffer_count: bool,
create_info_index: bool,
sysmem_pixel_format: bool,
format_features: bool,
sysmem_color_space_index: bool,
sampler_ycbcr_conversion_components: bool,
suggested_ycbcr_model: bool,
suggested_ycbcr_range: bool,
suggested_xchroma_offset: bool,
suggested_ychroma_offset: bool,
}

#[repr(C)]
pub struct VkBufferConstraintsInfoFUCHSIA {
s_type: bool,
p_next: bool,
create_info: bool,
required_format_features: bool,
buffer_collection_constraints: bool,
}

#[repr(C)]
pub struct VkSysmemColorSpaceFUCHSIA {
s_type: bool,
p_next: bool,
color_space: bool,
}

#[repr(C)]
pub struct VkImageFormatConstraintsInfoFUCHSIA {
s_type: bool,
p_next: bool,
image_create_info: bool,
required_format_features: bool,
flags: bool,
sysmem_pixel_format: bool,
color_space_count: bool,
p_color_spaces: bool,
}

#[repr(C)]
pub struct VkImageConstraintsInfoFUCHSIA {
s_type: bool,
p_next: bool,
format_constraints_count: bool,
p_format_constraints: bool,
buffer_collection_constraints: bool,
flags: bool,
}

#[repr(C)]
pub struct VkBufferCollectionConstraintsInfoFUCHSIA {
s_type: bool,
p_next: bool,
min_buffer_count: bool,
max_buffer_count: bool,
min_buffer_count_for_camping: bool,
min_buffer_count_for_dedicated_slack: bool,
min_buffer_count_for_shared_slack: bool,
}

#[repr(C)]
pub struct VkCudaModuleCreateInfoNV {
s_type: bool,
p_next: bool,
data_size: bool,
p_data: bool,
}

#[repr(C)]
pub struct VkCudaFunctionCreateInfoNV {
s_type: bool,
p_next: bool,
module: bool,
p_name: bool,
}

#[repr(C)]
pub struct VkCudaLaunchInfoNV {
s_type: bool,
p_next: bool,
function: bool,
grid_dim_x: bool,
grid_dim_y: bool,
grid_dim_z: bool,
block_dim_x: bool,
block_dim_y: bool,
block_dim_z: bool,
shared_mem_bytes: bool,
param_count: bool,
p_params: bool,
extra_count: bool,
p_extras: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT {
s_type: bool,
p_next: bool,
format_rgba10x6_without_ycb_cr_sampler: bool,
}

#[repr(C)]
pub struct VkFormatProperties3 {
s_type: bool,
p_next: bool,
linear_tiling_features: bool,
optimal_tiling_features: bool,
buffer_features: bool,
}

#[repr(C)]
pub struct VkDrmFormatModifierPropertiesList2EXT {
s_type: bool,
p_next: bool,
drm_format_modifier_count: bool,
p_drm_format_modifier_properties: bool,
}

#[repr(C)]
pub struct VkDrmFormatModifierProperties2EXT {
drm_format_modifier: bool,
drm_format_modifier_plane_count: bool,
drm_format_modifier_tiling_features: bool,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferFormatProperties2ANDROID {
s_type: bool,
p_next: bool,
format: bool,
external_format: bool,
format_features: bool,
sampler_ycbcr_conversion_components: bool,
suggested_ycbcr_model: bool,
suggested_ycbcr_range: bool,
suggested_xchroma_offset: bool,
suggested_ychroma_offset: bool,
}

#[repr(C)]
pub struct VkPipelineRenderingCreateInfo {
s_type: bool,
p_next: bool,
view_mask: bool,
color_attachment_count: bool,
p_color_attachment_formats: bool,
depth_attachment_format: bool,
stencil_attachment_format: bool,
}

#[repr(C)]
pub struct VkRenderingInfo {
s_type: bool,
p_next: bool,
flags: bool,
render_area: bool,
layer_count: bool,
view_mask: bool,
color_attachment_count: bool,
p_color_attachments: bool,
p_depth_attachment: bool,
p_stencil_attachment: bool,
}

#[repr(C)]
pub struct VkRenderingAttachmentInfo {
s_type: bool,
p_next: bool,
image_view: bool,
image_layout: bool,
resolve_mode: bool,
resolve_image_view: bool,
resolve_image_layout: bool,
load_op: bool,
store_op: bool,
clear_value: bool,
}

#[repr(C)]
pub struct VkRenderingFragmentShadingRateAttachmentInfoKHR {
s_type: bool,
p_next: bool,
image_view: bool,
image_layout: bool,
shading_rate_attachment_texel_size: bool,
}

#[repr(C)]
pub struct VkRenderingFragmentDensityMapAttachmentInfoEXT {
s_type: bool,
p_next: bool,
image_view: bool,
image_layout: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDynamicRenderingFeatures {
s_type: bool,
p_next: bool,
dynamic_rendering: bool,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceRenderingInfo {
s_type: bool,
p_next: bool,
flags: bool,
view_mask: bool,
#[cfg(vulkan)]
color_attachment_count: bool,
#[cfg(vulkansc)]
color_attachment_count: bool,
p_color_attachment_formats: bool,
depth_attachment_format: bool,
stencil_attachment_format: bool,
rasterization_samples: bool,
}

#[repr(C)]
pub struct VkAttachmentSampleCountInfoAMD {
s_type: bool,
p_next: bool,
color_attachment_count: bool,
p_color_attachment_samples: bool,
depth_stencil_attachment_samples: bool,
}

#[repr(C)]
pub struct VkMultiviewPerViewAttributesInfoNVX {
s_type: bool,
p_next: bool,
per_view_attributes: bool,
per_view_attributes_position_xonly: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageViewMinLodFeaturesEXT {
s_type: bool,
p_next: bool,
min_lod: bool,
}

#[repr(C)]
pub struct VkImageViewMinLodCreateInfoEXT {
s_type: bool,
p_next: bool,
min_lod: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
s_type: bool,
p_next: bool,
rasterization_order_color_attachment_access: bool,
rasterization_order_depth_attachment_access: bool,
rasterization_order_stencil_attachment_access: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceLinearColorAttachmentFeaturesNV {
s_type: bool,
p_next: bool,
linear_color_attachment: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
s_type: bool,
p_next: bool,
graphics_pipeline_library: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
s_type: bool,
p_next: bool,
graphics_pipeline_library_fast_linking: bool,
graphics_pipeline_library_independent_interpolation_decoration: bool,
}

#[repr(C)]
pub struct VkGraphicsPipelineLibraryCreateInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
s_type: bool,
p_next: bool,
descriptor_set_host_mapping: bool,
}

#[repr(C)]
pub struct VkDescriptorSetBindingReferenceVALVE {
s_type: bool,
p_next: bool,
descriptor_set_layout: bool,
binding: bool,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutHostMappingInfoVALVE {
s_type: bool,
p_next: bool,
descriptor_offset: bool,
descriptor_size: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceNestedCommandBufferFeaturesEXT {
s_type: bool,
p_next: bool,
nested_command_buffer: bool,
nested_command_buffer_rendering: bool,
nested_command_buffer_simultaneous_use: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceNestedCommandBufferPropertiesEXT {
s_type: bool,
p_next: bool,
max_command_buffer_nesting_level: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT {
s_type: bool,
p_next: bool,
shader_module_identifier: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT {
s_type: bool,
p_next: bool,
shader_module_identifier_algorithm_uuid: bool,
}

#[repr(C)]
pub struct VkPipelineShaderStageModuleIdentifierCreateInfoEXT {
s_type: bool,
p_next: bool,
identifier_size: bool,
p_identifier: bool,
}

#[repr(C)]
pub struct VkShaderModuleIdentifierEXT {
s_type: bool,
p_next: bool,
identifier_size: bool,
identifier: bool,
}

#[repr(C)]
pub struct VkImageCompressionControlEXT {
s_type: bool,
p_next: bool,
flags: bool,
compression_control_plane_count: bool,
p_fixed_rate_flags: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageCompressionControlFeaturesEXT {
s_type: bool,
p_next: bool,
image_compression_control: bool,
}

#[repr(C)]
pub struct VkImageCompressionPropertiesEXT {
s_type: bool,
p_next: bool,
image_compression_flags: bool,
image_compression_fixed_rate_flags: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
s_type: bool,
p_next: bool,
image_compression_control_swapchain: bool,
}

#[repr(C)]
pub struct VkImageSubresource2KHR {
s_type: bool,
p_next: bool,
image_subresource: bool,
}

#[repr(C)]
pub struct VkSubresourceLayout2KHR {
s_type: bool,
p_next: bool,
subresource_layout: bool,
}

#[repr(C)]
pub struct VkRenderPassCreationControlEXT {
s_type: bool,
p_next: bool,
disallow_merging: bool,
}

#[repr(C)]
pub struct VkRenderPassCreationFeedbackInfoEXT {
post_merge_subpass_count: bool,
}

#[repr(C)]
pub struct VkRenderPassCreationFeedbackCreateInfoEXT {
s_type: bool,
p_next: bool,
p_render_pass_feedback: bool,
}

#[repr(C)]
pub struct VkRenderPassSubpassFeedbackInfoEXT {
subpass_merge_status: bool,
description: bool,
post_merge_index: bool,
}

#[repr(C)]
pub struct VkRenderPassSubpassFeedbackCreateInfoEXT {
s_type: bool,
p_next: bool,
p_subpass_feedback: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
s_type: bool,
p_next: bool,
subpass_merge_feedback: bool,
}

#[repr(C)]
pub struct VkMicromapBuildInfoEXT {
s_type: bool,
p_next: bool,
r#type: bool,
flags: bool,
mode: bool,
dst_micromap: bool,
usage_counts_count: bool,
p_usage_counts: bool,
pp_usage_counts: bool,
data: bool,
scratch_data: bool,
triangle_array: bool,
triangle_array_stride: bool,
}

#[repr(C)]
pub struct VkMicromapCreateInfoEXT {
s_type: bool,
p_next: bool,
create_flags: bool,
buffer: bool,
offset: bool,
size: bool,
r#type: bool,
device_address: bool,
}

#[repr(C)]
pub struct VkMicromapVersionInfoEXT {
s_type: bool,
p_next: bool,
p_version_data: bool,
}

#[repr(C)]
pub struct VkCopyMicromapInfoEXT {
s_type: bool,
p_next: bool,
src: bool,
dst: bool,
mode: bool,
}

#[repr(C)]
pub struct VkCopyMicromapToMemoryInfoEXT {
s_type: bool,
p_next: bool,
src: bool,
dst: bool,
mode: bool,
}

#[repr(C)]
pub struct VkCopyMemoryToMicromapInfoEXT {
s_type: bool,
p_next: bool,
src: bool,
dst: bool,
mode: bool,
}

#[repr(C)]
pub struct VkMicromapBuildSizesInfoEXT {
s_type: bool,
p_next: bool,
micromap_size: bool,
build_scratch_size: bool,
discardable: bool,
}

#[repr(C)]
pub struct VkMicromapUsageEXT {
count: bool,
subdivision_level: bool,
format: bool,
}

#[repr(C)]
pub struct VkMicromapTriangleEXT {
data_offset: bool,
subdivision_level: bool,
format: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceOpacityMicromapFeaturesEXT {
s_type: bool,
p_next: bool,
micromap: bool,
micromap_capture_replay: bool,
micromap_host_commands: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceOpacityMicromapPropertiesEXT {
s_type: bool,
p_next: bool,
max_opacity2_state_subdivision_level: bool,
max_opacity4_state_subdivision_level: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureTrianglesOpacityMicromapEXT {
s_type: bool,
p_next: bool,
index_type: bool,
index_buffer: bool,
index_stride: bool,
base_triangle: bool,
usage_counts_count: bool,
p_usage_counts: bool,
pp_usage_counts: bool,
micromap: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDisplacementMicromapFeaturesNV {
s_type: bool,
p_next: bool,
displacement_micromap: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDisplacementMicromapPropertiesNV {
s_type: bool,
p_next: bool,
max_displacement_micromap_subdivision_level: bool,
}

#[repr(C)]
pub struct VkAccelerationStructureTrianglesDisplacementMicromapNV {
s_type: bool,
p_next: bool,
displacement_bias_and_scale_format: bool,
displacement_vector_format: bool,
displacement_bias_and_scale_buffer: bool,
displacement_bias_and_scale_stride: bool,
displacement_vector_buffer: bool,
displacement_vector_stride: bool,
displaced_micromap_primitive_flags: bool,
displaced_micromap_primitive_flags_stride: bool,
index_type: bool,
index_buffer: bool,
index_stride: bool,
base_triangle: bool,
usage_counts_count: bool,
p_usage_counts: bool,
pp_usage_counts: bool,
micromap: bool,
}

#[repr(C)]
pub struct VkPipelinePropertiesIdentifierEXT {
s_type: bool,
p_next: bool,
pipeline_identifier: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelinePropertiesFeaturesEXT {
s_type: bool,
p_next: bool,
pipeline_properties_identifier: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {
s_type: bool,
p_next: bool,
shader_early_and_late_fragment_tests: bool,
}

#[repr(C)]
pub struct VkExternalMemoryAcquireUnmodifiedEXT {
s_type: bool,
p_next: bool,
acquire_unmodified_memory: bool,
}

#[repr(C)]
pub struct VkExportMetalObjectCreateInfoEXT {
s_type: bool,
p_next: bool,
export_object_type: bool,
}

#[repr(C)]
pub struct VkExportMetalObjectsInfoEXT {
s_type: bool,
p_next: bool,
}

#[repr(C)]
pub struct VkExportMetalDeviceInfoEXT {
s_type: bool,
p_next: bool,
mtl_device: bool,
}

#[repr(C)]
pub struct VkExportMetalCommandQueueInfoEXT {
s_type: bool,
p_next: bool,
queue: bool,
mtl_command_queue: bool,
}

#[repr(C)]
pub struct VkExportMetalBufferInfoEXT {
s_type: bool,
p_next: bool,
memory: bool,
mtl_buffer: bool,
}

#[repr(C)]
pub struct VkImportMetalBufferInfoEXT {
s_type: bool,
p_next: bool,
mtl_buffer: bool,
}

#[repr(C)]
pub struct VkExportMetalTextureInfoEXT {
s_type: bool,
p_next: bool,
image: bool,
image_view: bool,
buffer_view: bool,
plane: bool,
mtl_texture: bool,
}

#[repr(C)]
pub struct VkImportMetalTextureInfoEXT {
s_type: bool,
p_next: bool,
plane: bool,
mtl_texture: bool,
}

#[repr(C)]
pub struct VkExportMetalIOSurfaceInfoEXT {
s_type: bool,
p_next: bool,
image: bool,
io_surface: bool,
}

#[repr(C)]
pub struct VkImportMetalIOSurfaceInfoEXT {
s_type: bool,
p_next: bool,
io_surface: bool,
}

#[repr(C)]
pub struct VkExportMetalSharedEventInfoEXT {
s_type: bool,
p_next: bool,
semaphore: bool,
event: bool,
mtl_shared_event: bool,
}

#[repr(C)]
pub struct VkImportMetalSharedEventInfoEXT {
s_type: bool,
p_next: bool,
mtl_shared_event: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT {
s_type: bool,
p_next: bool,
non_seamless_cube_map: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineRobustnessFeaturesEXT {
s_type: bool,
p_next: bool,
pipeline_robustness: bool,
}

#[repr(C)]
pub struct VkPipelineRobustnessCreateInfoEXT {
s_type: bool,
p_next: bool,
storage_buffers: bool,
uniform_buffers: bool,
vertex_inputs: bool,
images: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineRobustnessPropertiesEXT {
s_type: bool,
p_next: bool,
default_robustness_storage_buffers: bool,
default_robustness_uniform_buffers: bool,
default_robustness_vertex_inputs: bool,
default_robustness_images: bool,
}

#[repr(C)]
pub struct VkImageViewSampleWeightCreateInfoQCOM {
s_type: bool,
p_next: bool,
filter_center: bool,
filter_size: bool,
num_phases: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageProcessingFeaturesQCOM {
s_type: bool,
p_next: bool,
texture_sample_weighted: bool,
texture_box_filter: bool,
texture_block_match: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageProcessingPropertiesQCOM {
s_type: bool,
p_next: bool,
max_weight_filter_phases: bool,
max_weight_filter_dimension: bool,
max_block_match_region: bool,
max_box_filter_block_size: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceTilePropertiesFeaturesQCOM {
s_type: bool,
p_next: bool,
tile_properties: bool,
}

#[repr(C)]
pub struct VkTilePropertiesQCOM {
s_type: bool,
p_next: bool,
tile_size: bool,
apron_size: bool,
origin: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceAmigoProfilingFeaturesSEC {
s_type: bool,
p_next: bool,
amigo_profiling: bool,
}

#[repr(C)]
pub struct VkAmigoProfilingSubmitInfoSEC {
s_type: bool,
p_next: bool,
first_draw_timestamp: bool,
swap_buffer_timestamp: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {
s_type: bool,
p_next: bool,
attachment_feedback_loop_layout: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthClampZeroOneFeaturesEXT {
s_type: bool,
p_next: bool,
depth_clamp_zero_one: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceAddressBindingReportFeaturesEXT {
s_type: bool,
p_next: bool,
report_address_binding: bool,
}

#[repr(C)]
pub struct VkDeviceAddressBindingCallbackDataEXT {
s_type: bool,
p_next: bool,
flags: bool,
base_address: bool,
size: bool,
binding_type: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceOpticalFlowFeaturesNV {
s_type: bool,
p_next: bool,
optical_flow: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceOpticalFlowPropertiesNV {
s_type: bool,
p_next: bool,
supported_output_grid_sizes: bool,
supported_hint_grid_sizes: bool,
hint_supported: bool,
cost_supported: bool,
bidirectional_flow_supported: bool,
global_flow_supported: bool,
min_width: bool,
min_height: bool,
max_width: bool,
max_height: bool,
max_num_regions_of_interest: bool,
}

#[repr(C)]
pub struct VkOpticalFlowImageFormatInfoNV {
s_type: bool,
p_next: bool,
usage: bool,
}

#[repr(C)]
pub struct VkOpticalFlowImageFormatPropertiesNV {
s_type: bool,
p_next: bool,
format: bool,
}

#[repr(C)]
pub struct VkOpticalFlowSessionCreateInfoNV {
s_type: bool,
p_next: bool,
width: bool,
height: bool,
image_format: bool,
flow_vector_format: bool,
cost_format: bool,
output_grid_size: bool,
hint_grid_size: bool,
performance_level: bool,
flags: bool,
}

#[repr(C)]
pub struct VkOpticalFlowSessionCreatePrivateDataInfoNV {
s_type: bool,
p_next: bool,
id: bool,
size: bool,
p_private_data: bool,
}

#[repr(C)]
pub struct VkOpticalFlowExecuteInfoNV {
s_type: bool,
p_next: bool,
flags: bool,
region_count: bool,
p_regions: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFaultFeaturesEXT {
s_type: bool,
p_next: bool,
device_fault: bool,
device_fault_vendor_binary: bool,
}

#[repr(C)]
pub struct VkDeviceFaultAddressInfoEXT {
address_type: bool,
reported_address: bool,
address_precision: bool,
}

#[repr(C)]
pub struct VkDeviceFaultVendorInfoEXT {
description: bool,
vendor_fault_code: bool,
vendor_fault_data: bool,
}

#[repr(C)]
pub struct VkDeviceFaultCountsEXT {
s_type: bool,
p_next: bool,
address_info_count: bool,
vendor_info_count: bool,
vendor_binary_size: bool,
}

#[repr(C)]
pub struct VkDeviceFaultInfoEXT {
s_type: bool,
p_next: bool,
description: bool,
p_address_infos: bool,
p_vendor_infos: bool,
p_vendor_binary_data: bool,
}

#[repr(C)]
pub struct VkDeviceFaultVendorBinaryHeaderVersionOneEXT {
header_size: bool,
header_version: bool,
vendor_id: bool,
device_id: bool,
driver_version: bool,
pipeline_cache_uuid: bool,
application_name_offset: bool,
application_version: bool,
engine_name_offset: bool,
engine_version: bool,
api_version: bool,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT {
s_type: bool,
p_next: bool,
pipeline_library_group_handles: bool,
}

#[repr(C)]
pub struct VkDepthBiasInfoEXT {
s_type: bool,
p_next: bool,
depth_bias_constant_factor: bool,
depth_bias_clamp: bool,
depth_bias_slope_factor: bool,
}

#[repr(C)]
pub struct VkDepthBiasRepresentationInfoEXT {
s_type: bool,
p_next: bool,
depth_bias_representation: bool,
depth_bias_exact: bool,
}

#[repr(C)]
pub struct VkDecompressMemoryRegionNV {
src_address: bool,
dst_address: bool,
compressed_size: bool,
decompressed_size: bool,
decompression_method: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM {
s_type: bool,
p_next: bool,
shader_core_mask: bool,
shader_core_count: bool,
shader_warps_per_core: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM {
s_type: bool,
p_next: bool,
shader_core_builtins: bool,
}

#[repr(C)]
pub struct VkFrameBoundaryEXT {
s_type: bool,
p_next: bool,
flags: bool,
frame_id: bool,
image_count: bool,
p_images: bool,
buffer_count: bool,
p_buffers: bool,
tag_name: bool,
tag_size: bool,
p_tag: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceFrameBoundaryFeaturesEXT {
s_type: bool,
p_next: bool,
frame_boundary: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT {
s_type: bool,
p_next: bool,
dynamic_rendering_unused_attachments: bool,
}

#[repr(C)]
pub struct VkSurfacePresentModeEXT {
s_type: bool,
p_next: bool,
present_mode: bool,
}

#[repr(C)]
pub struct VkSurfacePresentScalingCapabilitiesEXT {
s_type: bool,
p_next: bool,
supported_present_scaling: bool,
supported_present_gravity_x: bool,
supported_present_gravity_y: bool,
min_scaled_image_extent: bool,
max_scaled_image_extent: bool,
}

#[repr(C)]
pub struct VkSurfacePresentModeCompatibilityEXT {
s_type: bool,
p_next: bool,
present_mode_count: bool,
p_present_modes: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT {
s_type: bool,
p_next: bool,
swapchain_maintenance1: bool,
}

#[repr(C)]
pub struct VkSwapchainPresentFenceInfoEXT {
s_type: bool,
p_next: bool,
swapchain_count: bool,
p_fences: bool,
}

#[repr(C)]
pub struct VkSwapchainPresentModesCreateInfoEXT {
s_type: bool,
p_next: bool,
present_mode_count: bool,
p_present_modes: bool,
}

#[repr(C)]
pub struct VkSwapchainPresentModeInfoEXT {
s_type: bool,
p_next: bool,
swapchain_count: bool,
p_present_modes: bool,
}

#[repr(C)]
pub struct VkSwapchainPresentScalingCreateInfoEXT {
s_type: bool,
p_next: bool,
scaling_behavior: bool,
present_gravity_x: bool,
present_gravity_y: bool,
}

#[repr(C)]
pub struct VkReleaseSwapchainImagesInfoEXT {
s_type: bool,
p_next: bool,
swapchain: bool,
image_index_count: bool,
p_image_indices: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthBiasControlFeaturesEXT {
s_type: bool,
p_next: bool,
depth_bias_control: bool,
least_representable_value_force_unorm_representation: bool,
float_representation: bool,
depth_bias_exact: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV {
s_type: bool,
p_next: bool,
ray_tracing_invocation_reorder: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV {
s_type: bool,
p_next: bool,
ray_tracing_invocation_reorder_reordering_hint: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV {
s_type: bool,
p_next: bool,
extended_sparse_address_space: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV {
s_type: bool,
p_next: bool,
extended_sparse_address_space_size: bool,
extended_sparse_image_usage_flags: bool,
extended_sparse_buffer_usage_flags: bool,
}

#[repr(C)]
pub struct VkDirectDriverLoadingInfoLUNARG {
s_type: bool,
p_next: bool,
flags: bool,
pfn_get_instance_proc_addr: bool,
}

#[repr(C)]
pub struct VkDirectDriverLoadingListLUNARG {
s_type: bool,
p_next: bool,
mode: bool,
driver_count: bool,
p_drivers: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM {
s_type: bool,
p_next: bool,
multiview_per_view_viewports: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR {
s_type: bool,
p_next: bool,
ray_tracing_position_fetch: bool,
}

#[repr(C)]
pub struct VkDeviceImageSubresourceInfoKHR {
s_type: bool,
p_next: bool,
p_create_info: bool,
p_subresource: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderCorePropertiesARM {
s_type: bool,
p_next: bool,
pixel_rate: bool,
texel_rate: bool,
fma_rate: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM {
s_type: bool,
p_next: bool,
multiview_per_view_render_areas: bool,
}

#[repr(C)]
pub struct VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM {
s_type: bool,
p_next: bool,
per_view_render_area_count: bool,
p_per_view_render_areas: bool,
}

#[repr(C)]
pub struct VkQueryLowLatencySupportNV {
s_type: bool,
p_next: bool,
p_queried_low_latency_data: bool,
}

#[repr(C)]
pub struct VkMemoryMapInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
memory: bool,
offset: bool,
size: bool,
}

#[repr(C)]
pub struct VkMemoryUnmapInfoKHR {
s_type: bool,
p_next: bool,
flags: bool,
memory: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderObjectFeaturesEXT {
s_type: bool,
p_next: bool,
shader_object: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderObjectPropertiesEXT {
s_type: bool,
p_next: bool,
shader_binary_uuid: bool,
shader_binary_version: bool,
}

#[repr(C)]
pub struct VkShaderCreateInfoEXT {
s_type: bool,
p_next: bool,
flags: bool,
stage: bool,
next_stage: bool,
code_type: bool,
code_size: bool,
p_code: bool,
p_name: bool,
set_layout_count: bool,
p_set_layouts: bool,
push_constant_range_count: bool,
p_push_constant_ranges: bool,
p_specialization_info: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderTileImageFeaturesEXT {
s_type: bool,
p_next: bool,
shader_tile_image_color_read_access: bool,
shader_tile_image_depth_read_access: bool,
shader_tile_image_stencil_read_access: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderTileImagePropertiesEXT {
s_type: bool,
p_next: bool,
shader_tile_image_coherent_read_accelerated: bool,
shader_tile_image_read_sample_from_pixel_rate_invocation: bool,
shader_tile_image_read_from_helper_invocation: bool,
}

#[repr(C)]
pub struct VkImportScreenBufferInfoQNX {
s_type: bool,
p_next: bool,
buffer: bool,
}

#[repr(C)]
pub struct VkScreenBufferPropertiesQNX {
s_type: bool,
p_next: bool,
allocation_size: bool,
memory_type_bits: bool,
}

#[repr(C)]
pub struct VkScreenBufferFormatPropertiesQNX {
s_type: bool,
p_next: bool,
format: bool,
external_format: bool,
screen_usage: bool,
format_features: bool,
sampler_ycbcr_conversion_components: bool,
suggested_ycbcr_model: bool,
suggested_ycbcr_range: bool,
suggested_xchroma_offset: bool,
suggested_ychroma_offset: bool,
}

#[repr(C)]
pub struct VkExternalFormatQNX {
s_type: bool,
p_next: bool,
external_format: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX {
s_type: bool,
p_next: bool,
screen_buffer_import: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixFeaturesKHR {
s_type: bool,
p_next: bool,
cooperative_matrix: bool,
cooperative_matrix_robust_buffer_access: bool,
}

#[repr(C)]
pub struct VkCooperativeMatrixPropertiesKHR {
s_type: bool,
p_next: bool,
_msize: bool,
_nsize: bool,
_ksize: bool,
_atype: bool,
_btype: bool,
_ctype: bool,
_result_type: bool,
saturating_accumulation: bool,
scope: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixPropertiesKHR {
s_type: bool,
p_next: bool,
cooperative_matrix_supported_stages: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderEnqueuePropertiesAMDX {
s_type: bool,
p_next: bool,
max_execution_graph_depth: bool,
max_execution_graph_shader_output_nodes: bool,
max_execution_graph_shader_payload_size: bool,
max_execution_graph_shader_payload_count: bool,
execution_graph_dispatch_address_alignment: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderEnqueueFeaturesAMDX {
s_type: bool,
p_next: bool,
shader_enqueue: bool,
}

#[repr(C)]
pub struct VkExecutionGraphPipelineCreateInfoAMDX {
s_type: bool,
p_next: bool,
flags: bool,
stage_count: bool,
p_stages: bool,
p_library_info: bool,
layout: bool,
base_pipeline_handle: bool,
base_pipeline_index: bool,
}

#[repr(C)]
pub struct VkPipelineShaderStageNodeCreateInfoAMDX {
s_type: bool,
p_next: bool,
p_name: bool,
index: bool,
}

#[repr(C)]
pub struct VkExecutionGraphPipelineScratchSizeAMDX {
s_type: bool,
p_next: bool,
size: bool,
}

#[repr(C)]
pub struct VkDispatchGraphInfoAMDX {
node_index: bool,
payload_count: bool,
payloads: bool,
payload_stride: bool,
}

#[repr(C)]
pub struct VkDispatchGraphCountInfoAMDX {
count: bool,
infos: bool,
stride: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCubicClampFeaturesQCOM {
s_type: bool,
p_next: bool,
cubic_range_clamp: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceYcbcrDegammaFeaturesQCOM {
s_type: bool,
p_next: bool,
ycbcr_degamma: bool,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM {
s_type: bool,
p_next: bool,
enable_ydegamma: bool,
enable_cb_cr_degamma: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCubicWeightsFeaturesQCOM {
s_type: bool,
p_next: bool,
selectable_cubic_weights: bool,
}

#[repr(C)]
pub struct VkSamplerCubicWeightsCreateInfoQCOM {
s_type: bool,
p_next: bool,
cubic_weights: bool,
}

#[repr(C)]
pub struct VkBlitImageCubicWeightsInfoQCOM {
s_type: bool,
p_next: bool,
cubic_weights: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageProcessing2FeaturesQCOM {
s_type: bool,
p_next: bool,
texture_block_match2: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageProcessing2PropertiesQCOM {
s_type: bool,
p_next: bool,
max_block_match_window: bool,
}

#[repr(C)]
pub struct VkSamplerBlockMatchWindowCreateInfoQCOM {
s_type: bool,
p_next: bool,
window_extent: bool,
window_compare_mode: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV {
s_type: bool,
p_next: bool,
descriptor_pool_overallocation: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceLayeredDriverPropertiesMSFT {
s_type: bool,
p_next: bool,
underlying_api: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalFormatResolveFeaturesANDROID {
s_type: bool,
p_next: bool,
external_format_resolve: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalFormatResolvePropertiesANDROID {
s_type: bool,
p_next: bool,
null_color_attachment_with_external_format_resolve: bool,
external_format_resolve_chroma_offset_x: bool,
external_format_resolve_chroma_offset_y: bool,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferFormatResolvePropertiesANDROID {
s_type: bool,
p_next: bool,
color_attachment_format: bool,
}

#[repr(C)]
pub struct VkLatencySleepModeInfoNV {
s_type: bool,
p_next: bool,
low_latency_mode: bool,
low_latency_boost: bool,
minimum_interval_us: bool,
}

#[repr(C)]
pub struct VkLatencySleepInfoNV {
s_type: bool,
p_next: bool,
signal_semaphore: bool,
value: bool,
}

#[repr(C)]
pub struct VkSetLatencyMarkerInfoNV {
s_type: bool,
p_next: bool,
present_id: bool,
marker: bool,
}

#[repr(C)]
pub struct VkGetLatencyMarkerInfoNV {
s_type: bool,
p_next: bool,
p_timings: bool,
}

#[repr(C)]
pub struct VkLatencyTimingsFrameReportNV {
s_type: bool,
p_next: bool,
present_id: bool,
input_sample_time_us: bool,
sim_start_time_us: bool,
sim_end_time_us: bool,
render_submit_start_time_us: bool,
render_submit_end_time_us: bool,
present_start_time_us: bool,
present_end_time_us: bool,
driver_start_time_us: bool,
driver_end_time_us: bool,
os_render_queue_start_time_us: bool,
os_render_queue_end_time_us: bool,
gpu_render_start_time_us: bool,
gpu_render_end_time_us: bool,
}

#[repr(C)]
pub struct VkOutOfBandQueueTypeInfoNV {
s_type: bool,
p_next: bool,
queue_type: bool,
}

#[repr(C)]
pub struct VkLatencySubmissionPresentIdNV {
s_type: bool,
p_next: bool,
present_id: bool,
}

#[repr(C)]
pub struct VkSwapchainLatencyCreateInfoNV {
s_type: bool,
p_next: bool,
latency_mode_enable: bool,
}

#[repr(C)]
pub struct VkLatencySurfaceCapabilitiesNV {
s_type: bool,
p_next: bool,
present_mode_count: bool,
p_present_modes: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCudaKernelLaunchFeaturesNV {
s_type: bool,
p_next: bool,
cuda_kernel_launch_features: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceCudaKernelLaunchPropertiesNV {
s_type: bool,
p_next: bool,
compute_capability_minor: bool,
compute_capability_major: bool,
}

#[repr(C)]
pub struct VkDeviceQueueShaderCoreControlCreateInfoARM {
s_type: bool,
p_next: bool,
shader_core_count: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSchedulingControlsFeaturesARM {
s_type: bool,
p_next: bool,
scheduling_controls: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceSchedulingControlsPropertiesARM {
s_type: bool,
p_next: bool,
scheduling_controls_flags: bool,
}

#[repr(C)]
pub struct VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG {
s_type: bool,
p_next: bool,
relaxed_line_rasterization: bool,
}

