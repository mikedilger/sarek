
use {enumerate_instance_extension_properties, enumerate_instance_layer_properties};
use instance::{ApplicationInfo, InstanceCreateInfo, Instance};
use Version;

#[test]
fn core_calls() {
    let eps = enumerate_instance_extension_properties(None).unwrap();
    for ep in &eps {
        println!("{} (version {})", ep.extension_name, ep.spec_version);
    }

    // Test enumerate_instance_extension_properties() again with a specific layer
    let eps = enumerate_instance_extension_properties(
        Some("VK_LAYER_LUNARG_parameter_validation")).unwrap();
    for ep in &eps {
        println!("{} (version {})", ep.extension_name, ep.spec_version);
    }

    let lps = enumerate_instance_layer_properties().unwrap();
    for lp in &lps {
        println!("{} (version {}, impl version {}): {}",
                 lp.layer_name, lp.spec_version, lp.implementation_version,
                 lp.description);
    }
}

#[test]
fn instance() {
    let instance = Instance::new(InstanceCreateInfo {
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
    }).unwrap();

    let _devices = instance.enumerate_physical_devices().unwrap();
}
