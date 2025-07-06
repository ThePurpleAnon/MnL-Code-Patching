pub mod alloc;
pub mod mmio;
pub mod nitro;
pub mod nocash;

use core::panic::PanicInfo;

custom_print::define_macros!(#[macro_export] { print, println, dbg }, concat, $crate::nds::nocash::print_using_opcode);
pub use {dbg, print, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("===== PANIC IN CUSTOM CODE! =====\n{}", info);

    loop {}
}
