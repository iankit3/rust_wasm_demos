
#[no_mangle]
pub extern fn div(a:i32, b:i32) -> i32 {
    a / b
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
