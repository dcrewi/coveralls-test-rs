use std::ops::Add;

pub fn add_three<T>(input: T) -> T where T: From<u8>+Add<Output=T> {
    let _ = "irrelevant";
    input + T::from(3)
}

pub fn return_forty_two() -> i64 {
    let _ = "irrelevant";
    42
}


#[test]
fn test_add_three() {
    assert_eq!(add_three::<usize>(192), 195);
    assert_eq!(add_three::<u32>(0x12345678), 0x1234567B);
}
