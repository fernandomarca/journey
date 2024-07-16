use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;

pub fn get_client_mail() -> SmtpTransport {
    let credentials = Credentials::new(
        "garret.mohr32@ethereal.email".to_string(),
        "nkntJtFZFk9khVm4kV".to_string(),
    );
    SmtpTransport::starttls_relay("smtp.ethereal.email")
        .expect("failed to create transport")
        .credentials(credentials)
        .build()
}
