use raylib::prelude::*;
use serde_json::{Result, Value};
use std::{fs, io::Error};

trait Desrialisable {
    fn as_vector2(&self) -> Vector2;
    fn as_color(&self) -> Color;
}

impl Desrialisable for Value {
    fn as_vector2(&self) -> Vector2 {
        let mut vec = Vector2::new(0.0, 0.0);

        println!("Vector node: {}", self);

        match &self[0] {
            Value::Number(num) => vec.x = num.as_f64().expect("Not a float") as f32,
            _ => panic!("not a number"),
        }
        match &self[1] {
            Value::Number(num) => vec.y = num.as_f64().expect("Not a float") as f32,
            _ => panic!("not a number"),
        }

        vec
    }

    fn as_color(&self) -> Color {
        let mut clr = Color::new(0, 0, 0, 0);

        match &self[0] {
            Value::Number(num) => clr.r = num.as_u64().expect("Not a float") as u8,
            _ => panic!("not a number"),
        }
        match &self[1] {
            Value::Number(num) => clr.g = num.as_u64().expect("Not a float") as u8,
            _ => panic!("not a number"),
        }
        match &self[2] {
            Value::Number(num) => clr.b = num.as_u64().expect("Not a float") as u8,
            _ => panic!("not a number"),
        }
        match &self[3] {
            Value::Number(num) => clr.a = num.as_u64().expect("Not a float") as u8,
            _ => panic!("not a number"),
        }

        clr
    }
}

pub fn deserialise_planets(path: String) -> Vec<Body> {
    let json_string = fs::read_to_string(path).expect("Error reading file");
    println!("Text in file: {}", &json_string);

    let data: Value = serde_json::from_str(&json_string).expect("lmao");

    let mut vec: Vec<Body> = vec![];

    let mut body_data = &data[0];
    let mut i = 0;
    while !body_data.is_null() {
        vec.push(Body {
            pos: body_data["pos"].as_vector2(),
            vel: body_data["vel"].as_vector2(),
            radius: body_data["radius"].as_f64().expect("") as f32,
            mass: body_data["mass"].as_f64().expect("") as f32,
            clr: body_data["clr"].as_color(),
            body_type: String::from(body_data["type"].as_str().expect("")),
        });

        i += 1;
        body_data = &data[i]
    }

    vec
}

#[derive(Debug)]
pub struct Body {
    pub pos: Vector2,
    pub vel: Vector2,
    pub mass: f32,
    pub radius: f32,
    pub clr: Color,
    pub body_type: String,
}

pub struct Star {
    pub pos: Vector2,
    pub clr: Color,
    pub size: f32,
}
