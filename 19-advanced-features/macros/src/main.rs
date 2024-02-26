fn main() {
    let v: Vec<u32> = vec![1, 2, 3];
}

// we use $ to declare a variable in the macro system
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// procedural macros
use proc_macro;

#[some_attribure]
pub fn some_name(input: TokenStream) -> TokenStream {
    // TokenStream type is defined in proc_macro crate
}