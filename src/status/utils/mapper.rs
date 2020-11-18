pub fn _result_mapper(value: i32) -> String {
    match value {
        -1 => "WRONG_ANSWER".to_owned(),
        0 => "SUCCESS".to_owned(),
        1 => "CPU_TIME_LIMIT_EXCEEDED".to_owned(),
        2 => "REAL_TIME_LIMIT_EXCEEDED".to_owned(),
        3 => "MEMORY_LIMIT_EXCEEDED".to_owned(),
        4 => "RUNTIME_ERROR".to_owned(),
        5 => "SYSTEM_ERROR".to_owned(),
        _ => "UNKNOWN_ERROR".to_owned(),
    }
}

pub fn _err_mapper(value: i32) -> String {
    match value {
        0 => "SUCCESS".to_owned(),
        -1 => "INVALID_CONFIG".to_owned(),
        -2 => "CLONE_FAILED".to_owned(),
        -3 => "PTHREAD_FAILED".to_owned(),
        -4 => "WAIT_FAILED".to_owned(),
        -5 => "ROOT_REQUIRED".to_owned(),
        -6 => "LOAD_SECCOMP_FAILED".to_owned(),
        -7 => "SETRLIMIT_FAILED".to_owned(),
        -8 => "DUP2_FAILED".to_owned(),
        -9 => "SETUID_FAILED".to_owned(),
        -10 => "EXECVE_FAILED".to_owned(),
        -11 => "SPJ_ERROR".to_owned(),
        _ => "UNKNOWN_ERROR".to_owned(),
    }
}