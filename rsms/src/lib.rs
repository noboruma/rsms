extern crate dlopen;

use dlopen::symbor::Library;

struct VTable {
    fns: std::vec::Vec<Box<dyn std::any::Any>>,
}

impl VTable {
    fn get_fn<T: 'static>(&self, index: usize) -> &T {
        let anyfn = &self.fns[index];
        return anyfn.downcast_ref::<T>().unwrap();
    }
}

trait IBundle {
    fn get_lib(&self) -> &std::sync::Arc<Library>;
    fn get_vtable(&self) -> &VTable;
    // TODO: getCacher
    fn start(&self) {
        let lib = self.get_lib().clone();
        let vtable = &self.get_vtable();
        let f = unsafe {lib.symbol::<unsafe extern "C" fn()>("stop").unwrap()};
        vtable.fns.push(Box::new(f));

        //let fun = unsafe {self.get_lib().symbol::<unsafe extern "C" fn()>("start").unwrap()};
        //let vtable = &self.get_vtable();
        //&self.get_vtable().fns.push(Box::new(fun));
        //vtable.fns.push(Box::new(unsafe { ));
        //unsafe {&self.get_lib().symbol::<unsafe extern "C" fn()>("start").unwrap()()};
    }

    fn stop(&self) {
        unsafe {&self.get_lib().symbol::<unsafe extern "C" fn()>("stop").unwrap()()};
    }
}

pub struct Bundle {
    lib: std::sync::Arc<Library>,
    vtable: VTable,
}

impl IBundle for Bundle {
    fn get_lib(&self) -> &std::sync::Arc<Library> {
        return &self.lib;
    }
    fn get_vtable(&self) -> &VTable {
        return &self.vtable;
    }
}

pub fn install(path : &std::path::Path) -> Bundle {
    let lib  = std::sync::Arc::new(Library::open(path).unwrap());
    return Bundle{lib: lib, vtable: VTable{fns: std::vec::Vec::new()}};
}

pub fn call_start_stop(a :&IBundle) {
    a.start();
    a.stop();
}

fn foo (x :i32) -> i32 {
    return x + 42;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let bundle = install(std::path::Path::new("/tmp/test.so"));
        call_start_stop(&bundle);

        let f : Box<fn(i32)->i32> = Box::new(|x:i32 | {return x + 42});
        //let a = f as Box<dyn std::any::Any>;

        //let r = a.downcast_ref::<fn(i32)->i32>().unwrap();

        let v = vec![f as Box<dyn std::any::Any>, Box::new(||{}) as Box<dyn std::any::Any>];
        let vtable = VTable {fns: v};

        let f = vtable.get_fn::<fn(i32) -> i32>(0);
        assert!(f(1) == 43);
    }
}
