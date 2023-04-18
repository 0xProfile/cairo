mod utils;

use std::{path::{PathBuf, Path}, fs, sync::Arc, error::Error};
use anyhow::{Context};
use cairo_lang_defs::db::DefsGroup;
use cairo_lang_lowering::ids::ConcreteFunctionWithBodyId;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_sierra_generator::db::SierraGenGroup;
use cairo_lang_sierra_to_casm::{compiler::{CairoProgram, CompilationError}, metadata::calc_metadata};
use cairo_lang_utils::{Upcast, unordered_hash_map::UnorderedHashMap};
use smol_str::SmolStr;
use core::result::Result::Ok;
use cairo_lang_compiler::{db::RootDatabase, compile_cairo_project_at_path, CompilerConfig, compile_prepared_db, project::setup_project};
use cairo_lang_filesystem::{ids::{Directory, CrateLongId, FileLongId, VirtualFile, FileId}, db::{CORELIB_CRATE_NAME, FilesGroup, FilesGroupEx}};
use cairo_lang_sierra::ProgramParser;
use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// pub fn build(&mut self) -> Result<RootDatabase> {
//     // NOTE: Order of operations matters here!
//     //   Errors if something is not OK are very subtle, mostly this results in missing
//     //   identifier diagnostics, or panics regarding lack of corelib items.

//     let mut db = RootDatabase::default();

//     if let Some(cfg_set) = &self.cfg_set {
//         db.use_cfg(cfg_set);
//     }

//     if self.detect_corelib {
//         let path =
//             detect_corelib().ok_or_else(|| anyhow!("Failed to find development corelib."))?;
//         init_dev_corelib(&mut db, path);
//     }

//     if let Some(config) = self.project_config.clone() {
//         update_crate_roots_from_project_config(&mut db, *config.clone());

//         if let Some(corelib) = config.corelib {
//             let core_crate = db.intern_crate(CrateLongId(CORELIB_CRATE_NAME.into()));
//             db.set_crate_root(core_crate, Some(corelib));
//         }
//     }

//     if let Some(precedence) = self.implicit_precedence.clone() {
//         db.set_implicit_precedence(Arc::new(
//             precedence
//                 .into_iter()
//                 .map(|name| get_core_ty_by_name(&db, name.into(), vec![]))
//                 .collect::<Vec<_>>(),
//         ));
//     }

//     if let Some(plugins) = self.plugins.clone() {
//         db.set_semantic_plugins(plugins);
//     }

//     Ok(db)
// }

// #[wasm_bindgen(module = "/readfile.js")]
// extern "C" {
//     #[wasm_bindgen(catch)]
//     fn read_file(path: &str) -> Result<String, JsValue>;
// }

#[wasm_bindgen]
pub fn test_compile() -> String {
    console_error_panic_hook::set_once();
    match compile_cairo_project_at_path(
        &PathBuf::from("/home/cairo-lang/cairo-lang/cairo-lang-compiler-wasm"),
        CompilerConfig { replace_ids: true, ..CompilerConfig::default() },
    ) {
        Ok(_) => {
            return "OK".to_string();
        }
        Err(e) => {
            log(&format!("Error: {}", e.to_string()));
            "Error".to_string()
        }

    }
}


#[wasm_bindgen]
pub struct RootDatabaseWASM (RootDatabase);

#[wasm_bindgen]
pub fn build_db(filename: String, content: String) -> Result<String, String>{
    console_error_panic_hook::set_once();
    let clone_filename = filename.clone();
    let mut db = RootDatabase::default();
    let corelib_path = PathBuf::from("/corelib/src");

    let core_crate = db.intern_crate(CrateLongId(CORELIB_CRATE_NAME.into()));
    let core_root_dir = Directory(corelib_path);

    db.set_crate_root(core_crate, Some(core_root_dir));
    // corelib path /corelib/src



    let root_path = PathBuf::from("/");

    let temp_path = PathBuf::from(filename.clone());
    let mut requested_function_ids = vec![];

    match setup_project(&mut db, temp_path.as_path()) {
        
        Ok(main_crate_ids) => {
            let new_file = db.intern_file(FileLongId::Virtual(VirtualFile {
                parent: Some(FileId::new(&db, root_path.clone())),
                name: SmolStr::new(clone_filename.as_str()),
                content: Arc::new(content),
            }));
            for crate_id in main_crate_ids {
                for module_id in db.crate_modules(crate_id).iter() {
                    let syntax_db:&dyn DefsGroup = db.upcast();
                    let module_file = db.module_main_file(*module_id).unwrap_or_else(|_| {
                        log(format!("Module {:?} has no main file", module_id).as_str());
                        return FileId::new(&db, root_path.clone());
                    });
                    log(&format!("Module file: {:?}", module_file));
                    // log(&format!("Module file content: {:?}", db.file_content(module_file).unwrap()));
                    // let file_syntax = db.file_syntax(module_file).unwrap();
                    // let temp = db.module_free_functions(*module_id);
                    // match temp {
                    //     Ok(temp) => {
                    //         for (free_func_id, _) in temp {
                    //             if let Some(function) =
                    //                 ConcreteFunctionWithBodyId::from_no_generics_free(db.upcast(), free_func_id)
                    //             {
                    //                 requested_function_ids.push(function)
                    //             }
                    //         }
                    //     }
                    //     Err(e) => {
                    //         log(&format!("Error:"));
                    //         return Err("Error".to_string());
                    //     }
                    // }
                    return Ok("OK".to_string());
                }
            }
            let p = db.get_sierra_program_for_functions(requested_function_ids);
            match p {
                Ok(program) => {
                    log("OK");
                    Ok(program.to_string())
                }
                Err(e) => {
                    log(&format!("Error in compile:"));
                    return Err("Error".to_string());
                }
            }
            // 
        }
        Err(e) => {
            log(&format!("Project Error: {}", e.to_string()));
            return Err("Project Error".to_string());
        }
    }
}

#[wasm_bindgen]
pub fn read_db(db: &RootDatabaseWASM) -> String {
    #[cfg(wasmpack_target="nodejs")]
    log("from nodejs");
    #[cfg(wasmpack_target="browser")]
    log("from browser");

    console_error_panic_hook::set_once();
    // db.0.crate_roots().iter().next().unwrap().1.0.to_str().unwrap().to_string();
    // match db.0.crate_roots().iter().next() {
    //     Some((_, dir)) => dir.0.to_str().unwrap().to_string(),
    //     None => "None".to_string()        
    // }

    // match read_file("./index.html") {
    //     Ok(buffer) => {
    //         let s = buffer.as_str();
    //         s.to_string()
    //     }
    //     Err(e) => {
    //         log(format!("Error: {}", e.as_string().unwrap()).as_str());
    //         "Error".to_string()
    //     }
    // }
    return "Hello".to_string();
}


#[wasm_bindgen]
pub fn parser(sierra_code: String) -> String {
    console_error_panic_hook::set_once();

    match ProgramParser::new().parse(&sierra_code) {
        Ok(program) => {
            // log("Parsing successful");
            let gas_usage_check = true;
            let cairo_program = cairo_lang_sierra_to_casm::compiler::compile(
                &program,
                &calc_metadata(&program, Default::default())
                    .with_context(|| "Failed calculating Sierra variables.").unwrap_or_else(|e| {
                        log(format!("Error: {}", e).as_str());
                        panic!("Error: {}", e)
                    }),
                gas_usage_check,
            )
            .with_context(|| "Compilation failed.");
        
            return format!("{:?}", cairo_program);
        }
        Err(e) => {
            log(format!("Error: {}", e).as_str());
            return "Error".to_string();
        }
    }


}
// init_logging(log::LevelFilter::Off);
// log::info!("Starting Sierra compilation.");



// fs::write(args.output, format!("{cairo_program}")).with_context(|| "Failed to write output.")