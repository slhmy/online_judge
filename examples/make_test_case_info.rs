#[macro_use] extern crate serde_derive;

use md5::Md5;
use digest::Digest;
use hex::ToHex;
use serde_json::json;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

fn hash_token<D: Digest>(key: &str, output: &mut [u8]) {
    let mut hasher = D::new();
    hasher.update(key.as_bytes());
    output.copy_from_slice(&hasher.finalize())
}

fn get_stripped_md5_output(output: &str) -> String {
    let mut buf = [0u8; 16];
    let x: &[_] = &['\n', '\r', '\t', ' '];
    hash_token::<Md5>(output.trim_end_matches(x), &mut buf);
    (&buf[..]).to_vec().encode_hex::<String>()
}

#[derive(Debug, Clone, Serialize)]
struct NormalTestCaseInfo {
    input_name: String,
    input_size: i32,
    output_name: String,
    output_size: i32,
    stripped_output_md5: String,
}

#[derive(Debug, Clone, Serialize)]
struct SpjTestCaseInfo {
    input_name: String, 
    input_size: i32,
}

fn make_normal_info(id: i32, region: String, test_case_number: i32) {
    let mut test_cases: BTreeMap::<String, NormalTestCaseInfo> = BTreeMap::new();
    let path = "data/test_case/".to_owned() + &region + "_" + &id.to_string() + "/";
    
    for i in 0..test_case_number {
        let test_case_id = i+1;
        let input_name = test_case_id.to_string() + ".in";
        let mut input_file = File::open(&(path.clone() + &input_name)).expect("cannot open file");
        let mut input_content = String::new();
        input_file.read_to_string(&mut input_content).unwrap();
        let output_name = test_case_id.to_string() + ".in";
        let mut output_file = File::open(&(path.clone() + &output_name)).expect("cannot open file");
        let mut output_content = String::new();
        output_file.read_to_string(&mut output_content).unwrap();

        test_cases.insert(test_case_id.to_string(), NormalTestCaseInfo{
            input_name: input_name,
            input_size: input_content.len() as i32,
            output_name: output_name,
            output_size: output_content.len() as i32,
            stripped_output_md5: get_stripped_md5_output(&output_content)
        });
    }
    
    let info = json!({
        "test_case_number": test_case_number,
        "spj": false,
        "test_cases": test_cases,
    });

    let mut file = File::create(&(path.clone() + "info")).expect("Error creating file");
    file.write_all(info.to_string().as_bytes()).expect("Error writing file");
}

fn make_spj_info(id: i32, region: String, test_case_number: i32) {
    let mut test_cases: BTreeMap::<String, SpjTestCaseInfo> = BTreeMap::new();
    let path = "data/test_case/".to_owned() + &region + "_" + &id.to_string() + "/";
    
    for i in 0..test_case_number {
        let test_case_id = i+1;
        let name = test_case_id.to_string() + ".in";
        let mut file = File::open(&(path.clone() + &name)).expect("cannot open file");
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        test_cases.insert(test_case_id.to_string(), SpjTestCaseInfo{
            input_name: name,
            input_size: content.len() as i32,
        });
    }
    
    let info = json!({
        "test_case_number": test_case_number,
        "spj": true,
        "test_cases": test_cases,
    });
    
    let mut file = File::create(&(path.clone() + "info")).expect("Error creating file");
    file.write_all(info.to_string().as_bytes()).expect("Error writing file");
}


fn main() {
    make_normal_info(1000, "global".to_owned(), 2);
    make_spj_info(1001, "global".to_owned(), 1);
}