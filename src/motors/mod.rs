pub mod dc_motor;
mod large_motor;
mod medium_motor;
pub mod servo_motor;
pub mod tacho_motor;

pub use self::dc_motor::DcMotor;
pub use self::servo_motor::ServoMotor;
pub use self::tacho_motor::TachoMotor;

pub use self::large_motor::LargeMotor;
pub use self::medium_motor::MediumMotor;

use crate::{Device, Port};

pub trait Motor: Device {}

pub enum MotorPort {
    OutA,
    OutB,
    OutC,
    OutD,
}

impl Port for MotorPort {
    fn address(&self) -> String {
        match self {
            MotorPort::OutA => "outA".to_owned(),
            MotorPort::OutB => "outB".to_owned(),
            MotorPort::OutC => "outC".to_owned(),
            MotorPort::OutD => "outD".to_owned(),
        }
    }
}