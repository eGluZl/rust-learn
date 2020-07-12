fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let parts = ["partridge in a pear tree", "Two turtle doves", "Three French hens", "Four calling birds", "Five gold rings",
        "Six geese a laying", "Seven swans a swimming", "Eight maids a milking", "Nine ladies dancing", "Ten lords a leaping", "Eleven pipers piping", "12 drummers drumming"];

    let in_part_9 = "Me me me me me me";

    let in_part_expect_9 = "My true love gave to me";

    for day in 0..12 {
        println!("On the {} day of Christmas", days[day]);
        if day == 8 {
            println!("{}", in_part_9);
        } else {
            println!("{}", in_part_expect_9);
        }

        let first_part = parts[0];


        for i in (0..day + 1).rev() {
            if i == 0 {
                if day == 0 {
                    println!("A {}", first_part);
                } else {
                    println!("And a {}", first_part);
                }
            } else if day >= 8 && i == 4 {
                println!("{}, badam-pam-pam", parts[4]);
            } else {
                println!("{}", parts[i]);
            }
        }


        println!();
    }
}
