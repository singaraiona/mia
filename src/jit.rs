use dynasmrt::{self, DynasmApi, DynasmLabelApi, ExecutableBuffer};
use dynasmrt::x64::Assembler;
use std::{io, slice, mem};
use std::io::Write;
use mia::{AST, Polyad};
use context::Context;
use dyad;
use polyad;

pub type JitFunction = extern "win64" fn(*const AST, usize, &mut Context) -> i64;
pub type Compiled = extern "win64" fn() -> i64;

lazy_static! {
    static ref _COMPILED: [(Polyad, JitFunction);2] =
        [(polyad::prin, prin), (polyad::prinl, prinl)];
}

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

fn get_compiled(p: Polyad) -> JitFunction {
    _COMPILED.iter().find(|f| f.0 as i64 == p as i64).map(|f| f.1).unwrap()
}

pub extern "win64" fn prin(offset: *const AST, len: usize, ctx: &mut Context) -> i64 {
    let args = unsafe { ::std::slice::from_raw_parts(offset, len) };
    print!("{}", args.iter().map(|a| a.to_string()).collect::<String>());
    1
}

pub extern "win64" fn prinl(offset: *const AST, len: usize, ctx: &mut Context) -> i64 {
    let args = unsafe { ::std::slice::from_raw_parts(offset, len) };
    println!("{}", args.iter().map(|a| a.to_string()).collect::<String>());
    1
}

pub enum Ret {
    Long,
    Float,
    Nil
}

pub struct Compiler<'a> {
   pub ctx: &'a Context,
   pub ret: Ret
}

impl<'a> Compiler<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        Compiler { ctx: ctx, ret: Ret::Nil }
    }

    pub fn compile(&mut self, ast: &AST) -> ExecutableBuffer {
        let mut ops = Assembler::new();
        self.compile_body(ast, &mut ops);
        dynasm!(ops
            ; pop rax
            ; ret
        );
        ops.finalize().unwrap()
    }

    fn compile_body(&mut self, ast: &AST, ops: &mut Assembler) {
        match *ast {
            AST::List(ref l) if !l.is_empty() => {
                match l[0] {
                    AST::Dyad(d) => {
                        self.compile_body(&l[2], ops);
                        self.compile_body(&l[1], ops);
                        compile_binop_i64!(ops, add);
                        self.ret = Ret::Long;
                    },
                    AST::Polyad(f) => {
                        let fun = get_compiled(f);
                        let offset = &l[1] as *const AST as i64;
                        let len = l.len() - 1;
                        dynasm!(ops
                            ; mov rcx, QWORD offset
                            ; mov rdx, QWORD len as i64
                            ; mov rax, QWORD fun as i64
                            ; sub rsp, BYTE 0x28
                            ; call rax
                            ; add rsp, BYTE 0x28
                            ; push rax
                        );
                    }
                    _ => unimplemented!(),
                }
            }
            AST::Long(l) => {
                dynasm!(ops
                    ; mov r8, QWORD l as i64
                    ; push r8
                );
                self.ret = Ret::Long;
            }
            _ => unimplemented!(),
        }
    }
}

macro_rules! jit_call {
    ($c:expr, $buf:expr) => {
        {
           match $c.ret {
                $crate::jit::Ret::Long => {
                    let call_fn: extern "win64" fn() -> i64 = unsafe { mem::transmute($buf.as_ptr()) };
                    Ok(long!(call_fn()))
                }
                $crate::jit::Ret::Float => {
                    let call_fn: extern "win64" fn() -> f64 = unsafe { mem::transmute($buf.as_ptr()) };
                    Ok(float!(call_fn()))
                }
                $crate::jit::Ret::Nil=> {
                    let call_fn: extern "win64" fn() -> i64 = unsafe { mem::transmute($buf.as_ptr()) };
                    let _ = call_fn();
                    Ok(NIL!())
                }
                _ => unimplemented!(),
           }
        }
    }
}
//
