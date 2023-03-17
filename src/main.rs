use std::io;

fn main() {
    println!("Program to convert temperatures between Fahrenheit and Celsius");

    loop {
        println!("Which option do you choose:");

        println!("Fahrenheit to Celsius - type 1");
        println!("Celcisus to Fahrenheit - type 2");
        println!("Quit - type 3");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 3 {
            break;
        } else {
            println!("Please enter a temperature:  ");

            let mut temp = String::new();
            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");

            let temp: f64 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match choice {
                1 => println!(
                    "Temperature in {temp} Fahrenheit is {} Celsius",
                    fah_to_cel(temp)
                ),
                2 => println!(
                    "Temperature in {temp} Celsius is {} Fahrenheit",
                    cel_to_fah(temp)
                ),
                _ => continue,
            }
        }
    }
}

fn fah_to_cel(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn cel_to_fah(temp: f64) -> f64 {
    1.8 * temp + 32.0
}

