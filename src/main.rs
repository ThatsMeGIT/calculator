use std::io::stdin;
mod operations;

fn main() {
    let mut operation: i32;
    let mut x: i32;
    let mut y: i32;
    let mut ergebnis: i32 = 0;
    let mut input = String::new();

    println!("Hallo, ich bin dein Taschenrechner.\n");
    println!("Gib mir gern eine Aufgabe.\n");

    loop {
        println!(
            "\n Bitte wähle deine Operation
    \n 1 => Addition
    \n 2 => Subtraction
    \n 3 => Multiplication
    \n 4 => Division"
        );

        stdin().read_line(&mut input).unwrap();
        operation = input.trim().parse::<i32>().unwrap();

        if operation > 0 && operation < 5 {
            input.clear();
            break;
        } else {
            input.clear();
            println!("\n Die angegebene Zahl ist leider nicht auswählbar")
        }
    }

    println!("\n Bitte gib deine erste Zahl ein");
    stdin().read_line(&mut input).unwrap();
    x = input.trim().parse::<i32>().unwrap();

    input.clear();

    println!("\n Bitte gib deine zweite Zahl ein");
    stdin().read_line(&mut input).unwrap();
    y = input.trim().parse::<i32>().unwrap();

    match operation {
        1 => ergebnis = operations::addition(x, y),
        2 => ergebnis = operations::subtraction(x, y),
        3 => ergebnis = operations::multiplication(x, y),
        4 => ergebnis = operations::division(x, y).expect("Teiler ist eine 0"),
        _ => {}
    }

    println!("Operation: {} \nZahl 1: {} n \nZahl 2: {}", operation, x, y);
    println!("Das Eregebniss ist: {}", ergebnis);
}
