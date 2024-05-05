#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod crud;

use crud::data::*;
use crud::note::*;
use crud::status::*;
use crud::tag::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_tables,
            fill_tables,
            tag_insert,
            tag_update,
            tag_delete,
            tag_select,
            status_insert,
            status_update,
            status_delete,
            status_select,
            note_insert,
            note_update,
            note_delete,
            note_select
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
