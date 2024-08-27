use vulkano::{
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    VulkanLibrary,
};

fn main() {
    // Create a Vulkan library instance
    let library = VulkanLibrary::new().expect("Failed to create Vulkan library");

    // Define the instance create info with default extensions
    let instance_create_info = InstanceCreateInfo {
        enabled_extensions: InstanceExtensions {
            ..InstanceExtensions::default()
        },
        ..Default::default()
    };

    // Create a Vulkan instance
    let instance = Instance::new(library, instance_create_info).expect("Failed to create instance");

    // Enumerate all physical devices (GPUs) available on the system
    for physical_device in instance
        .enumerate_physical_devices()
        .expect("Failed to enumerate physical devices")
    {
        println!(
            "Available device: {}",
            physical_device.properties().device_name
        );
    }
}
