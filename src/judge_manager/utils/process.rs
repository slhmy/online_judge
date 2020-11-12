use std::process::{Command, Stdio};
use std::io::{BufRead, Write, BufReader};

pub fn run_judge_client(token: String, judge_setting: String) -> String {
    // 启动子进程
    let mut p = Command::new("./exe/send_judge_request")
        .stdin(Stdio::piped())  // 将子进程的标准输入重定向到管道
        .stdout(Stdio::piped()) // 将子进程的标准输出重定向到管道
        .spawn()
        .unwrap();
    
    let p_stdin = p.stdin.as_mut().unwrap();
    let mut p_stdout = BufReader::new(p.stdout.as_mut().unwrap());
    let mut line = String::new();
    
    p_stdin.write(token.as_bytes()).unwrap();
    p_stdin.write("\n".as_bytes()).unwrap();    // 发送\n，子进程的read_line才会响应

    p_stdin.write(judge_setting.as_bytes()).unwrap();
    p_stdin.write("\n".as_bytes()).unwrap();

    // 接收消息
    line.clear();   // 需要清空，否则会保留上次的结果
    p_stdout.read_line(&mut line).unwrap();
    // 等待子进程结束
    p.wait().unwrap();

    line.trim().to_owned()
}