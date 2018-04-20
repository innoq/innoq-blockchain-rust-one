extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeRegistration {
    pub host: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub node_id: String,
    pub host: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeRegistered {
    pub message: String,
    pub node: Node,
}
