use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::api::{BDAddr, Central, Characteristic, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::Peripheral;

use futures::StreamExt;
use my_frame::MyFrame;
use tauri::ipc::{Channel, InvokeResponseBody};
use tauri::{Manager, State};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

mod my_frame;


const MAIN_CHARACTERISTIC: Uuid = uuid_from_u16(0xFFE1);


#[derive(Default)]
struct Settings {
    devices: Vec<Peripheral>,
    connected_device: Option<Peripheral>,
    main_characteristic: Option<Characteristic>,
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

    device.discover_services().await;
    let chars = device.characteristics();
    let main_char = chars.iter().find(|c|
        c.uuid == MAIN_CHARACTERISTIC
    ).unwrap();
    // settings.main_characteristic = Some(main_char.clone());
    device.subscribe(main_char).await;

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
async fn telemetry(state: State<'_, Mutex<Settings>>, on_event: Channel) -> Result<String, ()> {

    let mut settings = state.lock().await;

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: i32 = 0;

    if let Some(connected_device) = &settings.connected_device {

        // connected_device.discover_services().await;
        // let chars = connected_device.characteristics();
        // let main_char = chars.iter().find(|c|
        //     c.uuid == MAIN_CHARACTERISTIC
        // ).unwrap();
        // // settings.main_characteristic = Some(main_char.clone());
        // connected_device.subscribe(main_char).await;

        let mut notif_stream = connected_device.notifications().await.unwrap();
        let mut buffer: [u8; size_of::<MyFrame>()] = [0x00; size_of::<MyFrame>()];
        let mut tail: usize = 0;
        while let Some(data) = notif_stream.next().await {
            for byte in data.value {
                if tail == buffer.len() {
                    let my_frame = MyFrame::from_fixed(&buffer);
                    x += (my_frame.get_x() as i32)/200;
                    y += (my_frame.get_y() as i32)/200;
                    z += (my_frame.get_z() as i32)/200;
                    // println!("{} {} {} ({})", x/50, y/50, z/50, my_frame.to_string());
                    on_event.send(InvokeResponseBody::Raw(format!(
                        "{} {} {} ({})",
                        x/50, y/50, z/50, my_frame.to_string()
                    ).into())).unwrap();
                    tail = 0;
                }
                buffer[tail] = byte;
                tail += 1;
            }
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
