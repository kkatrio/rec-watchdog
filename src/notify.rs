use dotenv;
use mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;
use std::env;

pub async fn send_mail() {
    dotenv::dotenv().ok();
    let smtp_user = env::var("SMTP_USER").expect("SMTP_USER must be set in .env");
    let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set in .env");
    let from_addr = env::var("FROM_ADDR").expect("FROM_ADDR must be set in .env");
    let to_addr1 = env::var("TO_ADDR1").expect("TO_ADDR1 must be set in .env");
    let to_addr2 = env::var("TO_ADDR2").expect("TO_ADDR2 must be set in .env");

    // Build a simple multipart message
    let message = MessageBuilder::new()
        .from(("cam watch dog", from_addr.as_str()))
        .to(vec![
            ("kons", to_addr1.as_str()),
            ("gonzales", to_addr2.as_str()),
        ])
        .subject("Alarm")
        .html_body("<h1>Recordings are down!</h1>");
    //.text_body("One or all recordings have been shut down.");

    // Connect to the SMTP submissions port, upgrade to TLS and
    // authenticate using the provided credentials.
    SmtpClientBuilder::new("smtp.gmail.com", 587)
        .implicit_tls(false)
        .credentials((smtp_user.as_str(), smtp_pass.as_str()))
        .connect()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();
}
