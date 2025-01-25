use crate::{
    affinity_utils::bind_thread_to_cpu,
    cgroup::group_info::{get_background_group, get_middle_group, get_top_group},
};
use libc::pid_t;

const TOP: [&str; 1] = ["Thread-"];
const ONLY6: [&str; 0] = [];
const ONLY7: [&str; 1] = [" "];
const MIDDLE: [&str; 2] = ["RHIThread", "RenderThread"];
const BACKEND: [&str; 0] = [];

enum CmdType {
    Top,
    Middle,
    Background,
    Only6,
    Only7,
}

fn get_cmd_type(thread: &str) -> CmdType {
    // 使用 starts_with 方法匹配线程
    if TOP.iter().any(|&prefix| thread.starts_with(prefix)) {
        return CmdType::Top;
    }

    if ONLY6.iter().any(|&prefix| thread.starts_with(prefix)) {
        return CmdType::Only6;
    }

    if ONLY7.iter().any(|&prefix| thread.starts_with(prefix)) {
        return CmdType::Only7;
    }

    if MIDDLE.iter().any(|&prefix| thread.starts_with(prefix)) {
        return CmdType::Middle;
    }

    if BACKEND.iter().any(|&prefix| thread.starts_with(prefix)) {
        return CmdType::Background;
    }

    CmdType::Middle
}

fn execute_task(cmd_type: &CmdType, tid: pid_t) {
    match cmd_type {
        CmdType::Top => bind_thread_to_cpu(get_top_group(), tid),
        CmdType::Only6 => {
            let top_group = get_top_group();
            if top_group == [6, 7] {
                bind_thread_to_cpu(&[6], tid);
                return;
            }
            bind_thread_to_cpu(get_middle_group(), tid);
        }
        CmdType::Only7 => {
            let top_group = get_top_group();
            if top_group == [6, 7] {
                bind_thread_to_cpu(&[7], tid);
                return;
            }
            bind_thread_to_cpu(top_group, tid);
        }
        CmdType::Middle => bind_thread_to_cpu(get_middle_group(), tid),
        CmdType::Background => bind_thread_to_cpu(get_background_group(), tid),
    }
}

pub fn start_task(tid: pid_t, thread: &str) {
    let thread_type = get_cmd_type(thread);
    execute_task(&thread_type, tid);
}
