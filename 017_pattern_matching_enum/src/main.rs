enum ElementType {
    Alkali,
    TransitionMetal,
    NonMetal,
    Gas(GasType),
}

enum GasType {
    Noble,
    Inert,
}

fn main() {
    let e = ElementType::Gas(GasType::Noble);
    detect_element(e);

    let someNumber = Some(41);
    let sum = plus_one(someNumber);

    println!("{} is the sum of {} + 1", sum.unwrap(), someNumber.unwrap());
}

fn detect_element(e: ElementType) {
    match e {
        ElementType::Gas(gas_type) => {
            match gas_type {
                GasType::Noble => {
                    println!("Helium (He), Neon (Ne), Argon (Ar), Krypton (Kr), Xenon (Xe), Radon (Rn), Oganesson (Og) are all noble gases");
                },
                _ => {
                    println!("Not implemented.");
                }
            }
        }
        _ => {
            println!("Not implemented.");
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

