//! A trivial wrapper type for an LLVM execution engine

use crate::llvm::module::Module;
use llvm_sys::execution_engine::*;
use std::{ffi::CString, marker::PhantomData, mem};

pub struct ExecutionEngine<'c> {
    engine: LLVMExecutionEngineRef,
    pub(in crate::llvm) context: PhantomData<&'c crate::llvm::Context>,
}

impl<'c> ExecutionEngine<'c> {
    pub fn new(module: Module<'_>) -> ExecutionEngine<'_> {
        unsafe {
            let mut engine = mem::uninitialized();
            let mut out = mem::zeroed();
            LLVMCreateExecutionEngineForModule(&mut engine, module.module, &mut out);
            ExecutionEngine {
                engine,
                context: module.context,
            }
        }
    }

    /// Unsafe as the returned address is only valid for the lifetime of
    /// this ExecutionEngine.
    pub unsafe fn get_func_addr(&self, name: &str) -> u64 {
        let name = CString::new(name).unwrap();
        LLVMGetFunctionAddress(self.engine, name.as_ptr())
    }
}

impl<'c> Drop for ExecutionEngine<'c> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeExecutionEngine(self.engine);
        }
    }
}
