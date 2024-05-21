use rspotify::{prelude::*, AuthCodeSpotify};

fn extract_id(id_option: Option<String>) -> Option<String> {
    match id_option {
        Some(id) => Some(id),
        None => None, // You can handle this case differently if needed
    }
}

pub async fn get_device_id(spotify: &AuthCodeSpotify) -> Result<String, &'static str> {
    println!("Getting available devices");
    let devices = spotify.device().await;

    // println!("Response: {devices:?}");

    if let Ok(ref device_list) = devices {
        if device_list.is_empty() {
            println!("Warning: No devices found");
            return Err("Warning: No devices found");
        }

        println!("Device list: {:?}", device_list);

        /* just for debug other device but spotifyd
          if let Some(second_device) = device_list.get(1) {
             println!("Second device: {:?}", second_device);
             println!("Second device name: {:?}", second_device.name);
             println!("Second device id: {:?}", second_device.id);
             if let Some(id) = extract_id(second_device.id.clone()) {
                 println!("second ID: {}", id);

                 return Ok(id);
             }
          }
        */

        if let Some(first_device) = device_list.first() {
            println!("First device name: {:?}", first_device.name);
            println!("First device id: {:?}", first_device.id);

            if first_device.name.contains("Spotifyd") {
                if let Some(id) = extract_id(first_device.id.clone()) {
                    println!("ID: {}", id);

                    return Ok(id);
                }

                Err("No ID found")
            } else {
                // return the first device but spotifyd
                if let Some(id) = extract_id(first_device.id.clone()) {
                    println!("ID: {}", id);

                    Ok(id)
                } else {
                    println!("Warning: No Spotifyd devices found");
                    Err("Warning: No Spotifyd devices found")
                }
            }
        } else {
            println!("No devices found");
            Err("No devices found")
        }
    } else {
        println!("Failed to retrieve devices");
        Err("Failed to retrieve devices")
    }
}
