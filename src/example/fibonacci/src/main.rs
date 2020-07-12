fn main() {
    loop {
        let mut level = String::new();

        println!("Please input the level of fibonacci.");

        std::io::stdin().read_line(&mut level).expect("Fail to read line.");

        let level: i64 = match level.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let mut ans = Vec::with_capacity(level as usize);

        for l in 0..level {
            let a = gen(l);
            ans.push(a)
        }

        for f in ans.iter() {
            print!("{},", f);
        }
        println!();
    }
}

fn gen(level: i64) -> i64 {
    return if level < 0 {
        -1
    } else if level == 0 {
        0
    } else if level == 1 || level == 2 {
        1
    } else {
        gen(level - 1) + gen(level - 2)
    };
}
