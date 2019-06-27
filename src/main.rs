mod crawler;
mod core;
mod console;

fn main() {
    use crawler::util::*;
    let parse_data = read_file_as_string("MLT106.org").unwrap();
    //println!("{:?}", parse_data);
    use crawler::data::Config;
    let config = Config::from_file("borg.toml").unwrap();
    //println!("{:?}", config);
    use crawler::parse;
    println!("{:?}", parse::flashcards(&parse_data, config.flash));
}
