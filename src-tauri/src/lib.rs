use btleplug::api::{BDAddr, Central, Characteristic, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::Peripheral;

use tauri::{Manager, State};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};


#[derive(Default)]
struct Settings {
    devices: Vec<Peripheral>,
    connected_device: Option<Peripheral>,
}

#[tauri::command]
async fn scan(state: State<'_, Mutex<Settings>>) -> Result<Vec<(String, String)>, ()> {

    let mut state = state.lock().await;

    let manager = btleplug::platform::Manager::new().await.unwrap();
    let adapters = manager.adapters().await.unwrap();
    if adapters.is_empty() {
        return Err(());
    }

    let main_adapter = &adapters[0];
    main_adapter.start_scan(ScanFilter::default()).await;

    // temporary
    sleep(Duration::from_secs(2)).await;
    state.devices = main_adapter.peripherals().await.unwrap();
    if state.devices.is_empty() {
        return Err(());
    }
    
    let mut names_addresses = Vec::<(String, String)>::new();
    for peripheral in state.devices.iter() {
        let props_opt = peripheral.properties().await.unwrap();
        if let Some(props) = props_opt {
            names_addresses.push(
                (props.local_name.unwrap_or(String::from("unknown")),
                props.address.to_string())
            );
        }
    }

    Ok(names_addresses)
}

#[tauri::command]
async fn connect(state: State<'_, Mutex<Settings>>, addr_str: String) -> Result<(), ()> {

    let mut settings = state.lock().await;

    if let Some(connected_device) = &settings.connected_device {
        // if connected_device.address().to_string() == addr_str {
        //     return Err(());
        // } else {
        //     disconnect(state);
        // }
        return Err(());
    }

    let device = settings.devices.iter().find(|p|
        p.address() == BDAddr::from_str_delim(&addr_str).unwrap()
    ).unwrap();

    if let Err(_) = device.connect().await {
        return Err(());
    }
    settings.connected_device = Some(device.clone());   // not ideal :/
    Ok(())
}

#[tauri::command]
async fn disconnect(state: State<'_, Mutex<Settings>>) -> Result<(), ()> {

    let mut settings = state.lock().await;

    if let Some(connected_device) = &settings.connected_device {
        if let Err(_) = connected_device.disconnect().await {
            return Err(());
        }
        settings.connected_device = None;
        Ok(())
    } else {
        Err(())
    }
}

#[tauri::command]
async fn telemetry(state: State<'_, Mutex<Settings>>) -> Result<String, ()> {

    let mut settings = state.lock().await;

    if let Some(connected_device) = &settings.connected_device {

        connected_device.discover_services().await;
        let chars = connected_device.characteristics();
        
        for characteristic in chars {
            println!("{}", characteristic);
        }
        Ok("".into())
    } else {
        Err(())
    }
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(Settings::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            scan,
            connect,
            disconnect,
            telemetry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
