extern mod std;
extern mod cl (uuid = "1205b5b0-fbd0-4eeb-bfac-e85947419b4e");
extern mod opencl (uuid = "f83bfc2b-e3ee-4e4c-b324-70e379fbcff2");

use core::hashmap;
use std::json;
use std::json::ToJson;

macro_rules! cl_error(($v:expr) => (match $v { Ok(x) => x, Err(n) => fail!(fmt!("OpenCL error %?", n)) }))

macro_rules! cl_add_property(($map:ident : $name:expr, $prop:expr) => ($map.insert(~$name, $prop.to_json())))
macro_rules! cl_platform_property(($name:expr, $prop:expr) => (cl_add_property!(platform_object: $name, $prop)))
macro_rules! cl_device_property(($name:expr, $prop:expr) => (cl_add_property!(device_object: $name, $prop)))

macro_rules! cl_fp_config(($fp:expr) => ({
    let value = $fp;
    let mut device_config = ~hashmap::linear_map_with_capacity::<~str, json::Json>(6);

    cl_add_property!(device_config: "denormals", ((value & opencl::CL_FP_DENORM) == opencl::CL_FP_DENORM));
    cl_add_property!(device_config: "quiet_nan", ((value & opencl::CL_FP_INF_NAN) == opencl::CL_FP_INF_NAN));
    cl_add_property!(device_config: "round_to_nearest", ((value & opencl::CL_FP_ROUND_TO_NEAREST) == opencl::CL_FP_ROUND_TO_NEAREST));
    cl_add_property!(device_config: "round_to_zero", ((value & opencl::CL_FP_ROUND_TO_ZERO) == opencl::CL_FP_ROUND_TO_ZERO));
    cl_add_property!(device_config: "round_to_inf", ((value & opencl::CL_FP_ROUND_TO_INF) == opencl::CL_FP_ROUND_TO_INF));
    cl_add_property!(device_config: "fma", ((value & opencl::CL_FP_FMA) == opencl::CL_FP_FMA));

    device_config
}))

macro_rules! cl_execution_capabilities(($caps:expr) => ({
    let value = $caps;
    let mut caps_config = ~hashmap::linear_map_with_capacity::<~str, json::Json>(6);

    cl_add_property!(caps_config: "opencl", ((value & opencl::CL_EXEC_KERNEL) == opencl::CL_EXEC_KERNEL));
    cl_add_property!(caps_config: "native", ((value & opencl::CL_EXEC_NATIVE_KERNEL) == opencl::CL_EXEC_NATIVE_KERNEL));

    caps_config
}))

macro_rules! cl_queue_properties(($props:expr) => ({
    let value = $props;
    let mut props_config = ~hashmap::linear_map_with_capacity::<~str, json::Json>(6);

    cl_add_property!(props_config: "out_of_order", ((value & opencl::CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE) == opencl::CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE));
    cl_add_property!(props_config: "profiling", ((value & opencl::CL_QUEUE_PROFILING_ENABLE) == opencl::CL_QUEUE_PROFILING_ENABLE));

    props_config
}))

fn main() {
    let platforms = cl_error!(cl::Platform::all());

    let result = vec::map(platforms, |platform| {
        let mut platform_object = ~hashmap::linear_map_with_capacity::<~str, json::Json>(6);

        cl_platform_property!("profile", platform.profile());
        cl_platform_property!("version", platform.version());
        cl_platform_property!("name", platform.name());
        cl_platform_property!("vendor", platform.vendor());
        cl_platform_property!("extensions", vec::map(platform.extensions(), |x| { x.to_owned() }));

        let devices = cl_error!(platform.devices(opencl::CL_DEVICE_TYPE_ALL));
        platform_object.insert(~"devices", json::List(vec::map(devices, |device| {
            let mut device_object = ~hashmap::linear_map_with_capacity::<~str, json::Json>(16); // TODO: Calculate good capacity

            cl_device_property!("type", match device.device_type() {
                opencl::CL_DEVICE_TYPE_CPU => ~"CPU",
                opencl::CL_DEVICE_TYPE_GPU => ~"GPU",
                opencl::CL_DEVICE_TYPE_ACCELERATOR => ~"Accelerator",
                opencl::CL_DEVICE_TYPE_CUSTOM => ~"Custom",
                _ => ~"Unknown"
            });
            cl_device_property!("vendor_id", device.vendor_id());
            cl_device_property!("max_compute_units", device.max_compute_units());
            cl_device_property!("max_work_group_size", device.max_work_group_size());
            cl_device_property!("max_work_item_sizes", device.max_work_item_sizes());

            let mut preferred_vector_width_object = ~hashmap::linear_map_with_capacity::<~str, json::Json>(7);

            cl_add_property!(preferred_vector_width_object: "char", device.preferred_vector_width_char());
            cl_add_property!(preferred_vector_width_object: "short", device.preferred_vector_width_short());
            cl_add_property!(preferred_vector_width_object: "int", device.preferred_vector_width_int());
            cl_add_property!(preferred_vector_width_object: "long", device.preferred_vector_width_long());
            cl_add_property!(preferred_vector_width_object: "half", device.preferred_vector_width_half());
            cl_add_property!(preferred_vector_width_object: "float", device.preferred_vector_width_float());
            cl_add_property!(preferred_vector_width_object: "double", device.preferred_vector_width_double());

            let mut native_vector_width_object = ~hashmap::linear_map_with_capacity::<~str, json::Json>(7);

            cl_add_property!(native_vector_width_object: "char", device.native_vector_width_char());
            cl_add_property!(native_vector_width_object: "short", device.native_vector_width_short());
            cl_add_property!(native_vector_width_object: "int", device.native_vector_width_int());
            cl_add_property!(native_vector_width_object: "long", device.native_vector_width_long());
            cl_add_property!(native_vector_width_object: "half", device.native_vector_width_half());
            cl_add_property!(native_vector_width_object: "float", device.native_vector_width_float());
            cl_add_property!(native_vector_width_object: "double", device.native_vector_width_double());

            let mut vector_width_object = ~hashmap::linear_map_with_capacity::<~str, json::Json>(2);

            cl_add_property!(vector_width_object: "native", native_vector_width_object);
            cl_add_property!(vector_width_object: "preferred", preferred_vector_width_object);

            cl_device_property!("vector_width", vector_width_object);

            cl_device_property!("max_clock_frequency", device.max_clock_frequency());
            cl_device_property!("address_bits", device.address_bits());
            cl_device_property!("max_read_image_args", device.max_read_image_args());
            cl_device_property!("max_write_image_args", device.max_write_image_args());
            cl_device_property!("max_mem_alloc_size", device.max_mem_alloc_size());

            if (device.image_support()) {
                let mut image2d_object = ~hashmap::linear_map_with_capacity::<~str, json::Json>(2);
                let mut image3d_object = ~hashmap::linear_map_with_capacity::<~str, json::Json>(3);

                cl_add_property!(image2d_object: "max_width", device.image2d_max_width());
                cl_add_property!(image2d_object: "max_height", device.image2d_max_height());

                cl_add_property!(image3d_object: "max_width", device.image3d_max_width());
                cl_add_property!(image3d_object: "max_height", device.image3d_max_height());
                cl_add_property!(image3d_object: "max_depth", device.image3d_max_depth());

                let mut image_object = ~hashmap::linear_map_with_capacity::<~str, json::Json>(2);

                cl_add_property!(image_object: "2d", image2d_object);
                cl_add_property!(image_object: "3d", image3d_object);

                cl_device_property!("image", image_object);
            } else {
                cl_device_property!("image", ~hashmap::linear_map_with_capacity::<~str, json::Json>(0));
            }

            cl_device_property!("max_parameter_size", device.max_parameter_size());
            cl_device_property!("max_samplers", device.max_samplers());
            cl_device_property!("mem_base_addr_align", device.mem_base_addr_align());
            cl_device_property!("min_data_type_align_size", device.min_data_type_align_size());

            let mut fp_config_object = ~hashmap::linear_map_with_capacity::<~str, json::Json>(2);

            cl_add_property!(fp_config_object: "single", cl_fp_config!(device.single_fp_config()));
            cl_add_property!(fp_config_object: "double", cl_fp_config!(device.double_fp_config()));

            cl_device_property!("fp_config", fp_config_object);

            let mut global_mem_object = ~hashmap::linear_map_with_capacity::<~str, json::Json>(4);

            cl_add_property!(global_mem_object: "cache_type", match device.global_mem_cache_type() {
                opencl::CL_NONE => ~"none",
                opencl::CL_READ_ONLY_CACHE => ~"read only",
                opencl::CL_READ_WRITE_CACHE => ~"read/write",
                _ => ~"unknown"
            });

            cl_add_property!(global_mem_object: "cacheline_size", device.global_mem_cacheline_size());
            cl_add_property!(global_mem_object: "cache_size", device.global_mem_cache_size());
            cl_add_property!(global_mem_object: "size", device.global_mem_size());

            cl_device_property!("global_memory", global_mem_object);

            cl_device_property!("max_constant_buffer_size", device.max_constant_buffer_size());
            cl_device_property!("max_constant_args", device.max_constant_args());

            let mut local_mem_object = ~hashmap::linear_map_with_capacity::<~str, json::Json>(2);

            cl_add_property!(local_mem_object: "type", match device.local_mem_type() {
                opencl::CL_LOCAL => ~"local",
                opencl::CL_GLOBAL => ~"global",
                _ => ~"unknown"
            });
            cl_add_property!(local_mem_object: "size", device.local_mem_size());

            cl_device_property!("local_memory", local_mem_object);

            cl_device_property!("error_correction_support", device.error_correction_support());
            cl_device_property!("profiling_timer_resolution", device.profiling_timer_resolution());
            cl_device_property!("endian_little", device.endian_little());
            cl_device_property!("available", device.available());
            cl_device_property!("compiler_available", device.compiler_available());
            cl_device_property!("linker_available", device.linker_available());
            cl_device_property!("execution_capabilities", cl_execution_capabilities!(device.execution_capabilities()));
            cl_device_property!("queue_properties", cl_queue_properties!(device.queue_properties()));
            cl_device_property!("name", device.name());
            cl_device_property!("vendor", device.vendor());
            cl_device_property!("driver_version", device.driver_version());
            cl_device_property!("profile", device.profile());
            cl_device_property!("version", device.version());
            cl_device_property!("extensions", device.extensions());
            cl_device_property!("host_unified_memory", device.host_unified_memory());
            cl_device_property!("opencl_c_version", device.opencl_c_version());
            cl_device_property!("built_in_kernels", device.built_in_kernels());
            cl_device_property!("image_max_buffer_size", device.image_max_buffer_size());
            cl_device_property!("image_max_array_size", device.image_max_array_size());
            cl_device_property!("partition_max_sub_devices", device.partition_max_sub_devices());
            cl_device_property!("partition_properties", device.partition_properties());
            cl_device_property!("partition_affinity_domain", device.partition_affinity_domain());
            cl_device_property!("partition_type", device.partition_type());
            cl_device_property!("reference_count", device.reference_count());
            cl_device_property!("preferred_interop_user_sync", device.preferred_interop_user_sync());
            cl_device_property!("printf_buffer_size", device.printf_buffer_size());

            device_object.to_json()
        })));

        platform_object.to_json()
    });
    
    io::println(json::to_pretty_str(&json::List(result)));
}