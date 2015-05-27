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
