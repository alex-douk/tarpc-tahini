use tarpc::Response;
use serde_json::{Number, Value, Map};

use crate::tarpc::traits::{serialize_tahini_type, TahiniType};

pub fn transform_message<Resp: TahiniType>(item: Response<Resp>) -> Response<<Resp as TahiniType>::Intermediate> {
    let mut map = Map::new();
    map.insert(String::from("request_id"), Value::Number(Number::from(item.request_id)));
    map.insert(String::from("message"), serde_json::to_value(item.message.map(serialize_tahini_type)).unwrap());
    serde_json::from_value(Value::Object(map)).unwrap()
}