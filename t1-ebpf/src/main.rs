#![no_std]
#![no_main]

use aya_ebpf::{macros::tracepoint, programs::TracePointContext};
use aya_log_ebpf::info;

#[tracepoint]
pub fn t1(ctx: TracePointContext) -> u32 {
    match try_t1(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_t1(ctx: TracePointContext) -> Result<u32, u32> {
    info!(&ctx, "tracepoint sys_enter_execve called");
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
