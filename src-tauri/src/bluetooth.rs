use bluer::{Adapter, AdapterEvent, Device};
use futures_util::{pin_mut, StreamExt};
use std::time::Duration;
use tokio::time::timeout;

async fn get_adapter() -> Result<Adapter, bluer::Error> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;
    Ok(adapter)
}

fn is_airpods_name(name: &str) -> bool {
    name.contains("AirPods")
}

async fn get_device_name(device: &Device) -> Option<String> {
    if let Ok(Some(name)) = device.name().await {
        return Some(name);
    }
    if let Ok(alias) = device.alias().await {
        if !alias.contains(':') {
            return Some(alias);
        }
    }
    None
}

<<<<<<< HEAD
async fn find_paired_airpods(adapter: &Adapter) -> Result<Option<Device>, bluer::Error> {
    let device_addresses = adapter.device_addresses().await?;
=======
pub async fn wait_for_airpods(_start_state: u8) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    println!("Scanning for AirPods in pairing mode...");
>>>>>>> 8acb368 (final)

    println!("Known devices: {} total", device_addresses.len());

<<<<<<< HEAD
    for addr in device_addresses {
        let device = adapter.device(addr)?;
        let paired = device.is_paired().await.unwrap_or(false);

        if let Some(name) = get_device_name(&device).await {
            println!("  Device: {} ({}) paired={}", name, addr, paired);
            if is_airpods_name(&name) && paired {
                return Ok(Some(device));
=======
        let device_addresses = adapter.device_addresses().await?;
        for addr in device_addresses {
            let device = adapter.device(addr)?;
            if let Some(name) = get_device_name(&device).await {
                if is_airpods_name(&name) {
                    let connected = device.is_connected().await.unwrap_or(false);
                    if !connected {
                        println!("  Removing cached device: {} ({})", name, addr);
                        let _ = adapter.remove_device(addr).await;
                    }
                }
>>>>>>> 8acb368 (final)
            }
        } else {
            println!("  Device: <no name> ({}) paired={}", addr, paired);
        }
    }

    Ok(None)
}

pub async fn wait_for_airpods() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let adapter = get_adapter().await?;

    if let Some(device) = find_paired_airpods(&adapter).await? {
        let name = get_device_name(&device).await.unwrap_or_else(|| "AirPods".to_string());
        return Ok(name);
    }

    println!("No paired AirPods found in known devices, scanning...");
    let discover = adapter.discover_devices().await?;
    pin_mut!(discover);

    let scan_duration = Duration::from_secs(30);

    let result = timeout(scan_duration, async {
        while let Some(event) = discover.next().await {
            if let AdapterEvent::DeviceAdded(addr) = event {
                if let Ok(device) = adapter.device(addr) {
                    if let Some(name) = get_device_name(&device).await {
                        println!("  Discovered: {} ({})", name, addr);
                        if is_airpods_name(&name) {
                            return Ok(name);
                        }
                    }
                }
            }
        }
        Err("Discovery stream ended without finding AirPods".into())
    })
        .await;

    match result {
        Ok(inner) => inner,
        Err(_) => Err("Scan timed out — no AirPods found within 30 seconds".into()),
    }
}

pub async fn connect_airpods() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let adapter = get_adapter().await?;
<<<<<<< HEAD
    let device: Option<Device> = find_paired_airpods(&adapter).await?;
=======

    let device = {
        let device_addresses = adapter.device_addresses().await?;
        let mut found = None;
        for addr in device_addresses {
            let device = adapter.device(addr)?;
            if let Some(name) = get_device_name(&device).await {
                if is_airpods_name(&name) {
                    found = Some(device);
                    break;
                }
            }
        }
        found
    };
>>>>>>> 8acb368 (final)

    match device {
        Some(device) => {
            let name = get_device_name(&device).await.unwrap_or_else(|| "AirPods".to_string());
            let addr = device.address();

            if device.is_connected().await? {
                return Ok(format!("{} are already connected", name));
            }

<<<<<<< HEAD
            if !device.is_trusted().await? {
                device.set_trusted(true).await?;
            }

=======
            if !device.is_paired().await.unwrap_or(false) {
                println!("Pairing with {}...", name);
                match device.pair().await {
                    Ok(()) => println!("Paired with {}", name),
                    Err(e) => println!("Pair returned error (ignoring): {}", e),
                }
            }

            if !device.is_trusted().await.unwrap_or(false) {
                println!("Trusting {}...", name);
                let _ = device.set_trusted(true).await;
            }

>>>>>>> 8acb368 (final)
            let max_retries = 3;
            let mut last_error: Option<bluer::Error> = None;

            for attempt in 1..=max_retries {
                match device.connect().await {
                    Ok(()) => {
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        if device.is_connected().await? {
                            return Ok(format!("Connected to {} ({})", name, addr));
                        }
                    }
                    Err(e) => {
                        last_error = Some(e);
                        if attempt < max_retries {
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    }
                }
            }

            Err(format!(
                "Failed to connect to {} after {} attempts: {}",
                name,
                max_retries,
                last_error.map(|e| e.to_string()).unwrap_or_else(|| "unknown error".into())
            )
                .into())
        }
        None => {
            Err("No paired AirPods found. Please pair via Bluetooth settings first.".into())
        }
    }
}