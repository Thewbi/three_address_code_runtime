use std::rc::Rc;
use std::cell::RefCell;

#[allow(dead_code)]
pub fn unwrap<T>(last_elem: Rc<RefCell<T>>) -> T {
    let inner: RefCell<T> = Rc::try_unwrap(last_elem)
        .unwrap_or_else(|_| panic!("The last_elem was shared, failed to unwrap"));
    inner.into_inner()
}