use deku::prelude::*;


pub type MainStatusRegister = RR0;

/// 在软件使用说明书中称为 `USB1020_PARA_RR0`。
///
/// 主状态寄存器。
#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct RR0 {
    /// X 轴的驱动状态
    #[deku(bits = "1")]
    pub x_drv: bool,

    /// Y 轴的驱动状态
    #[deku(bits = "1")]
    pub y_drv: bool,

    /// Z 轴的驱动状态
    #[deku(bits = "1")]
    pub z_drv: bool,

    /// U 轴的驱动状态
    #[deku(bits = "1")]
    pub u_drv: bool,

    /// X 轴的出错状态
    #[deku(bits = "1")]
    pub x_error: bool,

    /// Y 轴的出错状态
    #[deku(bits = "1")]
    pub y_error: bool,

    /// Z 轴的出错状态
    #[deku(bits = "1")]
    pub z_error: bool,

    /// U 轴的出错状态
    #[deku(bits = "1")]
    pub u_error: bool,

    /// 插补驱动状态
    #[deku(bits = "1")]
    pub i_drv: bool,

    /// 表示是否可以写入连续插补的下一个数据
    #[deku(bits = "1")]
    pub c_next: bool,

    /// 表示在圆弧插补驱动中所在的象限
    #[deku(bits = "3")]
    pub zone: u8,

    /// 表示在位插补驱动中堆栈计数器(SC)的数值
    #[deku(bits = "2", pad_bits_after = "1")]
    pub bit_interpolate_stack_counter: u8,
}


pub type AxisStatusRegister = RR1;

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
/// 在软件使用说明书中称为 `USB1020_PARA_RR1`。
///
/// 每个轴各自拥有状态寄存器。
pub struct RR1 {
    /// 表示逻辑/实位计数器和 COMP+ 寄存器的大小关系是否满足逻辑/实位计数器 ≥ COMP+
    #[deku(bits = "1")]
    pub comp_plus: bool,

    /// 表示逻辑/实位计数器和 COMP- 寄存器的大小关系是否满足逻辑/实位计数器 < COMP-
    #[deku(bits = "1")]
    pub comp_minus: bool,

    /// 在加/减速驱动中加速
    #[deku(bits = "1")]
    pub v_ascending: bool,

    /// 在加/减速驱动中定速
    #[deku(bits = "1")]
    pub v_const: bool,

    /// 在加/减速驱动中减速
    #[deku(bits = "1")]
    pub v_descending: bool,

    /// 在 S 曲线加/减速驱动中，加速度/减速度增加
    #[deku(bits = "1")]
    pub a_ascending: bool,

    /// 在 S 曲线加/减速驱动中，加速度/减速度不变
    #[deku(bits = "1")]
    pub a_const: bool,

    /// 在 S 曲线加/减速驱动中，加速度/减速度减少
    #[deku(bits = "1")]
    pub a_descending: bool,

    /// 外部停止信号 IN0 有效使驱动停止
    #[deku(bits = "1")]
    pub in0: bool,

    /// 外部停止信号 IN1 有效使驱动停止
    #[deku(bits = "1")]
    pub in1: bool,

    /// 外部停止信号 IN2 有效使驱动停止
    #[deku(bits = "1")]
    pub in2: bool,

    /// 外部停止信号 IN3 有效使驱动停止
    #[deku(bits = "1")]
    pub in3: bool,

    /// 外部正方向限制信号(nLMTP)有效使驱动停止
    #[deku(bits = "1")]
    pub limit_plus: bool,

    /// 外部反方向限制信号(nLMTM)有效使驱动停止
    #[deku(bits = "1")]
    pub limit_minus: bool,

    /// 外部伺服马达报警信号(nALARM)有效使驱动停止
    #[deku(bits = "1")]
    pub servo_alarm: bool,

    /// 外部紧急停止信号(EMGN)使驱动停止
    #[deku(bits = "1")]
    pub emergency: bool,
}


pub type AxisErrorRegister = RR2;

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
/// 在软件使用说明书中称为 `USB1020_PARA_RR2`
pub struct RR2 {
    /// 设置正方向软件限位后，在正方向驱动中，逻辑/实位计数器大于 COMP+ 寄存器
    #[deku(bits = "1")]
    pub software_limit_plus: bool,

    /// 设置反方向软件限位后，在反方向驱动中，逻辑/实位计数器小于 COMP- 寄存器
    #[deku(bits = "1")]
    pub software_limit_minus: bool,

    /// 外部正方向限制信号(nLMTP)处于有效电平
    #[deku(bits = "1")]
    pub hardware_limit_plus: bool,

    /// 外部反方向限制信号(nLMTM)处于有效电平
    #[deku(bits = "1")]
    pub hardware_limit_minus: bool,

    /// 外部伺服马达报警信号(nALARM)设置为有效并处于有效状态
    #[deku(bits = "1")]
    pub servo_alarm: bool,

    /// 外部紧急停止信号处于低电平
    #[deku(bits = "1")]
    pub emergency: bool,

    /// Z 相编码信号在自动搜寻原点出错
    #[deku(bits = "1")]
    pub home_error: bool,

    /// 自动原点搜寻中执行的步数
    // #[deku(bits = "5", pad_bits_after = "4")]
    #[deku(pad_bits_after = "4")]
    pub home_step: AutoHomeSearchStep,
}

/// 自动原点搜寻中执行的步数
#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(
type = "u8",
bits = "5",
endian = "endian",
ctx = "endian: deku::ctx::Endian")]
pub enum AutoHomeSearchStep {
    /// 等待自动原点搜寻命令
    #[deku(id = "0")]
    Step0,

    /// 等待 IN0 信号在指定方向上有效
    #[deku(id = "3")]
    Step3,

    /// 等待 IN1 信号在指定方向上有效
    #[deku(id = "8")]
    Step8,

    /// 等待 IN1 信号在指定方向上有效
    #[deku(id = "12")]
    Step12,

    /// 等待 IN1 信号在指定方向上有效
    #[deku(id = "15")]
    Step15,

    /// IN2 信号在指定方向上有效
    #[deku(id = "20")]
    Step20,

    /// 第四步
    #[deku(id = "25")]
    Step25,
}


pub type ExternalSignalXY = RR3;
pub type ExternalSignalZU = RR4;

/// 在软件使用说明书中称为 `USB1020_PARA_RR3`
#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct RR3 {
    pub x: AxisExternalSignal,
    pub y: AxisExternalSignal,
}

/// 在软件使用说明书中称为 `USB1020_PARA_RR4`
#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct RR4 {
    pub z: AxisExternalSignal,
    pub u: AxisExternalSignal,
}

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct AxisExternalSignal {
    /// 外部停止信号 IN0 的电平状态
    #[deku(bits = "1")]
    pub in0: bool,

    /// 外部停止信号 IN1 的电平状态
    #[deku(bits = "1")]
    pub in1: bool,

    /// 外部停止信号 IN2 的电平状态
    #[deku(bits = "1")]
    pub in2: bool,

    /// 外部停止信号 IN3 的电平状态
    #[deku(bits = "1")]
    pub in3: bool,

    /// 外部正方向点动输入信号 EXPP 的电平状态
    #[deku(bits = "1")]
    pub p_p: bool,

    /// 外部反方向点动输入信号 EXPM 的电平状态
    #[deku(bits = "1")]
    pub p_m: bool,

    /// 外部伺服电机到位信号 INPOS 的电平状态
    #[deku(bits = "1")]
    pub in_position: bool,

    /// 外部伺服马达报警信号 ALARM 的电平状态
    #[deku(bits = "1")]
    pub servo_alarm: bool,
}


pub type InterruptRegister = RR5;

/// 在软件使用说明书中称为 `USB1020_PARA_RR5`。
///
/// 表明本次中断的原因。
#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct RR5 {
    /// 产生一个增量脉冲
    #[deku(bits = "1")]
    pub pulse: bool,

    /// 逻辑/实际位置计数器的值大于等于 COMP- 寄存器的值
    #[deku(bits = "1")]
    pub p_bigger_than_comp_minus: bool,

    /// 逻辑/实际位置计数器的值小于 COMP- 寄存器的值
    #[deku(bits = "1")]
    pub p_s_c_m: bool,

    /// 逻辑/实际位置计数器的值小于 COMP+ 寄存器的值时
    #[deku(bits = "1")]
    pub p_s_c_p: bool,

    /// 逻辑/实际位置计数器的值大于等于 COMP+ 寄存器的值
    #[deku(bits = "1")]
    pub p_b_c_p: bool,

    /// 在加/减速时，脉冲开始减速时
    #[deku(bits = "1")]
    pub c_dec: bool,

    /// 在加/减速时，开始定速时
    #[deku(bits = "1")]
    pub const_start: bool,

    /// 驱动结束
    #[deku(bits = "1")]
    pub drive_end: bool,

    /// 自动原点搜索结束时
    #[deku(bits = "1")]
    pub home_end: bool,

    /// 同步产生的中断
    #[deku(bits = "1", pad_bits_after = "6")]
    pub sync: bool,
}

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct PulseBitFiled {
    ///
    #[deku(bits = "1")]
    pub p_dir_sw_lmt: bool,

    ///
    #[deku(bits = "1")]
    pub m_dir_sw_lmt: bool,

    ///
    #[deku(bits = "1")]
    pub stop_mode: bool,

    ///
    #[deku(bits = "1")]
    pub p_log_lever: bool,

    ///
    #[deku(bits = "1")]
    pub m_log_lever: bool,

    ///
    // #[deku(bits = "1")]
    pub lp_ep: crate::common::CounterType,

    ///
    #[deku(bits = "1")]
    pub output_mode: bool,

    ///
    #[deku(bits = "1")]
    pub _t: bool,

    ///
    #[deku(bits = "1")]
    pub _r: bool,

    ///
    #[deku(pad_bits_after = "2")]
    // #[deku(bits = "1", pad_bits_after = "6")]
    pub input_mode: crate::common::InputMode,

    ///
    // #[deku(bits = "2")]
    pub alarm: Alarm,
    ///
    // #[deku(bits = "2")]
    pub inpos: InPos,
}

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(
type = "u8",
bits = "2",
endian = "endian",
ctx = "endian: deku::ctx::Endian")]
pub enum Alarm {
    /// A/B相方式
    #[deku(id = "0")]
    Disable,
    /// 上/下脉冲输入方式
    #[deku(id = "2")]
    Enable,
    /// 上/下脉冲输入方式
    #[deku(id = "3")]
    EnableWithLog,
}

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(
type = "u8",
bits = "2",
endian = "endian",
ctx = "endian: deku::ctx::Endian")]
pub enum InPos {
    /// A/B相方式
    #[deku(id = "0")]
    Disable,
    /// 上/下脉冲输入方式
    #[deku(id = "2")]
    Enable,
    /// 上/下脉冲输入方式
    #[deku(id = "3")]
    EnableWithLog,
}

pub enum PulseOutputMode {}