use crate::register::*;
// use deku::prelude::*;

#[test]
fn register_parse() {
    assert_eq!(
        RR0::try_from([0x00, 0x80].as_ref()).unwrap(),
        RR0 {
            x_drv: false,
            y_drv: false,
            z_drv: false,
            u_drv: false,
            x_error: false,
            y_error: false,
            z_error: false,
            u_error: false,
            i_drv: true,
            c_next: false,
            zone: 0,
            bit_interpolate_stack_counter: 0,
        }
    );
    assert_eq!(
        RR1::try_from([0x01, 0x00].as_ref()).unwrap(),
        RR1 {
            comp_plus: false,
            comp_minus: false,
            v_ascending: false,
            v_const: false,
            v_descending: false,
            a_ascending: false,
            a_const: false,
            a_descending: true,
            in0: false,
            in1: false,
            in2: false,
            in3: false,
            limit_plus: false,
            limit_minus: false,
            servo_alarm: false,
            emergency: false,
        }
    );
    assert_eq!(
        RR2::try_from([0x00, 0x00].as_ref()).unwrap(),
        RR2 {
            software_limit_plus: false,
            software_limit_minus: false,
            hardware_limit_plus: false,
            hardware_limit_minus: false,
            servo_alarm: false,
            emergency: false,
            home_error: false,
            home_step: AutoHomeSearchStep::Step0,
        }
    );
    assert_eq!(
        RR4::try_from([0xff, 0xff].as_ref()).unwrap(),
        RR4 {
            z: AxisExternalSignal {
                in0: true,
                in1: true,
                in2: true,
                in3: true,
                p_p: true,
                p_m: true,
                in_position: true,
                servo_alarm: true,
            },
            u: AxisExternalSignal {
                in0: true,
                in1: true,
                in2: true,
                in3: true,
                p_p: true,
                p_m: true,
                in_position: true,
                servo_alarm: true,
            },
        }
    );
}
