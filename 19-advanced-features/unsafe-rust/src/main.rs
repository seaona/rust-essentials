fn main() {
    // dereferencing a raw pointer
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

        // trying to use arbitrary memory is undefined
    let address = 0x012345usize;
    let r = address as *const i32;

        // we dereference a raw pointer in an unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
        // use case: when interfacing with C code
        // use casE: when buildoing up safe abstractions that the borrow checker doesn't understand
    
    // unsafe function or method
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // creating a safe abstracion over unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();

        assert!(mid <= len);

        // we arre borrowing from the same slice twice
        // but we are borrowing different parts of the slice
        // so we can use unsafe here
        // (&mut values[..mid], &mut values[mid..])
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    // Foreign Function Interface (FFI)
    extern "C" {
        // we list the names and signatures of external functions from another language
        // this defiens the application binary interface (ABI) the external function uses
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // global variables (static variables)
    static HELLO_WORLD: &str = "Hello, world!";

    println!("name is: {}", HELLO_WORLD);

    static mut COUNTER : u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // unsafe trait
    unsafe trait Foo {

    }

    unsafe impl Foo for i32 {
        
    }
}
