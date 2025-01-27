use gccjit::{FunctionType, RValue};
use rustc_codegen_ssa::mir::debuginfo::{FunctionDebugContext, VariableKind};
use rustc_codegen_ssa::traits::{BuilderMethods, DebugInfoBuilderMethods, DebugInfoMethods};
use rustc_middle::middle::cstore::CrateDepKind;
use rustc_middle::mir;
use rustc_middle::ty::{Instance, Ty};
use rustc_span::{SourceFile, Span, Symbol};
use rustc_span::def_id::LOCAL_CRATE;
use rustc_target::abi::Size;
use rustc_target::abi::call::FnAbi;

use crate::builder::Builder;
use crate::context::CodegenCx;

impl<'a, 'gcc, 'tcx> DebugInfoBuilderMethods for Builder<'a, 'gcc, 'tcx> {
    // FIXME(eddyb) find a common convention for all of the debuginfo-related
    // names (choose between `dbg`, `debug`, `debuginfo`, `debug_info` etc.).
    fn dbg_var_addr(&mut self, _dbg_var: Self::DIVariable, _scope_metadata: Self::DIScope, _variable_alloca: Self::Value, _direct_offset: Size, _indirect_offsets: &[Size]) {
        unimplemented!();
    }

    fn insert_reference_to_gdb_debug_scripts_section_global(&mut self) {
        // TODO(antoyo): replace with gcc_jit_context_new_global_with_initializer() if it's added:
        // https://gcc.gnu.org/pipermail/jit/2020q3/001225.html
        //
        // Call the function to initialize global values here.
        // We assume this is only called for the main function.
        use std::iter;

        for crate_num in self.cx.tcx.crates(()).iter().copied().chain(iter::once(LOCAL_CRATE)) {
            // FIXME(antoyo): better way to find if a crate is of proc-macro type?
            if crate_num == LOCAL_CRATE || self.cx.tcx.dep_kind(crate_num) != CrateDepKind::MacrosOnly {
                // NOTE: proc-macro crates are not included in the executable, so don't call their
                // initialization routine.
                let initializer_name = format!("__gccGlobalCrateInit{}", self.cx.tcx.crate_name(crate_num));
                let codegen_init_func = self.context.new_function(None, FunctionType::Extern, self.context.new_type::<()>(), &[],
                initializer_name, false);
                self.llbb().add_eval(None, self.context.new_call(None, codegen_init_func, &[]));
            }
        }

        // TODO(antoyo): insert reference to gdb debug scripts section global.
    }

    fn set_var_name(&mut self, _value: RValue<'gcc>, _name: &str) {
        unimplemented!();
    }

    fn set_dbg_loc(&mut self, _dbg_loc: Self::DILocation) {
        unimplemented!();
    }
}

impl<'gcc, 'tcx> DebugInfoMethods<'tcx> for CodegenCx<'gcc, 'tcx> {
    fn create_vtable_metadata(&self, _ty: Ty<'tcx>, _vtable: Self::Value) {
        // TODO(antoyo)
    }

    fn create_function_debug_context(&self, _instance: Instance<'tcx>, _fn_abi: &FnAbi<'tcx, Ty<'tcx>>, _llfn: RValue<'gcc>, _mir: &mir::Body<'tcx>) -> Option<FunctionDebugContext<Self::DIScope, Self::DILocation>> {
        // TODO(antoyo)
        None
    }

    fn extend_scope_to_file(&self, _scope_metadata: Self::DIScope, _file: &SourceFile) -> Self::DIScope {
        unimplemented!();
    }

    fn debuginfo_finalize(&self) {
        // TODO(antoyo)
    }

    fn create_dbg_var(&self, _variable_name: Symbol, _variable_type: Ty<'tcx>, _scope_metadata: Self::DIScope, _variable_kind: VariableKind, _span: Span) -> Self::DIVariable {
        unimplemented!();
    }

    fn dbg_scope_fn(&self, _instance: Instance<'tcx>, _fn_abi: &FnAbi<'tcx, Ty<'tcx>>, _maybe_definition_llfn: Option<RValue<'gcc>>) -> Self::DIScope {
        unimplemented!();
    }

    fn dbg_loc(&self, _scope: Self::DIScope, _inlined_at: Option<Self::DILocation>, _span: Span) -> Self::DILocation {
        unimplemented!();
    }
}
