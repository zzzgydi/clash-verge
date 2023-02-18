#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmds;
mod config;
mod core;
mod enhance;
mod feat;
mod utils;
mod deep_link;
use std::sync::{Arc, Mutex};

use crate::utils::{init, resolve, server, help};
use crate::core::handle::Handle;
use tauri::{api, SystemTray, Manager};
use once_cell::sync::Lazy;
use std::thread;
use help::focus_to_main_window_if_needed;

// This is not the best way to do it...
static mut NEED_WINDOW_BE_FOCUS:Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));

#[tokio::main]
async fn main() -> std::io::Result<()> {

    // Deep linking
    deep_link::prepare("app.clashverge");
    // Define handler
    let handler = | deep_link | async move {
        // Set need to be focus to true, it's handled in other thread
        unsafe{
            *crate::NEED_WINDOW_BE_FOCUS.lock().unwrap() = true;
        }
        // Convert deep link to something that import_profile can use
        let profile_url_and_name = help::convert_deeplink_to_url_for_import_profile(&deep_link);
        // If deep link is invalid, we pop up a message to user
        if profile_url_and_name.is_err(){
            Handle::notice_message("set_config::error", "Profile url is invalid");
            return
        }
        
        // Import profile
        let import_result = cmds::import_profile(profile_url_and_name.unwrap(), None).await;
        // If we couldn't import profile& we pop up a message to user
        if import_result.is_err(){
            Handle::notice_message("set_config::error",format!("Profile url is invalid | {}", import_result.err().unwrap()));
            return
        }
        Handle::notice_message("set_config::ok", "Profile added.");
    };
    // Register "clash" scheme
    let  deep_link_register_result = deep_link::register("clash",handler.clone()).await;
    // If we couldn't register, we log it
    if deep_link_register_result.is_err(){
        println!("We can't register \"clash\" scheme for program | {}",deep_link_register_result.err().unwrap())
    }
    // Register "clashmeta" scheme
    // deep_link_register_result = deep_link::register("clashmeta", handler).await;
    // If we couldn't register, we log it
    // if deep_link_register_result.is_err(){
    //     println!("We can't register \"clashmeta\" scheme for program | {}",deep_link_register_result.err().unwrap())
    // }

    // 单例检测
    if server::check_singleton().is_err() {
        println!("app exists");
        return Ok(());
    }

    crate::log_err!(init::init_config());

    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .system_tray(SystemTray::new())
        .setup(|app| Ok(resolve::resolve_setup(app)))
        .on_system_tray_event(core::tray::Tray::on_system_tray_event)
        .invoke_handler(tauri::generate_handler![
            // common
            cmds::get_sys_proxy,
            cmds::open_app_dir,
            cmds::open_logs_dir,
            cmds::open_web_url,
            cmds::open_core_dir,
            // cmds::kill_sidecar,
            cmds::restart_sidecar,
            // clash
            cmds::get_clash_info,
            cmds::get_clash_logs,
            cmds::patch_clash_config,
            cmds::change_clash_core,
            cmds::get_runtime_config,
            cmds::get_runtime_yaml,
            cmds::get_runtime_exists,
            cmds::get_runtime_logs,
            // verge
            cmds::get_verge_config,
            cmds::patch_verge_config,
            // cmds::update_hotkeys,
            // profile
            cmds::get_profiles,
            cmds::enhance_profiles,
            cmds::patch_profiles_config,
            cmds::view_profile,
            cmds::patch_profile,
            cmds::create_profile,
            cmds::import_profile,
            cmds::update_profile,
            cmds::delete_profile,
            cmds::read_profile_file,
            cmds::save_profile_file,
            // service mode
            cmds::service::check_service,
            cmds::service::install_service,
            cmds::service::uninstall_service,
        ]);

    #[cfg(target_os = "macos")]
    {
        use tauri::{Menu, MenuItem, Submenu};

        builder = builder.menu(
            Menu::new().add_submenu(Submenu::new(
                "Edit",
                Menu::new()
                    .add_native_item(MenuItem::Undo)
                    .add_native_item(MenuItem::Redo)
                    .add_native_item(MenuItem::Copy)
                    .add_native_item(MenuItem::Paste)
                    .add_native_item(MenuItem::Cut)
                    .add_native_item(MenuItem::SelectAll)
                    .add_native_item(MenuItem::CloseWindow)
                    .add_native_item(MenuItem::Quit),
            )),
        );
    }

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    
    // Focus thread
    let app_handle = app.app_handle();
    thread::spawn(move ||{focus_to_main_window_if_needed(&app_handle)});

    app.run(|app_handle, e| match e {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        tauri::RunEvent::Exit => {
            resolve::resolve_reset();
            api::process::kill_children();
            app_handle.exit(0);
        }
        #[cfg(target_os = "macos")]
        tauri::RunEvent::WindowEvent { label, event, .. } => {
            use tauri::Manager;

            if label == "main" {
                match event {
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        app_handle.get_window("main").map(|win| {
                            let _ = win.hide();
                        });
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });

    Ok(())
}
