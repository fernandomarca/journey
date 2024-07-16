use super::AppJsonResult;
use super::Database;
use crate::libs::mail::get_client_mail;
use crate::libs::participant;
use axum::Json;
use chrono::format::StrftimeItems;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Locale;
use chrono::Utc;
use lettre::message::Mailbox;
use lettre::message::MultiPart;
use lettre::message::SinglePart;
use lettre::Message;
use lettre::Transport;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use validator::Validate;
use validator::ValidateEmail;
use validator::ValidationError;

pub async fn create_trip(
    db: Database,
    Json(input): Json<CreateTripRequest>,
) -> AppJsonResult<Value> {
    let command = input.self_validate()?;

    let (trip, _participant) = db
        ._transaction()
        .run(|tx| async move {
            let trip = tx
                .trip()
                .create(
                    command.destination,
                    command.starts_at,
                    command.ends_at,
                    vec![],
                )
                .exec()
                .await?;

            let mut participants = vec![participant::create_unchecked(
                command.owner_email,
                trip.id.to_owned(),
                vec![
                    participant::name::set(Some(command.owner_name)),
                    participant::is_confirmed::set(true),
                    participant::is_owner::set(true),
                ],
            )];

            command.emails_to_invite.iter().for_each(|email| {
                participants.push(participant::create_unchecked(
                    email.to_owned(),
                    trip.id.to_owned(),
                    vec![
                        participant::name::set(None),
                        participant::is_confirmed::set(false),
                        participant::is_owner::set(false),
                    ],
                ))
            });

            tx.participant()
                .create_many(participants)
                .exec()
                .await
                .map(|participant| (trip, participant))
        })
        .await
        .map_err(|e| format!("trip transaction error {}", e))?;

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
        .map_err(|e| format!("from_email parse error: {:?}", e))?;

    let to_email = format!("{} <{}>", input.owner_name, input.owner_email)
        .parse::<Mailbox>()
        .map_err(|e| format!("to_email parse error: {:?}", e))?;

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

    let _message = tokio::task::spawn_blocking(move || {
        mail.send(
            &Message::builder()
                .from(from_email)
                .to(to_email)
                .subject(format!(
                    "Confirme sua viagem para {} em {}",
                    trip.destination, formatted_end_date
                ))
                .multipart(
                    MultiPart::alternative().singlepart(SinglePart::html(html_content.to_string())),
                )
                .unwrap(),
        )
    });

    // para aguardar o envio do email chame o await
    // sem o await o email será enviado em background
    // match _message.await {
    //     Ok(resp) => debug!("Email sent successfully {resp:?}"),
    //     Err(e) => println!("Error sending email: {:?}", e),
    // }

    Ok(Json::from(json!({ "tripId": trip.id })))
}

#[derive(Deserialize, Validate, Clone)]
pub struct CreateTripRequest {
    #[validate(length(min = 4))]
    destination: String,

    #[validate(custom(function = "validate_datetime"))]
    starts_at: String,

    #[validate(custom(function = "validate_datetime"))]
    ends_at: String,

    #[validate(length(min = 4))]
    owner_name: String,

    #[validate(email)]
    owner_email: String,

    #[validate(custom(function = "validate_emails"))]
    emails_to_invite: Vec<String>,
}

impl CreateTripRequest {
    fn self_validate(&self) -> Result<CreateTripCommand, String> {
        self.validate().map_err(|e| e.to_string())?;
        CreateTripCommand::new(
            self.destination.to_owned(),
            DateTime::parse_from_rfc3339(&self.starts_at).unwrap_or_default(),
            DateTime::parse_from_rfc3339(&self.ends_at).unwrap_or_default(),
            self.owner_name.to_owned(),
            self.owner_email.to_owned(),
            self.emails_to_invite.to_owned(),
        )
    }
}

#[derive(Debug, Clone)]
struct CreateTripCommand {
    destination: String,
    starts_at: DateTime<FixedOffset>,
    ends_at: DateTime<FixedOffset>,
    owner_name: String,
    owner_email: String,
    emails_to_invite: Vec<String>,
}

impl CreateTripCommand {
    pub fn new(
        destination: String,
        starts_at: DateTime<FixedOffset>,
        ends_at: DateTime<FixedOffset>,
        owner_name: String,
        owner_email: String,
        emails_to_invite: Vec<String>,
    ) -> Result<Self, String> {
        let command = Self {
            destination,
            starts_at,
            ends_at,
            owner_name,
            owner_email,
            emails_to_invite,
        };
        if command.starts_at < Utc::now() {
            return Err("invalid trip start date.".to_string());
        }
        if command.ends_at < command.starts_at {
            return Err("invalid trip end date.".to_string());
        }
        Ok(command)
    }
}

fn validate_datetime(datetime: &str) -> Result<(), ValidationError> {
    let result = DateTime::parse_from_rfc3339(datetime);
    match result {
        Ok(_) => Ok(()),
        Err(_e) => Err(ValidationError::new("datetime parse error")),
    }
}

fn validate_emails(emails: &Vec<String>) -> Result<(), ValidationError> {
    for email in emails {
        if !ValidateEmail::validate_email(email) {
            return Err(ValidationError::new("invalid email"));
        }
    }
    Ok(())
}
