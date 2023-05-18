fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.iter().filter(|x| **x).count()  as u8
    //  sheep.iter().filter(|&&x|x).count() as u8
}


#[test]
fn returns_correct_sheep_count() {
    assert_eq!(count_sheep(&[false]), 0);
    assert_eq!(count_sheep(&[true]), 1);
    assert_eq!(count_sheep(&[true, false]), 1);
}

/*
Counting sheep...
Consider an array/list of sheep where some sheep may be missing from their place. We need a function that counts the number of sheep present in the array (true means present).

For example,
```rust
&[true,  true,  true,  false,
true,  true,  true,  true ,
true,  false, true,  false,
true,  false, false, true ,
true,  true,  true,  true ,
false, false, true,  true]

```
The correct answer would be 17.
Hint: Don't forget to check for bad values like null/undefined

*/