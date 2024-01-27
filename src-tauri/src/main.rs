// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use tauri::Manager;
use serde::{Serialize, Deserialize};
use tauri::LogicalSize;
use tauri::Size;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    width: u32,
    height: u32,
    maximize: bool,
    fullscreen: bool
}

fn main() {
    tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![set_configjson])
      .setup(|app| {
        let config = get_configjson();
        let window = app.app_handle().get_window("abematviewer").unwrap();
        window.set_size(Size::Logical(LogicalSize { width: config.width as f64, height: config.height as f64})).unwrap();

        let js_eval = format!(
            r#"
                const {{ appWindow }} = window.__TAURI__.window;

                document.addEventListener('keydown', async function(e) {{
                  console.log(e);
                  if(e.key == 'F5' || (e.ctrlKey && e.key == 'r') || e.key == 'F7') {{
                    e.preventDefault();
                    e.stopPropagation();
                
                  }} else if(e.altKey && e.key == 'Enter') {{
                    appWindow.toggleMaximize();
                
                  }} else if(e.key == 'F11') {{
                    if(await appWindow.isFullscreen()) {{
                      appWindow.setDecorations(true);
                      appWindow.setTitle(true);
                      appWindow.setFullscreen(false);
                    }} else {{
                      appWindow.setDecorations(false);
                      appWindow.setTitle(false);
                      appWindow.setFullscreen(true);
                    }}
                  
                  }} else if(e.key == 'Escape') {{
                    appWindow.setFullscreen(false);
                    appWindow.setDecorations(true);
                    appWindow.setTitle(true);
                  }}
                }});

                document.addEventListener('contextmenu', function(e) {{
                  e.preventDefault();
                  e.stopPropagation();
                }}, false);
              
                document.addEventListener("selectstart", function(e) {{
                  e.preventDefault();
                  e.stopPropagation();
                }}, false);
            "#
        );
    
        window.eval(&js_eval).unwrap();

        Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

#[tauri::command]
fn set_configjson(width: u32, height: u32, maximize: bool, fullscreen: bool) {
    let mut w = width;
    let mut h = height;
    let mut m = maximize;
    let mut f = fullscreen;

    if width == 0 || height == 0 {
        let conf: Config = get_configjson();
        w = conf.width;
        h = conf.height;
        m = conf.maximize;
        f = conf.fullscreen;
    }

    let res = Config {
      width: w,
      height: h,
      maximize: m,
      fullscreen: f
    };

    let json = serde_json::to_string(&res).unwrap();
    let path = get_configpath();
  
    if !Path::new(&path).exists() {
      fs::create_dir_all(&path).expect("do not create directory.");
    }

    let mut f = fs::File::create(&format!("{}\\config.json", &path)).unwrap();
    f.write_all(json.as_bytes()).unwrap();
}

fn get_configjson() -> Config {
  let path = format!("{}\\config.json", get_configpath());
  let file = fs::read_to_string(path);
  let mut res = Config {
    width: 1280,
    height: 720,
    maximize: false,
    fullscreen: false
  };

  match file {
      Ok(val) => {
          let tmp: Config = serde_json::from_str(&val).unwrap();
          res.width = tmp.width;
          res.height = tmp.height;
          res.maximize = tmp.maximize;
          res.fullscreen = tmp.fullscreen;
      },
      Err(_) => {},
  }

  return res;
}

fn get_configpath() -> String {
  return env::temp_dir().to_str().unwrap().to_string() + "abematviewer";
}