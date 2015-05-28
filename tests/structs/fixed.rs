use serde;
use std::collections::HashMap;
#[derive(Default)]
pub struct Data1 {
    pub i32: i32,
    pub i64: i64,
    pub u32: u32,
    pub u64: u64,
    pub f32: f32,
    pub f64: f64,
    pub string: String,
    pub i32a: Vec<i32>,
    pub hash: HashMap<String, Data1>,
}
#[automatically_derived]
impl ::serde::de::Deserialize for Data1 {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<Data1, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
            }
            impl ::serde::de::Deserialize for __Field {
                #[inline]
                fn deserialize<D>(deserializer: &mut D)
                 -> ::std::result::Result<__Field, D::Error> where
                 D: ::serde::de::Deserializer {
                    use std::marker::PhantomData;
                    struct __FieldVisitor<D> {
                        phantom: PhantomData<D>,
                    }
                    impl <__D> ::serde::de::Visitor for __FieldVisitor<__D>
                     where __D: ::serde::de::Deserializer {
                        type
                        Value
                        =
                        __Field;
                        fn visit_str<E>(&mut self, value: &str)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                "i32" => { Ok(__Field::__field0) }
                                "i64" => { Ok(__Field::__field1) }
                                "u32" => { Ok(__Field::__field2) }
                                "u64" => { Ok(__Field::__field3) }
                                "f32" => { Ok(__Field::__field4) }
                                "f64" => { Ok(__Field::__field5) }
                                "string" => { Ok(__Field::__field6) }
                                "i32a" => { Ok(__Field::__field7) }
                                "hash" => { Ok(__Field::__field8) }
                                _ => {
                                    Err(::serde::de::Error::unknown_field_error(value))
                                }
                            }
                        }
                        fn visit_bytes<E>(&mut self, value: &[u8])
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match ::std::str::from_utf8(value) {
                                Ok(s) => self.visit_str(s),
                                _ => Err(::serde::de::Error::syntax_error()),
                            }
                        }
                    }
                    deserializer.visit(__FieldVisitor::<D>{phantom:
                                                               PhantomData,})
                }
            }
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                Data1;
                #[inline]
                fn visit_map<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Data1, __V::Error> where
                 __V: ::serde::de::MapVisitor {
                    {
                        let mut __field0 = None;
                        let mut __field1 = None;
                        let mut __field2 = None;
                        let mut __field3 = None;
                        let mut __field4 = None;
                        let mut __field5 = None;
                        let mut __field6 = None;
                        let mut __field7 = None;
                        let mut __field8 = None;
                        loop  {
                            match try!(visitor . visit_key (  )) {
                                Some(key) => {
                                    match key {
                                        __Field::__field0 => {
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field1 => {
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field2 => {
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field3 => {
                                            __field3 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field4 => {
                                            __field4 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field5 => {
                                            __field5 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field6 => {
                                            __field6 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field7 => {
                                            __field7 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field8 => {
                                            __field8 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                    }
                                }
                                _ => break ,
                            }
                        }
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "i32" )),
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None =>
                                try!(visitor . missing_field ( "i64" )),
                            };
                        let __field2 =
                            match __field2 {
                                Some(__field2) => __field2,
                                None =>
                                try!(visitor . missing_field ( "u32" )),
                            };
                        let __field3 =
                            match __field3 {
                                Some(__field3) => __field3,
                                None =>
                                try!(visitor . missing_field ( "u64" )),
                            };
                        let __field4 =
                            match __field4 {
                                Some(__field4) => __field4,
                                None =>
                                try!(visitor . missing_field ( "f32" )),
                            };
                        let __field5 =
                            match __field5 {
                                Some(__field5) => __field5,
                                None =>
                                try!(visitor . missing_field ( "f64" )),
                            };
                        let __field6 =
                            match __field6 {
                                Some(__field6) => __field6,
                                None =>
                                try!(visitor . missing_field ( "string" )),
                            };
                        let __field7 =
                            match __field7 {
                                Some(__field7) => __field7,
                                None =>
                                try!(visitor . missing_field ( "i32a" )),
                            };
                        let __field8 =
                            match __field8 {
                                Some(__field8) => __field8,
                                None =>
                                try!(visitor . missing_field ( "hash" )),
                            };
                        try!(visitor . end (  ));
                        Ok(Data1{i32: __field0,
                                 i64: __field1,
                                 u32: __field2,
                                 u64: __field3,
                                 f32: __field4,
                                 f64: __field5,
                                 string: __field6,
                                 i32a: __field7,
                                 hash: __field8,})
                    }
                }
            }
            deserializer.visit_named_map("Data1",
                                         __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
#[automatically_derived]
impl ::serde::ser::Serialize for Data1 {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a Data1,
            }
            impl <'__a> ::serde::ser::MapVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "i32" , &self.value.i32 , ))))
                        }
                        1usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "i64" , &self.value.i64 , ))))
                        }
                        2usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "u32" , &self.value.u32 , ))))
                        }
                        3usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "u64" , &self.value.u64 , ))))
                        }
                        4usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "f32" , &self.value.f32 , ))))
                        }
                        5usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "f64" , &self.value.f64 , ))))
                        }
                        6usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "string" , &self.value.string , ))))
                        }
                        7usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "i32a" , &self.value.i32a , ))))
                        }
                        8usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "hash" , &self.value.hash , ))))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(9usize) }
            }
            serializer.visit_named_map("Data1",
                                       Visitor{value: self, state: 0,})
        }
    }
}
pub struct Example_2_2 {
    pub hr: u32,
    pub avg: f32,
    pub rbi: u32,
}
#[automatically_derived]
impl ::serde::de::Deserialize for Example_2_2 {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<Example_2_2, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            #[allow(non_camel_case_types)]
            enum __Field { __field0, __field1, __field2, }
            impl ::serde::de::Deserialize for __Field {
                #[inline]
                fn deserialize<D>(deserializer: &mut D)
                 -> ::std::result::Result<__Field, D::Error> where
                 D: ::serde::de::Deserializer {
                    use std::marker::PhantomData;
                    struct __FieldVisitor<D> {
                        phantom: PhantomData<D>,
                    }
                    impl <__D> ::serde::de::Visitor for __FieldVisitor<__D>
                     where __D: ::serde::de::Deserializer {
                        type
                        Value
                        =
                        __Field;
                        fn visit_str<E>(&mut self, value: &str)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                "hr" => { Ok(__Field::__field0) }
                                "avg" => { Ok(__Field::__field1) }
                                "rbi" => { Ok(__Field::__field2) }
                                _ => {
                                    Err(::serde::de::Error::unknown_field_error(value))
                                }
                            }
                        }
                        fn visit_bytes<E>(&mut self, value: &[u8])
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match ::std::str::from_utf8(value) {
                                Ok(s) => self.visit_str(s),
                                _ => Err(::serde::de::Error::syntax_error()),
                            }
                        }
                    }
                    deserializer.visit(__FieldVisitor::<D>{phantom:
                                                               PhantomData,})
                }
            }
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                Example_2_2;
                #[inline]
                fn visit_map<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Example_2_2, __V::Error> where
                 __V: ::serde::de::MapVisitor {
                    {
                        let mut __field0 = None;
                        let mut __field1 = None;
                        let mut __field2 = None;
                        loop  {
                            match try!(visitor . visit_key (  )) {
                                Some(key) => {
                                    match key {
                                        __Field::__field0 => {
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field1 => {
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field2 => {
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                    }
                                }
                                _ => break ,
                            }
                        }
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "hr" )),
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None =>
                                try!(visitor . missing_field ( "avg" )),
                            };
                        let __field2 =
                            match __field2 {
                                Some(__field2) => __field2,
                                None =>
                                try!(visitor . missing_field ( "rbi" )),
                            };
                        try!(visitor . end (  ));
                        Ok(Example_2_2{hr: __field0,
                                       avg: __field1,
                                       rbi: __field2,})
                    }
                }
            }
            deserializer.visit_named_map("Example_2_2",
                                         __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
#[automatically_derived]
impl ::serde::ser::Serialize for Example_2_2 {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a Example_2_2,
            }
            impl <'__a> ::serde::ser::MapVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "hr" , &self.value.hr , ))))
                        }
                        1usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "avg" , &self.value.avg , ))))
                        }
                        2usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "rbi" , &self.value.rbi , ))))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(3usize) }
            }
            serializer.visit_named_map("Example_2_2",
                                       Visitor{value: self, state: 0,})
        }
    }
}
impl Default for Example_2_2 {
    fn default() -> Self { Example_2_2{hr: 65, avg: 0.278, rbi: 147,} }
}
pub struct Example_2_3 {
    pub american: Vec<&'static str>,
    pub national: Vec<&'static str>,
}
#[automatically_derived]
impl ::serde::ser::Serialize for Example_2_3 {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a Example_2_3,
            }
            impl <'__a> ::serde::ser::MapVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "american" , &self.value.american ,
                                         ))))
                        }
                        1usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "national" , &self.value.national ,
                                         ))))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(2usize) }
            }
            serializer.visit_named_map("Example_2_3",
                                       Visitor{value: self, state: 0,})
        }
    }
}
impl Default for Example_2_3 {
    fn default() -> Self {
        Example_2_3{american:
                        vec!("Boston Red Sox" , "Detroit Tigers" ,
                             "New York Yankees"),
                    national:
                        vec!("New York Mets" , "Chicago Cubs" ,
                             "Atlanta Braves"),}
    }
}
pub struct Player {
    pub name: String,
    pub hr: u32,
    pub avg: f32,
}
#[automatically_derived]
impl ::serde::de::Deserialize for Player {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<Player, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            #[allow(non_camel_case_types)]
            enum __Field { __field0, __field1, __field2, }
            impl ::serde::de::Deserialize for __Field {
                #[inline]
                fn deserialize<D>(deserializer: &mut D)
                 -> ::std::result::Result<__Field, D::Error> where
                 D: ::serde::de::Deserializer {
                    use std::marker::PhantomData;
                    struct __FieldVisitor<D> {
                        phantom: PhantomData<D>,
                    }
                    impl <__D> ::serde::de::Visitor for __FieldVisitor<__D>
                     where __D: ::serde::de::Deserializer {
                        type
                        Value
                        =
                        __Field;
                        fn visit_str<E>(&mut self, value: &str)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                "name" => { Ok(__Field::__field0) }
                                "hr" => { Ok(__Field::__field1) }
                                "avg" => { Ok(__Field::__field2) }
                                _ => {
                                    Err(::serde::de::Error::unknown_field_error(value))
                                }
                            }
                        }
                        fn visit_bytes<E>(&mut self, value: &[u8])
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match ::std::str::from_utf8(value) {
                                Ok(s) => self.visit_str(s),
                                _ => Err(::serde::de::Error::syntax_error()),
                            }
                        }
                    }
                    deserializer.visit(__FieldVisitor::<D>{phantom:
                                                               PhantomData,})
                }
            }
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                Player;
                #[inline]
                fn visit_map<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Player, __V::Error> where
                 __V: ::serde::de::MapVisitor {
                    {
                        let mut __field0 = None;
                        let mut __field1 = None;
                        let mut __field2 = None;
                        loop  {
                            match try!(visitor . visit_key (  )) {
                                Some(key) => {
                                    match key {
                                        __Field::__field0 => {
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field1 => {
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field2 => {
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                    }
                                }
                                _ => break ,
                            }
                        }
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "name" )),
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None =>
                                try!(visitor . missing_field ( "hr" )),
                            };
                        let __field2 =
                            match __field2 {
                                Some(__field2) => __field2,
                                None =>
                                try!(visitor . missing_field ( "avg" )),
                            };
                        try!(visitor . end (  ));
                        Ok(Player{name: __field0,
                                  hr: __field1,
                                  avg: __field2,})
                    }
                }
            }
            deserializer.visit_named_map("Player",
                                         __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
#[automatically_derived]
impl ::serde::ser::Serialize for Player {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a Player,
            }
            impl <'__a> ::serde::ser::MapVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "name" , &self.value.name , ))))
                        }
                        1usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "hr" , &self.value.hr , ))))
                        }
                        2usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "avg" , &self.value.avg , ))))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(3usize) }
            }
            serializer.visit_named_map("Player",
                                       Visitor{value: self, state: 0,})
        }
    }
}
pub struct Example_2_4(Vec<Player>);
#[automatically_derived]
impl ::serde::de::Deserialize for Example_2_4 {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<Example_2_4, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                Example_2_4;
                fn visit_seq<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Example_2_4, __V::Error> where
                 __V: ::serde::de::SeqVisitor {
                    {
                        let __field0 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream_error());
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(Example_2_4(__field0))
                    }
                }
            }
            deserializer.visit_named_seq("Example_2_4",
                                         __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
#[automatically_derived]
impl ::serde::ser::Serialize for Example_2_4 {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a Example_2_4,
            }
            impl <'__a> ::serde::ser::SeqVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            let v =
                                try!(serializer . visit_seq_elt (
                                     & self.value.0 ));
                            Ok(Some(v))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(1usize) }
            }
            serializer.visit_named_seq("Example_2_4",
                                       Visitor{value: self, state: 0,})
        }
    }
}
impl Default for Example_2_4 {
    fn default() -> Self {
        Example_2_4(vec!(Player {
                         name : "Mark McGwire" . to_string (  ) , hr : 65 ,
                         avg : 0.278 } , Player {
                         name : "Sammy Sosa" . to_string (  ) , hr : 63 , avg
                         : 0.288 }))
    }
}
pub struct PlayerStat {
    pub hr: u32,
    pub avg: f32,
}
#[automatically_derived]
impl ::serde::de::Deserialize for PlayerStat {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<PlayerStat, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            #[allow(non_camel_case_types)]
            enum __Field { __field0, __field1, }
            impl ::serde::de::Deserialize for __Field {
                #[inline]
                fn deserialize<D>(deserializer: &mut D)
                 -> ::std::result::Result<__Field, D::Error> where
                 D: ::serde::de::Deserializer {
                    use std::marker::PhantomData;
                    struct __FieldVisitor<D> {
                        phantom: PhantomData<D>,
                    }
                    impl <__D> ::serde::de::Visitor for __FieldVisitor<__D>
                     where __D: ::serde::de::Deserializer {
                        type
                        Value
                        =
                        __Field;
                        fn visit_str<E>(&mut self, value: &str)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                "hr" => { Ok(__Field::__field0) }
                                "avg" => { Ok(__Field::__field1) }
                                _ => {
                                    Err(::serde::de::Error::unknown_field_error(value))
                                }
                            }
                        }
                        fn visit_bytes<E>(&mut self, value: &[u8])
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match ::std::str::from_utf8(value) {
                                Ok(s) => self.visit_str(s),
                                _ => Err(::serde::de::Error::syntax_error()),
                            }
                        }
                    }
                    deserializer.visit(__FieldVisitor::<D>{phantom:
                                                               PhantomData,})
                }
            }
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                PlayerStat;
                #[inline]
                fn visit_map<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<PlayerStat, __V::Error> where
                 __V: ::serde::de::MapVisitor {
                    {
                        let mut __field0 = None;
                        let mut __field1 = None;
                        loop  {
                            match try!(visitor . visit_key (  )) {
                                Some(key) => {
                                    match key {
                                        __Field::__field0 => {
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field1 => {
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                    }
                                }
                                _ => break ,
                            }
                        }
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "hr" )),
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None =>
                                try!(visitor . missing_field ( "avg" )),
                            };
                        try!(visitor . end (  ));
                        Ok(PlayerStat{hr: __field0, avg: __field1,})
                    }
                }
            }
            deserializer.visit_named_map("PlayerStat",
                                         __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
#[automatically_derived]
impl ::serde::ser::Serialize for PlayerStat {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a PlayerStat,
            }
            impl <'__a> ::serde::ser::MapVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "hr" , &self.value.hr , ))))
                        }
                        1usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "avg" , &self.value.avg , ))))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(2usize) }
            }
            serializer.visit_named_map("PlayerStat",
                                       Visitor{value: self, state: 0,})
        }
    }
}
pub struct Example_2_6(HashMap<String, PlayerStat>);
#[automatically_derived]
impl ::serde::de::Deserialize for Example_2_6 {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<Example_2_6, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                Example_2_6;
                fn visit_seq<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Example_2_6, __V::Error> where
                 __V: ::serde::de::SeqVisitor {
                    {
                        let __field0 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream_error());
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(Example_2_6(__field0))
                    }
                }
            }
            deserializer.visit_named_seq("Example_2_6",
                                         __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
#[automatically_derived]
impl ::serde::ser::Serialize for Example_2_6 {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a Example_2_6,
            }
            impl <'__a> ::serde::ser::SeqVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            let v =
                                try!(serializer . visit_seq_elt (
                                     & self.value.0 ));
                            Ok(Some(v))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(1usize) }
            }
            serializer.visit_named_seq("Example_2_6",
                                       Visitor{value: self, state: 0,})
        }
    }
}
impl Default for Example_2_6 {
    fn default() -> Self {
        let mut h = HashMap::new();
        h.insert("Mark McGwire".to_string(), PlayerStat{hr: 65, avg: 0.278,});
        h.insert("Samy Sosa".to_string(), PlayerStat{hr: 63, avg: 0.288,});
        Example_2_6(h)
    }
}
pub struct Example_2_7(Vec<Vec<&'static str>>);
#[automatically_derived]
impl ::serde::ser::Serialize for Example_2_7 {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a Example_2_7,
            }
            impl <'__a> ::serde::ser::SeqVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            let v =
                                try!(serializer . visit_seq_elt (
                                     & self.value.0 ));
                            Ok(Some(v))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(1usize) }
            }
            serializer.visit_named_seq("Example_2_7",
                                       Visitor{value: self, state: 0,})
        }
    }
}
impl Default for Example_2_7 {
    fn default() -> Self {
        Example_2_7(vec!(vec ! [ "Mark McGwire" , "Sammy Sosa" , "Ken Griffey"
                         ] , vec ! [ "Chicago Cubs" , "St Louis Cardinals" ]))
    }
}
pub struct GameEvent {
    pub time: &'static str,
    pub player: &'static str,
    pub action: &'static str,
}
#[automatically_derived]
impl ::serde::ser::Serialize for GameEvent {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a GameEvent,
            }
            impl <'__a> ::serde::ser::MapVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "time" , &self.value.time , ))))
                        }
                        1usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "player" , &self.value.player , ))))
                        }
                        2usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "action" , &self.value.action , ))))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(3usize) }
            }
            serializer.visit_named_map("GameEvent",
                                       Visitor{value: self, state: 0,})
        }
    }
}
pub struct Example_2_8(Vec<GameEvent>);
#[automatically_derived]
impl ::serde::ser::Serialize for Example_2_8 {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a Example_2_8,
            }
            impl <'__a> ::serde::ser::SeqVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            let v =
                                try!(serializer . visit_seq_elt (
                                     & self.value.0 ));
                            Ok(Some(v))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(1usize) }
            }
            serializer.visit_named_seq("Example_2_8",
                                       Visitor{value: self, state: 0,})
        }
    }
}
impl Default for Example_2_8 {
    fn default() -> Self {
        Example_2_8(vec!(GameEvent {
                         time : "20:03:20" , player : "Sammy Sosa" , action :
                         "strike (miss)" , } , GameEvent {
                         time : "20:03:47" , player : "Sammy Sosa" , action :
                         "grand slam" , }))
    }
}
pub struct Example_2_9 {
    pub hr: Vec<&'static str>,
    pub rbi: Vec<&'static str>,
}
#[automatically_derived]
impl ::serde::ser::Serialize for Example_2_9 {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a Example_2_9,
            }
            impl <'__a> ::serde::ser::MapVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "hr" , &self.value.hr , ))))
                        }
                        1usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "rbi" , &self.value.rbi , ))))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(2usize) }
            }
            serializer.visit_named_map("Example_2_9",
                                       Visitor{value: self, state: 0,})
        }
    }
}
impl Default for Example_2_9 {
    fn default() -> Self {
        Example_2_9{hr: vec!("Mark McGwire" , "Sammy Sosa"),
                    rbi: vec!("Sammy Sosa" , "Ken Griffey"),}
    }
}
pub struct Example_2_11(HashMap<(String, String), Vec<String>>);
#[automatically_derived]
impl ::serde::de::Deserialize for Example_2_11 {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<Example_2_11, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                Example_2_11;
                fn visit_seq<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Example_2_11, __V::Error> where
                 __V: ::serde::de::SeqVisitor {
                    {
                        let __field0 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream_error());
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(Example_2_11(__field0))
                    }
                }
            }
            deserializer.visit_named_seq("Example_2_11",
                                         __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
#[automatically_derived]
impl ::serde::ser::Serialize for Example_2_11 {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a Example_2_11,
            }
            impl <'__a> ::serde::ser::SeqVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            let v =
                                try!(serializer . visit_seq_elt (
                                     & self.value.0 ));
                            Ok(Some(v))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(1usize) }
            }
            serializer.visit_named_seq("Example_2_11",
                                       Visitor{value: self, state: 0,})
        }
    }
}
impl Default for Example_2_11 {
    fn default() -> Self {
        let mut h = HashMap::new();
        h.insert(("Detroit Tigers".to_string(), "Chicago cubs".to_string()),
                 vec!("2001-07-23" . to_string (  )));
        Example_2_11(h)
    }
}
pub struct CartItem {
    item: String,
    quantity: u32,
}
#[automatically_derived]
impl ::serde::de::Deserialize for CartItem {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<CartItem, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            #[allow(non_camel_case_types)]
            enum __Field { __field0, __field1, }
            impl ::serde::de::Deserialize for __Field {
                #[inline]
                fn deserialize<D>(deserializer: &mut D)
                 -> ::std::result::Result<__Field, D::Error> where
                 D: ::serde::de::Deserializer {
                    use std::marker::PhantomData;
                    struct __FieldVisitor<D> {
                        phantom: PhantomData<D>,
                    }
                    impl <__D> ::serde::de::Visitor for __FieldVisitor<__D>
                     where __D: ::serde::de::Deserializer {
                        type
                        Value
                        =
                        __Field;
                        fn visit_str<E>(&mut self, value: &str)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                "item" => { Ok(__Field::__field0) }
                                "quantity" => { Ok(__Field::__field1) }
                                _ => {
                                    Err(::serde::de::Error::unknown_field_error(value))
                                }
                            }
                        }
                        fn visit_bytes<E>(&mut self, value: &[u8])
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match ::std::str::from_utf8(value) {
                                Ok(s) => self.visit_str(s),
                                _ => Err(::serde::de::Error::syntax_error()),
                            }
                        }
                    }
                    deserializer.visit(__FieldVisitor::<D>{phantom:
                                                               PhantomData,})
                }
            }
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                CartItem;
                #[inline]
                fn visit_map<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<CartItem, __V::Error> where
                 __V: ::serde::de::MapVisitor {
                    {
                        let mut __field0 = None;
                        let mut __field1 = None;
                        loop  {
                            match try!(visitor . visit_key (  )) {
                                Some(key) => {
                                    match key {
                                        __Field::__field0 => {
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field1 => {
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                    }
                                }
                                _ => break ,
                            }
                        }
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "item" )),
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None =>
                                try!(visitor . missing_field ( "quantity" )),
                            };
                        try!(visitor . end (  ));
                        Ok(CartItem{item: __field0, quantity: __field1,})
                    }
                }
            }
            deserializer.visit_named_map("CartItem",
                                         __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
#[automatically_derived]
impl ::serde::ser::Serialize for CartItem {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a CartItem,
            }
            impl <'__a> ::serde::ser::MapVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "item" , &self.value.item , ))))
                        }
                        1usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "quantity" , &self.value.quantity ,
                                         ))))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(2usize) }
            }
            serializer.visit_named_map("CartItem",
                                       Visitor{value: self, state: 0,})
        }
    }
}
pub fn example_2_12_new() -> Vec<CartItem> {
    vec!(CartItem { item : "Super Hoop" . to_string (  ) , quantity : 1 } ,
         CartItem { item : "Basketball" . to_string (  ) , quantity : 4 } ,
         CartItem { item : "Big Shoes" . to_string (  ) , quantity : 1 } ,)
}
pub struct LogEntry {
    time: Option<String>,
    date: Option<String>,
    user: String,
    warning: Option<String>,
    fatal: Option<String>,
    stack: Option<Vec<StackFrame>>,
}
#[automatically_derived]
impl ::serde::de::Deserialize for LogEntry {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<LogEntry, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
            }
            impl ::serde::de::Deserialize for __Field {
                #[inline]
                fn deserialize<D>(deserializer: &mut D)
                 -> ::std::result::Result<__Field, D::Error> where
                 D: ::serde::de::Deserializer {
                    use std::marker::PhantomData;
                    struct __FieldVisitor<D> {
                        phantom: PhantomData<D>,
                    }
                    impl <__D> ::serde::de::Visitor for __FieldVisitor<__D>
                     where __D: ::serde::de::Deserializer {
                        type
                        Value
                        =
                        __Field;
                        fn visit_str<E>(&mut self, value: &str)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                "Time" => { Ok(__Field::__field0) }
                                "Date" => { Ok(__Field::__field1) }
                                "User" => { Ok(__Field::__field2) }
                                "Warning" => { Ok(__Field::__field3) }
                                "Fatal" => { Ok(__Field::__field4) }
                                "Stack" => { Ok(__Field::__field5) }
                                _ => {
                                    Err(::serde::de::Error::unknown_field_error(value))
                                }
                            }
                        }
                        fn visit_bytes<E>(&mut self, value: &[u8])
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match ::std::str::from_utf8(value) {
                                Ok(s) => self.visit_str(s),
                                _ => Err(::serde::de::Error::syntax_error()),
                            }
                        }
                    }
                    deserializer.visit(__FieldVisitor::<D>{phantom:
                                                               PhantomData,})
                }
            }
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                LogEntry;
                #[inline]
                fn visit_map<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<LogEntry, __V::Error> where
                 __V: ::serde::de::MapVisitor {
                    {
                        let mut __field0 = None;
                        let mut __field1 = None;
                        let mut __field2 = None;
                        let mut __field3 = None;
                        let mut __field4 = None;
                        let mut __field5 = None;
                        loop  {
                            match try!(visitor . visit_key (  )) {
                                Some(key) => {
                                    match key {
                                        __Field::__field0 => {
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field1 => {
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field2 => {
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field3 => {
                                            __field3 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field4 => {
                                            __field4 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field5 => {
                                            __field5 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                    }
                                }
                                _ => break ,
                            }
                        }
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "Time" )),
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None =>
                                try!(visitor . missing_field ( "Date" )),
                            };
                        let __field2 =
                            match __field2 {
                                Some(__field2) => __field2,
                                None =>
                                try!(visitor . missing_field ( "User" )),
                            };
                        let __field3 =
                            match __field3 {
                                Some(__field3) => __field3,
                                None =>
                                try!(visitor . missing_field ( "Warning" )),
                            };
                        let __field4 =
                            match __field4 {
                                Some(__field4) => __field4,
                                None =>
                                try!(visitor . missing_field ( "Fatal" )),
                            };
                        let __field5 =
                            match __field5 {
                                Some(__field5) => __field5,
                                None =>
                                try!(visitor . missing_field ( "Stack" )),
                            };
                        try!(visitor . end (  ));
                        Ok(LogEntry{time: __field0,
                                    date: __field1,
                                    user: __field2,
                                    warning: __field3,
                                    fatal: __field4,
                                    stack: __field5,})
                    }
                }
            }
            deserializer.visit_named_map("LogEntry",
                                         __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
#[automatically_derived]
impl ::serde::ser::Serialize for LogEntry {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a LogEntry,
            }
            impl <'__a> ::serde::ser::MapVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "Time" , &self.value.time , ))))
                        }
                        1usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "Date" , &self.value.date , ))))
                        }
                        2usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "User" , &self.value.user , ))))
                        }
                        3usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "Warning" , &self.value.warning ,
                                         ))))
                        }
                        4usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "Fatal" , &self.value.fatal , ))))
                        }
                        5usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "Stack" , &self.value.stack , ))))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(6usize) }
            }
            serializer.visit_named_map("LogEntry",
                                       Visitor{value: self, state: 0,})
        }
    }
}
pub struct StackFrame {
    file: String,
    line: u64,
    code: String,
}
#[automatically_derived]
impl ::serde::de::Deserialize for StackFrame {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<StackFrame, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            #[allow(non_camel_case_types)]
            enum __Field { __field0, __field1, __field2, }
            impl ::serde::de::Deserialize for __Field {
                #[inline]
                fn deserialize<D>(deserializer: &mut D)
                 -> ::std::result::Result<__Field, D::Error> where
                 D: ::serde::de::Deserializer {
                    use std::marker::PhantomData;
                    struct __FieldVisitor<D> {
                        phantom: PhantomData<D>,
                    }
                    impl <__D> ::serde::de::Visitor for __FieldVisitor<__D>
                     where __D: ::serde::de::Deserializer {
                        type
                        Value
                        =
                        __Field;
                        fn visit_str<E>(&mut self, value: &str)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                "file" => { Ok(__Field::__field0) }
                                "line" => { Ok(__Field::__field1) }
                                "code" => { Ok(__Field::__field2) }
                                _ => {
                                    Err(::serde::de::Error::unknown_field_error(value))
                                }
                            }
                        }
                        fn visit_bytes<E>(&mut self, value: &[u8])
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match ::std::str::from_utf8(value) {
                                Ok(s) => self.visit_str(s),
                                _ => Err(::serde::de::Error::syntax_error()),
                            }
                        }
                    }
                    deserializer.visit(__FieldVisitor::<D>{phantom:
                                                               PhantomData,})
                }
            }
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                StackFrame;
                #[inline]
                fn visit_map<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<StackFrame, __V::Error> where
                 __V: ::serde::de::MapVisitor {
                    {
                        let mut __field0 = None;
                        let mut __field1 = None;
                        let mut __field2 = None;
                        loop  {
                            match try!(visitor . visit_key (  )) {
                                Some(key) => {
                                    match key {
                                        __Field::__field0 => {
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field1 => {
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                        __Field::__field2 => {
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value (  )));
                                        }
                                    }
                                }
                                _ => break ,
                            }
                        }
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "file" )),
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None =>
                                try!(visitor . missing_field ( "line" )),
                            };
                        let __field2 =
                            match __field2 {
                                Some(__field2) => __field2,
                                None =>
                                try!(visitor . missing_field ( "code" )),
                            };
                        try!(visitor . end (  ));
                        Ok(StackFrame{file: __field0,
                                      line: __field1,
                                      code: __field2,})
                    }
                }
            }
            deserializer.visit_named_map("StackFrame",
                                         __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
#[automatically_derived]
impl ::serde::ser::Serialize for StackFrame {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a StackFrame,
            }
            impl <'__a> ::serde::ser::MapVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    match self.state {
                        0usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "file" , &self.value.file , ))))
                        }
                        1usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "line" , &self.value.line , ))))
                        }
                        2usize => {
                            self.state += 1;
                            Ok(Some(try!(serializer . visit_map_elt (
                                         "code" , &self.value.code , ))))
                        }
                        _ => Ok(None),
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(3usize) }
            }
            serializer.visit_named_map("StackFrame",
                                       Visitor{value: self, state: 0,})
        }
    }
}
pub fn example_2_28_new() -> Vec<LogEntry> {
    vec!(LogEntry {
         time : Some ( "2001-11-23 15:01:42 -5" . to_string (  ) ) , date :
         None , user : "ed" . to_string (  ) , warning : Some (
         "This is an error message for the log file" . to_string (  ) ) ,
         fatal : None , stack : None } , LogEntry {
         time : Some ( "2001-11-23 15:02:31 -5" . to_string (  ) ) , date :
         None , user : "ed" . to_string (  ) , warning : Some (
         "A slightly different error message." . to_string (  ) ) , fatal :
         None , stack : None } , LogEntry {
         time : None , date : Some ( "2001-11-23 15:03:17 -5" . to_string (  )
         ) , user : "ed" . to_string (  ) , warning : None , fatal : Some (
         r#"Unknown variable "bar""# . to_string (  ) ) , stack : Some (
         vec ! [
         StackFrame {
         file : "TopClass.py" . to_string (  ) , line : 23 , code :
         r#"x = MoreObject("345\n")"# . to_string (  ) , } , StackFrame {
         file : "MoreClass.py" . to_string (  ) , line : 58 , code :
         "foo = bar" . to_string (  ) , } ] ) } ,)
}
