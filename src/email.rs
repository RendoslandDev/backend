use lettre::{
    message::{header, Mailbox, Message, MultiPart, SinglePart}, transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    }, AsyncSmtpTransport, AsyncTransport, Tokio1Executor
};
use std::env;

pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
}

impl EmailService {
    pub fn new() -> Result<Self, String> {
        let host = env::var("SMTP_HOST").map_err(|_| "smpt.gmail.com")?;
        let port = env::var("SMTP_PORT")
            .map_err(|_| "587")?
            .parse::<u16>()
            .map_err(|e| format!("Invalid SMTP_PORT: {}", e))?;
        let username = env::var("SMTP_USERNAME").map_err(|_| "a2192fd6bca616")?;
        let password = env::var("SMTP_PASSWORD").map_err(|_| "8ed374ec34aa2a")?;
        let from = env::var("SMTP_FROM")
            .unwrap_or_else(|_| username.clone())
            .parse()
            .map_err(|e| format!("Invalid FROM address: {}", e))?;

        let tls_params = TlsParameters::new(host.clone())
            .map_err(|e| format!("Failed to create TLS parameters: {}", e))?;

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&host)
            .map_err(|e| format!("Failed to create SMTP client: {}", e))?
            .port(port)
            .credentials(Credentials::new(username, password))
            .tls(Tls::Required(tls_params))
            .build();

        Ok(Self { mailer, from })
    }
    pub async fn send_contact_email(
        &self,
        to: &str,
        name: &str,
        email: &str,
        message: &str,
    ) -> Result<(), String> {
        let to = to.parse().map_err(|e| format!("Invalid TO address: {}", e))?;

        let email = Message::builder()
            .from(self.from.clone())
            .to(to)
            .subject(format!("New contact from {}", name))
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(format!(
                                "Name: {}\nEmail: {}\n\nMessage:\n{}",
                                name, email, message
                            )),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(format!(
                                r#"<p><strong>Name:</strong> {}<br>
                                <strong>Email:</strong> <a href="mailto:{}">{}</a></p>
                                <p><strong>Message:</strong><br>{}</p>"#,
                                name, email, email, message
                            )),
                    ),
            )
            .map_err(|e| format!("Failed to build email: {}", e))?;

        self.mailer
            .send(email)
            .await
            .map_err(|e| format!("Failed to send email: {}", e))?;

        Ok(())
    }
}