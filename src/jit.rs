use dynasmrt::{self, DynasmApi, DynasmLabelApi};
use std::{io, slice, mem};
use std::io::Write;
use mia::AST;

pub type JitDyad<T> = extern "win64" fn(T, T) -> T;

pub extern "win64" fn print(buffer: *const u8, length: u64) -> bool {
    io::stdout().write_all(unsafe {
        slice::from_raw_parts(buffer, length as usize)
    }).is_ok()
}

pub fn plus_i64(jit: &mut dynasmrt::x64::Assembler) {
    dynasm!(jit
        ; mov rax, rcx
        ; add rax, rdx
        ; ret
    );
}
