use std::io;

fn main() {
    println!("How many grams of flour are you baking with?");

    let mut grams_of_flour: String = String::new();
    io::stdin().read_line(&mut grams_of_flour).expect("Failed to read line.");
    let grams_of_flour = match parse_user_input(grams_of_flour) {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("Entered {grams_of_flour} grams of flour");

    println!("What percentage hydration?");
    let mut hydration: String = String::new();
    io::stdin().read_line(&mut hydration).expect("Failed to read line.");
    let hydration = match parse_user_input(hydration) {
        Ok(num) => num,
        Err(_) => 0,
    } as f64 / (100 as f64);

    println!("Entered {:.2} hydration", hydration);

    println!("How much started are you using? For current itteration calculator to work, use 50/50 split of water to flour.");
    let mut starter: String = String::new();
    io::stdin().read_line(&mut starter).expect("Failed to read line.");
    let starter = match parse_user_input(starter) {
        Ok(num) => num,
        Err(_) => 0,
    };
    
    println!("Entered {starter} grams of starter");

    println!("Your sourdough recipe looks as following:");
    println!("{} grams of flour", grams_of_flour - (starter / 2));
    println!("{} grams of water", ((grams_of_flour as f64) * hydration) - ((starter / 2)) as f64);
    println!("{} grams of salt", grams_of_flour as f64 * 0.02);
    println!("{} grams of sourdough starter", starter);

    fn parse_user_input(input: String) -> Result<u32, std::num::ParseIntError>{
        input.trim().replace(" ", "").parse()
    }
}
