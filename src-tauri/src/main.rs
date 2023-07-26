// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{Manager, State};


struct NewBoxCount {
    count: Mutex<u32>,
}



fn main() {
    tauri::Builder::default()
        
        .manage(NewBoxCount { count: 0.into() })

        .invoke_handler(tauri::generate_handler![
            close_splashscreen,
            add_box,
            // check_box_count,
        ])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]  // must be async so that it doesn't run on the main thread.
async fn close_splashscreen(window: tauri::Window) {
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();  // Close splashscreen window
    }
    window.get_window("main").unwrap().show().unwrap();  // Show main window
}


#[tauri::command]
async fn add_box(new: bool, box_count: State<'_, NewBoxCount>) -> Result<(), ()> {
    let mut count = box_count.count.lock().unwrap();
    *count += 1;
    Ok(())
}


// #[tauri::command]
// async fn check_box_count(noargs: bool, box_count: State<'_, NewBoxCount>) -> Result<u32, ()> {
// // async fn check_box_count(box_count: State<'_, NewBoxCount>) -> Result<u32, ()> {
// // fn check_box_count(box_count: State<'_, NewBoxCount>) -> u32 {
//     let count = box_count.count.lock().unwrap();
//     println!("Current Box Count (Checking): {}", &count);
//     Ok(*count)
//     // *count
// }