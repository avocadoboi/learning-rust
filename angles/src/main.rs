use std::fmt;

#[derive(Copy, Clone)]
struct Radians(f64);
#[derive(Copy, Clone)]
struct Degrees(f64);

enum AngleType {
    Radians,
    Degrees
}
impl fmt::Display for AngleType {
    fn fmt(&self, p_formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(p_formatter, "{}", match self { 
            AngleType::Radians => "Radians", 
            AngleType::Degrees => "Degrees" 
        })
    }
}

trait Angle: Copy + fmt::Display + From<Radians> + From<Degrees> {
    fn to_degrees(&self) -> Degrees;
    fn to_radians(&self) -> Radians;
    fn get_type(&self) -> AngleType;

    // Just to demonstrate...
    fn sin(&self) -> f64 {
        self.to_radians().0.sin()
    }
    fn cos(&self) -> f64 {
        self.to_radians().0.cos()
    }
}

impl From<Radians> for Degrees {
    fn from(radians: Radians) -> Degrees {
        Degrees(radians.0/std::f64::consts::PI*180.)
    }
}
impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Radians {
        Radians(degrees.0*std::f64::consts::PI/180.)
    }
}

macro_rules! impl_angle {
    ($type_name: path) => {
        impl fmt::Display for $type_name {
            fn fmt(&self, p_formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(p_formatter, "{}", self.0)
            }
        }
        impl Angle for $type_name {
            fn to_degrees(&self) -> Degrees {
                Degrees::from(*self)
            }
            fn to_radians(&self) -> Radians {
                Radians::from(*self)
            }
            fn get_type(&self) -> AngleType {
                self.get_type()
            }
        }
    }
}

impl_angle!(Radians);
impl Radians {
    fn get_type(&self) -> AngleType {
        AngleType::Radians
    }
}
impl_angle!(Degrees);
impl Degrees {
    fn get_type(&self) -> AngleType {
        AngleType::Degrees
    }
}

/*
    An example of a function taking a generic angle 
    and converting it to its opposite angle type.
*/
fn print_angle_as_opposite_type(angle: impl Angle) {
    match angle.get_type() {
        AngleType::Radians => println!(
            "{} {} is {} {}", angle, angle.get_type(), 
            angle.to_degrees(), AngleType::Degrees
        ),
        AngleType::Degrees => println!(
            "{} {} is {} {}", angle, angle.get_type(), 
            angle.to_radians(), AngleType::Radians
        ),
    }
}

fn main() {
    let radians = Radians(std::f64::consts::FRAC_PI_2);

    let degrees = Degrees::from(radians);

    println!("\n{} radians is {} degrees.", radians, degrees);

    println!("\nSine of {} radians is: {}", radians, radians.sin());
    println!("Sine of {} degrees is: {}", degrees, degrees.sin());

    println!("\nNow I will call a function that prints degrees as radians.");
    print_angle_as_opposite_type(degrees);

    println!("\nNow I will call the same function, but this time it'll convert radians to degrees!");
    print_angle_as_opposite_type(radians);

    println!()
}
