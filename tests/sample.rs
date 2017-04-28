
extern crate sarek;

use sarek::{enumerate_instance_extension_properties, enumerate_instance_layer_properties};
use sarek::{Version, InstanceLoader, Instance};
use sarek::instance::{ApplicationInfo, InstanceCreateInfo, ValidationCheck};

#[test]
pub fn main() {
    // Print all available extensions (except layer extensions)
    let eps = enumerate_instance_extension_properties(None).unwrap();
    for ep in &eps {
        println!("{} (version {})", ep.extension_name, ep.spec_version);
    }

    // Print extensions to a particular layer
    let eps = enumerate_instance_extension_properties(
        Some("VK_LAYER_LUNARG_parameter_validation")).unwrap();
    for ep in &eps {
        println!("{} (version {})", ep.extension_name, ep.spec_version);
    }

    // Print available layers
    let lps = enumerate_instance_layer_properties().unwrap();
    for lp in &lps {
        println!("{} (version {}, impl version {}): {}",
                 lp.layer_name, lp.spec_version, lp.implementation_version,
                 lp.description);
    }


    // Initialize an instance loader
    let mut loader = InstanceLoader::new();

    // Create an instance
    let instance = Instance::new(
        &mut loader,
        InstanceCreateInfo {
            application_info: ApplicationInfo {
                application_name: "Test Application".to_owned(),
                application_version: Version(0,1,0),
                engine_name: "Test Engine".to_owned(),
                engine_version: Version(0,1,0),
            },
            enabled_layer_count: 1,
            enabled_layer_names: vec!["VK_LAYER_LUNARG_parameter_validation".to_owned()],
            enabled_extension_count: 0,
            enabled_extension_names: vec![],
        },
        vec![ ValidationCheck::ValidationCheckAll ]
    ).unwrap();

    // Enumerate physical devices
    let devices = instance.enumerate_physical_devices(&mut loader).unwrap();

    for device in &devices {
        let properties = device.get_properties(&mut loader);
        println!("Device: {:?}", properties);
    }
}
