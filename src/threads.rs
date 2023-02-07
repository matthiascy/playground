use core::arch::asm;

const STACK_SIZE: isize = 48;

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64, // stack pointer
}

unsafe fn gt_switch(new: *const ThreadContext) {
    // asm!(
    //     "mov rsp, [{new} + 0x00]",
    //     "ret",
    //     new = in(reg) new,
    // );
    asm!(
        "mov rsp, [{0} + 0x00]",
        "ret",
        in(reg) new,
    );
}

fn hello() -> ! {
    println!("Walking on the new stack!");
    loop {}
}

fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0u8; STACK_SIZE as usize];

    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(STACK_SIZE);
        let sb_aligned = (stack_bottom as usize & !0xf) as *mut u8;
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);
        ctx.rsp = sb_aligned.offset(-16) as u64;
        gt_switch(&mut ctx);
    }
}
