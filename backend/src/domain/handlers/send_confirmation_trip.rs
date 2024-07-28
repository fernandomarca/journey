use super::event_listener_trait::EventListener;
use crate::domain::events::domain_event_trait::DomainEvent;
use crate::domain::events::trip_created_event::TripCreatedEvent;
use crate::libs::mail::get_client_mail;
use chrono::format::StrftimeItems;
use chrono::Locale;
use lettre::message::MultiPart;
use lettre::message::SinglePart;
use lettre::{message::Mailbox, Message, Transport};
use tracing::error;

pub struct SendConfirmationTripHandler {}

impl SendConfirmationTripHandler {
    pub fn new() -> Self {
        SendConfirmationTripHandler {}
    }
}

impl EventListener for SendConfirmationTripHandler {
    fn on_event(&self, event: &DomainEvent) {
        let trip_event = event.to_struct::<TripCreatedEvent>();
        if let Ok(trip) = trip_event {
            let formatted_start_date = trip
                .starts_at
                .format_with_items(StrftimeItems::new_with_locale("%d %B %Y", Locale::pt_BR))
                .to_string();

            let formatted_end_date = trip
                .ends_at
                .format_with_items(StrftimeItems::new_with_locale("%d %B %Y", Locale::pt_BR))
                .to_string();

            let confirmation_link = format!("http://localhost:3333/trips/{}/confirm", trip.id);

            let mail = get_client_mail();

            let from_email = "Equipe plann.er <oi@plann.er>"
                .parse::<Mailbox>()
                .map_err(|e| {
                    error!("from_email parse error {}", e);
                })
                .unwrap();

            let to_email = format!("{} <{}>", trip.owner_name, trip.owner_email)
                .parse::<Mailbox>()
                .map_err(|e| {
                    error!("to_email parse error {}", e);
                })
                .unwrap();

            let html_content = r#"
                              <div style="font-family: sans-serif; font-size: 16px; line-height: 1.6;">
                                <p>Você solicitou a criação de uma viagem para <strong>{destination}</strong> nas datas de <strong>{starts_at}</strong> até <strong>{ends_at}</strong>.</p>
                                <p></p>
                                <p>Para confirmar sua viagem, clique no link abaixo:</p>
                                <p></p>
                                <p>
                                  <a href="{confirmationLink}">Confirmar viagem</a>
                                </p>
                                <p></p>
                                <p>Caso você não saiba do que se trata esse e-mail, apenas ignore esse e-mail.</p>
                              </div>
                          "#.trim().replace("{destination}", trip.destination.as_str()).replace("{starts_at}", formatted_start_date.as_str()).replace("{ends_at}", formatted_end_date.as_str()).replace("{confirmationLink}", &confirmation_link);

            let _send_email_result = tokio::task::spawn_blocking(move || {
                mail.send(
                    &Message::builder()
                        .from(from_email.clone())
                        .to(to_email)
                        .subject(format!(
                            "Confirme sua viagem para {} em {}",
                            trip.destination, formatted_end_date
                        ))
                        .multipart(
                            MultiPart::alternative()
                                .singlepart(SinglePart::html(html_content.to_string())),
                        )
                        .unwrap(),
                )
            });

            // para aguardar o envio do email chame o await
            // sem o await o email será enviado em background
            // match _send_email_result.await {
            //     Ok(resp) => debug!("Email sent successfully {resp:?}"),
            //     Err(e) => println!("Error sending email: {:?}", e),
            // }
        };
    }

    fn get_subject(&self) -> String {
        "trip_created_event".to_string()
    }
}
