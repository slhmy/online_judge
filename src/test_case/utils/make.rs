use md5::Md5;
use digest::Digest;
use hex::ToHex;
use serde_json::json;
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;
use std::process::Command;

fn hash_token<D: Digest>(key: &str, output: &mut [u8]) {
    let mut hasher = D::new();
    hasher.update(key.as_bytes());
    output.copy_from_slice(&hasher.finalize())
}

fn get_stripped_md5_output(output: &str) -> String {
    let mut buf = [0u8; 16];
    info!("{}", output.trim_end());
    hash_token::<Md5>(output.trim_end(), &mut buf);
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

pub fn make_normal_info(name: String, zip_buf: &[u8]) -> i32 {
    let mut test_case_number = 0;
    let mut test_cases: BTreeMap::<String, NormalTestCaseInfo> = BTreeMap::new();
    let path = "data/test_case/".to_owned() + &name;

    // backup old directory
    let backup_uuid = Uuid::new_v4().to_hyphenated().to_string();
    let backup_path = "data/test_case/".to_owned() + &backup_uuid;
    let is_backuped = match fs::rename(&path, &backup_path) {
        Ok(_) => true,
        Err(_) => false,
    };
    // create new directory
    match fs::create_dir_all(&path) {
        Ok(_) => info!("Created {}", path),
        Err(e) => info!("{}", e),
    }
    let zip_path = path.clone() + "/" + "raw.zip";
    let mut file = File::create(zip_path.clone()).expect("Error making zip.");
    file.write_all(zip_buf).expect("Error writing zip.");
    let mut p = Command::new("unzip")
        .args(&["-o", &zip_path, "-d", &path])
        .spawn()
        .expect("Failed to unzip.");
    p.wait().unwrap();
    
    loop {
        let test_case_id = test_case_number + 1;
        let input_name = test_case_id.to_string() + ".in";
        let mut input_file = match File::open(path.clone() + "/" + &input_name) {
            Ok(file) => file,
            Err(_) => {
                info!("Can't find file {}", path.clone() + "/" + &input_name);
                break;
            }
        };
        let mut input_content = String::new();
        input_file.read_to_string(&mut input_content).unwrap();
        
        let output_name = test_case_id.to_string() + ".out";
        let mut output_file = match File::open(path.clone() + "/" + &output_name) {
            Ok(file) => file,
            Err(_) => {
                info!("Can't find file {}", path.clone() + "/" + &input_name);
                break;
            }
        };
        let mut output_content = String::new();
        output_file.read_to_string(&mut output_content).unwrap();

        test_cases.insert(test_case_id.to_string(), NormalTestCaseInfo{
            input_name: input_name,
            input_size: input_content.len() as i32,
            output_name: output_name,
            output_size: output_content.len() as i32,
            stripped_output_md5: get_stripped_md5_output(&output_content)
        });

        test_case_number += 1;
    }
    
    let info = json!({
        "test_case_number": test_case_number,
        "spj": false,
        "test_cases": test_cases,
    });

    let mut file = File::create(&(path.clone() + "/" + "info")).expect("Error creating info");
    file.write_all(info.to_string().as_bytes()).expect("Error writing info");

    if test_case_number == 0 {
        // if is_backuped recover from backup
        if is_backuped {
            match fs::remove_dir_all(&path) {
                Ok(_) => info!("Removed {}", path),
                Err(_) => info!("Unknow error while removing directory"),
            }
            fs::rename(&backup_path, &path).unwrap();
        }
    } else {
        if is_backuped { // remove backup
            fs::remove_dir_all(&backup_path).unwrap();
        } else { // remove failed directory
            fs::remove_dir_all(&path).unwrap();
        }
    }

    test_case_number
}

pub fn make_spj_info(name: String, zip_buf: &[u8]) -> i32 {
    let mut test_case_number = 0;
    let mut test_cases: BTreeMap::<String, SpjTestCaseInfo> = BTreeMap::new();
    let path = "data/test_case/".to_owned() + &name;

    // backup old directory
    let backup_uuid = Uuid::new_v4().to_hyphenated().to_string();
    let backup_path = "data/test_case/".to_owned() + &backup_uuid;
    let is_backuped = match fs::rename(&path, &backup_path) {
        Ok(_) => true,
        Err(_) => false,
    };
    // create new directory
    match fs::create_dir_all(&path) {
        Ok(_) => info!("Created {}", path),
        Err(e) => info!("{}", e),
    }
    let zip_path = path.clone() + "/" + "raw.zip";
    let mut file = File::create(zip_path.clone()).expect("Error making zip.");
    file.write_all(zip_buf).expect("Error writing zip.");
    let mut p = Command::new("unzip")
        .args(&["-o", &zip_path, "-d", &path])
        .spawn()
        .expect("Failed to unzip.");
    p.wait().unwrap();
    
    loop {
        // check if spj_src.c exists
        if test_case_number == 0 {
            match File::open(path.clone() + "/spj_src.c") {
                Ok(file) => file,
                Err(_) => {
                    info!("Can't find file {}", path.clone() + "/spj_src.c");
                    break;
                }
            };
        }
        let name = (test_case_number + 1).to_string() + ".in";
        let mut file = match File::open(path.clone() + "/" + &name) {
            Ok(file) => file,
            Err(_) => {
                info!("Can't find file {}", path.clone() + "/" + &name);
                break;
            }
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        test_cases.insert(test_case_number.to_string(), SpjTestCaseInfo{
            input_name: name,
            input_size: content.len() as i32,
        });

        test_case_number += 1;
    }
    
    let info = json!({
        "test_case_number": test_case_number,
        "spj": true,
        "test_cases": test_cases,
    });
    
    let mut file = File::create(path.clone() + "/" + "info").expect("Error creating info");
    file.write_all(info.to_string().as_bytes()).expect("Error writing info");

    if test_case_number == 0 {
        // if is_backuped recover from backup
        if is_backuped {
            match fs::remove_dir_all(&path) {
                Ok(_) => info!("Removed {}", path),
                Err(_) => info!("Unknow error while removing directory"),
            }
            fs::rename(&backup_path, &path).unwrap();
        }
    } else {
        if is_backuped { // remove backup
            fs::remove_dir_all(&backup_path).unwrap();
        } else { // remove failed directory
            fs::remove_dir_all(&path).unwrap();
        }
    }

    test_case_number
}