extern crate dlopen;

use dlopen::symbor::Library;

trait IBundle {
    fn start();
    fn stop();
}

pub struct Bundle {
    lib: std::sync::Arc<Library>,
    start: Box<Fn()>,
    stop: Box<Fn()>,
}


pub fn install(path : &std::path::Path) -> Bundle {
    let lib  = std::sync::Arc::new(Library::open(path).unwrap());
    let libc = lib.clone();
    let start= move || { unsafe {libc.symbol::<unsafe extern "C" fn()>("start").unwrap()()}; };
    let libc = lib.clone();
    let stop = move || { unsafe {libc.symbol::<unsafe extern "C" fn()>("stop").unwrap()()}; };
    return Bundle{start:Box::new(start), stop:Box::new(stop), lib:lib.clone()};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        assert_eq!(install(), 43);
    }
}
