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