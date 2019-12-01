extern crate helpers;

fn recalculate_fuel(sum_mass: u32, mass: u32) -> u32 {
    let fuel_divided = mass / 3;
    if fuel_divided <= 2 {
        return sum_mass;
    }
    let fuel_increment = fuel_divided - 2;
    return recalculate_fuel(fuel_increment + sum_mass, fuel_increment);
}

fn sum_mass(it: impl Iterator<Item=u32>) -> u32 {
    it.fold(0, recalculate_fuel)
}

fn main() {
    let data = helpers::file::parse_data::<u32>(helpers::args::path_from_args().unwrap());
    println!("Fuel needed: {}", sum_mass(data));
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    struct ExpectedModuleFuel {
        mass: u32,
        fuel: u32,
    }

    const MODULE_1: ExpectedModuleFuel = ExpectedModuleFuel { mass: 14, fuel: 2 };
    const MODULE_2: ExpectedModuleFuel = ExpectedModuleFuel { mass: 1969, fuel: 966 };
    const MODULE_3: ExpectedModuleFuel = ExpectedModuleFuel { mass: 100756, fuel: 50346 };

    #[test]
    fn test_recalculate_fuel() {
        assert_eq!(MODULE_1.fuel, recalculate_fuel(0, MODULE_1.mass));
        assert_eq!(MODULE_2.fuel, recalculate_fuel(0, MODULE_2.mass));
        assert_eq!(MODULE_3.fuel, recalculate_fuel(0, MODULE_3.mass));
    }

    #[test]
    fn test_sum_mass() {
        let sumarized_mass = sum_mass(vec![MODULE_1.mass, MODULE_2.mass, MODULE_3.mass].into_iter());
        assert_eq!(MODULE_1.fuel + MODULE_2.fuel + MODULE_3.fuel, sumarized_mass)
    }
}