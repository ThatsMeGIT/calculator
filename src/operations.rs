pub fn addition(x: i32, y: i32) -> i32 {
    x + y
}

pub fn subtraction(x: i32, y: i32) -> i32 {
    x - y
}

pub fn multiplication(x: i32, y: i32) -> i32 {
    x * y
}

pub fn division(x: i32, y: i32) -> Option<i32> {
    (y != 0).then(|| x / y)
}
