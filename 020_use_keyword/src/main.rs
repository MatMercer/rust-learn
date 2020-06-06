pub mod very {
    pub mod long {
        pub mod name {
            pub fn very_long_name_function() {
                println!("Oh, you got here!");
            }
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum CarType {
    Flex,
    Etanol,
    Gasoline,
}

fn main() {
    use very::long::name::very_long_name_function as f;
    use TrafficLight::{Red};
    use CarType::*;

    let red = Red;
    let car = Gasoline;

    f();
}
