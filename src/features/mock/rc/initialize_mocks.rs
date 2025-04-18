use lazy_static::lazy_static;
use std::time;

use crate::features::bus_driver::schemas::BurritoPosRecord;

lazy_static! {
    static ref s_mock_records: Vec<BurritoPosRecord> =
        serde_json::from_str::<Vec<BurritoPosRecord>>(include_str!(
            "../assets/route1.json"
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

/// If the app is running in mock mode, a fake bus route is loaded from the `route1.json` file
/// and start sending requests to the driver endpoint to simulate the bus route.
///
/// This is also a great example of how easy a driver client could be implemented.
pub fn initialize_mocks() {
    if !*crate::env::IS_MOCKED {
        return;
    }

    if s_mock_records.is_empty() {
        return;
    }

    println!("🐢 Initializing mocked route");

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
                let _ = client
                    .post(format!("{}/driver", *crate::env::SELF_URL))
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
                    .header("x-bus-id", "burrito_mock")
                    .send()
                    .await;

                record_idx += 1;
            }
        }
    });
}
