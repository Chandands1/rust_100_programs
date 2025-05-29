fn print_ascii_initials() {
    let r = r#"
RRRR   SSSS
R   R  S    
R   R  S    
RRRR   SSSS 
R  R      S
R   R     S
R    R SSSS"#;

    println!("{}", r);
}

fn main() {
    println!("ASCII Art Initials:\n");
    print_ascii_initials();
}