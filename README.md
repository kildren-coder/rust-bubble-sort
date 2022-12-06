# rust-bubble-sort

Substrate入门课第3节的作业:冒泡排序的Rust实现

本实现能对所有含 `PartialOrd` trait 的类型冒泡排序。

由于泛型约束仅为 `PartialOrd`，所以函数可能接收含有 `NAN` 这类未实现 `Ord` trait的输入。

`NAN` 相关元素的顺序无定义,该实现会将这类元素统一移置到输入切片的末尾,并将可排序元素按从小到大的顺序排序,返回可排序元素的个数.

### 测试样例

实现中包含三组测试,分别针对
- i32
- f32
- struct

共三种类型.

由于 Rust 中 `NAN != NAN`,所以f32类型的测试必定报错,但可通过测试结果知道函数正确返回了可排序元素个数,并将相关元素正确排序,`NAN`亦放置到了最后面.

#### i32 测试
![image](https://github.com/kildren-coder/rust-bubble-sort/blob/main/img/2022-12-06_180059.png)

#### struct 测试
测试的数据结构为扑克牌,扑克牌首先会根据数字大小进行排序,若颜色相同,再以花色排序:
![image](https://github.com/kildren-coder/rust-bubble-sort/blob/main/img/2022-12-06_181116.png)


测试数据:

![image](https://github.com/kildren-coder/rust-bubble-sort/blob/main/img/2022-12-06_180748.png)

#### f32 测试
在这类测试中,需要先检查函数是否正确返回了可排序元素个数,再比较数据是否正确排序.
![image](https://github.com/kildren-coder/rust-bubble-sort/blob/main/img/2022-12-06_181238.png)

#### 测试结果
`i32`及`struct`测试均可通过,失败测试均为`f32`测试,从报错中可知数量检查是没问题的,排序顺序也是正确的.

![image](https://github.com/kildren-coder/rust-bubble-sort/blob/main/img/2022-12-06_181404.png)