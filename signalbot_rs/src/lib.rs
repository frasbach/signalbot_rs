use ureq::{Error, post};

pub fn send(receiver: String, message: String, signal_service: String){
    let resolved_receiver = resolve_receiver(receiver);
    let uri = format!("http://{}/v2/send", signal_service);
    match post(&*uri).send_json(ureq::json!({
        "base64_attachments": [],
        "message": message,
        "number": resolved_receiver,
        "recipients": [resolved_receiver],
    })) {
        Ok(_response) => {
            println!("Sending was successfully!");
        },
        Err(Error::Status(code, response)) => {
            println!("Error: {} - {}", code, response.status_text());
        }
        Err(_) => {
            println!("Some IO-Error: occurred");
        }
    };
}

fn resolve_receiver(receiver: String) -> String {
    if is_phone_number(&receiver) {
        return receiver;
    } else if receiver.starts_with("group.") && receiver.ends_with("==")  {
        return receiver;
    }
    else {
        panic!("Invalid receiver: {} - seems to be neither a phone_number nor a group_id", receiver);
    }
}

fn is_phone_number(receiver: &String) -> bool {
    if !receiver.starts_with("+") {
        return false;
    } else if receiver[1..].chars().count() > 15 {
        return false;
    }
    true
}

