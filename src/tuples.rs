pub fn run() {
    // Tupla: puede guardar hasta 12 valores
    // Se accede como si fuera un json (.), pero tiene las posiciones como un array
    // [0],[1],[2]...

    let person: (&str, &str, i8) = ("Brad", "Mass", 57);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}
