
fn som(x: i64, y: i64) -> i64 {
    x +y
}
fn maxi(x: i64, y: i64) -> i64 {
   return  x.max(y);
}
fn mini(x: i64, y: i64) -> i64 {
    x.min(y)
}
fn gcdi(m: i64, n: i64) -> i64 {
    let m = m.abs();
    let n = n.abs();
    if n == 0 {
        m
    }
    else {
        gcdi(n, m % n)
    }
}
fn lcmu(a: i64, b: i64) -> i64 {
    let a = a.abs();
    let b = b.abs();
    a * b / gcdi(a, b)
}

// first parameter: dots have to be replaced by function of two variables
fn oper_array( f: fn (x : i64 , y: i64) -> i64, a: &[i64], init: i64) -> Vec<i64> {

    let mut res = vec![];
    let mut tmb = 0;
    for i in 0.. a.len() {
        if i ==0 {
             tmb  = f(init , a[i]);
            res.push( tmb)
        } else {
            tmb  = f(tmb , a[i]);
            res.push( tmb)
        }
    }

    res
}


//
fn testing_som(a: &[i64], exp:  &Vec<i64>) -> () {
    assert_eq!(&oper_array(som, a, 0), exp);
}
fn testing_lcmu(a: &[i64], exp:  &Vec<i64>) -> () {
    assert_eq!(&oper_array(lcmu, a, a[0]), exp);
}
fn testing_gcdi(a: &[i64], exp:  &Vec<i64>) -> () {
    assert_eq!(&oper_array(gcdi, a, a[0]), exp);
}
fn testing_maxi(a: &[i64], exp:  &Vec<i64>) -> () {
    assert_eq!(&oper_array(maxi, a, a[0]), exp);
}
fn testing_mini(a: &[i64], exp:  &Vec<i64>) -> () {
    assert_eq!(&oper_array(mini, a, a[0]), exp);
}


#[test]
fn basics_som() {
    testing_som(&[ 18, 69, -90, -78, 65, 40 ], &vec![ 18, 87, -3, -81, -16, 24 ]);
}
#[test]
fn basics_lcmu() {
    testing_lcmu(&[ 18, 69, -90, -78, 65, 40 ], &vec![ 18, 414, 2070, 26910, 26910, 107640 ]);
}
#[test]
fn basics_maxi() {
    testing_maxi(&[ 18, 69, -90, -78, 65, 40 ], &vec![ 18, 69, 69, 69, 69, 69 ]);
}
#[test]
fn basics_mini() {
    testing_mini(&[ 18, 69, -90, -78, 65, 40 ], &vec![ 18, 18, -90, -90, -90, -90]);
}
#[test]
fn basics_gcdi() {
    testing_gcdi(&[ 18, 69, -90, -78, 65, 40 ], &vec![ 18, 3, 3, 3, 1, 1 ]);
}