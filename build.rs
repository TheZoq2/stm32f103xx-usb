use std::env;

fn main() {
    let families: Vec<_> = env::vars().filter_map(|(key, _value)| {
        if key.starts_with("CARGO_FEATURE_STM32") {
            Some(key[14..].to_ascii_lowercase())  // Strip 'CARGO_FEATURE_'
        } else {
            None
        }
    }).collect();

    if families.is_empty() {
        panic!("No family features selected");
    }
    if families.len() > 1 {
        panic!("More than one family feature selected");
    }

    let family = families.first().unwrap();

    let buffer_size;
    let access_scheme;
    let lpm_support;
    let bcd_support;
    let dp_pull_up_support;
    let peripheral_bus;
    match family.as_str() {
        "stm32f103xx" => {
            buffer_size = 512;
            access_scheme = "1x16";
            lpm_support = false;
            bcd_support = false;
            dp_pull_up_support = false;
            peripheral_bus = "apb1";
        }
        other => panic!("Unknown family: {}", other),
    }

    println!("cargo:rustc-cfg=usb_buffer_size=\"{}\"", buffer_size);
    println!("cargo:rustc-cfg=usb_access_scheme=\"{}\"", access_scheme);
    if lpm_support {
        println!("cargo:rustc-cfg=usb_lpm_support");
    }
    if bcd_support {
        println!("cargo:rustc-cfg=usb_bcd_support");
    }
    if dp_pull_up_support {
        println!("cargo:rustc-cfg=usb_dp_pull_up_support");
    }
    println!("cargo:rustc-cfg=usb_peripheral_bus=\"{}\"", peripheral_bus);
}