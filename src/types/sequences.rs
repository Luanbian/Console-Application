pub fn example() {
    let tuple = (1, 2, 3);
    println!("Tuple: {:?}", tuple);
    println!("Tuple.0: {}", tuple.0);

    let (a, b, c) = tuple;
    println!("a: {}, b: {}, c: {}", a, b, c);

    let mut array = [0; 10];
    println!("Array: {:?}", array);

    array[0] = 1;
    array[3] = 4;
    array[9] = 10;
    println!("Array: {:?}", array);

    let mut slice = &array[1..4];   
    println!("Slice: {:?}", slice);

    slice = &array[2..5];
    slice.iter().for_each(|x| print!("{} ", x));
    println!();

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Vec: {:?}", vec);

    let mut vec = vec![1, 2, 3];
    vec.push(10);
    println!("Vec: {:?}", vec);
    println!("Vec.pop: {}", vec.pop().unwrap());
}