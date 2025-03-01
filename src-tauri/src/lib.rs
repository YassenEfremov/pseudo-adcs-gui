use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::api::{BDAddr, Central, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::Peripheral;

use futures::StreamExt;
use pseudo_adcs_protocol::message::{Message, MessagePayload, PushState, TEL};
use tauri::ipc::{Channel, InvokeResponseBody};
use tauri::{Manager, State};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use uuid::Uuid;


const NOTIFY_WRITE_CHARAC_UUID: Uuid = uuid_from_u16(0xFFE1);


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
                (props.local_name.unwrap_or(String::new()),
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
        if connected_device.address().to_string() == addr_str {
            return Err(());
        }// else {
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
    let characteristics = device.characteristics();
    let notify_write_charac: &Characteristic;
    if let Some(charac) = characteristics.iter().find(|c|
        c.uuid == NOTIFY_WRITE_CHARAC_UUID
    ) {
        notify_write_charac = charac;
        device.subscribe(notify_write_charac).await;
    } else {
        device.disconnect().await;
        return Err(())
    }

    settings.connected_device = Some(device.clone());
    settings.main_characteristic = Some(notify_write_charac.clone());

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

    let mut notif_stream;
    
    {
        let settings = state.lock().await;
        if let Some(connected_device) = &settings.connected_device {
            notif_stream = connected_device.notifications().await.unwrap();
        } else {
            return Err(());
        }
    }

    let mut message = Message::new();

    while let Some(data) = notif_stream.next().await {
        let settings = state.lock().await;
        if let None = settings.connected_device {
            // device could be disconnected while we wait for data
            break;
        }
        for byte in data.value {
            if let PushState::Done = message.push_byte(byte) {
                if let Some(MessagePayload::TEL(ref tel_payload)) = message.payload {
                    // println!("{} {} {} ()", x/20, y/20, z/20, /*my_frame.to_string()*/);
                    on_event.send(InvokeResponseBody::Raw(format!(r#"
                        {{
                            "x": {{
                                "angle": {},
                                "acc": {}
                            }},
                            "y": {{
                                "angle": {},
                                "acc": {}
                            }},
                            "z": {{
                                "angle": {},
                                "acc": {}
                            }}
                        }}"#,
                        tel_payload.get_x(), tel_payload.get_x_acc(),
                        tel_payload.get_y(), tel_payload.get_y_acc(),
                        tel_payload.get_z(), tel_payload.get_z_acc()
                    ).into())).unwrap();
                }
            }
        }
    }
    Ok("".into())
}

#[tauri::command]
async fn set_attitude(state: State<'_, Mutex<Settings>>, new_x: i32, new_y: i32, new_z: i32) -> Result<(), String> {

    let settings = state.lock().await;
    if let Some(connected_device) = &settings.connected_device {
        if let Some(charac) = &settings.main_characteristic {
            let bytes = [
                0x02,
                (new_x as i16).to_be_bytes()[0], (new_x as i16).to_be_bytes()[1],
                (new_y as i16).to_be_bytes()[0], (new_y as i16).to_be_bytes()[1],
                (new_z as i16).to_be_bytes()[0], (new_z as i16).to_be_bytes()[1],
            ];
            println!("sending: {:?}", bytes);
            // connected_device.write(&charac, &[0x02], WriteType::WithoutResponse).await;
            connected_device.write(&charac, &bytes, WriteType::WithoutResponse).await;
        }
    } else {
        return Err("".into());
    }

    Ok(())
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
            set_attitude
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
