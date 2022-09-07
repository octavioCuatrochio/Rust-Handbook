// primitive str = fixed size, located in memory.
// String = growable, used when you need to modify your strings
pub fn run() {
    // primitive str
    let _hello = "Hello";

    // String
    let mut alo = String::from("Alo");

    // Push char
    alo.push('a');

    // Push String
    alo.push_str("lolol");

    println!("{}", alo);

    println!("Is empty: {}", alo.is_empty());

    println!("Contains 'Alo': {}", alo.contains("Alo"));

    // Get Length
    println!("Length: {}", alo.len());
}
