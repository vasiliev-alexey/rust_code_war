fn number(bus_stops:&[(i32,i32)]) -> i32 {
    //  let mut in_bus = 0;
    // for x in bus_stops {
    //     in_bus = in_bus + x.0 - x.1;
    // }
    // in_bus
    bus_stops.iter().fold( 0  , |x , y| x + y.0 - y.1)
}

#[test]
fn returns_expected() {
    assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
    assert_eq!(number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]), 17);
    assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
}