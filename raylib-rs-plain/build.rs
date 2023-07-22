use serde::{Deserialize, Serialize};
use std::{fs, env};
use raylib_rs_plain_common as rl_common;
use regex::Regex;
use std::process::Command;
use convert_case::{Case, Casing};

fn main() {
    let content = fs::read_to_string(
        "../raylib-rs-plain-sys/".to_owned() + rl_common::RAYLIB_REPOSITORY_PATH + "/parser/output/raylib_api.json"
    ).unwrap();

    let raylib_api:RaylibApi = serde_json::from_str(&content).unwrap();

    generate_function(&raylib_api);
	generate_define(&raylib_api);
}

#[derive(Serialize, Deserialize)]
struct RaylibApi {
    defines: Vec<Identifier>,
    functions: Vec<FunctionIdentifier>,
}

#[derive(Serialize, Deserialize)]
struct Identifier {
    name:String,
    #[serde(rename = "type")]
    type_item:String,
    value:serde_json::Value,
}

#[derive(Serialize, Deserialize)]
struct FunctionIdentifier {
    name:String,
    description:String,
    #[serde(rename = "returnType")]
    return_type:String, // todo I want to parse this at a high level to enum, not string.
}

fn generate_function(raylib_api:&RaylibApi) {
    let mut raylib_function = String::new();
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
	raylib_function.push_str(&("/* automatically generated by ".to_owned() + &pkg_name + " */\n"));

    for identifier in &raylib_api.functions {
        let comment = format!(
            "/** {} */\n",
             identifier.description,
       );
        let body = format!(
            "pub fn {}({}){} {{ {} }}\n",
            identifier.name.to_case(Case::Snake),
            "", // todo args
            c_to_rs_return_type(identifier.return_type.as_str()),
            identifier.name,
        );
        raylib_function.push_str(&(comment + &body + "\n"));
    }

    fs::write("./src/function.rs", raylib_function).unwrap();
    Command::new("rustfmt")
        .arg("src/function.rs")
        .status().unwrap();
}

fn c_to_rs_return_type(c_type:&str) -> String {
    let rs_type:Option<&str> =  match c_type {
        "void" => Option::None,
        _ => Option::Some(c_type),
    };
    return match rs_type {
        Option::None => "".to_owned(),
        Option::Some(val) => " -> ".to_owned() + val,
    }
}

fn generate_define(raylib_api:&RaylibApi) {
    let mut raylib_define = String::new();
	let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
	raylib_define.push_str(&("/* automatically generated by ".to_owned() + &pkg_name + " */\n"));
    raylib_define.push_str("use raylib_rs_plain_sys as rl;\n");
    raylib_define.push_str("pub use rl::Color;\n");
    let reg = Regex::new(r"[^0-9,]").unwrap();
    for identifier in &raylib_api.defines {
        if identifier.type_item == "COLOR" {
            println!("{}", identifier.value);
            let color_value = identifier.value.to_string();
            let line = reg.replace_all(&color_value, "");
            let color_array:Vec<&str> = line.split(",").collect();
            let line = format!(
                "pub const {}:Color = Color {{r:{}, g:{}, b:{}, a:{}}};",
                identifier.name,
                color_array[0],
                color_array[1],
                color_array[2],
                color_array[3]
            );
            raylib_define.push_str(&(line + "\n"));
        }
    }

    fs::write("./src/color_define.rs", raylib_define).unwrap();
}

