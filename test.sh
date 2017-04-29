#!/bin/sh

SDK="VulkanSDK/1.0.46.0"

if [ -d "/ocl/eob" ] ; then
    export LD_LIBRARY_PATH=/ocl/eob/${SDK}/x86_64/lib
    export VK_LAYER_PATH=/ocl/eob/${SDK}/x86_64/etc/explicit_layer.d
elif [ -d "/z/devel/EoB/2017_siege" ] ; then
    export LD_LIBRARY_PATH=/z/devel/EoB/2017_siege/${SDK}/x86_64/lib
    export VK_LAYER_PATH=/z/devel/EoB/2017_siege/${SDK}/x86_64/etc/explicit_layer.d
elif [ -d "/c" ] ; then
    export VK_LAYER_PATH=/c/${SDK}/Bin
else
    echo "Unknown path for VulkanSDK"
    exit 1
fi

# Enable layers here, if desired:
export VK_INSTANCE_LAYERS="VK_LAYER_GOOGLE_threading:VK_LAYER_LUNARG_parameter_validation:VK_LAYER_LUNARG_object_tracker:VK_LAYER_LUNARG_core_validation:VK_LAYER_LUNARG_swapchain:VK_LAYER_GOOGLE_unique_objects"

RUST_BACKTRACE=1 cargo test \
              --no-default-features \
              --features="vk_1_0_3 khr_surface khr_xcb_surface khr_xlib_surface ext_debug_report nv_external_memory_capabilities" \
              -- --nocapture
