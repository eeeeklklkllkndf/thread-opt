use crate::{
    cgroup::group_info::{get_background_group, get_middle_group, get_top_group},
    utils::affinity_setter::bind_thread_to_cpu,
};
use compact_str::CompactString;
use hashbrown::HashMap;
use libc::pid_t;
#[cfg(debug_assertions)]
use log::debug;
#[cfg(debug_assertions)]
use std::time::Instant;

// 定义线程类型
enum CmdType {
    Top,
    Middle,
    Background,
    Only6,
    Only7,
}

// 定义通用策略类
pub struct Policy {
    top: &'static [&'static str],
    only6: &'static [&'static str],
    only7: &'static [&'static str],
    middle: &'static [&'static str],
    backend: &'static [&'static str],
}

impl Policy {
    // 构造函数
    pub const fn new(
        top: &'static [&'static str],
        only6: &'static [&'static str],
        only7: &'static [&'static str],
        middle: &'static [&'static str],
        backend: &'static [&'static str],
    ) -> Self {
        Self {
            top,
            only6,
            only7,
            middle,
            backend,
        }
    }

    // 根据线程名称获取线程类型
    fn get_cmd_type(&self, comm: &str) -> CmdType {
        if self.top.iter().any(|&prefix| comm.starts_with(prefix)) {
            return CmdType::Top;
        }
        if self.only6.iter().any(|&prefix| comm.starts_with(prefix)) {
            return CmdType::Only6;
        }
        if self.only7.iter().any(|&prefix| comm.starts_with(prefix)) {
            return CmdType::Only7;
        }
        if self.middle.iter().any(|&prefix| comm.starts_with(prefix)) {
            return CmdType::Middle;
        }
        if self.backend.iter().any(|&prefix| comm.starts_with(prefix)) {
            return CmdType::Background;
        }
        CmdType::Middle
    }

    // 执行策略
    pub fn execute_policy(&self, task_map: &HashMap<pid_t, CompactString>) {
        // smol::block_on(async {
        // // let total_tasks = task_map.len();
        // // let mut task_count = 0;
        // #[cfg(debug_assertions)]
        // let start = Instant::now();

        // for (tid, comm) in task_map {
        // let tid = *tid;
        // let cmd_type = self.get_cmd_type(comm);
        // let handle = smol::spawn(async move {
        // execute_task(&cmd_type, tid);
        // });
        // // task_count += 1;
        // // if task_count == total_tasks {
        // handle.await;
        // // }
        // }
        // #[cfg(debug_assertions)]
        // {
        // let end = start.elapsed();
        // debug!(
        // "多线程:一轮绑定核心完成时间: {:?} 数组长度{}",
        // end,
        // task_map.len()
        // );
        // }
        // });

        #[cfg(debug_assertions)]
        let start = Instant::now();
        for (tid, comm) in task_map {
            let cmd_type = self.get_cmd_type(comm);
            execute_task(&cmd_type, *tid);
        }
        #[cfg(debug_assertions)]
        {
            let end = start.elapsed();

            debug!(
                "单线程:一轮绑定核心完成时间: {:?} 数组长度{}",
                end,
                task_map.len()
            );
        }
    }
}

// 执行线程绑定任务
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
        CmdType::Only7 => bind_thread_to_cpu(&[7], tid),
        CmdType::Middle => bind_thread_to_cpu(get_middle_group(), tid),
        CmdType::Background => bind_thread_to_cpu(get_background_group(), tid),
    }
}
