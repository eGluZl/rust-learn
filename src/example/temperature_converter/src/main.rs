fn main() {
    loop {
        let mut converter_type = String::new();

        println!("Please select a type of convert.");

        println!("Input 0 means Celsius to Fahrenheit.");

        println!("Input 1 means Fahrenheit to Celsius");

        std::io::stdin().read_line(&mut converter_type).expect("Fail to read line.");

        let converter_type: u32 = match converter_type.trim().parse() {
            Ok(num) => {
                if num != 0 && num != 1 {
                    println!("Please input 0 or 1");
                    continue;
                } else {
                    num
                }
            }
            Err(_) => continue
        };

        let mut temperature = String::new();

        println!("Please input the temperature.");

        std::io::stdin().read_line(&mut temperature).expect("Fail to read line.");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let answer;

        if converter_type == 0 {
            answer = temperature * 1.8 + 32.0
        } else if converter_type == 1 {
            answer = (temperature - 32.0) / 1.8
        } else {
            answer = -1.0;
        }

        println!("The convert result is {}", answer);
    }
}
