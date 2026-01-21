use core::arch::asm;

pub enum Trap<I, E> {
    Interrupt(I),
    Exception(E),
}

#[no_mangle]
pub extern "C" fn trap_handler() {
    let mut rd: usize;
    #[cfg(target_arch = "riscv32")]
    {
        asm!(
            csrrs rd, csr, x0
        )
    }
}
