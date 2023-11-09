use ext_php_rs::{
    builders::ModuleBuilder,
    ffi::display_ini_entries,
    info_table_end,
    info_table_row,
    info_table_start,
    prelude::*,
    zend::ModuleEntry,
};

#[php_startup]
fn startup_function(ty: i32, module_number: i32) {
    // let ini_entries: Vec<IniEntryDef> = vec![
        // IniEntryDef::new(
        //     "your_project.some_var".to_owned(),
        //     "1000".to_owned(),
        //     IniEntryPermission::All,
        // ),
    // ];
    // IniEntryDef::register(ini_entries, module_number);
}

extern "C" fn request_startup(_ty: i32, _module_number: i32) -> i32 {
    0
}

extern "C" fn request_shutdown(_ty: i32, _module_number: i32) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn php_module_info(module: *mut ModuleEntry) {
    info_table_start!();
    info_table_row!("Version", env!("CARGO_PKG_VERSION"));
    info_table_end!();

    unsafe { display_ini_entries(module) };
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    let module = module
        .info_function(php_module_info)
        .request_startup_function(request_startup)
        .request_shutdown_function(request_shutdown);
    module
}
