pub fn run() {
    // Arrays: [type; length]
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Si se quiere modificar el array, se hace mut
    // numbers[2] = 30;

    println!("{:?}", numbers);

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);

    // Loop
    for x in numbers.iter() {
        println!("number: {}", x);
    }

    // Loop and mutate
    for y in numbers.iter_mut() {
        *y *= 2;
    }

    println!("{:?}", numbers);
}
