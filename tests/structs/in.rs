use std::collections::HashMap;

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
