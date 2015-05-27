pub const DATA1_DEFAULT: &'static str = 
r##"i32: 0
i64: 0
u32: 0
u64: 0
f32: 0.0
f64: 0.0
string: ''
i32a: []
hash: {}
"##;

pub const DATA1_DEFAULT_JSON: &'static str = 
r##"{
  "i32": 0, 
  "i64": 0, 
  "u32": 0, 
  "u64": 0, 
  "f32": 0.0, 
  "f64": 0.0, 
  "string": "", 
  "i32a": [], 
  "hash": {}
}"##;

pub const LIST1_DEFAULT: &'static str = 
r##"- string
- 5
- 3.2
"##;

pub const EXAMPLE_2_1: &'static str = 
r##"- Mark McGwire
- Sammy Sosa
- Ken Griffey
"##;

pub const EXAMPLE_2_2: &'static str = 
r##"hr: 65
avg: 0.278
rbi: 147
"##;

pub const EXAMPLE_2_3: &'static str = 
r##"american:
- Boston Red Sox
- Detroit Tigers
- New York Yankees
national:
- New York Mets
- Chicago Cubs
- Atlanta Braves
"##;

pub const EXAMPLE_2_4: &'static str = 
r##"- avg: 0.278
  hr: 65
  name: Mark McGwire
- avg: 0.288
  hr: 63
  name: Sammy Sosa
"##;

pub const EXAMPLE_2_5: &'static str = 
r##"- [name, hr, avg]
- [Mark McGwire, 65, 0.278]
- [Sammy Sosa, 63, 0.288]
"##;

pub const EXAMPLE_2_6: &'static str = 
r##"Mark McGwire: {hr: 65, avg: 0.278}
Sammy Sosa: {hr: 63, avg: 0.288}
"##;

pub const EXAMPLE_2_7: &'static str = 
r##"---
- Mark McGwire
- Sammy Sosa
- Ken Griffey
---
- Chicago Cubs
- St Louis Cardinals
"##;

pub const EXAMPLE_2_8: &'static str = 
r##"---
time: '20:03:20'
player: Sammy Sosa
action: strike (miss)
...
---
time: '20:03:47'
player: Sammy Sosa
action: grand sl
...
"##;

pub const EXAMPLE_2_9: &'static str = 
r##"---
hr:
- Mark McGwire
- Sammy Sosa
rbi:
- Sammy Sosa
- Ken Griffey
"##;

pub const EXAMPLE_2_11: &'static str = 
r##"? - Detroit Tigers
  - Chicago cubs
:
  - 2001-07-23
"##;

pub const EXAMPLE_2_12: &'static str = 
r##"---
- item: Super Hoop
  quantity: 1
- item: Basketball
  quantity: 4
- item: Big Shoes
  quantity: 1
"##;

