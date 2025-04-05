//! Starforge Render - Vulkan Core
//!
//! This module stores the core Vulkan context information, like the instance, device, queues, and allocator

use ash::vk;
use starforge_core::{StarforgeError, StarforgeResult};
use std::ffi::{CStr, CString};
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct QueueInfo {
    pub queue: vk::Queue,
    pub family_index: u32,
}

/// Manages Vulkan validation layer setup and teardown
struct ValidationLayers {
    debug_utils: ash::ext::debug_utils::Instance,
    debug_messenger: vk::DebugUtilsMessengerEXT,
}

pub struct Context {
    // Keeps the entry point alive
    _entry: ash::Entry,
    instance: ash::Instance,
    validation: Option<ValidationLayers>, // Holds validation state if enabled

    physical_device: vk::PhysicalDevice,
    physical_device_properties: vk::PhysicalDeviceProperties,

    // The primary handle for interaction with the GPU
    device: ash::Device,
    // Queues obtained from the logical device
    graphics_queue: QueueInfo,
    compute_queue: Option<QueueInfo>,
    transfer_queue: Option<QueueInfo>,

    // The memory allocator instance
    allocator: vk_mem::Allocator,

    enabled_device_extensions: Vec<CString>,
}

impl Context {
    pub fn new(
        application_name: &CStr,
        application_version: u32,
        required_instance_extensions: &[&CStr],
        enable_validation: bool,
    ) -> StarforgeResult<Self> {
        unsafe {
            // Step 1: Load Vulkan Library
            let entry =
                ash::Entry::load().map_err(|e| StarforgeError::RendererError(e.to_string()))?;

            // Step 2: Define app info
            let app_info = vk::ApplicationInfo::default()
                .application_name(application_name)
                .application_version(application_version)
                .engine_name(CStr::from_bytes_with_nul_unchecked(
                    b"StarforgeVulkanRenderer\0",
                ))
                .engine_version(vk::make_api_version(0, 0, 1, 0))
                .api_version(vk::API_VERSION_1_3);

            // Step 3: Validation Layers
            let validation_layer_name =
                CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0");
            let mut required_layers = Vec::new();
            if enable_validation {
                // Check if validation layers are available
                let available_layers = entry
                    .enumerate_instance_layer_properties()
                    .map_err(|e| StarforgeError::RendererError(e.to_string()))?;
                let validation_available = available_layers.iter().any(|layer| {
                    CStr::from_ptr(layer.layer_name.as_ptr()) == validation_layer_name
                });

                if validation_available {
                    info!("Validation layers enabled.");
                    required_layers.push(validation_layer_name.as_ptr());
                } else {
                    warn!("Warning: Validation layers requested but not available.");
                }
            }

            // Step 4: Instance Extensions
            let mut instance_extensions = required_instance_extensions
                .iter()
                .map(|&s| s.as_ptr())
                .collect::<Vec<_>>();
            if enable_validation {
                instance_extensions.push(ash::ext::debug_utils::NAME.as_ptr());
            }
            instance_extensions.push(ash::khr::get_physical_device_properties2::NAME.as_ptr());
            instance_extensions.push(ash::khr::get_surface_capabilities2::NAME.as_ptr());

            // Step 5: Create Vulkan Instance
            let instance_create_info = vk::InstanceCreateInfo::default()
                .application_info(&app_info)
                .enabled_layer_names(&required_layers)
                .enabled_extension_names(&instance_extensions);

            let instance = entry
                .create_instance(&instance_create_info, None)
                .map_err(|e| StarforgeError::RendererError(e.to_string()))?;

            // Step 6: Setup Debug Messenger
            let mut validation = None;
            if enable_validation && required_layers.len() > 0 {
                let debug_utils = ash::ext::debug_utils::Instance::new(&entry, &instance);
                let debug_create_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
                    .message_severity(
                        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                            | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                            | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
                    )
                    .message_type(
                        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                    )
                    .pfn_user_callback(Some(vulkan_debug_callback));

                let debug_messenger = debug_utils
                    .create_debug_utils_messenger(&debug_create_info, None)
                    .map_err(|e| StarforgeError::RendererError(e.to_string()))?;
                validation = Some(ValidationLayers {
                    debug_utils,
                    debug_messenger,
                });

                info!("Vulkan Debug Messenger created");
            }

            // --- Physical Device Selection ---
            // Step 7: Define required device extensions
            let required_device_extensions = [
                ash::khr::swapchain::NAME,
                ash::khr::external_memory_fd::NAME,
                ash::khr::external_semaphore_fd::NAME,
                ash::khr::external_fence_fd::NAME,
                ash::ext::external_memory_dma_buf::NAME,
                // Format Modifiers (Often needed for DMA-BUF)
                ash::ext::image_drm_format_modifier::NAME,
                // HDR Capabilities
                ash::ext::hdr_metadata::NAME,
                // Timeline Semaphores
                ash::khr::timeline_semaphore::NAME,
            ];
            let required_device_extensions_cstr: Vec<CString> = required_device_extensions
                .iter()
                .map(|&s| CString::new(s.to_bytes()).unwrap()) // Handle potential errors
                .collect();

            // Step 8: Select Physical Device
            let physical_devices = instance
                .enumerate_physical_devices()
                .map_err(|e| StarforgeError::RendererError(e.to_string()))?;

            let selection = physical_devices.into_iter().find_map(|pdev| {
                // Check extensions support
                let Ok(available_extensions) = instance.enumerate_device_extension_properties(pdev)
                else {
                    return None;
                };
                let all_extensions_supported = required_device_extensions.iter().all(|&req_ext| {
                    available_extensions.iter().any(|avail_ext| {
                        CStr::from_ptr(avail_ext.extension_name.as_ptr()) == req_ext
                    })
                });
                if !all_extensions_supported {
                    return None;
                }

                // Check queue families
                let queue_families = instance.get_physical_device_queue_family_properties(pdev);
                let graphics_family = queue_families.iter().enumerate().find_map(|(i, props)| {
                    if props.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                        Some(i as u32)
                    } else {
                        None
                    }
                });
                if graphics_family.is_none() {
                    return None;
                }
                let compute_family = queue_families.iter().enumerate().find_map(|(i, props)| {
                    if props.queue_flags.contains(vk::QueueFlags::COMPUTE) {
                        Some(i as u32)
                    } else {
                        None
                    }
                });
                let transfer_family = queue_families.iter().enumerate().find_map(|(i, props)| {
                    if props.queue_flags.contains(vk::QueueFlags::TRANSFER) {
                        Some(i as u32)
                    } else {
                        None
                    }
                });

                // Check features (example: timeline semaphores)
                let mut timeline_semaphore_features =
                    vk::PhysicalDeviceTimelineSemaphoreFeatures::default();
                let mut features2 = vk::PhysicalDeviceFeatures2::default()
                    .push_next(&mut timeline_semaphore_features);
                instance.get_physical_device_features2(pdev, &mut features2);
                if timeline_semaphore_features.timeline_semaphore == vk::FALSE {
                    return None;
                }

                // TODO: Add checks for other required features (external memory, modifiers, etc.)
                // using VkPhysicalDeviceFeatures2 and extension-specific feature structs.

                // TODO: Check surface support for at least one queue family if needed here?
                // Usually checked when creating the swapchain, but ensures compatibility.

                Some((
                    pdev,
                    graphics_family.unwrap(),
                    compute_family,
                    transfer_family,
                )) // Return device and graphics queue index
            });

            let (
                physical_device,
                graphics_queue_family_index,
                compute_queue_family_index_opt,
                transfer_queue_family_index_opt,
            ) = selection.ok_or_else(|| {
                StarforgeError::RendererError("No suitable physical device found".to_string())
            })?;
            let mut pdev_props = vk::PhysicalDeviceProperties2::default();
            instance.get_physical_device_properties2(physical_device, &mut pdev_props);
            info!(
                "Selected physical device: {:?}",
                String::from_utf8_lossy(
                    CStr::from_ptr(pdev_props.properties.device_name.as_ptr()).to_bytes()
                )
            );

            // -- Logical Device Creation --
            // Step 9: Define Queues to Create
            let queue_priorities = [1.0f32];
            let mut queue_create_infos = vec![
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(graphics_queue_family_index)
                    .queue_priorities(&queue_priorities),
                // Add more queue create infos if using separate compute/transfer queues
            ];
            if let Some(compute_queue_family_index) = compute_queue_family_index_opt {
                queue_create_infos.push(
                    vk::DeviceQueueCreateInfo::default()
                        .queue_family_index(compute_queue_family_index)
                        .queue_priorities(&queue_priorities),
                );
            }
            if let Some(transfer_queue_family_index) = transfer_queue_family_index_opt {
                queue_create_infos.push(
                    vk::DeviceQueueCreateInfo::default()
                        .queue_family_index(transfer_queue_family_index)
                        .queue_priorities(&queue_priorities),
                );
            }

            // Step 10: Define Features to Enable (fetch required features properly)
            let physical_device_features = vk::PhysicalDeviceFeatures::default();
            // Enable features needed, e.g., samplerAnisotropy
            // Use VkPhysicalDeviceFeatures2 for extension features
            let mut timeline_semaphore_features =
                vk::PhysicalDeviceTimelineSemaphoreFeatures::default().timeline_semaphore(true); // Enable timeline semaphores
            // Chain other feature structs here (e.g., for modifiers, external memory)
            let mut features2 = vk::PhysicalDeviceFeatures2::default()
                .features(physical_device_features) // Base features
                .push_next(&mut timeline_semaphore_features);

            // Step 11: Create Logical Device
            let enabled_device_extensions_ptr: Vec<*const i8> = required_device_extensions
                .iter()
                .map(|s| s.as_ptr())
                .collect();

            let device_create_info = vk::DeviceCreateInfo::default()
                .queue_create_infos(&queue_create_infos)
                .enabled_extension_names(&enabled_device_extensions_ptr)
                .push_next(&mut features2);

            let device = instance
                .create_device(physical_device, &device_create_info, None)
                .map_err(|e| StarforgeError::RendererError(e.to_string()))?;

            // Step 12: Get Queue Handles
            let graphics_queue = device.get_device_queue(graphics_queue_family_index, 0);
            let graphics_queue_info = QueueInfo {
                queue: graphics_queue,
                family_index: graphics_queue_family_index,
            };
            let compute_queue =
                if let Some(compute_queue_family_index) = compute_queue_family_index_opt {
                    let compute_queue = device.get_device_queue(compute_queue_family_index, 0);
                    Some(QueueInfo {
                        queue: compute_queue,
                        family_index: compute_queue_family_index,
                    })
                } else {
                    None
                };
            let transfer_queue =
                if let Some(transfer_queue_family_index) = transfer_queue_family_index_opt {
                    let transfer_queue = device.get_device_queue(transfer_queue_family_index, 0);
                    Some(QueueInfo {
                        queue: transfer_queue,
                        family_index: transfer_queue_family_index,
                    })
                } else {
                    None
                };

            // --- Memory Allocator ---
            // Step 13: Create vk-mem Allocator
            let mut allocator_create_info =
                vk_mem::AllocatorCreateInfo::new(&instance, &device, physical_device);
            allocator_create_info.vulkan_api_version = app_info.api_version;
            let allocator = vk_mem::Allocator::new(allocator_create_info).map_err(|e| {
                StarforgeError::RendererError(format!(
                    "Failed to create Vulkan memory allocator: {}",
                    e
                ))
            })?;

            // -- Final Construction
            info!("Vulkan Context initialized successfully.");
            Ok(Self {
                _entry: entry,
                instance,
                validation,
                physical_device,
                physical_device_properties: pdev_props.properties,
                device,
                graphics_queue: graphics_queue_info,
                compute_queue,
                transfer_queue,
                allocator,
                enabled_device_extensions: required_device_extensions_cstr,
            })
        }
    }

    // --- Public Getters ---

    pub fn instance(&self) -> &ash::Instance {
        &self.instance
    }
    pub fn physical_device(&self) -> vk::PhysicalDevice {
        self.physical_device
    }
    pub fn device(&self) -> &ash::Device {
        &self.device
    }
    pub fn graphics_queue(&self) -> &QueueInfo {
        &self.graphics_queue
    }
    pub fn allocator(&self) -> &vk_mem::Allocator {
        &self.allocator
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        info!("Dropping Vulkan Context...");
        unsafe {
            if let Err(e) = self.device.device_wait_idle() {
                error!("Failed to wait for device idle: {}", e);
            }

            // Step 1: Destroy Logical Device
            self.device.destroy_device(None);

            // Step 2: Destroy Debug Messenger
            if let Some(validation) = self.validation.take() {
                validation
                    .debug_utils
                    .destroy_debug_utils_messenger(validation.debug_messenger, None);
            }

            // Step 3: Destroy Vulkan Instance
            self.instance.destroy_instance(None);
        }
        info!("Vulkan Context destroyed.");
    }
}

/// --- Debug Callback ---
extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message = unsafe { CStr::from_ptr((*p_callback_data).p_message) };
    let ty = format!("{:?}", message_type).to_lowercase();
    match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            info!("Vulkan Info {}: {}", ty, message.to_string_lossy())
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            warn!("Vulkan Warning {}: {}", ty, message.to_string_lossy())
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            error!("Vulkan Error {}: {}", ty, message.to_string_lossy())
        }
        _ => info!("Vulkan Message {}: {}", ty, message.to_string_lossy()),
    }
    vk::FALSE // Should return false according to spec
}
