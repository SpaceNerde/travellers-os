#![no_std]
#![no_main]

use core::panic::PanicInfo;

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024;
    config
};
bootloader_api::entry_point!(kernel_main, config = &CONFIG);

// entry point for kernel
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    panic!()
}
