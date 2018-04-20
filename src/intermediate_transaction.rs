use chain::current_timestamp;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntermediateTransaction {
    pub id: String,
    pub timestamp: u64,
    pub payload: String,
    pub confirmed: bool,
}

impl IntermediateTransaction {
    pub fn new(payload: &str) -> IntermediateTransaction {
        IntermediateTransaction {
            id: Uuid::new_v4().to_string(),
            timestamp: current_timestamp(),
            payload: String::from(payload),
            confirmed: false,
        }
    }
}