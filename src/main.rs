extern crate ocl;
use ocl::{
    Platform, Device
};

fn main() {
    let platform = Platform::default();
    let device = Device::first(platform);
    match device {
        Err(_why) => println!("Error!"),
        Ok(dev) => println!("The name of the default device is {}!", dev.name().unwrap()),
    };
}
