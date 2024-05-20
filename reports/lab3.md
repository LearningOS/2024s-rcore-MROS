## 實現
spawn 基本上是 fork 跟 exec 的代碼黏貼各半。

stride 用了 BinaryHeap 來維護優先級。

## 簡答題

- 实际情况是轮到 p1 执行吗？为什么？
    - 不是 p2.pass + 10 = 250 + 10，會溢位 = 5，下一次仍然是 p2 執行。


我们之前要求进程优先级 >= 2 其实就是为了解决这个问题。可以证明， 在不考虑溢出的情况下 , 在进程优先级全部 >= 2 的情况下，如果严格按照算法执行，那么 STRIDE_MAX – STRIDE_MIN <= BigStride / 2。

- 为什么？尝试简单说明（不要求严格证明）。
    - 初始 stride 皆為零，滿足 STRIDE_MAX – STRIDE_MIN <= BigStride / 2 ，經過一次調度後， STRIDE_MIN 頂多增加 BigStride / 2 ，並不可能使得任何一對 stride 的差大於 BigStride /2 ，故 STRIDE_MAX – STRIDE_MIN <= BigStride / 2 仍然成立。

- 已知以上结论，考虑溢出的情况下，可以为 Stride 设计特别的比较器，让 BinaryHeap<Stride> 的 pop 方法能返回真正最小的 Stride。补全下列代码中的 partial_cmp 函数，假设两个 Stride 永远不会相等。
```
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else if self.0 > other.0 {
            if (self.0 - other.0 > BigStride / 2) {
                other.0
            } else {
                self.o
            }
        } else {
            if (other.0 - self.0 > BigStride / 2) {
                self.o
            } else {
                other.0
            }

        }
    }
}
```

2. 

## 榮譽準則
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

《你交流的对象说明》

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

《你参考的资料说明》

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

