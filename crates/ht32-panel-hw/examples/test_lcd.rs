use ht32_panel_hw::{LCD_VID, LCD_PID, LcdDevice};
use hidapi::HidApi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("HT32 Panel Hardware Test");
    println!("------------------------");
    println!("Searching for device VID:{:04x} PID:{:04x}", LCD_VID, LCD_PID);

    let api = HidApi::new()?;
    let devices: Vec<_> = api.device_list().collect();

    println!("Found {} HID devices total:", devices.len());
    
    let mut found = false;
    for device in devices {
        if device.vendor_id() == LCD_VID && device.product_id() == LCD_PID {
            println!("MATCH FOUND:");
            println!("  Path: {:?}", device.path());
            println!("  Serial: {:?}", device.serial_number());
            println!("  Manufacturer: {:?}", device.manufacturer_string());
            println!("  Product: {:?}", device.product_string());
            println!("  Interface: {}", device.interface_number());
            // println!("  Usage Page: {:04x}", device.usage_page());
            // println!("  Usage: {:04x}", device.usage());
            found = true;
        }
    }

    if found {
        println!("\nAttempting to open LCD device...");
        match LcdDevice::open() {
            Ok(_) => println!("SUCCESS: Device opened successfully!"),
            Err(e) => println!("FAILURE: Could not open device: {}", e),
        }
    } else {
        println!("\nFAILURE: Target device not found in HID list.");
        println!("Check USB connection and permissions (udev rules).");
    }

    Ok(())
}
