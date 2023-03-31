use signalbot_rs;
use dotenv::dotenv;
use envy;
use serde::Deserialize;


fn main() {
    println!("Hello, from Test!");
    let config = load_env();
    signalbot_rs::send(config.receiver_phone_number, "Hello from Test!".to_string(), config.signal_service);

}


fn load_env() -> Configuration {
    dotenv().ok();
    let c = envy::prefixed("SB__")
        .from_env::<Configuration>()
        .expect("Please provide SB__SIGNAL_SERVICE, SB__RECEIVER_PHONE_NUMBER and SB__SENDER_PHONE_NUMBER env vars");
    println!("Config used to run signalbot-wa: {:#?}", c);
    c
}

#[derive(Deserialize, Debug)]
struct Configuration {
    #[serde(default="default_signal_service")]
    signal_service: String,
    receiver_phone_number: String,
    sender_phone_number: String,
}

fn default_signal_service() -> String {
    "127.0.0.1:8080".to_string()
}