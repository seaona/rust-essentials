use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPoiners created.");

    drop(c);

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPoiners created.");
}

// at the end of main, c and d will go out of scope, so then the drop will run
// we don't need to call the drop method explicitly