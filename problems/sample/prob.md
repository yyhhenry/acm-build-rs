# 示例题目-三个方阵

> 每年 9 月，都是一些人晒太阳的好时节。

小杰暂时代替了军训教官的职责，管理三个方阵。

他下令解散之后，发现人群变成了一簇一簇的样子，具体而言，有$n$堆人，第$i$堆有$a_i$人。

他怀疑有部分人离开了指定区域，想要清点人数。

他毕竟只是暂时代替，不清楚总人数，他唯一知道的是，这三个方阵的人数应当相同。

请统计这些人堆，推断最小可能的单个方阵总人数，以及三个方阵总共至少缺了几个人。

## 输入描述

一行一个整数$n$，表示人堆数，满足$1\leq n\leq 40$。

接下来一行$n$个整数$a_i$，表示每堆人数，满足$1\leq a_i\leq 20$。

## 输出描述

一行两个整数，分别表示最小可能的单个方阵总人数，以及三个方阵总共至少缺了几个人。

## 输入样例

```in
3
1 4 2
```

## 输出样例

```out
3 2
```

## 样例解释

目前的人堆共有 7 人，最小可能每个方阵各有 3 人，那么至少缺了 2 人。

## 提示

数据的$10\%$是样例。
