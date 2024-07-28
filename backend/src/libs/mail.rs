use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;

pub fn get_client_mail() -> SmtpTransport {
    let credentials = Credentials::new(
        "halle.kiehn@ethereal.email".to_string(),
        "aVUHDJT76RC5CfcdxJ".to_string(),
    );
    SmtpTransport::starttls_relay("smtp.ethereal.email")
        .expect("failed to create transport")
        .credentials(credentials)
        .build()
}
