use usb1020::common::Axis;
use usb1020::USB1020Controller;

fn main() {
    let controller = USB1020Controller::fast_open().unwrap();

    // dbg!(controller.device.device().device_descriptor().unwrap());
    // controller.device.set_active_configuration().unwrap();

    // dbg!(controller.read_cv(Axis::X).unwrap());
    dbg!(controller.get_rr0().unwrap());

    // for device in rusb::devices().unwrap().iter() {
    //     let device_desc = device.device_descriptor().unwrap();
    //     println!("{:?}", device_desc);
    // }
}
