use std::process::{Command, Stdio};
use std::io::{BufRead, Write, BufReader};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // 启动子进程
    let mut p = Command::new("./exe/send_judge_request")
        .stdin(Stdio::piped())  // 将子进程的标准输入重定向到管道
        .stdout(Stdio::piped()) // 将子进程的标准输出重定向到管道
        .spawn()
        .unwrap();
    
    let p_stdin = p.stdin.as_mut().unwrap();
    let mut p_stdout = BufReader::new(p.stdout.as_mut().unwrap());
    let mut line = String::new();
    
    let msg = "b82fd881d1303ba9794e19b7f4a5e2b79231d065f744e72172ad9ee792909126";
    println!("write to stdid:{}", msg);
    p_stdin.write(msg.as_bytes()).unwrap();
    p_stdin.write("\n".as_bytes()).unwrap();    // 发送\n，子进程的read_line才会响应

    let mut file = File::open("judge_setting_example").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("write to stdid:{}", contents);
    p_stdin.write(contents.as_bytes()).unwrap();
    p_stdin.write("\n".as_bytes()).unwrap();    // 发送\n，子进程的read_line才会响应

    // 接收消息
    line.clear();   // 需要清空，否则会保留上次的结果
    p_stdout.read_line(&mut line).unwrap();
    println!("{}", line.trim());

    // 等待子进程结束
    p.wait().unwrap();
}