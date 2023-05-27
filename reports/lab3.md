## 编程作业
首先确定要实现的有`spawn`和`set_priority`，在看完测例之后决定先实现`spawn`，根据标准`spawn`实现参考了`fork`和`exec`，结合了`fork`中关联父子进程的内容与`exec`中初始化进程的内容。然后实现了前面2章的内容。
在实现`set_priority`时，在`TaskControlBlock` 中加入了`stride`和`priority`，在进程调度中，使用了文档中注意里面提到过堆结构，在进程组成的堆中找到`stride`的进程，然后更新该值，再fetch出。注意这里要为`TaskControlBlock`实现`PartialEq`、`PartialOrd`、`Eq`、`Ord`等`trait`。



## 问答作业
#### 1.
不是，因为使用的是`u8`储存`stride`，250 + 10发生了溢出，p2的stride小于p1，所以还是p2执行

#### 2.
当`priority`大于2时，`pass`一定<=`BigStride/2`，所以就会有`PASS_MAX – PASS_MIN <= BigStride / 2`

#### 3.
~~~rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        const BIG_STRIDE:u64 = u64::MAX;
	    (self.0 - other.0).cmp(BIG_STRIDE / 2)
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

~~~


## 荣誉准则

1.  在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
    
	无
    
2.  此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
	[rust中的动态分配](http://rcore-os.cn/rCore-Tutorial-Book-v3/chapter4/1rust-dynamic-allocation.html)
	[标准spawn](https://man7.org/linux/man-pages/man3/posix_spawn.3.html)

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。