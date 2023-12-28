use serde::{Deserialize, Serialize};
use std::{fs, env};
use raylib_rs_plain_common as rl_common;
use regex::Regex;
use std::process::Command;
use convert_case::{Case, Casing};

fn main() {
    let raylib_api_json_path = "../raylib-rs-plain-sys/".to_owned() + rl_common::RAYLIB_REPOSITORY_PATH + "/parser/output/raylib_api.json";
    let content = fs::read_to_string(raylib_api_json_path).unwrap();
    let raylib_api:RaylibApi = serde_json::from_str(&content).unwrap();

    generate_function(&raylib_api);
	generate_define(&raylib_api);
}

#[derive(Serialize, Deserialize)]
struct RaylibApi {
    defines: Vec<Identifier>,
    functions: Vec<FunctionIdentifier>,
    structs: Vec<StructIdentifier>,
    aliases: Vec<AliaseIdentifier>,
    callbacks: Vec<CallbackIdentifier>,
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
    params: Option<Vec<ArgIdentifier>>,
}

#[derive(Serialize, Deserialize)]
struct ArgIdentifier {
    #[serde(rename = "type")]
    arg_type:String,
    name:String,
}

#[derive(Serialize, Deserialize)]
struct StructIdentifier {
    name:String,
}

#[derive(Serialize, Deserialize)]
struct AliaseIdentifier {
    name:String,
}

#[derive(Serialize, Deserialize)]
struct CallbackIdentifier {
    name:String,
}

fn generate_function(raylib_api:&RaylibApi) {
    let mut raylib_function = String::new();
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
	raylib_function.push_str(&("/* automatically generated by ".to_owned() + &pkg_name + " */\n"));
    raylib_function.push_str("#![allow(clippy::not_unsafe_ptr_arg_deref, clippy::too_many_arguments)]\n");
    raylib_function.push_str(generate_header(raylib_api).join("\n").as_str());

    for identifier in &raylib_api.functions {
        let comment = format!(
            "/** {} */\n",
             identifier.description,
        );
        let return_type = c_to_rs_return_type(identifier.return_type.as_str());
        let body = format!(
            "pub fn {}({}){} {{ {} }}\n",
            identifier.name.to_case(Case::Snake),
            generate_arg(&identifier.params),
            return_type,
            generate_function_body(identifier, return_type.as_str()),
        );
        raylib_function.push_str(&(comment + &body + "\n"));
    }

    fs::write("./src/function.rs", raylib_function).unwrap();
    Command::new("rustfmt")
        .arg("src/function.rs")
        .status().unwrap();
}

fn generate_arg(_params:&Option<Vec<ArgIdentifier>>) -> String {
    if _params.is_none() {
        return "".to_string();
    }
    let params = _params.as_ref().unwrap();

    let rs_params:Vec<String> = params.iter().filter_map(
        |param|
        // Variable length arguments not supported in rust
        if param.name == "args" && param.arg_type == "..." {
            Option::None
        } else {
            Option::Some(
                fix_reserved_keyword(param.name.to_case(Case::Snake).as_str()) + ":" + c_to_rs_type(param.arg_type.as_str()).as_str()
            )
        }
    ).collect();
    return rs_params.join(", ");
}

fn fix_reserved_keyword(name:&str) -> String {
    return match name {
        "box" => "box_".to_owned(),
        "type" => "type_".to_owned(),
        _ => name.to_owned(),
    }
}

fn generate_function_body(function:&FunctionIdentifier, return_type:&str) -> String {
    let mut body = String::new();
    if !return_type.is_empty() {
        body += "return ";
    };

    let arg:String = match &function.params {
        Option::None => "".to_string(),
        Option::Some(params) => {
        let rs_params:Vec<String> = params.iter().filter_map(
            |param|
            // Variable length arguments not supported in rust
            if param.name == "args" && param.arg_type == "..." {
                Option::None
            } else {
                Option::Some(fix_reserved_keyword(param.name.to_case(Case::Snake).as_str()))
            }
        ).collect();
        rs_params.join(", ")
        },
    };

    body += ("unsafe { rl::".to_owned()
        + function.name.as_str()
        + "(" + arg.as_str() + ") };"
    ).as_str();
    return body;
}

fn generate_header(raylib_api:&RaylibApi) -> Vec<String> {
    let mut header:Vec<String> = vec![
        "use raylib_rs_plain_sys as rl;".to_string(),
        // "use std::ffi::CString;".to_string(),
        "use ::std::os::raw::c_int;".to_string(),
        "use ::std::os::raw::c_uint;".to_string(),
        "use ::std::os::raw::c_long;".to_string(),
        "use ::std::os::raw::c_void;".to_string(),
        "use ::std::os::raw::c_uchar;".to_string(),
        "use ::std::os::raw::c_char;".to_string(),
        "\n".to_string(),
    ];

    let struct_use:Vec<String> = raylib_api.structs.iter().map(
        |identifier|
        "pub use rl::".to_owned() + &identifier.name + ";"
    ).collect();
    header.extend(struct_use);

    let aliases:Vec<String> = raylib_api.aliases.iter().map(
        |identifier|
        "pub use rl::".to_owned() + &identifier.name + ";"
    ).collect();
    header.extend(aliases);

    let callbacks:Vec<String> = raylib_api.callbacks.iter().map(
        |identifier|
        "pub use rl::".to_owned() + &identifier.name + ";"
    ).collect();
    header.extend(callbacks);

    header.push("\n".to_string());
    return header;
}

fn c_to_rs_type(c_type:&str) -> String {
    let mut modifier:String = String::new();
    let mut unprocessed_elements:Vec<&str> = Vec::new();
    for type_element in c_type.split(' ') {
        let mut asterisk_part:String = String::new();
        for asterisk in type_element.chars() {
            if asterisk != '*' {
                continue;
            }
            // First asterisk if there are multiple asterisks.
            if type_element.len() > 1 && !asterisk_part.contains('*') {
                asterisk_part += "*mut ";
                continue;
            }
            if c_type.contains("const") {
                asterisk_part += "*";
            } else {
                asterisk_part += "*mut ";
            }
        }
        if !asterisk_part.is_empty() {
            modifier = asterisk_part + modifier.as_str();
            continue;
        }

        if type_element == "const" {
            modifier += (type_element.to_owned() + " ").as_str();
            continue;
        }

        unprocessed_elements.push(type_element);
    }

    let c_type_main = unprocessed_elements.join(" ");
    let rust_type = match c_type_main.as_str() {
        "unsigned char" => "c_uchar",
        "unsigned int" => "c_uint",
        "int" => "c_int",
        "long" => "c_long",
        "float" => "f32",
        "double" => "f64",
        "void" => "c_void",
        "char" => "c_char",
        _ => c_type_main.as_str(),
    };
    return modifier + rust_type;
}

fn c_to_rs_return_type(c_type:&str) -> String {
    if c_type == "void" {
        return "".to_owned();
    }

    return " -> ".to_owned() + c_to_rs_type(c_type).as_str();
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
            let color_array:Vec<&str> = line.split(',').collect();
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

