//! Cranelift backend prototype.

use crate::{CodegenConfig, CodegenError, ObjectFile, OptLevel, Program};
use cranelift_codegen::ir::{AbiParam, InstBuilder};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::{isa, settings::Flags};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{DataDescription, Linkage, Module, default_libcall_names};
use cranelift_object::{ObjectBuilder, ObjectModule};
use std::sync::Arc;
use target_lexicon::Triple;

/// Cranelift backend for native object emission.
pub struct CraneliftBackend {
    config: CodegenConfig,
}

impl CraneliftBackend {
    #[must_use]
    pub fn new(config: CodegenConfig) -> Self {
        Self { config }
    }

    /// Compile a Ted program into a native object file.
    pub fn compile_object(&self, program: &Program) -> Result<ObjectFile, CodegenError> {
        let mut flag_builder = settings::builder();
        flag_builder
            .set("opt_level", opt_level_str(self.config.opt_level))
            .map_err(|e| CodegenError::Backend(e.to_string()))?;
        flag_builder
            .set("is_pic", "true")
            .map_err(|e| CodegenError::Backend(e.to_string()))?;

        let flags = Flags::new(flag_builder);
        let isa = select_isa(self.config.target.as_deref(), flags)?;

        let module_name = sanitize_module_name(&program.module_name);
        let builder = ObjectBuilder::new(isa, module_name, default_libcall_names())
            .map_err(|e| CodegenError::Backend(e.to_string()))?;
        let mut module = ObjectModule::new(builder);

        let mut data_ids = Vec::new();
        if !program.prints.is_empty() {
            for (idx, text) in program.prints.iter().enumerate() {
                let data_id = declare_cstring(&mut module, &program.module_name, idx, text)?;
                data_ids.push(data_id);
            }
        }

        // TODO: Lower from MIR into CLIF. For now, emit a stub main that returns 0.
        let mut ctx = module.make_context();
        let sig = module.make_signature();
        ctx.func.signature = sig;
        ctx.func
            .signature
            .returns
            .push(AbiParam::new(cranelift_codegen::ir::types::I32));

        let puts_sig = make_puts_signature(&module);
        let puts_id = module
            .declare_function("puts", Linkage::Import, &puts_sig)
            .map_err(|e| CodegenError::Backend(e.to_string()))?;

        let func_id = module
            .declare_function("main", Linkage::Export, &ctx.func.signature)
            .map_err(|e| CodegenError::Backend(e.to_string()))?;

        let mut func_ctx = FunctionBuilderContext::new();
        {
            let mut builder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);
            let block = builder.create_block();
            builder.switch_to_block(block);
            builder.seal_block(block);
            if !data_ids.is_empty() {
                let puts_ref = module.declare_func_in_func(puts_id, builder.func);
                let ptr_ty = module.isa().pointer_type();
                for data_id in &data_ids {
                    let gv = module.declare_data_in_func(*data_id, builder.func);
                    let ptr = builder.ins().global_value(ptr_ty, gv);
                    builder.ins().call(puts_ref, &[ptr]);
                }
            }
            let zero = builder.ins().iconst(cranelift_codegen::ir::types::I32, 0);
            builder.ins().return_(&[zero]);
            builder.finalize();
        }

        module
            .define_function(func_id, &mut ctx)
            .map_err(|e| CodegenError::Backend(e.to_string()))?;
        module.clear_context(&mut ctx);

        let product = module.finish();
        let bytes = product
            .emit()
            .map_err(|e| CodegenError::Backend(e.to_string()))?;

        Ok(ObjectFile { bytes })
    }
}

fn opt_level_str(level: OptLevel) -> &'static str {
    match level {
        OptLevel::None => "none",
        OptLevel::Speed => "speed",
        OptLevel::SpeedAndSize => "speed_and_size",
    }
}

fn make_puts_signature(module: &ObjectModule) -> cranelift_codegen::ir::Signature {
    let mut sig = module.make_signature();
    sig.params.push(AbiParam::new(module.isa().pointer_type()));
    sig.returns
        .push(AbiParam::new(cranelift_codegen::ir::types::I32));
    sig
}

fn declare_cstring(
    module: &mut ObjectModule,
    module_name: &str,
    index: usize,
    text: &str,
) -> Result<cranelift_module::DataId, CodegenError> {
    let name = format!("{}_str_{}", sanitize_module_name(module_name), index);
    let data_id = module
        .declare_data(&name, Linkage::Local, false, false)
        .map_err(|e| CodegenError::Backend(e.to_string()))?;

    let mut data_ctx = DataDescription::new();
    let mut bytes = text.as_bytes().to_vec();
    bytes.push(0);
    data_ctx.define(bytes.into_boxed_slice());
    module
        .define_data(data_id, &data_ctx)
        .map_err(|e| CodegenError::Backend(e.to_string()))?;
    Ok(data_id)
}

fn select_isa(target: Option<&str>, flags: Flags) -> Result<Arc<dyn isa::TargetIsa>, CodegenError> {
    if let Some(target) = target {
        let triple = target
            .parse::<Triple>()
            .map_err(|e| CodegenError::Unsupported(e.to_string()))?;
        let isa_builder =
            isa::lookup(triple).map_err(|e| CodegenError::Unsupported(e.to_string()))?;
        return isa_builder
            .finish(flags)
            .map_err(|e| CodegenError::Backend(e.to_string()));
    }

    let isa_builder =
        cranelift_native::builder().map_err(|e| CodegenError::Unsupported(e.to_string()))?;
    isa_builder
        .finish(flags)
        .map_err(|e| CodegenError::Backend(e.to_string()))
}

fn sanitize_module_name(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
            out.push(ch);
        }
    }
    if out.is_empty() {
        "ted".to_string()
    } else {
        out
    }
}
