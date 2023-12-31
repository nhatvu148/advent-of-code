pub fn raw_pointers() {
    let num: i32 = 10;
    let num_ptr: *const i32 = &num;
    let mut num_2: i32 = 20;
    let num_ptr_2: *mut i32 = &mut num_2;

    let numb: Box<i32> = Box::new(30);
    let numb_ptr: *const i32 = &*numb;
    let mut numb_2: Box<i32> = Box::new(50);
    let numb_ptr_2: *mut i32 = &mut *numb_2;

    unsafe {
        println!("{}", *numb_ptr_2);
    }

    // as *mut [T] as *mut T
    let mut s = [1, 2, 3];
    let ptr: *mut i32 = s.as_mut_ptr();
    let first_element = unsafe { &mut *ptr };
    *first_element = 4;
    let third_element = unsafe { &mut *ptr.add(2) };
    *third_element = 18;
    println!("{s:?}");
}
