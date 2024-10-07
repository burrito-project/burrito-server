use rocket::{http::Status, Response, State};

use crate::core::responses::RawResponse;
use crate::core::AppState;
use crate::features::bot::entities::WhatsappMessage;
use crate::features::bus_driver::schemas::BusServiceState;
use crate::features::bus_status;

macro_rules! on_route_just_started_message_template {
    () => {
        "🚍 El burrito acaba de iniciar su ruta, vuelve en un momento"
    };
}

macro_rules! on_route_stationary_message_template {
    () => {
        r#"🚍 En el "{}", recogiendo pasajeros"#
    };
}

macro_rules! on_route_moving_message_template {
    () => {
        r#"🚍 Yendo hacia "{}"!
(a {} metros)"#
    };
}

macro_rules! out_of_service_message_template {
    () => {
        r#"🚍🪧 El transporte interno está fuera de servicio
Actualizado hace {}"#
    };
}

macro_rules! accident_message_template {
    () => {
        r#"🚍🪧 El transporte interno está temporalmente fuera de servicio
Actualizado hace {}"#
    };
}

macro_rules! resting_message_template {
    () => {
        r#"🚍💤 Los choferes se encuentran en descanso
Actualizado hace {}"#
    };
}

macro_rules! off_message_template {
    () => {
        r#"🚍 El burrito está apagado"#
    };
}

pub async fn whatsapp_new_text_message_handler(
    state: &State<AppState>,
    message: WhatsappMessage,
) -> RawResponse<'static> {
    let number_id = &message.entry[0].changes[0].value.metadata.phone_number_id;
    let message = &message.entry[0].changes[0].value.messages[0];

    let burrito_status = bus_status::handlers::get_burrito_status_handler(1, state).await;
    let last_stop = &burrito_status.last_stop;
    let pos = &burrito_status.positions[0];
    let updated_at = pos.formatted_time_ago();

    let formatted_message = match pos.sts {
        BusServiceState::OnRoute => {
            if last_stop.is_none() {
                on_route_just_started_message_template!().to_string()
            } else {
                let last_stop = last_stop.as_ref().unwrap();
                if last_stop.has_reached {
                    format!(on_route_stationary_message_template!(), last_stop.name,)
                } else {
                    format!(
                        on_route_moving_message_template!(),
                        last_stop.name, last_stop.distance as i64,
                    )
                }
            }
        }
        BusServiceState::OutOfService => {
            format!(out_of_service_message_template!(), updated_at)
        }
        BusServiceState::Resting => {
            format!(resting_message_template!(), updated_at)
        }
        BusServiceState::Accident => {
            format!(accident_message_template!(), updated_at)
        }
        BusServiceState::Off => off_message_template!().into(),
    };

    let body = if pos.sts == BusServiceState::OnRoute {
        serde_json::json!({
          "messaging_product": "whatsapp",
          "recipient_type": "individual",
          "to": message.from,
          "type": "interactive",
          "interactive": {
            "type": "button",
            "body": {
              "text": formatted_message,
            },
            "action": {
              "buttons": [
                {
                  "type": "reply",
                  "reply": {
                    "id": message.id,
                    "title": "Ver en mapa"
                  }
                },
              ]
            }
          }
        })
    } else {
        serde_json::json!({
          "messaging_product": "whatsapp",
          "recipient_type": "individual",
          "to": message.from,
          "type": "text",
          "text": {
            "body": formatted_message,
          }
        })
    };

    let client = reqwest::Client::new();
    let reply_result = client
        .post(format!(
            "https://graph.facebook.com/v20.0/{}/messages",
            number_id
        ))
        .header("content-type", "application/json")
        .header(
            "authorization",
            format!("Bearer {}", crate::env::AUTH_WHATSAPP_ACCESS_TOKEN.as_str()),
        )
        .body(body.to_string())
        .send()
        .await;

    match reply_result {
        Ok(w) => {
            println!("{:#?}", w);
        }
        Err(e) => {
            eprintln!("{:#?}", e);
        }
    }

    Response::build().status(Status::Ok).finalize().into()
}
