use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};

use tokio::time::{sleep, Duration};


#[tauri::command]
async fn ble() -> Result<Vec<String>, ()> {
    let manager = Manager::new().await.unwrap();
    let adapters = manager.adapters().await.unwrap();
    if adapters.is_empty() {
        return Ok(vec![String::from("no adapters (BL is off or not supported)")]);
    }
    // let mut adapters_info = Vec::<String>::new();
    // for adapter in adapters.iter() {
    //     adapters_info.push(adapter.adapter_info().await.unwrap());
    // }
    // Ok(adapters_info)


    let main_adapter = &adapters[0];
    main_adapter.start_scan(ScanFilter::default()).await;

    sleep(Duration::from_secs(2)).await;
    let peripherals = main_adapter.peripherals().await.unwrap();
    if peripherals.is_empty() {
        return Ok(vec![String::from("no peripherals")]);
    }
    
    let mut names = Vec::<String>::new();
    for peripheral in peripherals.iter() {
        let props_opt = peripheral.properties().await.unwrap();
        if let Some(props) = props_opt {
            names.push(props.local_name.unwrap_or(String::from("unknown")));
        }
    }
    // peripherals[0].discover_services().await;
    // let chars = peripherals[0].characteristics();
    // if chars.is_empty() {
    //     return Ok(vec![String::from("no chars")]);
    // }
    // let mut chars_str = Vec::<String>::new();
    // for charec in chars.iter() {
    //     chars_str.push(charec.to_string());
    // }
    // // chars.iter().find(|c| c.str)
    

    Ok(names)
}

#[tauri::command]
async fn pair() {

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![ble, pair])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
