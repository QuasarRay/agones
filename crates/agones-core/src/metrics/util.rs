use std::collections::BTreeMap;

pub(crate) const NAME: &str = "name";
pub(crate) const NAMESPACE: &str = "namespace";
pub(crate) const FLEET_NAME: &str = "fleet_name";
pub(crate) const TYPE: &str = "type";
pub(crate) const STATUS_CODE: &str = "status_code";
pub(crate) const VERB: &str = "verb";
pub(crate) const ENDPOINT: &str = "endpoint";
pub(crate) const EMPTY: &str = "empty";
pub(crate) const COUNTER: &str = "counter";
pub(crate) const LIST: &str = "list";

pub type Labels = BTreeMap<String, String>;

pub fn parse_labels(s: &str) -> Result<Labels, String> {
    let mut labels = Labels::new();
    if s.is_empty() {
        return Ok(labels);
    }

    for pair in s.split(',') {
        let mut parts = pair.split('=');
        let (Some(key), Some(value), None) = (parts.next(), parts.next(), parts.next()) else {
            return Err(format!(
                "invalid labels: {s}, expect key=value,key2=value2"
            ));
        };
        let (key, value) = (key.trim(), value.trim());

        if key.is_empty() {
            return Err("invalid key: can not be empty".into());
        }
        if value.is_empty() {
            return Err(format!("invalid value for key {key}: can not be empty"));
        }

        labels.insert(key.into(), value.into());
    }
    Ok(labels)
}