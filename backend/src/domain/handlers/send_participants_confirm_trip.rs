use super::event_listener_trait::EventListener;
use crate::application::participant_service::ParticipantService;
use crate::domain::events::domain_event_trait::DomainEvent;
use crate::domain::events::trip_confirmed_event::TripConfirmedEvent;
use crate::domain::participant::Participant;
use crate::libs::mail::get_client_mail;
use chrono::format::StrftimeItems;
use chrono::Locale;
use futures::future::join_all;
use lettre::message::Mailbox;
use lettre::message::MultiPart;
use lettre::message::SinglePart;
use lettre::Message;
use lettre::Transport;
use std::sync::Arc;

pub struct SendParticipantsConfirmTripHandler {
    pub participant_service: Arc<ParticipantService>,
}

impl SendParticipantsConfirmTripHandler {
    pub fn new(participant_service: Arc<ParticipantService>) -> Self {
        SendParticipantsConfirmTripHandler {
            participant_service,
        }
    }
}

impl EventListener for SendParticipantsConfirmTripHandler {
    fn on_event(&self, event: &DomainEvent) {
        let event_data = event.to_struct::<TripConfirmedEvent>();

        if let Ok(event_data) = event_data {
            let destination = event_data.destination;
            let formatted_start_date = event_data
                .starts_at
                .format_with_items(StrftimeItems::new_with_locale("%d %B %Y", Locale::pt_BR))
                .to_string();
            let formatted_end_date = event_data
                .ends_at
                .format_with_items(StrftimeItems::new_with_locale("%d %B %Y", Locale::pt_BR))
                .to_string();

            send_emails(
                self.participant_service.clone(),
                &event_data.id.to_string(),
                destination,
                formatted_start_date,
                formatted_end_date,
            );
        }
    }

    fn get_subject(&self) -> String {
        "trip_confirmed_event".to_string()
    }
}

fn send_emails(
    participant_service: Arc<ParticipantService>,
    trip_id: &str,
    destination: String,
    formatted_start_date: String,
    formatted_end_date: String,
) {
    // buscar participants is_owner: false
    let trip_id = trip_id.to_owned();
    let task = tokio::task::spawn(async move {
        let mut tasks = Vec::new();

        let participants = participant_service
            .find_participants_by_trip_id(&trip_id.to_owned())
            .await;

        if let Ok(participants) = participants {
            for participant in participants.iter() {
                let mail = get_client_mail();

                let name = participant.name().map_or("", |v| v);
                let email = &participant.email();
                let participant_id = &participant.id();

                let from_email = "Equipe plann.er <oi@plann.er>"
                    .parse::<Mailbox>()
                    .map_err(|e| format!("from_email parse error: {:?}", e))
                    .unwrap();

                let to_email = format!("{} <{}>", name, email)
                    .parse::<Mailbox>()
                    .map_err(|e| format!("to_email parse error: {:?}", e))
                    .unwrap();

                let confirmation_link =
                    format!("http://localhost:3333/participants/{participant_id}/confirm");

                let html_content = r#"
                  <div style="font-family: sans-serif; font-size: 16px; line-height: 1.6;">
                    <p>Você foi convidado(a) para participar de uma viagem para <strong>{destination}</strong> nas datas de <strong>{formatted_start_date}</strong> até <strong>{formatted_end_date}</strong>.</p>
                    <p></p>
                    <p>Para confirmar sua presença na viagem, clique no link abaixo:</p>
                    <p></p>
                    <p>
                      <a href="{confirmation_link}">Confirmar viagem</a>
                    </p>
                    <p></p>
                    <p>Caso você não saiba do que se trata esse e-mail, apenas ignore esse e-mail.</p>
                  </div>
                "#.trim().replace("{destination}", &destination)
                .replace("{formatted_start_date}",&formatted_start_date)
                .replace("{formatted_end_date}", &formatted_end_date)
                .replace("{confirmation_link}", &confirmation_link);

                let message = Message::builder()
                    .from(from_email)
                    .to(to_email)
                    .subject(format!(
                        "Confirme sua presença na viagem para {} em {}",
                        destination, formatted_start_date
                    ))
                    .multipart(
                        MultiPart::alternative()
                            .singlepart(SinglePart::html(html_content.to_string())),
                    )
                    .unwrap();

                let task = tokio::task::spawn_blocking(move || {
                    let _message = mail.send(&message);
                });
                tasks.push(task);
            }

            // for task in tasks {
            //     task.await.unwrap();
            // }
            let _results = join_all(tasks);
        }
    });
}
