use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let device_file = if env::var_os("CARGO_FEATURE_STM32F0X0").is_some() {
            "src/stm32f0/stm32f0x0/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F0X1").is_some() {
            "src/stm32f0/stm32f0x1/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F0X2").is_some() {
            "src/stm32f0/stm32f0x2/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F0X8").is_some() {
            "src/stm32f0/stm32f0x8/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F100").is_some() {
            "src/stm32f1/stm32f100/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F101").is_some() {
            "src/stm32f1/stm32f101/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F102").is_some() {
            "src/stm32f1/stm32f102/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F103").is_some() {
            "src/stm32f1/stm32f103/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F107").is_some() {
            "src/stm32f1/stm32f107/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F215").is_some() {
            "src/stm32f2/stm32f215/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F217").is_some() {
            "src/stm32f2/stm32f217/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F301").is_some() {
            "src/stm32f3/stm32f301/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F302").is_some() {
            "src/stm32f3/stm32f302/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F303").is_some() {
            "src/stm32f3/stm32f303/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F373").is_some() {
            "src/stm32f3/stm32f373/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F3X4").is_some() {
            "src/stm32f3/stm32f3x4/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F3X8").is_some() {
            "src/stm32f3/stm32f3x8/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F401").is_some() {
            "src/stm32f4/stm32f401/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F405").is_some() {
            "src/stm32f4/stm32f405/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F407").is_some() {
            "src/stm32f4/stm32f407/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F410").is_some() {
            "src/stm32f4/stm32f410/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F411").is_some() {
            "src/stm32f4/stm32f411/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F412").is_some() {
            "src/stm32f4/stm32f412/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F413").is_some() {
            "src/stm32f4/stm32f413/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F427").is_some() {
            "src/stm32f4/stm32f427/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F429").is_some() {
            "src/stm32f4/stm32f429/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F446").is_some() {
            "src/stm32f4/stm32f446/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F469").is_some() {
            "src/stm32f4/stm32f469/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F7X2").is_some() {
            "src/stm32f7/stm32f7x2/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F7X3").is_some() {
            "src/stm32f7/stm32f7x3/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F7X5").is_some() {
            "src/stm32f7/stm32f7x5/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F7X6").is_some() {
            "src/stm32f7/stm32f7x6/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F7X7").is_some() {
            "src/stm32f7/stm32f7x7/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F7X9").is_some() {
            "src/stm32f7/stm32f7x9/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32H7X3").is_some() {
            "src/stm32h7/stm32h7x3/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32L0X1").is_some() {
            "src/stm32l0/stm32l0x1/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32L0X2").is_some() {
            "src/stm32l0/stm32l0x2/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32L0X3").is_some() {
            "src/stm32l0/stm32l0x3/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32L100").is_some() {
            "src/stm32l1/stm32l100/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32L151").is_some() {
            "src/stm32l1/stm32l151/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32L162").is_some() {
            "src/stm32l1/stm32l162/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32L4X1").is_some() {
            "src/stm32l4/stm32l4x1/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32L4X2").is_some() {
            "src/stm32l4/stm32l4x2/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32L4X3").is_some() {
            "src/stm32l4/stm32l4x3/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32L4X5").is_some() {
            "src/stm32l4/stm32l4x5/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32L4X6").is_some() {
            "src/stm32l4/stm32l4x6/device.x"
        } else {
            panic!("No device features selected");
        };
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }
    println!("cargo:rerun-if-changed=build.rs");
}
