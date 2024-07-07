use crate::parser::{Object, ObjectType};

pub struct Transpiler<'a> {
    root_object: &'a Object
}

impl<'a> Transpiler<'a> {
    pub fn new(root_object: &'a Object) -> Self {
        Self {
            root_object
        }
    }

    pub fn visit_object(&self, obj: &Object, depth: i8) -> String {
        match &obj.obj_type {
            ObjectType::Map(map) => {
                if map.len() == 0 {
                    return "{}".to_owned();
                }
                let mut built_string = String::from("{\n");
                let mut current_key_index = 0;
                for (x, value) in map {
                    current_key_index += 1;
                    for _ in 0..(depth + 1) {
                        built_string.push_str("  ")
                    }
                    built_string.push_str(&format!("\"{x}\""));
                    built_string.push_str(": ");
                    let formatted_value = &self.visit_object(value, depth + 1);
                    built_string.push_str(formatted_value);
                    let map_length: i8 = map.len().try_into().unwrap();
                    if current_key_index != (map_length) {
                        built_string.push_str(",\n");
                    } else {
                        built_string.push_str("\n");
                    }
                }
                for _ in 0..(depth) {
                    built_string.push_str("  ")
                }
                built_string.push_str("}");
                return built_string;
            },
            ObjectType::Array(array) => {
                let mut built_string = String::from("");
                if array.len() == 0 { return "[]".to_string() };

                built_string.push_str("[\n");
                for _ in 0..(depth + 1) {
                    built_string.push_str("  ");
                }

                let mut i = 0;
                for obj in array {
                    i += 1;
                    let x = self.visit_object(obj, depth + 1);
                    built_string.push_str(&x);
                    if i != array.len() {
                        built_string.push_str(",\n");
                        for _ in 0..(depth + 1) {
                            built_string.push_str("  ");
                        }
                    } else {
                        built_string.push_str("\n");
                    }
                }
                for _ in 0..(depth) {
                    built_string.push_str("  ");
                }
                built_string.push_str("]");
                return built_string;
            },
            ObjectType::String(string) => {
                return format!("\"{string}\"");
            },
            ObjectType::Number(num) => {
                return format!("{num}");
            },
            ObjectType::Null => {
                return format!("null");
            },
            ObjectType::False => {
                return format!("false");
            },
            ObjectType::True => {
                return format!("true");
            },
        }
    }

    pub fn format(&self) -> String {
        let depth = 0;
        return format!("{}", self.visit_object(self.root_object, depth));
    }
}

