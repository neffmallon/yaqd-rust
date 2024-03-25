use std::collections::HashMap;
use apache_avro::{Schema};
use apache_avro::types::{Record, Value};
use serde::Deserialize;

pub const HANDSHAKE_REQUEST_SCHEMA: &str = r#"    {
        "type": "record",
        "name": "HandshakeRequest",
        "namespace": "org.apache.avro.ipc",
        "fields": [
            {
                "name": "clientHash",
                "type": {"type": "fixed", "name": "MD5", "size": 16}
            },
            {"name": "clientProtocol", "type": ["null", "string"]},
            {"name": "serverHash", "type": "MD5"},
            {"name": "meta", "type": ["null", {"type": "map", "values": "bytes"}]}
        ]
    }"#;
// You cannot serialize into Value::Fixed, so don't even bother
#[derive(PartialEq, Deserialize, Debug, Clone)]
pub struct HandshakeRequest{
    #[serde(rename = "clientHash")]
    client_hash: String,
    #[serde(rename = "clientProtocol")]
    client_protocol: Option<String>,
    #[serde(rename = "serverHash")]
    server_hash: String,
    meta: Option<HashMap<String, Vec<u8>>>
}

impl From<HandshakeRequest> for Value{
    fn from(val: HandshakeRequest) -> Self {
        let schema = Schema::parse_str(HANDSHAKE_REQUEST_SCHEMA).unwrap();
        let mut record = Record::new(&schema).unwrap();
        // put all the things in the record:
        record.put("clientHash", Value::Fixed(16, val.client_hash.into_bytes()));
        record.put("clientProtocol",
            match &val.client_protocol {
                None => Value::Union(0,Box::new(Value::Null)),
                Some(s) => Value::Union(1,Box::new(Value::String(s.clone())))
            }
        );
        record.put("serverHash", Value::Fixed(16, val.server_hash.into_bytes()));
        record.put("meta",
            match &val.meta{
                None => Value::Union(0, Box::new(Value::Null)),
                Some(m) => Value::Union(1, Box::new(m.clone().into())),
            }
        );
        record.into()
    }
}

pub const HANDSHAKE_RESPONSE_SCHEMA: &str = r#"{
    "type": "record",
    "name": "HandshakeResponse", "namespace": "org.apache.avro.ipc",
    "fields": [
    {"name": "match",
        "type": {"type": "enum", "name": "HandshakeMatch",
        "symbols": ["BOTH", "CLIENT", "NONE"]}},
    {"name": "serverProtocol",
        "type": ["null", "string"]},
    {"name": "serverHash",
        "type": ["null", {"type": "fixed", "name": "MD5", "size": 16}]},
    {"name": "meta",
        "type": ["null", {"type": "map", "values": "bytes"}]}
    ]
}"#;

#[derive(Deserialize, Debug, Eq, Clone, PartialEq)]
pub enum HandshakeMatch{BOTH, CLIENT, NONE}
#[derive(Deserialize, Debug, Eq, Clone, PartialEq)]
pub struct HandshakeResponse{
    #[serde(rename = "match")]
    pub match_: HandshakeMatch,
    #[serde(rename = "serverProtocol")]
    pub server_protocol: Option<String>,
    #[serde(rename = "serverHash")]
    pub server_hash: Option<String>,
    pub meta: Option<HashMap<String, Vec<u8>>>
}
impl From<HandshakeResponse> for Value{
    fn from(val: HandshakeResponse) -> Self {
        let schema = Schema::parse_str(HANDSHAKE_RESPONSE_SCHEMA).unwrap();
        let mut record = Record::new(&schema).unwrap();
        // put all the things in the record:
        record.put("match",
            match val.match_{
                HandshakeMatch::BOTH => Value::Enum(0, "BOTH".into()),
                HandshakeMatch::CLIENT => Value::Enum(1, "CLIENT".into()),
                HandshakeMatch::NONE => Value::Enum(2, "NONE".into()),
            }
        );
        record.put("serverProtocol",
            match &val.server_protocol {
               None => Value::Union(0,Box::new(Value::Null)),
               Some(s) => Value::Union(1,Box::new(Value::String(s.clone())))
            }
        );
        record.put("serverHash",
            match &val.server_hash {
               None => Value::Union(0,Box::new(Value::Null)),
               Some(s) => Value::Union(1,Box::new(Value::Fixed(16, s.clone().into_bytes()))),
            }
        );
        record.put("meta",
            match &val.meta{
               None => Value::Union(0, Box::new(Value::Null)),
               Some(m) => Value::Union(1, Box::new(m.clone().into())),
            }
        );
        record.into()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::hash::Hash;
    use apache_avro::{from_avro_datum, from_value, Schema, to_avro_datum};
    use apache_avro::types::Value;
    use crate::handshakes::{HANDSHAKE_REQUEST_SCHEMA, HANDSHAKE_RESPONSE_SCHEMA, HandshakeMatch, HandshakeRequest, HandshakeResponse};

    #[test]
    fn test_handshake_encode(){
        let schema = Schema::parse_str(HANDSHAKE_REQUEST_SCHEMA).unwrap();
        let record = HandshakeRequest {
            client_hash: "                ".into(),
            client_protocol: None,
            server_hash: "                ".into(),
            meta: Some(HashMap::new())
        };
        let new_record: HandshakeRequest = from_value::<HandshakeRequest>(
            &from_avro_datum(
                &schema,
                &mut &(to_avro_datum::<Value>(&schema, record.clone().into()).unwrap())[..],
                None
            ).unwrap()
        ).unwrap();
        assert_eq!(record, new_record);
        let record = HandshakeRequest {
            client_hash: "       aaaaa    ".into(),
            client_protocol: Some("asdqwerasdfasdf".into()),
            server_hash: "   aaaaa        ".into(),
            meta: Some(HashMap::new())
        };
        let new_record: HandshakeRequest = from_value::<HandshakeRequest>(
            &from_avro_datum(
                &schema,
                &mut &(to_avro_datum::<Value>(&schema, record.clone().into()).unwrap())[..],
                None
            ).unwrap()
        ).unwrap();
        assert_eq!(record, new_record);
    }

    #[test]
    fn test_handshake_response(){
        let schema = Schema::parse_str(HANDSHAKE_RESPONSE_SCHEMA).unwrap();
        let record = HandshakeResponse {
            match_: HandshakeMatch::BOTH,
            server_hash: Some("                ".into()),
            meta: Some(HashMap::new()),
            server_protocol: None,
        };

        let encoded = to_avro_datum::<Value>(&schema, record.clone().into()).unwrap();
        // read the record:
        let new_record: HandshakeResponse = from_value::<HandshakeResponse>(
            &from_avro_datum(
                &schema,
                &mut &(to_avro_datum::<Value>(&schema, record.clone().into()).unwrap())[..],
                None
            ).unwrap()
        ).unwrap();
        assert_eq!(record, new_record);
        let record = HandshakeResponse {
            match_: HandshakeMatch::BOTH,
            server_hash: None,
            meta: None,
            server_protocol: Some("HashMap::new()".into()),
        };

        let encoded = to_avro_datum::<Value>(&schema, record.clone().into()).unwrap();
        // read the record:
        let new_record: HandshakeResponse = from_value::<HandshakeResponse>(
            &from_avro_datum(
                &schema,
                &mut &(to_avro_datum::<Value>(&schema, record.clone().into()).unwrap())[..],
                None
            ).unwrap()
        ).unwrap();
        assert_eq!(record, new_record);
    }
}