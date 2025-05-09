// my_os/src/lang_items.rs
use crate::sbi::shutdown;
use core::panic::PanicInfo;
use crate::println;

#[panic_handler]

fn panic(_info: &PanicInfo) -> ! {
    if let Some(location) = _info.location() {
        println!("the error is at {}:{}:{}",
                 location.file(),
                 location.line(),
                 _info.message()
        );
    }
    else {
        println!("the error is at {}", _info.message());
    }
    shutdown(true)
}

