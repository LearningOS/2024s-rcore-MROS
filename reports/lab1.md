## 實現
在 trap 進系統呼叫的入口記錄了系統呼叫的次數。至於時間，在 `TaskControlBlockInner` 中新增一個任務開始時間 `start_time`，要計算任務已經運行了多久時，就取得當前時間再扣除 `start_time`。

## 簡答題

1. 
地址錯誤以及非法指令錯誤
```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003ac, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```

版本：
```
[rustsbi] RustSBI version 0.4.0-alpha.1, adapting to RISC-V SBI v2.0.0
[rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.3
```

2.1
a0 用於傳遞參數，但 __restore 並沒有用到參數，a0 對其無用，接下來就會被覆寫回 __trapall 紀錄在棧上的值。

__restore 有兩用途：
- 從系統調用返回用戶態，恢復存在棧上的寄存器狀態
- 切換任務，從 A 任務切換回 B 任務時，把存在 kernel stack 上的寄存器恢復

2.2
特別處理了 sstatus, sepc, sscratch。

sstatus 紀錄 trap 之前所處的特權級。

sepc 紀錄觸發 trap 的指令地址。

sscratch 紀錄 user stack 位置。

這幾項都決定跳到用戶態之後的行為。

2.3
x2 是 sp ，等等還要用，最後 `csrrw sp, sscratch, sp` 就會把 user stack 弄回來了。

x4 是 線程寄存器，現在還用不到。

2.4
sscratch->kernel stack, sp->user stack

2.5
`sret`

2.6
sp->kernel stack, sscratch->user stack

2.7
`ecall`


## 榮譽準則
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

《你交流的对象说明》

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

《你参考的资料说明》

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
