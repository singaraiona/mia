use dynasmrt::{self, DynasmApi, DynasmLabelApi};
use std::{io, slice, mem};
use std::io::Write;
use mia::AST;

pub extern "win64" fn print(buffer: *const u8, length: u64) -> bool {
    io::stdout().write_all(unsafe {
        slice::from_raw_parts(buffer, length as usize)
    }).is_ok()
}

pub fn eval_seq(seq: &[AST]) -> AST {
    let mut ops = dynasmrt::x64::Assembler::new();
    let string = "Hello World!";

    dynasm!(ops
        ; ->hello:
        ; .bytes string.as_bytes()
    );

    let hello = ops.offset();
    dynasm!(ops
        ; lea rcx, [->hello]
        ; xor edx, edx
        ; mov dl, BYTE string.len() as _
        ; mov rax, QWORD print as _
        ; sub rsp, BYTE 0x28
        ; call rax
        ; add rsp, BYTE 0x28
        ; ret
    );

    let buf = ops.finalize().unwrap();

    let hello_fn: extern "win64" fn() -> bool = unsafe {
        mem::transmute(buf.ptr(hello))
    };

    hello_fn();

    NIL!()
}


