use sha1::Sha1;
use hmac::{Hmac, Mac, NewMac};
use base64::encode;
use chrono::prelude::*;
use std::collections::BTreeMap;
use uuid::Uuid;
use percent_encoding::{utf8_percent_encode, AsciiSet,CONTROLS};
use rand::{thread_rng, Rng};
use crate::statics::{ ACCESS_KEY_ID, ACCESS_SECRET };

const FRAGMENT: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'!')
        .add(b'"')
        .add(b'#')
        .add(b'$')
        .add(b'%')
        .add(b'&')
        .add(b'\'')
        .add(b'(')
        .add(b')')
        //.add(b'*')
        //.add(b'+')
        .add(b',')
        //.add(b'-')
        //.add(b'.')
        .add(b'/')
        .add(b':')
        .add(b';')
        .add(b'<')
        .add(b'=')
        .add(b'>')
        .add(b'?')
        .add(b'@')
        .add(b'[')
        .add(b'\\')
        .add(b']')
        .add(b'^')
        //.add(b'_')
        .add(b'`')
        .add(b'{')
        .add(b'|')
        .add(b'}')
        //.add(b'~')
    ;
    
fn special_url_encode(value: String) -> String {
    let encode_value = utf8_percent_encode(&value, &FRAGMENT).to_string();
    encode_value.replace("+", "%20").replace("*", "%2A").replace("%7E", "~")
}

fn sign(access_secret: String, string_to_sign: String) -> String {
    // Create alias for HMAC-Sha1
    type HmacSha1 = Hmac<Sha1>;
    // Create HMAC-Sha1 instance which implements `Mac` trait
    let mut mac = HmacSha1::new_varkey(access_secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(string_to_sign.as_bytes());
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    encode(code_bytes)
}

pub fn get_url(mobile: &str) -> (String, String) {
    let access_key_id = (*ACCESS_KEY_ID).clone();
    let mut access_secret = (*ACCESS_SECRET).clone();
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(100000, 999999);
    let utc: DateTime<Utc> = Utc::now();
    let timestamp = utc.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let signature_nonce = Uuid::new_v4().to_hyphenated().to_string();
    // system parameter
    let mut paras = BTreeMap::<&str, &str>::new();
    paras.insert("SignatureMethod", "HMAC-SHA1");
    paras.insert("SignatureNonce", &signature_nonce);
    paras.insert("AccessKeyId", &access_key_id);
    paras.insert("SignatureVersion", "1.0");
    paras.insert("Timestamp", &timestamp);
    paras.insert("Format", "json");
    // business parameter
    paras.insert("Action", "SendSms");
    paras.insert("Version", "2017-05-25");
    paras.insert("RegionId", "cn-hangzhou");
    paras.insert("PhoneNumbers", mobile);
    paras.insert("SignName", "SHUOJ");
    let template_param = format!("{{\"code\":\"{}\"}}", &n.to_string());
    paras.insert("TemplateParam", &template_param);
    paras.insert("TemplateCode", "SMS_205115097");
    //paras.insert("OutId", "123");
    // remove key which contains word "Signature"
    if !paras.get("Signature").is_none() {
        paras.remove("Signature");
    }

    let mut sorted_query_string = "".to_string();
    for (key, value) in paras.iter_mut() {
        sorted_query_string.push('&');
        sorted_query_string.push_str(&special_url_encode(key.to_string()));
        sorted_query_string.push('=');
        sorted_query_string.push_str(&special_url_encode(value.to_string()));
    }
    let mut tmp_string = sorted_query_string.clone();
    tmp_string.remove(0);
    let mut string_to_sign =  "".to_string();
    string_to_sign.push_str("GET");
    string_to_sign.push('&');
    string_to_sign.push_str(&special_url_encode("/".to_string()));
    string_to_sign.push('&');
    string_to_sign.push_str(&special_url_encode(tmp_string.clone()));

    access_secret.push('&');
    let sign = sign(access_secret, string_to_sign.clone());
    let signature = special_url_encode(sign);
    let final_url = format!("http://dysmsapi.aliyuncs.com/?Signature={}{}", signature, sorted_query_string);
    (final_url, n.to_string())
}