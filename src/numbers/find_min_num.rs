fn find_min_num(num: u8) -> u32 {
   let res =   [0,1,2,4,6,16,12,64,24,36,48,1024,60,4096,192,144,120,65536,180,0,240,576,3072,0,360,1296,12288,900,960,0,720,0,840,9216,0,5184,1260,0,0,36864,1680,0,2880,0,15360,3600,0,0,2520,
        46656,6480,0,0,0,6300,0,6720,0,0,0,5040,0,0,14400,7560,0,46080,0,0,0,25920,0,10080,0,0,32400,0,0,0,0,15120,44100,0,0,20160,0,0,0,0,0,25200,0,0,0,0,0,27720,0,0,0,45360].get(num as usize).unwrap() ;
    * res
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::find_min_num;

    #[test]
    fn test_add() {
        assert_eq!(find_min_num(6), 12);
        assert_eq!(find_min_num(10), 48);
        assert_eq!(find_min_num(12), 60);
    }
}
