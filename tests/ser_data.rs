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
r##"- Mark McGwire
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

pub const EXAMPLE_2_10: &'static str = 
r##"---
hr:
  - Mark McGwire
  # Following node labeled SS
  - &SS Sammy Sosa
rbi:
  - *SS # Subsequent occurrence
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

pub const EXAMPLE_2_13: &'static str = 
r##"--- |-
  \//||\/||
  // ||  ||__
"##;

pub const EXAMPLE_2_14: &'static str = 
r##"--- >-
  Mark McGwire's
  year was crippled
  by a knee injury.
"##;

pub const EXAMPLE_2_15: &'static str = 
r##">
  Sammy Sosa completed another
  fine season with great stats.

    63 Home Runs
    0.288 Batting Average

  What a year!
"##;

pub const EXAMPLE_2_16: &'static str = 
r##"name: Mark McGwire
accomplishment: >
  Mark set a major league
  home run record in 1998.
stats: |
  65 Home Runs
  0.278 Batting Average
"##;

pub const EXAMPLE_2_17: &'static str = 
r##"unicode: "Sosa did fine.\u263A"
control: "\b1998\t1999\t2000\n"
hex esc: "\x0d\x0a is \r\n"

single: '"Howdy!" he cried.'
quoted: ' # Not a ''comment''.'
tie-fighter: '|\-*-/|'
"##;

pub const EXAMPLE_2_18: &'static str = 
r##"plain:
  This unquoted scalar
  spans many lines.

quoted: "So does this
  quoted scalar.\n"
"##;

pub const EXAMPLE_2_19: &'static str = 
r##"canonical: 12345
decimal: +12345
octal: 0o14
hexadecimal: 0xC
"##;

pub const EXAMPLE_2_20: &'static str = 
r##"canonical: 1.23015e+3
exponential: 12.3015e+02
fixed: 1230.15
negative infinity: -.inf
not a number: .NaN
"##;

pub const EXAMPLE_2_21: &'static str = 
r##"null:
booleans: [ true, false ]
string: '012345'
"##;

pub const EXAMPLE_2_22: &'static str = 
r##"canonical: 2001-12-15T02:59:43.1Z
iso8601: 2001-12-14t21:59:43.10-05:00
spaced: 2001-12-14 21:59:43.10 -5
date: 2002-12-14
"##;

pub const EXAMPLE_2_23: &'static str = 
r##"---
not-date: !!str 2002-04-28

picture: !!binary |
 R0lGODlhDAAMAIQAAP//9/X
 17unp5WZmZgAAAOfn515eXv
 Pz7Y6OjuDg4J+fn5OTk6enp
 56enmleECcgggoBADs=

application specific tag: !something |
 The semantics of the tag
 above may be different for
 different documents.
"##;

pub const EXAMPLE_2_24: &'static str = 
r##"%TAG ! tag:clarkevans.com,2002:
--- !shape
  # Use the ! handle for presenting
  # tag:clarkevans.com,2002:circle
- !circle
  center: &ORIGIN {x: 73, y: 129}
  radius: 7
- !line
  start: *ORIGIN
  finish: { x: 89, y: 102 }
- !label
  start: *ORIGIN
  color: 0xFFEEBB
  text: Pretty vector drawing.
"##;

pub const EXAMPLE_2_25: &'static str = 
r##"# Sets are represented as a
# Mapping where each key is
# associated with a null value
--- !!set
? Mark McGwire
? Sammy Sosa
? Ken Griff
"##;

pub const EXAMPLE_2_26: &'static str = 
r##"# Ordered maps are represented as
# A sequence of mappings, with
# each mapping having one key
--- !!omap
- Mark McGwire: 65
- Sammy Sosa: 63
- Ken Griffy: 58
"##;

pub const EXAMPLE_2_27: &'static str = 
r##"
--- !<tag:clarkevans.com,2002:invoice>
invoice: 34843
date   : 2001-01-23
bill-to: &id001
    given  : Chris
    family : Dumars
    address:
        lines: |
            458 Walkman Dr.
            Suite #292
        city    : Royal Oak
        state   : MI
        postal  : 48046
ship-to: *id001
product:
    - sku         : BL394D
      quantity    : 4
      description : Basketball
      price       : 450.00
    - sku         : BL4438H
      quantity    : 1
      description : Super Hoop
      price       : 2392.00
tax  : 251.42
total: 4443.52
comments:
    Late afternoon is best.
    Backup contact is Nancy
    Billsmer @ 338-4338.
"##;

pub const EXAMPLE_2_28: &'static str = 
r##"---
Time: '2001-11-23 15:01:42
  -5'
User: ed
Warning: This is an error
  message for the log
  file
---
Time: '2001-11-23 15:02:31
  -5'
User: ed
Warning: A slightly different
  error message.
---
Date: '2001-11-23 15:03:17
  -5'
User: ed
Fatal: Unknown variable
  "bar"
Stack:
- file: TopClass.py
  line: 23
  code: x = MoreObject("345\n")
- file: MoreClass.py
  line: 58
  code: foo = bar
"##;

