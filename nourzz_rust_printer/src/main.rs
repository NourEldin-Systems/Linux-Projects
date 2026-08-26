use notify::{EventKind, RecursiveMode, Result, Watcher};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use windows_sys::Win32::Graphics::Gdi::{
    BI_RGB, BITMAPINFOHEADER, CreateDCW, DIB_RGB_COLORS, DeleteDC, SRCCOPY, StretchDIBits,
};
use windows_sys::Win32::Storage::Xps::{DOCINFOW, EndDoc, EndPage, StartDocW, StartPage};

const WATCHED_FOLDER_PATH: &str = "C:\\Users\\Osama\\Desktop\\WatchFolder";
const TARGET_PRINTER_NAME: &str = "HP LaserJet Professional P1102 (Copy 1)";

fn main() -> Result<()> {
    // Ensure the folder exists before monitoring it
    fs::create_dir_all(WATCHED_FOLDER_PATH)?;
    println!(
        "Daemon online. Monitoring pipeline at: {}",
        WATCHED_FOLDER_PATH
    );

    // 1. Set up a message-passing channel so the file system can talk to our loop
    let (transmitter, receiver) = std::sync::mpsc::channel();
    let mut folder_watcher = notify::recommended_watcher(transmitter)?;

    // Tell the watcher to target our specific folder path
    folder_watcher.watch(Path::new(WATCHED_FOLDER_PATH), RecursiveMode::NonRecursive)?;

    // 2. Continuous event monitoring loop
    for folder_update_result in receiver {
        match folder_update_result {
            Ok(file_system_event) => {
                // We only care when a file is newly dropped (Created) or completely written (Modified)
                if let EventKind::Create(_) | EventKind::Modify(_) = file_system_event.kind {
                    // Loop through all file paths involved in this folder update
                    for detected_file_path in file_system_event.paths {
                        if let Some(isolated_file_name) = detected_file_path.file_name() {
                            // Convert the filename OS string into a standard readable Rust string slice
                            if let Some(file_name_string_slice) = isolated_file_name.to_str() {
                                // DYNAMIC MATCHING: Catch any file that starts with "report" and ends with ".png"
                                if file_name_string_slice.starts_with("report")
                                    && file_name_string_slice.ends_with(".png")
                                {
                                    println!(
                                        "Target sequence payload detected: {}! Processing...",
                                        file_name_string_slice
                                    );

                                    // Give the Windows OS a tiny fraction of a second to completely
                                    // finish writing the file to disk and release its hardware handle
                                    std::thread::sleep(Duration::from_millis(100));

                                    // Create a unique status feedback file name tied to this specific report
                                    // e.g., "report1_status.txt", "report2_status.txt"
                                    let base_name = file_name_string_slice
                                        .strip_suffix(".png")
                                        .unwrap_or("report");
                                    let success_log_name = format!("{}_success.txt", base_name);
                                    let error_log_name = format!("{}_error.txt", base_name);

                                    match process_and_send_to_printer(&detected_file_path) {
                                        Ok(_) => {
                                            write_status_feedback_file(
                                                &success_log_name,
                                                "Print job sent to driver cache successfully.",
                                            );
                                            let _ = fs::remove_file(&detected_file_path);
                                            // Clear image for the next print run
                                        }
                                        Err(error_message) => {
                                            write_status_feedback_file(
                                                &error_log_name,
                                                &format!("Pipeline failure: {}", error_message),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(watcher_error) => println!("Watcher event error context: {:?}", watcher_error),
        }
    }

    Ok(())
}

// Our low-overhead Win32 printing engine extracted into a clear function
fn process_and_send_to_printer(image_file_path: &Path) -> std::result::Result<(), String> {
    // Decode the image using the Rust image crate
    let decoded_image = image::open(image_file_path)
        .map_err(|err| format!("Failed to decode image layout: {}", err))?
        .to_rgba8();

    let (image_width, image_height) = decoded_image.dimensions();
    let raw_pixel_bytes = decoded_image.as_raw();

    unsafe {
        let subsystem_driver_name: Vec<u16> = "WINSPOOL\0".encode_utf16().collect();
        let printer_name_wide: Vec<u16> =
            TARGET_PRINTER_NAME.encode_utf16().chain(Some(0)).collect();

        // Open a Device Context (DC) handle straight to the HP driver pipeline
        let hardware_device_context = CreateDCW(
            subsystem_driver_name.as_ptr(),
            printer_name_wide.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
        );

        if hardware_device_context == std::ptr::null_mut() {
            return Err("Could not connect to printer driver channel.".to_string());
        }

        let document_title_wide: Vec<u16> = "DaemonAutomatedPrint\0".encode_utf16().collect();
        let document_metadata_config = DOCINFOW {
            cbSize: std::mem::size_of::<DOCINFOW>() as i32,
            lpszDocName: document_title_wide.as_ptr(),
            lpszOutput: std::ptr::null(),
            lpszDatatype: std::ptr::null(),
            fwType: 0,
        };

        // Describe the layout of our raw pixels so the Windows GDI layer can read it
        let bitmap_info_header = BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: image_width as i32,
            biHeight: -(image_height as i32), // Negative height enforces an upright top-down image
            biPlanes: 1,
            biBitCount: 32, // 32-bit RGBA channel matching our .to_rgba8() output
            biCompression: BI_RGB as u32,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        };

        // Execute the native sequential Win32 spooler printing stream
        if StartDocW(hardware_device_context, &document_metadata_config) > 0 {
            StartPage(hardware_device_context);

            // Blast our raw pixel array directly over the driver canvas context
            StretchDIBits(
                hardware_device_context,
                0,
                0,
                image_width as i32,
                image_height as i32,
                0,
                0,
                image_width as i32,
                image_height as i32,
                raw_pixel_bytes.as_ptr() as *const _,
                &bitmap_info_header as *const _ as *const _,
                DIB_RGB_COLORS,
                SRCCOPY,
            );

            EndPage(hardware_device_context);
            EndDoc(hardware_device_context);
        }

        // Clean up the device context handle from system memory
        DeleteDC(hardware_device_context);
    }
    Ok(())
}

// Drop a clean feedback status file inside the watch folder
fn write_status_feedback_file(status_filename: &str, status_message: &str) {
    let output_file_path = Path::new(WATCHED_FOLDER_PATH).join(status_filename);
    if let Ok(mut status_file) = File::create(output_file_path) {
        let _ = status_file.write_all(status_message.as_bytes());
    }
}
