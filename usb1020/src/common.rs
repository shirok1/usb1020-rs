use deku::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
    U,
}

impl Axis {
    /// 形如 `0x10f` 的轴数，用于传输
    pub fn as_data(&self)->u16{
        match self {
            Axis::X => 0x010f,
            Axis::Y => 0x020f,
            Axis::Z => 0x040f,
            Axis::U => 0x080f,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OneOrAllAxis {
    One(Axis),
    All,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Clockwise {
    /// CW
    Clockwise,
    /// CCW
    CounterClockwise,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DecelerationMode {
    Auto,
    Manual,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LvDv {
    /// 定长驱动
    DV,
    /// 连续驱动
    LV,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LineOrCurve {
    /// 直线
    Line,
    /// S 曲线
    SCurve,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    /// 反向
    Minus,
    /// 正向
    Plus,
}

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(
type = "u8",
bits = "1",
endian = "endian",
ctx = "endian: deku::ctx::Endian")]
pub enum CounterType {
    /// 逻辑位置计数器
    #[deku(id = "0")]
    Logic,
    /// 实位计数器
    #[deku(id = "1")]
    Fact,
}

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(
type = "u8",
bits = "1",
endian = "endian",
ctx = "endian: deku::ctx::Endian")]
pub enum InputMode {
    /// A/B相方式
    #[deku(id = "0")]
    ABPhase,
    /// 上/下脉冲输入方式
    #[deku(id = "1")]
    UpDownPulse,
}
