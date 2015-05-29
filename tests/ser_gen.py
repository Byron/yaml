import sys
from butility import OrderedDict

# Import needed to get yaml into the global namespace
import bkvstore

import yaml
import json

def write_const_str_rs(stream, const_name, data):
    stream.write("pub const %s: &'static str = \n" % const_name.upper())
    stream.write('r##"%s"##;\n\n' % data)

if __name__ == '__main__':
    data1 = OrderedDict((
        ('i32' , 0),
        ('i64' , 0),
        ('u32' , 0),
        ('u64' , 0),
        ('f32' , 0.0),
        ('f64' , 0.0),
        ('string' , ""),
        ('i32a' , []),
        ('hash' , dict()),
    ))
    write_const_str_rs(sys.stdout, 'data1_default', yaml.dump(data1))
    write_const_str_rs(sys.stdout, 'data1_default_json', json.dumps(data1, indent=2))
    write_const_str_rs(sys.stdout, 'data1_default_canonical', yaml.dump(data1, canonical=True, version=(1,2)))

    opts_pretty = dict(default_flow_style=False)

    # default_flow_style = False: use block-style for lists/dicts in any cause
    write_const_str_rs(sys.stdout, 'list1_default', yaml.dump(["string", 5, 3.2], **opts_pretty))

    # Example 2.1.  Sequence of Scalars
    # (ball players)
    write_const_str_rs(sys.stdout, 'example_2_1', yaml.dump(["Mark McGwire", 
                                                             "Sammy Sosa", 
                                                             "Ken Griffey"], **opts_pretty))


    d = OrderedDict((
        ('hr', 65),
        ('avg', 0.278),
        ('rbi', 147)
    ))
    # Example 2.2.  Mapping Scalars to Scalars
    # (player statistics)
    write_const_str_rs(sys.stdout, 'example_2_2', yaml.dump(d, **opts_pretty))

    # Example 2.3.  Mapping Scalars to Sequences
    # (ball clubs in each league)
    d  = OrderedDict((
        ('american', ["Boston Red Sox", "Detroit Tigers", "New York Yankees"]),
        ('national', ["New York Mets", "Chicago Cubs", "Atlanta Braves"]),
    ))
    write_const_str_rs(sys.stdout, 'example_2_3', yaml.dump(d, **opts_pretty))


    d = [
    {
        'name': 'Mark McGwire',
        'hr':   65,
        'avg':  0.278,
    },
    {
        'name': 'Sammy Sosa',
        'hr':   63,
        'avg':  0.288,
    }
    ]

    #  Example 2.4.  Sequence of Mappings
    # (players statistics)
    write_const_str_rs(sys.stdout, 'example_2_4', yaml.dump(d, **opts_pretty))

    # Example 2.5. Sequence of Sequences
    d = [
        ['name'        , 'hr', 'avg'],
        ['Mark McGwire', 65, 0.278],
        ['Sammy Sosa'  , 63, 0.288],
    ]

    write_const_str_rs(sys.stdout, 'example_2_5', yaml.dump(d))

    # Example 2.6. Mapping of Mappings
    d = OrderedDict((
    ('Mark McGwire', OrderedDict((('hr', 65), ('avg', 0.278)))),
    ('Sammy Sosa', OrderedDict((('hr', 63), ('avg', 0.288)))),
    ))
    write_const_str_rs(sys.stdout, 'example_2_6', yaml.dump(d))

    d = ([
            "Mark McGwire",
            "Sammy Sosa",
            "Ken Griffey",
        ],
        [
            "Chicago Cubs",
            "St Louis Cardinals",
        ]
    )

    opts = opts_pretty.copy()
    # opts['explicit_start'] = True
    # Example 2.7.  Two Documents in a Stream
    write_const_str_rs(sys.stdout, 'example_2_7', yaml.dump_all(d, **opts))
    opts['explicit_start'] = True

    # Example 2.8.  Play by Play Feed
    # from a Game
    d = [OrderedDict((
            ('time', '20:03:20'),
            ('player', 'Sammy Sosa'),
            ('action', 'strike (miss)'),
        )),
        OrderedDict((
            ('time', '20:03:47'),
            ('player', 'Sammy Sosa'),
            ('action', 'grand sl'),
        ))
    ]
    opts['explicit_end'] = True
    write_const_str_rs(sys.stdout, 'example_2_8', yaml.dump_all(d, **opts))

    
    d = OrderedDict((
        ('hr', ["Mark McGwire", "Sammy Sosa"]),
        ('rbi', ["Sammy Sosa", "Ken Griffey"])
    ))

    # Example 2.9.  Single Document 
    opts['explicit_end'] = False
    write_const_str_rs(sys.stdout, 'example_2_9', yaml.dump(d, **opts))


    # Example 2.10  Node for Sammy Sosa
    # appears twice in this document
    s = "Sammy Sosa"
    d = OrderedDict((
        ('hr', ["Mark McGwire", s]),
        ('rbi', [s, "Ken Griffey"])
    ))
    res = """---
hr:
- Mark McGwire
- Sammy Sosa
rbi:
- Sammy Sosa
- Ken Griffey
"""
    assert res == yaml.dump(d, **opts), "Not actually supported by PyYaml"
    # But we put it anyway ... maybe useful for deserialzation testing
    d = """---
hr:
  - Mark McGwire
  # Following node labeled SS
  - &SS Sammy Sosa
rbi:
  - *SS # Subsequent occurrence
  - Ken Griffey
"""
    write_const_str_rs(sys.stdout, 'example_2_10', d)

    # NOTE: Can't produce the example in python as we must use tuples for dict keys, which 
    # don't translate to lists.
    s = """? - Detroit Tigers
  - Chicago cubs
:
  - 2001-07-23
"""

    # Example 2.11. Mapping between Sequences
    write_const_str_rs(sys.stdout, 'example_2_11', s)

    d = [
        OrderedDict((
            ('item', 'Super Hoop'),
            ('quantity', 1),
        )),
        OrderedDict((
           ('item', 'Basketball'),
           ('quantity', 4),
        )),
        OrderedDict((
            ('item', 'Big Shoes'),
            ('quantity', 1),
        ))
    ]
    # Example 2.12. Compact Nested Mapping
    write_const_str_rs(sys.stdout, 'example_2_12', yaml.dump(d, **opts))

    d = ("\//||\/||\n" +
         "// ||  ||__")

    opts['default_style'] = '|'
    # Example 2.13.  In literals
    write_const_str_rs(sys.stdout, 'example_2_13', yaml.dump(d, **opts))

    d = "Mark McGwire's year was crippled by a knee injury."
    opts['default_style'] = '>'
    opts['width'] = 14

    # Example 2.14.  In the folded scalars,
    # newlines become spaces
    write_const_str_rs(sys.stdout, 'example_2_14', yaml.dump(d, **opts))


    d = """>
  Sammy Sosa completed another
  fine season with great stats.

    63 Home Runs
    0.288 Batting Average

  What a year!
"""
    # Example 2.15.  Folded newlines are preserved
    # for "more indented" and blank lines
    # NOTE: For deserialization testing only, as the example can't be manufactured as there are two 
    # different line-break styles within one string literal
    write_const_str_rs(sys.stdout, 'example_2_15', d)


    d = """name: Mark McGwire
accomplishment: >
  Mark set a major league
  home run record in 1998.
stats: |
  65 Home Runs
  0.278 Batting Average
"""
    # Example 2.16.  Indentation determines scope
    write_const_str_rs(sys.stdout, 'example_2_16', d)

    d = r"""unicode: "Sosa did fine.\u263A"
control: "\b1998\t1999\t2000\n"
hex esc: "\x0d\x0a is \r\n"

single: '"Howdy!" he cried.'
quoted: ' # Not a ''comment''.'
tie-fighter: '|\-*-/|'
"""
    # Example 2.17. Quoted Scalars
    write_const_str_rs(sys.stdout, 'example_2_17', d)

    d = r"""plain:
  This unquoted scalar
  spans many lines.

quoted: "So does this
  quoted scalar.\n"
"""

    # Example 2.18. Multi-line Flow Scalars
    write_const_str_rs(sys.stdout, 'example_2_18', d)

    d = """canonical: 12345
decimal: +12345
octal: 0o14
hexadecimal: 0xC
"""
    
    # Example 2.19. Integers
    write_const_str_rs(sys.stdout, 'example_2_19', d)

    d = """canonical: 1.23015e+3
exponential: 12.3015e+02
fixed: 1230.15
negative infinity: -.inf
not a number: .NaN
"""

    # Example 2.20. Floating Point
    write_const_str_rs(sys.stdout, 'example_2_20', d)

    d = """null:
booleans: [ true, false ]
string: '012345'
"""

    # Example 2.21. Miscellaneous
    write_const_str_rs(sys.stdout, 'example_2_21', d)


    d = """canonical: 2001-12-15T02:59:43.1Z
iso8601: 2001-12-14t21:59:43.10-05:00
spaced: 2001-12-14 21:59:43.10 -5
date: 2002-12-14
"""
    # Example 2.22. Timestamps
    write_const_str_rs(sys.stdout, 'example_2_22', d)

    d = """---
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
"""

    # Example 2.23. Various Explicit Tags
    write_const_str_rs(sys.stdout, 'example_2_23', d)

    d = """%TAG ! tag:clarkevans.com,2002:
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
"""

    # Example 2.24. Global Tags
    write_const_str_rs(sys.stdout, 'example_2_24', d)


    d = """# Sets are represented as a
# Mapping where each key is
# associated with a null value
--- !!set
? Mark McGwire
? Sammy Sosa
? Ken Griff
"""
    # Example 2.25. Unordered Sets
    write_const_str_rs(sys.stdout, 'example_2_25', d)

    d = """# Ordered maps are represented as
# A sequence of mappings, with
# each mapping having one key
--- !!omap
- Mark McGwire: 65
- Sammy Sosa: 63
- Ken Griffy: 58
"""

    # Example 2.26. Ordered Mappings
    write_const_str_rs(sys.stdout, 'example_2_26', d)


    d = """
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
"""

    # Example 2.27. Invoice
    write_const_str_rs(sys.stdout, 'example_2_27', d)

    d = [
        OrderedDict((
            ('Time', '2001-11-23 15:01:42 -5'),
            ('User', 'ed'),
            ('Warning', 'This is an error message for the log file'),
        )),
        OrderedDict((
            ('Time', '2001-11-23 15:02:31 -5'), 
            ('User', 'ed'),
            ('Warning', 'A slightly different error message.'),
        )),
        OrderedDict((
            ('Date', '2001-11-23 15:03:17 -5'),
            ('User', 'ed'),
            ('Fatal', 'Unknown variable "bar"'),
            ('Stack', [
                OrderedDict((
                    ('file', 'TopClass.py'),
                    ('line', 23),
                    ('code', r'x = MoreObject("345\n")')
                )),
                OrderedDict((
                    ('file', 'MoreClass.py'),
                    ('line', 58),
                    ('code', 'foo = bar'),
                )),
            ]),
        ))
    ]

    opts['width'] = 20
    opts['default_style'] = None

    # Example 2.28. Log File
    write_const_str_rs(sys.stdout, 'example_2_28', yaml.dump_all(d, **opts))


    # unfortunately, this will put the python/tuple type into the YAML
    d = { (1,2): 3 }
    s = """?
- 1
- 2
: 3
"""
    write_const_str_rs(sys.stdout, 'explicit_mapping_entry', s)

    # UNIT TESTING
    d = None
    opts['explicit_start'] = True
    opts['explicit_end'] = None
    opts['default_flow_style'] = None
    write_const_str_rs(sys.stdout, 'document_indicator_start', yaml.dump(d, **opts))

else:
    raise AssertionError("Cannot be used as library")
