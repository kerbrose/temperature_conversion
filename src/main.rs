use std::io;

fn main() {
    let mut conversion_type = String::new();
    loop {
        println!(
            "Type fc to convert Fahrenheit to Celsius, or cf to convert Celsius to Fahrenheit"
        );
        io::stdin()
            .read_line(&mut conversion_type)
            .expect("Wrong conversion format!");

        match conversion_type.trim() {
            "fc" => {
                conversion_type = "fc".to_string();
                break;
            }
            "cf" => {
                conversion_type = "cf".to_string();
                break;
            }
            _ => continue,
        }
    }
    loop {
        println!("Please input temperature");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read the line");

        let temperature: u32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if conversion_type == "fc" {
            let result = (temperature - 32) * 5 / 9;
            println!("The temperature in Celsius is: {}", result);
        } else if conversion_type == "cf" {
            let result = (temperature * 9 / 5) + 32;
            println!("The temperature in Fahrenheit is: {}", result);
        }
    }
}
