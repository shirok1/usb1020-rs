use crate::common::Axis;
use crate::register::*;
use crate::Error::{DecodeError, DeviceNotFound, USBError};
use common::OneOrAllAxis;
use rusb::{Device, DeviceHandle, GlobalContext};
use std::time::Duration;

pub mod common;
pub mod register;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct USB1020Controller {
    device_handle: DeviceHandle<GlobalContext>,
}

// impl Drop for USB1020Controller {
//     fn drop(&mut self) {
//         drop(self.device)
//         // todo!()
//     }
// }

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("internal usb error: {0}")]
    USBError(#[from] rusb::Error),

    #[error("read not 8 bytes?")]
    ReadNot8Bytes,

    #[error("failed to decode register/struct: {0}")]
    DecodeError(#[from] deku::DekuError),

    #[error("no suitable device")]
    DeviceNotFound,
}

impl USB1020Controller {
    const VENDOR_ID: u16 = 0x04b4;
    const PRODUCT_ID: u16 = 0x1020;

    const USB_TIMEOUT: Duration = Duration::from_millis(100);

    pub fn device_handle(&self) -> &DeviceHandle<GlobalContext> {
        &self.device_handle
    }

    pub fn fast_open() -> Result<Self, Error> {
        let mut device_handle = rusb::open_device_with_vid_pid(Self::VENDOR_ID, Self::PRODUCT_ID)
            .ok_or(DeviceNotFound)?;
        device_handle.set_active_configuration(1)?;
        device_handle.claim_interface(0)?;

        let this = Self { device_handle };
        this.init_device()?;

        Ok(this)
    }

    fn init_device(&self) -> Result<(), Error> {
        self.write_pipe(0, 0x8000)?;

        self.write_pipe(0, 0x010f)?;
        self.write_pipe(1, 0x0000)?;
        self.write_pipe(2, 0x0100)?; // TODO: X_PULSE_BF
        self.write_pipe(3, 0x0f00)?;

        self.write_pipe(0, 0x020f)?;
        self.write_pipe(1, 0x0000)?;
        self.write_pipe(2, 0x0100)?; // TODO: Y_PULSE_BF
        self.write_pipe(3, 0x0f00)?;

        self.write_pipe(0, 0x040f)?;
        self.write_pipe(1, 0x0000)?;
        self.write_pipe(2, 0x0100)?; // TODO: Z_PULSE_BF
        self.write_pipe(3, 0x0f00)?;

        self.write_pipe(0, 0x080f)?;
        self.write_pipe(1, 0x0000)?;
        self.write_pipe(2, 0x0100)?; // TODO: U_PULSE_BF
        self.write_pipe(3, 0x0f00)?;

        self.write_pipe(4, 0xffff)?;
        self.write_pipe(5, 0)?;
        self.set_lp(OneOrAllAxis::All, 0)?;
        self.set_ep(OneOrAllAxis::All, 0)?;

        Ok(())
    }

    pub fn device_filter(device: &Device<GlobalContext>) -> bool {
        matches!(device.device_descriptor(), Ok(desc)
            if desc.vendor_id() == Self::VENDOR_ID && desc.product_id() == Self::PRODUCT_ID)
    }

    fn bulk_write_0x02(&self, buf: &[u8]) -> Result<usize, Error> {
        self.device_handle
            .write_bulk(0x02, buf, Self::USB_TIMEOUT)
            .map_err(USBError)
    }

    fn bulk_write_0x04(&self, buf: &[u8]) -> Result<usize, Error> {
        self.device_handle
            .write_bulk(0x04, buf, Self::USB_TIMEOUT)
            .map_err(USBError)
    }

    fn write_pipe(&self, pipe: u8, data: u16) -> Result<(), Error> {
        let mut buf = [0u8; 512];
        buf[0] = pipe;
        self.bulk_write_0x02(buf.as_ref())?;
        let [data_0, data_1] = data.to_le_bytes();
        buf[0] = data_0;
        buf[1] = data_1;
        self.bulk_write_0x04(buf.as_ref())?;
        Ok(())
    }

    fn bulk_read_0x88(&self, buf: &mut [u8]) -> Result<usize, Error> {
        self.device_handle
            .read_bulk(0x88, buf, Self::USB_TIMEOUT)
            .map_err(USBError)
    }

    //    fn bulk_read_0x86(&self, buf: &mut [u8]) -> Result<usize, Error> {
    //        self.device_handle
    //            .read_bulk(0x86, buf, Self::USB_TIMEOUT)
    //            .map_err(USBError)
    //    }

    fn pull_rr(&self, rr_no: u8) -> Result<[u8; 2], Error> {
        let mut buf = [0u8; 512];
        buf[0] = rr_no;
        self.bulk_write_0x02(buf.as_ref())?;

        let mut buf = [0u8; 8];
        self.bulk_read_0x88(buf.as_mut())?; // dispose for some reason
        self.bulk_read_0x88(buf.as_mut())?;
        Ok([buf[0], buf[1]])
    }

    pub fn get_rr0(&self) -> Result<MainStatusRegister, Error> {
        let buf = self.pull_rr(0)?;
        MainStatusRegister::try_from(buf.as_ref()).map_err(DecodeError)
    }

    pub fn get_rr1(&self, axis: Axis) -> Result<AxisStatusRegister, Error> {
        self.write_pipe(0, axis.as_data())?;
        let buf = self.pull_rr(1)?;
        AxisStatusRegister::try_from(buf.as_ref()).map_err(DecodeError)
    }

    pub fn get_rr2(&self, axis: Axis) -> Result<AxisErrorRegister, Error> {
        self.write_pipe(0, axis.as_data())?;
        let buf = self.pull_rr(2)?;
        AxisErrorRegister::try_from(buf.as_ref()).map_err(DecodeError)
    }

    pub fn get_rr3(&self) -> Result<RR3, Error> {
        let buf = self.pull_rr(4)?;
        RR3::try_from(buf.as_ref()).map_err(DecodeError)
    }

    pub fn get_rr4(&self) -> Result<RR4, Error> {
        let buf = self.pull_rr(5)?;
        RR4::try_from(buf.as_ref()).map_err(DecodeError)
    }
    /// 读取中断寄存器 RR5 并将其置为 0
    pub fn get_rr5(&self, axis: Axis) -> Result<RR5, Error> {
        self.write_pipe(0, axis.as_data())?;
        let buf = self.pull_rr(3)?;
        self.write_pipe(0, 0x45)?; // TODO: 此处是否清除了 RR5 内容？
        RR5::try_from(buf.as_ref()).map_err(DecodeError)
    }

    pub fn read_br(&self, axis: Axis) -> Result<u32, Error> {
        self.write_pipe(
            0,
            match axis {
                Axis::X => 0x0114,
                Axis::Y => 0x0214,
                Axis::Z => 0x0414,
                Axis::U => 0x0814,
            },
        )?;

        self.read_u32_data()
    }

    pub fn read_ca(&self, axis: Axis) -> Result<u16, Error> {
        self.write_pipe(
            0,
            match axis {
                Axis::X => 0x0113,
                Axis::Y => 0x0213,
                Axis::Z => 0x0413,
                Axis::U => 0x0813,
            },
        )?;

        self.read_u16_data()
    }

    pub fn read_cv(&self, axis: Axis) -> Result<u16, Error> {
        self.write_pipe(
            0,
            match axis {
                Axis::X => 0x0112,
                Axis::Y => 0x0212,
                Axis::Z => 0x0412,
                Axis::U => 0x0812,
            },
        )?;

        self.read_u16_data()
    }

    pub fn read_ep(&self, axis: Axis) -> Result<u32, Error> {
        self.write_pipe(
            0,
            match axis {
                Axis::X => 0x0111,
                Axis::Y => 0x0211,
                Axis::Z => 0x0411,
                Axis::U => 0x0811,
            },
        )?;

        self.read_u32_data()
    }

    pub fn read_lp(&self, axis: Axis) -> Result<u32, Error> {
        self.write_pipe(
            0,
            match axis {
                Axis::X => 0x0110,
                Axis::Y => 0x0210,
                Axis::Z => 0x0410,
                Axis::U => 0x0810,
            },
        )?;

        self.read_u32_data()
    }

    fn read_u16_data(&self) -> Result<u16, Error> {
        //        Ok(u16::from_le_bytes(self.pull_rr(6)?))
        self.pull_rr(6).map(u16::from_le_bytes)
    }

    fn read_u32_data(&self) -> Result<u32, Error> {
        let [ep_0, ep_1] = self.pull_rr(7)?;
        let [ep_2, ep_3] = self.pull_rr(6)?;
        Ok(u32::from_le_bytes([ep_0, ep_1, ep_2, ep_3])) // TODO: 验证顺序
    }

    fn set_u32_data(&self, data: u32) -> Result<(), Error> {
        let [data_0, data_1, data_2, data_3] = data.to_le_bytes();

        self.write_pipe(7, u16::from_le_bytes([data_0, data_1]))?;
        self.write_pipe(6, u16::from_le_bytes([data_2, data_3]))?; // TODO: verify

        Ok(())
    }

    pub fn set_lp(&self, axis: OneOrAllAxis, data: u32) -> Result<(), Error> {
        self.set_u32_data(data)?;
        self.write_pipe(
            0,
            match axis {
                OneOrAllAxis::One(Axis::X) => 0x0109,
                OneOrAllAxis::One(Axis::Y) => 0x0209,
                OneOrAllAxis::One(Axis::Z) => 0x0409,
                OneOrAllAxis::One(Axis::U) => 0x0809,
                OneOrAllAxis::All => 0x0f09,
            },
        )?;
        Ok(())
    }

    pub fn set_ep(&self, axis: OneOrAllAxis, data: u32) -> Result<(), Error> {
        self.set_u32_data(data)?;
        self.write_pipe(
            0,
            match axis {
                OneOrAllAxis::One(Axis::X) => 0x010a,
                OneOrAllAxis::One(Axis::Y) => 0x020a,
                OneOrAllAxis::One(Axis::Z) => 0x040a,
                OneOrAllAxis::One(Axis::U) => 0x080a,
                OneOrAllAxis::All => 0x0f0a,
            },
        )?;
        Ok(())
    }
    pub fn set_dev_inc_rate(&self, axis: OneOrAllAxis, data: u32) -> Result<(), Error> {
        self.set_u32_data(data)?;
        self.write_pipe(
            0,
            match axis {
                OneOrAllAxis::One(Axis::X) => 0x010e,
                OneOrAllAxis::One(Axis::Y) => 0x020e,
                OneOrAllAxis::One(Axis::Z) => 0x040e,
                OneOrAllAxis::One(Axis::U) => 0x080e,
                OneOrAllAxis::All => 0x0f0e,
            },
        )?;
        Ok(())
    }
}
