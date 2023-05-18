fn string_to_industrial(time: &str) -> f64 {
    let times = time.split(":").map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>();
    ((times[0] * 60.0 + times[1])   * 60.0 / 36.0).round() / 100.0
}


fn to_industrial(time: u32) -> f64 {
    (time as f64 * 60.0 / 36.0).round() / 100.0
}

fn to_normal(time: f64) -> String {
    // todo!();
    let ret = time *100.0*36.0;
    let  h = (ret/3600.0).floor();

    let m =  (ret %3600.0 /60.0).round();
    // println!("{}  {}", h ,  m);
     format!("{}:{:0>2}", h, m)


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(0.02, to_industrial(1));
        assert_eq!(0.03, to_industrial(2));
        assert_eq!(1.75, to_industrial(105));
        assert_eq!(0.05, string_to_industrial("0:03"));
        assert_eq!(0.07, string_to_industrial("0:04"));
        assert_eq!(1.75, string_to_industrial("1:45"));
         assert_eq!("1:45", to_normal(1.75));
         assert_eq!("0:20", to_normal(0.33));
    }
}
