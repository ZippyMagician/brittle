#[derive(PartialEq, Clone, Debug)]
pub struct Date {
    year: Option<u8>,
    month: Option<u8>,
    day: Option<u8>,
    // TODO: Kinda lazy way to store timestamps rn
    timestamp: Option<String>,
}

impl Date {
    pub fn new(date: String) -> Self {
        let data: Vec<String>;
        let date_data: Vec<u8>;

        if date.contains("T") {
            data = date.split("T").map(|x| x.to_string()).collect();
            date_data = data[0]
                .split("-")
                .map(|x| x.parse::<u8>().unwrap())
                .collect();
            Self {
                year: Some(date_data[0]),
                month: Some(date_data[1]),
                day: Some(date_data[2]),
                timestamp: Some(data[1].clone()),
            }
        } else if date.contains(" ") {
            data = date.split(" ").map(|x| x.to_string()).collect();
            date_data = data[0]
                .split("-")
                .map(|x| x.parse::<u8>().unwrap())
                .collect();
            Self {
                year: Some(date_data[0]),
                month: Some(date_data[1]),
                day: Some(date_data[2]),
                timestamp: Some(data[1].clone()),
            }
        } else {
            // Either only timestamp or only date
            if date.contains("-") {
                date_data = date.split("-").map(|x| x.parse::<u8>().unwrap()).collect();
                Self {
                    year: Some(date_data[0]),
                    month: Some(date_data[1]),
                    day: Some(date_data[2]),
                    timestamp: None,
                }
            } else {
                Self {
                    year: None,
                    month: None,
                    day: None,
                    timestamp: Some(date),
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}-{}-{} {}",
            self.year.unwrap_or(00),
            self.month.unwrap_or(00),
            self.day.unwrap_or(00),
            self.timestamp.clone().unwrap_or(String::new())
        )
    }
}
