use alloc::vec;
use alloc::vec::Vec;

/// 使用安全性算法檢測是否將構成死鎖
pub struct DeadlockCheck {
    is_enabled: bool,
    // 空閑資源數量，available[j] == x ，代表第 i 種資源 共有 x 個
    available: Vec<u32>,
    // allocation[i][j] == x ，表示線程 i 已取得 x 個第 j 種資源
    allocation: Vec<Vec<u32>>,
    // need[i] == Some(j) ，表示線程 i 已請求但尚未取得第 j 種資源
    // 也就是已經呼叫 lock ，但尚在等待其他線程 unlock
    need: Vec<Option<usize>>,
}

impl DeadlockCheck {
    /// 在僅有一條主線程的狀況下初始成員變數
    pub fn new() -> Self {
        DeadlockCheck {
            is_enabled: false,
            available: Vec::new(),
            allocation: vec![vec![]], // 進程初始就有一條主線程
            need: vec![None],
        }
    }
    fn thread_count(&self) -> usize {
        self.allocation.len()
    }
    /// 啟用死所檢測算法
    pub fn enable(&mut self) {
        self.is_enabled = true;
    }
    /// 關閉死所檢測算法
    pub fn disable(&mut self) {
        self.is_enabled = false;
    }
    /// 有新線程加入，維護成員變數
    pub fn add_thread(&mut self, tid: usize) {
        while self.allocation.len() < tid + 1 {
            self.allocation.push(vec![0; self.available.len()]);
        }
        while self.need.len() < tid + 1 {
            self.need.push(None);
        }
    }
    /// 有新資源加入，維護成員變數
    /// mutex, semaphore 創建時，添加資源種類，mutex 的資源數量恆為 1
    pub fn add_resource(&mut self, resource_id: usize, resource_number: u32) {
        while self.available.len() < resource_id + 1 {
            self.available.push(0);
        }
        for allocation_i in &mut self.allocation {
            while allocation_i.len() < resource_id + 1 {
                allocation_i.push(0);
            }
        }
        self.available[resource_id] = resource_number;
    }
    /// 線程請求資源(lock, semaphore_down)時調用，檢查是否可能造成死鎖
    pub fn request_resource(&mut self, tid: usize, resource_id: usize) -> bool {
        let mut finish = vec![false; self.thread_count()];
        let mut work = self.available.clone();
        assert_eq!(self.need[tid], None);
        self.need[tid] = Some(resource_id);

        let mut get_exitable_thread = || -> Option<usize> {
            for tid in 0..self.thread_count() {
                if finish[tid] != true {
                    match self.need[tid] {
                        None => {
                            finish[tid] = true;
                            return Some(tid);
                        }
                        Some(resource_id) => {
                            // 目前的 mutex/semaphore 一次只允許要求一個資源
                            if work[resource_id] >= 1 {
                                for release_resource_id in 0..self.allocation[tid].len() {
                                    work[release_resource_id] +=
                                        self.allocation[tid][release_resource_id];
                                }
                                finish[tid] = true;
                                return Some(tid);
                            }
                        }
                    }
                }
            }
            None
        };

        while let Some(_tid) = get_exitable_thread() {}

        // 若所有線程都能結束，則此次資源獲取是安全的
        let is_safe = !finish.contains(&false);

        if !self.is_enabled {
            return true;
        }
        is_safe
    }
    /// 線程獲取資源(lock, semaphore_down)
    pub fn acquire_resource(&mut self, tid: usize, resource_id: usize) {
        assert_eq!(Some(resource_id), self.need[tid]);
        self.need[tid] = None;
        self.available[resource_id] -= 1;
        self.allocation[tid][resource_id] += 1;
    }
    /// 線程釋放資源(unlock, semaphore_down)
    pub fn release_resource(&mut self, tid: usize, resource_id: usize) {
        self.available[resource_id] += 1;
        self.allocation[tid][resource_id] -= 1;
    }
}
