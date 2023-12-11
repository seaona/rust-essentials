fn main() {
    // no initial values, we annotate the type
    let v: Vec<i32> = Vec::new();

    // we use the macro vec!
    // we add initial values, so no type annotation is needed
    let v2 = vec![1, 2, 3];

    // updating a vector
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // reading elements
    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {third}");

    let third2: Option<&i32> = v4.get(2);
    match third2 {
        Some(third2) => println!("The third element is {third2}"),
        None => println!("There is no third element"),
    }


    //let does_not_exist1 = &v4[100]; //panic
    let does_not_exist2 = v4.get(100);


    // reference error
    let mut v5 = vec![1, 2, 3, 4, 5];
    let first = &v5[0]; // we take an immutable reference and then try to mutate it

    // adding a new element onto the end of the vector might require allocating new memory
    // and copying the old elements to the new space, if there isn't enough room to put all eelems next to each other
    // then the reference to the first element would be pointing to deallocated memory
    v5.push(6);
    //println!("The first element is: {first}");


    // iterating over values - immutable reference
    let v6 = vec![100, 32, 57];
    for i in &v6 {
        println!("{i}");
    }

    // iterating over values - mutable reference
    let v6 = vec![100, 32, 57];
    for i in &mut v6 {
        // we use the * dereference operator to get the value i before we can use the += operator
        *i += 50;
    }

}
