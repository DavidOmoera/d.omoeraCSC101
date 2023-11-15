pub mod french;
pub mod spanish;

/// `default_greeting` function simply returns a String.
pub fn default_greeting() -> String {
    let message = "Hello, World!".to_string(); // You can change the message to whatever you want.
    message
}

/// `default_greeting2` function returns a String and takes an optional message parameter.
pub fn default_greeting2(message: Option<String>) -> String {
    match message {
        Some(msg) => msg,
        None => "Hello, World!".to_string(), // You can change the default message here.
    }
}
