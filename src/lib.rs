use crate::devices::MaschineMikroMk2;
use hidapi::HidApi;

pub mod colour;
pub mod controller;
pub mod devices;
pub mod display;
pub mod error;
pub mod events;
pub mod fonts;

pub fn get_device(hid_api: &HidApi) -> Result<devices::MaschineMikroMk2, error::Error> {
    let device = hid_api
        .open(
            devices::MaschineMikroMk2::VENDOR_ID,
            MaschineMikroMk2::PRODUCT_ID,
        )
        .expect("Cannot open device");

    Ok(devices::MaschineMikroMk2::new(device))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
