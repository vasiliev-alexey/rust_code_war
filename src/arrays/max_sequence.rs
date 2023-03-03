fn max_sequence(seq: &[i32]) -> i32 {
    if seq.len() < 1  ||
        seq.iter().filter( | a |  a > &&0).count() < 1 {
        return 0;
    }

    if seq.len() < 2 {
        return seq[0];
    }
    let mut sum: i32 = 0;
    for i in 0.. seq.len(){
        let mut temp: Vec<i32> = Vec::new();
        for j in i.. seq.len(){
            temp.push(seq[j]);
            if temp.iter().sum::<i32>().gt(&sum) {
                sum =  temp.iter().sum();
            }
        }
    }
    return sum;
}

/*
fn max_sequence(seq: &[i32]) -> i32 {
    let mut m = 0;
    seq.iter().fold(0, |prev, &v|{
        let p = v.max(prev + v);
        m = m.max(p);
        p
    });
    m
}
*/

#[cfg(test)]
mod tests {
    use super::max_sequence;

    #[test]
    fn sample_tests() {
        assert_eq!(max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sequence(&[11]), 11);
        assert_eq!(max_sequence(&[-32]), 0);
    }
}