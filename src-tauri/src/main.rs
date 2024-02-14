// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{sync::Mutex, time::Instant};

// use::sysinfo::{System, SystemExt, ProcessExt};
use sysinfo::System;
use tauri::{window, CustomMenuItem, Event, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, UserAttentionType};
use std::{thread, time};
struct AppState {
  system: Mutex<System>,
  client_start: Mutex<Option<Instant>>,
  game_start: Mutex<Option<Instant>>,
}

fn main() {
  let state = AppState {
    system: Mutex::new(System::new()),
    client_start: Mutex::new(None),
    game_start: Mutex::new(None),
  };

  let update_thread = thread::spawn(move || {
    loop {
      print_process_info(&state);
      thread::sleep(time::Duration::from_secs(1));
    }
  });
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide);
  let system_tray = SystemTray::new().with_menu(tray_menu);


  tauri::Builder::default()
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick { tray_id, position, size, .. } => {
        let window = app.get_window("main").unwrap();
        window.show().unwrap();
        window.unminimize().unwrap();
        window.set_focus().unwrap();
      }
      SystemTrayEvent::MenuItemClick {id, ..} => {
        match id.as_str() {
          "quit" => {
            std::process::exit(0);
          }
          "hide" => {
            let window = app.get_window("main").unwrap();
            window.hide().unwrap();
          } 
          _ => {}
        }
      }
      _ => {}
    })
    .on_window_event(|event| match event.event() {
      tauri::WindowEvent::CloseRequested { api, .. } => {
        event.window().hide().unwrap();
        api.prevent_close();
      }
      _ => {}
    })
    // .manage(state)
    // .invoke_handler(tauri::generate_handler![print_process_info])
    .build(tauri::generate_context!())
    .expect("error while building application")
    .run(|_app_handle, event| match event {
      tauri::RunEvent::ExitRequested { api, .. } => {
        api.prevent_exit();
      }
      _ => {}
    })
}

fn print_process_info(state: &AppState){
  let mut system = state.system.lock().unwrap();
  system.refresh_processes();
  let mut client_start = state.client_start.lock().unwrap();
  let mut game_start = state.game_start.lock().unwrap();
  if client_start.is_none() {
    let runs = system.processes_by_exact_name("LeagueClient.exe").count() > 0;
    println!("Client runs: {}", runs);
    if runs {
      *client_start = Some(Instant::now());
      println!("Client started.")
    }
  } else {
    let duration = client_start.unwrap().elapsed();
    println!("Client is running for {:?}", duration);
  }
  if game_start.is_none() {
    let runs = system.processes_by_exact_name("League of Legends.exe").count() > 0;
    println!("Game runs: {}", runs);
    if runs {
      *game_start = Some(Instant::now());
      println!("Game started.")
    }
  }else {
    let duration = game_start.unwrap().elapsed();
    println!("Game is running for {:?}", duration);
  }

}
