fn main() {
    println!(include_str!("123456789123456"));

    let token = std::env::var("").expect("");

    Client::builder(token)
        .event_handler(DiscordEventHandler)
        .expect("                                  ア");
}
