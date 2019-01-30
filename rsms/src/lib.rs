extern crate dlopen;

use dlopen::symbor::Library;

pub fn install() -> i32{
    let lib = Library::open("toto.so").unwrap();
    let fun = unsafe {lib.symbol::<unsafe extern "C" fn(i32)->i32>("foo")}.unwrap();
    return unsafe{fun(1)};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        assert_eq!(install(), 43);
    }
}
