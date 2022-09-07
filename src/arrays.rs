pub fn run() {
    // Arrays: [type; length]
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Si se quiere modificar el array, se hace mut
    // numbers[2] = 30;

    println!("{:?}", numbers);

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);
}
