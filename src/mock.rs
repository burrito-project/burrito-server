use lazy_static::lazy_static;
use std::time;

use crate::entities::burrito_state_record::BurritoStateRecord;

lazy_static! {
    static ref s_mock_records: Vec<BurritoStateRecord> =
        serde_json::from_str::<Vec<BurritoStateRecord>>(include_str!(
            "../static/mocks/route1.json"
        ))
        .unwrap()
        .iter()
        .rev() // reverse to get older to newer
        .cloned()
        .collect::<Vec<_>>()
    ;
    static ref MOCK_END_TIME: time::SystemTime = {
        s_mock_records
            .iter()
            .last()
            .unwrap()
            .timestamp
    };
    static ref MOCK_START_TIME: time::SystemTime = {
        s_mock_records
            .iter()
            .next()
            .unwrap()
            .timestamp
    };
}

pub fn initialize_mocks() {
    if !*crate::env::IS_MOCKED {
        return;
    }

    println!("🐢 Initializing mocked route");

    if s_mock_records.is_empty() {
        return;
    }

    let client = reqwest::Client::new();
    let mut record_idx = 0;
    let mut last_startup = std::time::SystemTime::now();

    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(300));

        loop {
            interval.tick().await;

            if record_idx == s_mock_records.len() {
                record_idx = 0;
                last_startup = std::time::SystemTime::now();
                continue;
            }

            let now = std::time::SystemTime::now();
            let elapsed = now.duration_since(last_startup).unwrap();

            let current_record = &s_mock_records[record_idx];

            let fake_elapsed = current_record
                .timestamp
                .duration_since(*MOCK_START_TIME)
                .unwrap();

            if elapsed > fake_elapsed {
                client
                    .post(format!("{}/status", crate::SELF_URL))
                    .body(
                        serde_json::json!({
                            "lt": current_record.lt,
                            "lg": current_record.lg,
                            "sts": current_record.sts,
                        })
                        .to_string(),
                    )
                    .header("content-type", "application/json")
                    .header("authorization", crate::env::AUTH_DRIVER_PASSPHRASE.clone())
                    .send()
                    .await
                    .unwrap();

                record_idx += 1;
            }
        }
    });
}
