// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, io};
use std::fs::{File, read_dir, write};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] io::Error)
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}


#[tauri::command]
fn create_file(filename: String) -> Result<(), Error> {
    File::create(filename.to_string())?;
    Ok(())
}

#[tauri::command]
fn create_folder(folder: String) -> Result<(), Error> {
    fs::create_dir(folder)?;
    Ok(())
}

#[tauri::command]
fn write_to_file(filename: String, content: String) -> Result<(), Error> {
    write(filename, content)?;
    Ok(())
}

#[tauri::command]
fn get_files_from_dir(dir: String) -> Result<Vec<String>, Error> {
    let entries = read_dir(dir)?
        .map(|res| res.map(|e| format!("{}", e.path().display())))
        .collect::<Result<Vec<_>, io::Error>>()?;
    Ok(entries)
}

#[tauri::command]
fn read_file_content(filename: String) -> Result<String, Error> {
    let text = fs::read_to_string(filename)?;
    Ok(text)
}

#[tauri::command]
fn check_if_dir(filename: String) -> Result<bool, Error> {
    let metadata = fs::metadata(filename)?;
    Ok(metadata.file_type().is_dir())
}

#[tauri::command]
fn delete_file(filename: String) -> Result<(), Error> {
    fs::remove_file(filename)?;
    Ok(())
}

#[tauri::command]
fn delete_folder(folder: String) -> Result<(), Error> {
    fs::remove_dir(folder)?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_file,
            write_to_file,
            get_files_from_dir,
            read_file_content,
            check_if_dir,
            create_folder,
            delete_file,
            delete_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
