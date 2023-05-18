pub fn bmi(weight: u32, height: f32) -> &'static str {
    match  weight as f32 / (height.powf(2.0)) {
        x if  x >= 0.0 &&  18.5 <=x => "Underweight",
        x if  x >= 25.0 => "Normal",
        x if  x >= 30.0 => "Overweight",
        _ => "Obese",
    }

}

mod tests {

    use super::*;
    #[test]
    fn basic_tests() {
        assert_eq!(bmi(50, 1.80), "Underweight");
        assert_eq!(bmi(80, 1.80), "Normal");
        assert_eq!(bmi(90, 1.80), "Overweight");
        assert_eq!(bmi(110, 1.80), "Obese");
    }
}

//https://www.codewars.com/kata/57a429e253ba3381850000fb/train/rust

/*
if bmi <= 18.5 return "Underweight"

if bmi <= 25.0 return "Normal"

if bmi <= 30.0 return "Overweight"

if bmi > 30 return "Obese"
*/