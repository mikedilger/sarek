
use {enumerate_instance_extension_properties, enumerate_instance_layer_properties};

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
