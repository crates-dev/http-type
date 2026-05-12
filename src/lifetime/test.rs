use crate::*;

struct TestLifetimeStruct {
    value: i32,
}

impl Lifetime for TestLifetimeStruct {
    unsafe fn leak(&self) -> &'static Self {
        let boxed: Box<Self> = Box::new(Self { value: self.value });
        Box::leak(boxed)
    }
    unsafe fn leak_mut(&self) -> &'static mut Self {
        let mut boxed: Box<Self> = Box::new(Self { value: self.value });
        let reference: *mut Self = std::ptr::addr_of_mut!(*boxed);
        std::mem::forget(boxed);
        unsafe { &mut *reference }
    }
}

#[test]
fn test_lifetime_trait_leak() {
    let data: TestLifetimeStruct = TestLifetimeStruct { value: 42 };
    let leaked: &'static TestLifetimeStruct = unsafe { data.leak() };
    assert_eq!(leaked.value, 42);
}

#[test]
fn test_lifetime_trait_leak_mut() {
    let data: TestLifetimeStruct = TestLifetimeStruct { value: 10 };
    let leaked: &'static mut TestLifetimeStruct = unsafe { data.leak_mut() };
    assert_eq!(leaked.value, 10);
    leaked.value = 99;
    assert_eq!(leaked.value, 99);
}

#[test]
fn test_lifetime_trait_leak_preserves_value() {
    let data: TestLifetimeStruct = TestLifetimeStruct { value: 100 };
    let leaked: &'static TestLifetimeStruct = unsafe { data.leak() };
    assert_eq!(leaked.value, 100);
}
