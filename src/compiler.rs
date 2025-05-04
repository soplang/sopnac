use cranelift::prelude::*;
use cranelift_module::{Module, Linkage, DataDescription};   // DataDescription is the new API
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;      // add `tempfile = "3"` to Cargo.toml

pub fn compile_and_link(code: &str, exe_name: &str) -> Result<()> {
    // 1. create a temp object file
    let obj_file = NamedTempFile::new()?;
    let obj_path = obj_file.path().to_str().unwrap().to_owned();

    // 2. generate machine code into that object
    compile_to_object(code, &obj_path)?;

    // 3. invoke system linker (cc) -> final binary
    Command::new("cc")
        .args(&[&obj_path, "-o", exe_name])
        .status()?;

    // 4. temp file auto‑deleted when `obj_file` drops
    Ok(())
}

pub fn compile_to_object(input: &str, obj_path: &str) -> Result<()> {
    /* ---------- parse: qor("Text") ---------- */
    let trimmed = input.trim();
    let content = if trimmed.starts_with("qor(\"") && trimmed.ends_with("\")") {
        &trimmed[5..trimmed.len() - 2]
    } else {
        panic!("Only qor(\"text\") is supported for now.");
    };

    /* ---------- ISA / module setup ---------- */
    let isa = cranelift::codegen::isa::lookup(Triple::host())?
        .finish(cranelift::codegen::settings::Flags::new(
            cranelift::codegen::settings::builder(),
        ))?;

    let builder =
        ObjectBuilder::new(isa, "sopnac", cranelift_module::default_libcall_names())?;
    let mut module = ObjectModule::new(builder);

    /* ---------- data section: embed string literal ---------- */
    let mut data_desc = DataDescription::new();
    data_desc.define(format!("{}\0", content).into_bytes().into_boxed_slice());
    let str_id = module.declare_data("qor_str", Linkage::Local, false, false)?;
    module.define_data(str_id, &data_desc)?;

    /* ---------- build `main` function ---------- */
    let mut ctx = module.make_context();
    let mut func_ctx = FunctionBuilderContext::new();
    ctx.func.signature.returns.push(AbiParam::new(types::I32));

    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);
    let entry = builder.create_block();
    builder.switch_to_block(entry);
    builder.seal_block(entry);

    let ptr_ty = module.target_config().pointer_type();
    let str_gv = module.declare_data_in_func(str_id, builder.func);
    let str_ptr = builder.ins().symbol_value(ptr_ty, str_gv);

    /* declare and call `puts` */
    let mut puts_sig = module.make_signature();
    puts_sig.params.push(AbiParam::new(ptr_ty));
    puts_sig.returns.push(AbiParam::new(types::I32));

    let puts_id = module.declare_function("puts", Linkage::Import, &puts_sig)?;
    let puts_ref = module.declare_func_in_func(puts_id, builder.func);

    builder.ins().call(puts_ref, &[str_ptr]);

    // separate creation to avoid double mutable borrow
    let zero = builder.ins().iconst(types::I32, 0);
    builder.ins().return_(&[zero]);

    builder.finalize();

    /* ---------- finalize & emit object ---------- */
    let main_id = module.declare_function("main", Linkage::Export, &ctx.func.signature)?;
    module.define_function(main_id, &mut ctx)?;
    let obj = module.finish();
    let bytes = obj.emit()?;

    File::create(output)?.write_all(&bytes)?;
    Ok(())
}
