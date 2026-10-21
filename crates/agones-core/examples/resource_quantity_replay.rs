//! Ordinary pinned-source replay; no original-test or formal-proof credit.
use agones_core::ResourceQuantity;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Case {
    value: String,
    other: String,
}

fn main() {
    let cases: Vec<Case> = serde_json::from_reader(std::io::stdin()).unwrap();
    let rows: Vec<_> = cases
        .into_iter()
        .map(|input| match input.value.parse::<ResourceQuantity>() {
            Err(error) => json!({"error":error.to_string()}),
            Ok(quantity) => {
                let mut row = json!({"canonical":quantity.to_string()});
                if let Ok(other) = input.other.parse::<ResourceQuantity>() {
                    match std::panic::catch_unwind(|| quantity.compare(&other)) {
                        Ok(order) => {
                            row["comparison"] = json!(match order {
                                std::cmp::Ordering::Less => -1,
                                std::cmp::Ordering::Equal => 0,
                                std::cmp::Ordering::Greater => 1,
                            })
                        }
                        Err(panic) => {
                            row["panic"] = json!(
                                panic
                                    .downcast_ref::<&str>()
                                    .map(|s| s.to_string())
                                    .or_else(|| panic.downcast_ref::<String>().cloned())
                                    .unwrap_or_else(|| "unknown panic".into())
                            )
                        }
                    }
                }
                row
            }
        })
        .collect();
    serde_json::to_writer(std::io::stdout(), &rows).unwrap();
}
