pub fn language_filter(language: &str) -> bool {
    match language {
        "c" => true,
        "cpp" => true,
        "java" => true,
        "py2" => true,
        "py3" => true,
        _ => false,
    }
}

pub fn judge_type_filter(judge_type: &str) -> bool {
    match judge_type {
        "OI" => true,
        "ACM" => true,
        _ => false,
    }
}

pub fn setting_filter(language: &str, max_cpu_time:i32, max_memory: i32) -> (i32, i32) {
    match language {
        "c" => (max_cpu_time, max_memory),
        "cpp" => (max_cpu_time, max_memory),
        "java" => (max_cpu_time * 2, max_memory * 2),
        "py2" => (max_cpu_time * 2, max_memory * 2),
        "py3" => (max_cpu_time * 2, max_memory * 2),
        _ => (max_cpu_time * 2, max_memory * 2),
    }
}