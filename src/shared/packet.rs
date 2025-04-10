pub struct Packet {
    pub header: String,
    pub data: Option<String>,
}

impl Packet {
    pub fn new(header: String, data: Option<String>) -> Self {
        Self { header, data }
    }

    pub fn unwrap_data(&self) -> String {
        match &self.data {
            Some(value) => value.clone(),
            None => "_".to_string(),
        }
    }

    pub fn marshall(&self) -> String {
        format!("{};{}", self.header, self.unwrap_data())
    }

    pub fn unmarshall(serialized: String) -> Packet {
        let parts: Vec<&str> = serialized.splitn(2, ';').collect();

        let header = parts.get(0).unwrap_or(&"_").to_string();
        let data = if parts.len() == 2 && parts[1] != "_" {
            Some(parts[1].to_string())
        } else {
            None
        };

        Packet::new(header, data)
    }
}
