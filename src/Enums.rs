// Enums (way to group related value under a single value type, may have associated data)

#[derive(Debug)]
enum CarColors {
    Red,
    Green,
    Blue,
    Silver,
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E),
}



enum GivenOptions<T> {
    None,
    Some(T),
}

fn create_car_color_blue() -> CarColors {
    let my_car_color: CarColors = CarColors::Blue;
    my_car_color
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not under 5!".to_string())
    }
}

fn remainder_zero(num_check: f32) -> GivenOptions<f32> {
    let remainder = num_check % 10.0;
    if remainder != 0.0 {
        GivenOptions::Some(remainder)
    } else {
        GivenOptions::None
    }
}

fn check_under_five_build_in(num_check: u8) -> Result<u8, String> {
    if num_check < 5 {
        Ok(num_check)
    } else {
        Err("Not under 5!".to_string())
    }
}

fn remainder_zero_build_in(num_check: f32) -> Option<f32> {
    let remainder = num_check % 10.0;
    if remainder != 0.0 {
        Some(remainder)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_color: CarColors = create_car_color_blue();
        dbg!(car_color);

        let under_five_res: GivenResult<u8, String> = check_under_five(2);
        dbg!(under_five_res);

        let under_five_res: GivenResult<u8, String> = check_under_five(7);
        dbg!(under_five_res);

        // let remainder = remainder_zero(12.2);
        // dbg!(remainder);

        let under_five_res: Result<u8, String> = check_under_five_build_in(2);
        dbg!(under_five_res);

        let remainder_build = remainder_zero_build_in(12.3);
        dbg!(remainder_build);
    }
}


fn main(){
    print!("test");
}