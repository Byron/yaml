use std::collections::HashMap;
use serde;

#[derive(Serialize, Deserialize, Default)]
pub struct Data1 {
    pub i32: i32,
    pub i64: i64,
    pub u32: u32,
    pub u64: u64,
    pub f32: f32,
    pub f64: f64,
    pub string: String,
    pub i32a: Vec<i32>,
    pub hash: HashMap<String, Data1>
}

#[derive(Serialize, Deserialize)]
pub struct Example_2_2 {
    pub hr: u32,
    pub avg: f32,
    pub rbi: u32,
}

impl Default for Example_2_2 {
    fn default() -> Self {
        Example_2_2 {
            hr: 65,
            avg: 0.278,
            rbi: 147
        }
    }
}

#[derive(Serialize)]
pub struct Example_2_3 {
    pub american: Vec<&'static str>,
    pub national: Vec<&'static str>,
}


impl Default for Example_2_3 {
    fn default() -> Self {
        Example_2_3 {
            american: vec!["Boston Red Sox", "Detroit Tigers", "New York Yankees"],
            national: vec!["New York Mets", "Chicago Cubs", "Atlanta Braves"],
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub hr: u32,
    pub avg: f32,
}


#[derive(Serialize, Deserialize)]
pub struct Example_2_4(Vec<Player>);

impl Default for Example_2_4 {
    fn default() -> Self {
        Example_2_4(vec![
            Player {
                name: "Mark McGwire".to_string(),
                hr: 65,
                avg: 0.278
            },
            Player {
                name: "Sammy Sosa".to_string(),
                hr: 63,
                avg: 0.288
            }])
    }
}


#[derive(Serialize, Deserialize)]
pub struct PlayerStat {
    pub hr: u32,
    pub avg: f32,
}


#[derive(Serialize, Deserialize)]
pub struct Example_2_6(HashMap<String, PlayerStat>);

impl Default for Example_2_6 {
    fn default() -> Self {
        let mut h = HashMap::new();
        h.insert("Mark McGwire".to_string(), PlayerStat {hr: 65, avg: 0.278});
        h.insert("Samy Sosa".to_string(), PlayerStat {hr: 63, avg: 0.288});
        Example_2_6(h)
    }
}

#[derive(Serialize)]
pub struct Example_2_7(Vec<Vec<&'static str>>);

impl Default for Example_2_7 {
    fn default() -> Self {
        Example_2_7(vec![
            vec!["Mark McGwire", "Sammy Sosa", "Ken Griffey"],
            vec!["Chicago Cubs", "St Louis Cardinals"]
        ])
    }
}

#[derive(Serialize)]
pub struct GameEvent {
    pub time: &'static str,
    pub player: &'static str,
    pub action: &'static str,
}

#[derive(Serialize)]
pub struct Example_2_8(Vec<GameEvent>);

impl Default for Example_2_8 {
    fn default() -> Self {
        Example_2_8(vec![
            GameEvent {
                time: "20:03:20",
                player: "Sammy Sosa",
                action: "strike (miss)",
            },
            GameEvent {
                time: "20:03:47",
                player: "Sammy Sosa",
                action: "grand slam",
            }
        ])
    }
}

#[derive(Serialize)]
pub struct Example_2_9 {
    pub hr: Vec<&'static str>,
    pub rbi: Vec<&'static str>,
}

impl Default for Example_2_9 {
    fn default() -> Self { 
        Example_2_9 {
            hr: vec!["Mark McGwire", "Sammy Sosa"],
            rbi: vec!["Sammy Sosa", "Ken Griffey"],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Example_2_11(HashMap<(String, String), Vec<String>>);

impl Default for Example_2_11 {
    fn default() -> Self { 
        let mut h = HashMap::new();
        h.insert(("Detroit Tigers".to_string(), "Chicago cubs".to_string()), 
                  vec!["2001-07-23".to_string()]);
        Example_2_11(h)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CartItem {
    item: String,
    quantity: u32,
}

pub fn example_2_12_new() -> Vec<CartItem> {
    vec![
        CartItem {
            item: "Super Hoop".to_string(),
            quantity: 1
        },
        CartItem {
            item: "Basketball".to_string(),
            quantity: 4
        },
        CartItem {
            item: "Big Shoes".to_string(),
            quantity: 1
        },
    ]
}

#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    #[serde(rename="Time")]
    time: Option<String>,
    #[serde(rename="Date")]
    date: Option<String>,
    #[serde(rename="User")]
    user: String,
    #[serde(rename="Warning")]
    warning: Option<String>,
    #[serde(rename="Fatal")]
    fatal: Option<String>,
    #[serde(rename="Stack")]
    stack: Option<Vec<StackFrame>>
}

#[derive(Serialize, Deserialize)]
pub struct StackFrame {
    file: String,
    line: u64,
    code: String,
}



pub fn example_2_28_new() -> Vec<LogEntry> {
    vec![
        LogEntry {
            time: Some("2001-11-23 15:01:42 -5".to_string()),
            date: None,
            user: "ed".to_string(),
            warning: Some("This is an error message for the log file".to_string()),
            fatal: None,
            stack: None
        },
        LogEntry {
            time: Some("2001-11-23 15:02:31 -5".to_string()),
            date: None,
            user: "ed".to_string(),
            warning: Some("A slightly different error message.".to_string()),
            fatal: None,
            stack: None
        },
        LogEntry {
            time: None,
            date: Some("2001-11-23 15:03:17 -5".to_string()),
            user: "ed".to_string(),
            warning: None,
            fatal: Some(r#"Unknown variable "bar""#.to_string()),
            stack: Some(
                vec![
                    StackFrame {
                        file: "TopClass.py".to_string(),
                        line: 23,
                        code: r#"x = MoreObject("345\n")"#.to_string(),
                    },
                    StackFrame {
                        file: "MoreClass.py".to_string(),
                        line: 58,
                        code: "foo = bar".to_string(),
                    }
                ]
            )
        },
    ]
}

#[derive(Serialize, Deserialize)]
pub struct SingleOptKey {
    pub key: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct DualOptKey {
    pub key1: Option<Vec<u32>>,
    pub key2: Option<u32>,
}