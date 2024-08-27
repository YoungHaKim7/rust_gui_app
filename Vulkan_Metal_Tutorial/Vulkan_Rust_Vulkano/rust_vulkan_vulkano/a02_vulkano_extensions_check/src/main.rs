use vulkano::{
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    VulkanLibrary,
};

fn main() {
    // Load the Vulkan library
    let library = VulkanLibrary::new()
        .unwrap_or_else(|err| panic!("Couldn't load Vulkan library: {:?}", err));

    // Define instance extensions
    let extensions = InstanceExtensions {
        khr_surface: true,
        // Use platform-specific extensions
        #[cfg(target_os = "android")]
        khr_android_surface: true,
        #[cfg(target_os = "windows")]
        khr_win32_surface: true,
        #[cfg(target_os = "linux")]
        khr_xcb_surface: true, // Or `khr_wayland_surface` for Wayland
        ..InstanceExtensions::empty()
    };

    // Create a Vulkan instance with the specified extensions
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: extensions,
            ..Default::default()
        },
    )
    .unwrap_or_else(|err| panic!("Couldn't create instance: {:?}", err));

    println!("Vulkan instance created successfully!");
}
