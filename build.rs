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
        } else if env::var_os("CARGO_FEATURE_STM32F730").is_some() {
            "src/stm32f7/stm32f730/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F745").is_some() {
            "src/stm32f7/stm32f745/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F750").is_some() {
            "src/stm32f7/stm32f750/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F765").is_some() {
            "src/stm32f7/stm32f765/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F7X2").is_some() {
            "src/stm32f7/stm32f7x2/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F7X3").is_some() {
            "src/stm32f7/stm32f7x3/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F7X6").is_some() {
            "src/stm32f7/stm32f7x6/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F7X7").is_some() {
            "src/stm32f7/stm32f7x7/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F7X9").is_some() {
            "src/stm32f7/stm32f7x9/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G030").is_some() {
            "src/stm32g0/stm32g030/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G031").is_some() {
            "src/stm32g0/stm32g031/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G041").is_some() {
            "src/stm32g0/stm32g041/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G070").is_some() {
            "src/stm32g0/stm32g070/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G071").is_some() {
            "src/stm32g0/stm32g071/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G07X").is_some() {
            "src/stm32g0/stm32g07x/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G081").is_some() {
            "src/stm32g0/stm32g081/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G431").is_some() {
            "src/stm32g4/stm32g431/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G441").is_some() {
            "src/stm32g4/stm32g441/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G471").is_some() {
            "src/stm32g4/stm32g471/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G473").is_some() {
            "src/stm32g4/stm32g473/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G474").is_some() {
            "src/stm32g4/stm32g474/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G483").is_some() {
            "src/stm32g4/stm32g483/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G484").is_some() {
            "src/stm32g4/stm32g484/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32H743").is_some() {
            "src/stm32h7/stm32h743/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32H743V").is_some() {
            "src/stm32h7/stm32h743v/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32H747CM4").is_some() {
            "src/stm32h7/stm32h747cm4/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32H747CM7").is_some() {
            "src/stm32h7/stm32h747cm7/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32H753").is_some() {
            "src/stm32h7/stm32h753/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32H753V").is_some() {
            "src/stm32h7/stm32h753v/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32H7B3").is_some() {
            "src/stm32h7/stm32h7b3/device.x"
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
        } else if env::var_os("CARGO_FEATURE_STM32L552").is_some() {
            "src/stm32l5/stm32l552/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32L562").is_some() {
            "src/stm32l5/stm32l562/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32MP157").is_some() {
            "src/stm32mp/stm32mp157/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32WB55").is_some() {
            "src/stm32wb/stm32wb55/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32WLE5").is_some() {
            "src/stm32wl/stm32wle5/device.x"
        } else {
            panic!("No device features selected");
        };
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }
    println!("cargo:rerun-if-changed=build.rs");
}
