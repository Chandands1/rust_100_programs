use rand::Rng;

fn coin_toss() -> &'static str {
    if rand::thread_rng().gen_bool(0.5) {
        "Heads"
    } else {
        "Tails"
    }
}

fn main() {
    println!("Coin Toss Simulator");
    println!("-------------------");
    
    let mut heads_count = 0;
    let mut tails_count = 0;
    let total_tosses = 10;

    for i in 1..=total_tosses {
        let result = coin_toss();
        println!("Toss {}: {}", i, result);
        
        match result {
            "Heads" => heads_count += 1,
            "Tails" => tails_count += 1,
            _ => unreachable!(),
        }
    }

    println!("\nResults:");
    println!("Heads: {} ({}%)", heads_count, heads_count * 100 / total_tosses);
    println!("Tails: {} ({}%)", tails_count, tails_count * 100 / total_tosses);
}