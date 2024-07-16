use super::AppJsonResult;
use super::Database;
use crate::libs::mail::get_client_mail;
use crate::libs::prisma::participant;
use crate::libs::trip;
use axum::extract::Path;
use axum::Json;
use chrono::format::StrftimeItems;
use chrono::Locale;
use futures::future::join_all;
use lettre::message::Mailbox;
use lettre::message::MultiPart;
use lettre::message::SinglePart;
use lettre::Message;
use lettre::Transport;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;
use validator::Validate;

pub async fn create_invite(
    db: Database,
    Path(trip_id): Path<Uuid>,
    Json(input): Json<CreateInviteRequest>,
) -> AppJsonResult<Value> {
    let command = input.self_validate()?;

    let trip = db
        .trip()
        .find_unique(trip::id::equals(trip_id.to_string()))
        .exec()
        .await
        .map_err(|e| format!("find error {}", e))?;

    match trip {
        Some(trip) => {
            let participant = db
                .participant()
                .create(command.email, trip::id::equals(trip_id.to_string()), vec![])
                .exec()
                .await
                .map_err(|e| format!("create error {}", e))?;

            send_emails(&[participant.clone()], &trip).await;

            Ok(Json::from(json!({ "participantId": participant.id })))
        }
        None => Err("trip not found".to_string()),
    }
}

#[derive(Deserialize, Validate, Clone)]
pub struct CreateInviteRequest {
    #[validate(email)]
    email: String,
}

impl CreateInviteRequest {
    fn self_validate(&self) -> Result<CreateInviteCommand, String> {
        self.validate().map_err(|e| e.to_string())?;

        Ok(CreateInviteCommand::new(self.email.to_owned()))
    }
}

#[derive(Debug, Clone)]
struct CreateInviteCommand {
    email: String,
}

impl CreateInviteCommand {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}

async fn send_emails(participants: &[participant::Data], trip: &trip::Data) {
    let mut tasks = Vec::new();

    let formatted_start_date = trip
        .starts_at
        .format_with_items(StrftimeItems::new_with_locale("%d %B %Y", Locale::pt_BR))
        .to_string();
    let formatted_end_date = trip
        .ends_at
        .format_with_items(StrftimeItems::new_with_locale("%d %B %Y", Locale::pt_BR))
        .to_string();

    let destination = trip.destination.clone();

    for participant in participants.iter() {
        let mail = get_client_mail();

        let name = participant.name.clone().unwrap_or_default();
        let email = &participant.email;
        let participant_id = &participant.id;

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
                MultiPart::alternative().singlepart(SinglePart::html(html_content.to_string())),
            )
            .unwrap();

        let task = tokio::task::spawn_blocking(move || {
            let _message = mail.send(&message);
        });
        tasks.push(task);
    }
    let _results = join_all(tasks);
}
