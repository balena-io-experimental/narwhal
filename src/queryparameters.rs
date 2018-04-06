use std::collections::HashMap;
use std::fmt::Display;
use url::form_urlencoded;
use serde_json;

pub struct QueryParameters {
    encoder: form_urlencoded::Serializer<String>
}

pub type QueryFilter = HashMap<String, Vec<String>>;

impl QueryParameters {
    pub fn new() -> QueryParameters {
        QueryParameters {
            encoder: form_urlencoded::Serializer::new(String::new()),
        }
    }
    pub fn add<T: Display>(&mut self, name: &str, value: T) {
        self.encoder.append_pair(&name, &value.to_string());
    }
    pub fn add_filter(&mut self, filter: QueryFilter) {

        // Don't worry about the unwrap, as the type system says
        // it has to be serializable
        let filter_str = serde_json::to_string(&filter).unwrap();
        self.encoder.append_pair("filter", &filter_str);
    }

    pub fn to_string(&mut self) -> String {
        self.encoder.finish()
    }
}

pub fn generate_path(path_base: &str, maybe_args: Option<&mut QueryParameters>) -> String {
    let mut path = String::from(path_base);

    if let Some(a) = maybe_args {
        path.push('?');
        path.push_str(&a.to_string());
    }

    return path;
}
