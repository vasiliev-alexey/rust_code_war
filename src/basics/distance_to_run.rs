pub fn run(speed: u32, time: u32) -> u32 {
    println!("{speed}, {time}");
    let mut distance_run = (time * speed) as i32;
    let   max_sprints = (time as f64 / 2.0).ceil() as u32;
    let tmp_speed: i32 = speed as i32;

    println!("{speed}, {time} {max_sprints}");

    for i in 0..(max_sprints ) {
        if tmp_speed - (3 * i) as i32 >= 0 {
            distance_run = distance_run + tmp_speed - (3 * i) as i32;
        }
    }
    return distance_run as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newbie() {
        assert_eq!(run(2, 3), 8);
    }

    #[test]
    fn not_so_fast() {
        assert_eq!(run(1, 1), 2);
    }

    #[test]
    fn long_trip() {
        assert_eq!(run(49, 50), 2875);
    }

    #[test]
    fn no_way_home() {
        assert_eq!(run(829, 135), 161453);
    }

    #[test]
    fn wwwwwwwwwww() {
        assert_eq!(run(741, 4), 4443);
    }

}