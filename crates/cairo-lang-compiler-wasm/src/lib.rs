mod utils;

use std::{path::{PathBuf, Path}, fs};

use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_filesystem::{ids::{Directory, CrateLongId}, db::{CORELIB_CRATE_NAME, FilesGroup, FilesGroupEx}};
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
pub struct RootDatabaseWASM (RootDatabase);

#[wasm_bindgen]
pub fn build_db(path: String) ->  RootDatabaseWASM {
    let mut db = RootDatabase::default();
    let core_crate = db.intern_crate(CrateLongId(CORELIB_CRATE_NAME.into()));
    let core_root_dir = Directory(PathBuf::from(path));
    db.set_crate_root(core_crate, Some(core_root_dir));
    RootDatabaseWASM(db)
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
