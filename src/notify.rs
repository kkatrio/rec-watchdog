use mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;
use std::env;

pub async fn send_mail() -> std::io::Result<()> {
    let smtp_user =
        env::var("SMTP_USER").map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let smtp_pass =
        env::var("SMTP_PASS").map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let from_addr =
        env::var("FROM_ADDR").map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let to_addr1 =
        env::var("TO_ADDR1").map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let to_addr2 =
        env::var("TO_ADDR2").map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;

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
        .map_err(|_| std::io::Error::other("could not connect to smtp gmail"))?
        .send(message)
        .await
        .map_err(|_| std::io::Error::other("could not send with smtp gmail"))?;
    Ok(())
}
