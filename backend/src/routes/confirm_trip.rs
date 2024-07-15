use super::AppResult;
use super::Database;
use crate::libs::mail::get_client_mail;
use crate::libs::participant;
use crate::libs::trip;
use axum::extract::Path;
use axum::response::Redirect;
use chrono::format::StrftimeItems;
use chrono::Locale;
use futures::future::join_all;
use lettre::message::Mailbox;
use lettre::message::MultiPart;
use lettre::message::SinglePart;
use lettre::Message;
use lettre::Transport;
use uuid::Uuid;

pub async fn confirm_trip(db: Database, Path(trip_id): Path<Uuid>) -> AppResult<Redirect> {
    let trip = db
        .trip()
        .find_unique(trip::id::equals(trip_id.to_string()))
        .with(trip::participants::fetch(vec![
            participant::is_owner::equals(false),
        ]))
        .exec()
        .await
        .map_err(|e| format!("find error {}", e))?;

    match trip {
        Some(trip) if (trip.is_confirmed) => Ok(Redirect::to(
            format!("http://localhost:3000/trips/{}", trip_id).as_str(),
        )),
        Some(trip) => {
            db.trip()
                .update(
                    trip::id::equals(trip_id.to_string()),
                    vec![trip::is_confirmed::set(true)],
                )
                .exec()
                .await
                .map_err(|e| format!("update error {}", e))?;

            // Send email

            // let participants = db
            //     .participant()
            //     .find_many(vec![
            //         participant::trip_id::equals(trip.id),
            //         participant::is_owner::equals(false),
            //     ])
            //     .exec()
            //     .await
            //     .map_err(|e| format!("find participants error {}", e))?;
            let participants = trip.participants().ok();

            send_emails(participants, &trip).await;
            Ok(Redirect::to(
                format!("http://localhost:3000/trips/{}", trip_id).as_str(),
            ))
        }
        None => Err("Trip not found".to_string()),
    }
}

async fn send_emails(participants: Option<&Vec<participant::Data>>, trip: &trip::Data) {
    let mut tasks = Vec::new();

    for participant in participants.into_iter().flatten() {
        let mail = get_client_mail();
        let destination = trip.destination.clone();
        let formatted_start_date = trip
            .starts_at
            .format_with_items(StrftimeItems::new_with_locale("%d %B %Y", Locale::pt_BR))
            .to_string();
        let formatted_end_date = trip
            .ends_at
            .format_with_items(StrftimeItems::new_with_locale("%d %B %Y", Locale::pt_BR))
            .to_string();
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
      .replace("{formatted_start_date}", &formatted_start_date)
      .replace("{formatted_end_date}", &formatted_end_date)
      .replace("{confirmation_link}", &confirmation_link);

        let task = tokio::task::spawn_blocking(move || {
            let _message = mail.send(
                &Message::builder()
                    .from(from_email)
                    .to(to_email)
                    .subject(format!(
                        "Confirme sua presença na viagem para {} em {}",
                        &destination, &formatted_start_date
                    ))
                    .multipart(
                        MultiPart::alternative()
                            .singlepart(SinglePart::html(html_content.to_string())),
                    )
                    .unwrap(),
            );
        });
        tasks.push(task);
    }

    // for task in tasks {
    //     task.await.unwrap();
    // }
    let _results = join_all(tasks);
}
