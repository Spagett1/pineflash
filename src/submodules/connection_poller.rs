use crate::Flasher;
use serialport::SerialPortType::UsbPort;

impl Flasher {
    pub fn check_connections(&mut self) -> Option<String> {

        let ports = serialport::available_ports().expect("No ports found!");
        let mut type_of_pinecil: Option<String> = None;
        let mut v1: bool = false;
        let mut v2: bool = false;

        for device in rusb::devices().unwrap().iter() {
            let device_info = device.device_descriptor().unwrap();

            if device_info.vendor_id() == 10473 && device_info.product_id() == 393 {
                // pinecil v1 connected
                v1 = true;
                type_of_pinecil = Some("Pinecilv1".to_string())
            }
        }

        for device in ports {

            if let UsbPort(info) = device.port_type {
                if info.serial_number.clone().unwrap().contains("000000020000") {
                    // pinecil v2 connected
                    v2 = true;
                    type_of_pinecil = Some("Pinecilv2".to_string())
                }
            }
        }
        if v1 && v2 && self.config.iron_connected.as_ref() != Some(&"Both".to_string()) {
            type_of_pinecil = Some("Both".to_string());
            self.config.logs.push_str("Both v1 and v2 are detected")
        }
        else if self.config.iron_connected.is_none() && type_of_pinecil.is_some() {
            self.config.logs.push_str(format!("Pineflash: {} detected\n", type_of_pinecil.clone().unwrap()).as_str());
        } 
        else if self.config.iron_connected.is_some() && type_of_pinecil.is_none() {
            self.config.logs.push_str(format!("Pineflash: {} disconnected\n", self.config.iron_connected.clone().unwrap()).as_str());
        }
        type_of_pinecil
    }
}