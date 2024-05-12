//! Process management syscalls
use core::mem::size_of;

use crate::mm::{translated_byte_buffer, MapPermission};
use crate::task::{current_user_token, mmap_current, munmap_current};
use crate::timer::get_time_us;
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, get_current_syscall_times,
        suspend_current_and_run_next, TaskStatus,
    },
    timer::get_time_ms,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

// 在應用程式給定的虛址上寫入數據
unsafe fn write_structure<T: Sized>(ptr: *const u8, data: T) {
    let len = size_of::<T>();
    let pages = translated_byte_buffer(current_user_token(), ptr, len);
    let u8_data = core::slice::from_raw_parts((&data as *const T) as *const u8, len);
    let mut i: usize = 0;
    for page in pages {
        page.copy_from_slice(&u8_data[i..(i + page.len())]);
        i += page.len();
    }
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let time = get_time_us();
    unsafe {
        write_structure(
            ts as *const u8,
            TimeVal {
                sec: time / 1000000,
                usec: time % 100000,
            },
        );
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    unsafe {
        write_structure(
            ti as *const u8,
            TaskInfo {
                status: TaskStatus::Running,
                syscall_times: get_current_syscall_times(),
                time: get_time_ms(),
            },
        )
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if start % 4096 != 0 {
        return -1;
    }
    // 末三位都是 0 ，不可讀、寫、執行，記憶體分配了也沒有意義
    if port & 7 == 0 {
        return -1;
    }
    // 僅定義讀、寫、執行，四位以上無定義
    if port > 7 {
        return -1;
    }

    // let permission = MapPermission::U | MapPermission::R | MapPermission::W | MapPermission::X;
    let mut permission = MapPermission::U;
    if (port & 1) != 0 {
        permission |= MapPermission::R;
    }
    if (port & 2) != 0 {
        permission |= MapPermission::W;
    }
    if (port & 4) != 0 {
        permission |= MapPermission::X;
    }
    if mmap_current(start.into(), (start + len).into(), permission) {
        0
    } else {
        -1
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if start % 4096 != 0 {
        return -1;
    }
    if munmap_current(start.into(), (start + len).into()) {
        0
    } else {
        -1
    }
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
