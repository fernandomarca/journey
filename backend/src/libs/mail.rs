use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;

pub fn get_client_mail() -> SmtpTransport {
    let credentials = Credentials::new(
        "emory.zulauf47@ethereal.email".to_string(),
        "8HyCRjZ3GKMYUj684b".to_string(),
    );
    SmtpTransport::starttls_relay("smtp.ethereal.email")
        .expect("failed to create transport")
        .credentials(credentials)
        .build()
}
