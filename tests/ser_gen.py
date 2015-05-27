import sys
from butility import OrderedDict

# Import needed to get yaml into the global namespace
import bkvstore
import yaml

def write_const_str_rs(stream, const_name, data):
    stream.write("pub const %s: &'static str = \n" % const_name.upper())
    stream.write('r##"%s"##;\n\n' % yaml.dump(data))

if __name__ == '__main__':
    write_const_str_rs(sys.stdout, 'data1_default', OrderedDict((
        ('i32' , 0),
        ('i64' , 0),
        ('u32' , 0),
        ('u64' , 0),
        ('f32' , 0.0),
        ('f64' , 0.0),
        ('string' , ""),
        ('i32a' , []),
        ('hash' , dict()),
    )))
else:
    raise AssertionError("Cannot be used as library")
