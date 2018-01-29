use dynasmrt::{self, DynasmApi, DynasmLabelApi};
use dynasmrt::x64::Assembler;
use std::{io, slice, mem};
use std::io::Write;
use mia::AST;
use context::Context;
use dyad;
use polyad;

// Primitives
pub extern "win64" fn prin(offset: *const AST, len: usize, ctx: &mut Context) {
    let args = unsafe { ::std::slice::from_raw_parts(offset, len) };
    print!("{}", args.iter().map(|a| a.to_string()).collect::<String>());
}

pub extern "win64" fn prinl(offset: *const AST, len: usize, ctx: &mut Context) {
    let args = unsafe { ::std::slice::from_raw_parts(offset, len) };
    println!("{}", args.iter().map(|a| a.to_string()).collect::<String>());
}
//
macro_rules! compile_binop_i64 {
    ($ops:expr, $op:tt) => {
        dynasm!($ops
            ; pop rcx
            ; pop rax
            ; $op rax, rcx
            ; push rax
        );
    }
}

fn _compile(ast: &AST, ops: &mut Assembler, ctx: &mut Context) {
    match *ast {
        AST::List(ref l) if !l.is_empty() => {
            match l[0] {
                AST::Dyad(d) => {
                    _compile(&l[2], ops, ctx);
                    _compile(&l[1], ops, ctx);
                    compile_binop_i64!(ops, add);
                },
                AST::Polyad(f) => {
                    let fun = prinl;
                    let offset = &l[1] as *const AST as i64;
                    let len = l.len() - 1;
                    dynasm!(ops
                        ; mov rcx, QWORD offset
                        ; mov rdx, QWORD len as _
                        ; mov rax, QWORD fun as _
                        ; sub rsp, BYTE 0x28
                        ; call rax
                        ; add rsp, BYTE 0x28
                        ; push rax
                    );
                }
                _ => unimplemented!(),
            }
        }
        AST::Long(l) => { dynasm!(ops; push l as _); }
        _ => unimplemented!(),
    }
}

pub fn compile(ast: &AST, ops: &mut Assembler, ctx: &mut Context) {
    _compile(ast, ops, ctx);
    dynasm!(ops
        ; pop rax
        ; ret
    );
}
//
