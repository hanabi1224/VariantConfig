pub mod fn_translator;
mod frontend;
mod utils;
mod variant_value;

use ahash::RandomState;
use anyhow;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use fn_translator::*;
use frontend::*;
use hashbrown::HashMap;
use std::mem;
pub use variant_value::*;

const N_PARAMS: usize = 20;
// TODO: Use macro instead
pub type FnSignature = fn(
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
) -> bool;
pub const BOOL: Type = types::B1;
pub const INT: Type = types::I64;

pub struct FnJitter {
    module: JITModule,
    random_state: RandomState,
    params: HashMap<String, Variable>,
    pub func: FnSignature,
}

impl FnJitter {
    pub fn new(input: &str) -> anyhow::Result<Self> {
        let builder = JITBuilder::new(cranelift_module::default_libcall_names());
        let mut module = JITModule::new(builder);
        let mut ctx = module.make_context();
        let random_state = RandomState::new();
        let mut params = HashMap::with_capacity(N_PARAMS);
        let fn_ptr = Self::compile(input, &mut module, &mut ctx, &random_state, &mut params)
            .map_err(|e| {
                anyhow::anyhow!(format!("Unable to parse condition `{}` :{}", input, e))
            })?;
        Ok(Self {
            module: module,
            random_state,
            params,
            func: unsafe { mem::transmute::<_, FnSignature>(fn_ptr) },
        })
    }

    pub fn evaluate(&self, params: &HashMap<String, VariantValue>) -> bool {
        let mut a: [i64; N_PARAMS] = [0; N_PARAMS];
        for (k, idx) in &self.params {
            if let Some(v) = params.get(k) {
                a[idx.index()] = v.to_i64(&self.random_state);
            }
        }

        (self.func)(
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13],
            a[14], a[15], a[16], a[17], a[18], a[19],
        )
    }

    fn compile(
        input: &str,
        module: &mut JITModule,
        ctx: &mut codegen::Context,
        random_state: &RandomState,
        params: &mut HashMap<String, Variable>,
    ) -> anyhow::Result<*const u8> {
        let stmts = parser::statements(&input).map_err(|e| e)?;
        for _ in 0..N_PARAMS {
            ctx.func.signature.params.push(AbiParam::new(INT));
        }
        ctx.func.signature.returns.push(AbiParam::new(BOOL));

        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_context);

        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        let mut return_value = ValueWrapper::new(builder.ins().bconst(BOOL, false), BOOL);
        let mut trans = FunctionTranslator::new(random_state, &mut builder, params, entry_block);
        for expr in stmts {
            return_value = trans.translate_expr(expr)?;
        }
        return_value = trans.convert_int_to_bool(return_value);
        trans.return_and_finalize(return_value.value);

        let id = module
            .declare_function("fn", Linkage::Export, &ctx.func.signature)
            .map_err(|e| e)?;
        module
            .define_function(
                id,
                ctx,
                &mut codegen::binemit::NullTrapSink {},
                &mut codegen::binemit::NullStackMapSink {},
            )
            .map_err(|e| e)?;
        module.clear_context(ctx);
        module.finalize_definitions();
        let code = module.get_finalized_function(id);
        Ok(code)
    }

    pub unsafe fn free_memory(self) {
        self.module.free_memory();
    }
}
