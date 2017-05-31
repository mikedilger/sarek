
#[repr(C)]
pub enum ValidationCheck {
    ValidationCheckAll = 0,
}

impl From<ValidationCheck> for VkValidationCheckEXT {
    fn from(c: ValidationCheck) -> VkValidationCheckEXT {
        unsafe {
            mem::transmute(c)
        }
    }
}

impl Instance {
    #[allow(unused_variables)]
    pub fn new(mut loader: InstanceLoader, create_info: InstanceCreateInfo,
               mut disable_validation_checks: Vec<ValidationCheck>)
               -> Result<(Instance, InstanceLoader), Error>
    {
        let extension_names = get_extension_names();

        // Setup strings for passing into vkCreateInstance down below.  These must
        // not go out of scope until after that function is called.
        let (extension_names_owned, extension_names) = {
            let mut extension_names_owned: Vec<CString> = Vec::new();
            for ref name in extension_names {
                extension_names_owned.push( CString::new(name.as_bytes())? );
            }
            let extension_names: Vec<*const c_char> = extension_names_owned.iter()
                .map(|name| name.as_ptr())
                .collect();
            (extension_names_owned, extension_names)
        };

        let app_name = CString::new(create_info.application_info.application_name.as_bytes())?;
        let engine_name = CString::new(create_info.application_info.engine_name.as_bytes())?;

        let (layer_names_owned, layer_names) = {
            let mut layer_names_owned: Vec<CString> = Vec::new();
            for ref name in &create_info.enabled_layer_names {
                layer_names_owned.push( CString::new(name.as_bytes())? );
            }
            let layer_names: Vec<*const c_char> = layer_names_owned.iter()
                .map(|name| name.as_ptr())
                .collect();
            (layer_names_owned, layer_names)
        };

        let app_info = {
            VkApplicationInfo {
                sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: ptr::null(),
                pApplicationName: app_name.as_ptr(),
                applicationVersion: create_info.application_info
                    .application_version.to_vk(),
                pEngineName: engine_name.as_ptr(),
                engineVersion: create_info.application_info
                    .engine_version.to_vk(),
                apiVersion: vk_make_version!(1, 0, 3),
            }
        };

        let mut disable_validation_checks: Vec<VkValidationCheckEXT> =
            disable_validation_checks.drain(..).map(|vc| From::from(vc)).collect();
        let vflags = VkValidationFlagsEXT {
            sType: VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT,
            pNext: ptr::null(),
            disabledValidationCheckCount: disable_validation_checks.len() as u32,
            pDisabledValidationChecks: disable_validation_checks.as_mut_ptr(),
        };

        let create_info = {
            VkInstanceCreateInfo {
                sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                pNext: if disable_validation_checks.len() > 0 {
                    unsafe { mem::transmute(&vflags) }
                } else {
                    ptr::null()
                },
                flags: VK_INSTANCE_CREATE_DUMMY,
                pApplicationInfo: &app_info,
                enabledLayerCount: layer_names.len() as u32,
                ppEnabledLayerNames: if layer_names.len() > 0 {
                    layer_names.as_ptr()
                } else {
                    ptr::null()
                },
                enabledExtensionCount: extension_names.len() as u32,
                ppEnabledExtensionNames: if extension_names.len() > 0 {
                    extension_names.as_ptr()
                } else {
                    ptr::null()
                }
            }
        };

        let instance = unsafe {
            let mut instance: VkInstance = ptr::null_mut();
            vk_try!((loader.0.core_null_instance.vkCreateInstance)(
                &create_info,
                ptr::null(),
                &mut instance
            ));
            assert!(instance != ptr::null_mut());
            instance
        };

        // Load instance functions
        loader.load(instance)?;

        Ok((Instance(instance), loader))
    }
}
