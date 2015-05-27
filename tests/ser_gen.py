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
    opts['explicit_start'] = True
    # Example 2.7.  Two Documents in a Stream
    write_const_str_rs(sys.stdout, 'example_2_7', yaml.dump_all(d, **opts))

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


else:
    raise AssertionError("Cannot be used as library")
