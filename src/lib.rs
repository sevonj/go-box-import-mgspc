mod convert;
mod error;
mod types;
mod util;

wit_bindgen::generate!({
    world: "plugin",
    path: "go_box_importer.wit",
});

struct GoBoxImportTemplate;

impl Guest for GoBoxImportTemplate {
    fn get_id() -> String {
        "gobox.import-template".to_string()
    }

    fn get_name() -> String {
        "Go-Box Importer Template".to_string()
    }

    fn get_description() -> String {
        "Rust content importer template".to_string()
    }
}

export!(GoBoxImportTemplate);
