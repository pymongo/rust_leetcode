/*! https://leetcode.com/problems/nim-game/
nim游戏规则：有若干个石头，两人每回合轮流拿走一些石头，每个人可以拿走1-3块石头
如果轮到A的回合时，石头数量是4的倍数，那么A必败(博弈问题的必败态)
或者利用二进制判断是不是 4 的倍数，
只需要通过和 3 （二进制 11）进行相与，
如果是 4 的倍数，那么结果一定是 0。

算法如下：
    x&3==0，则是4的倍数。
原理：
先来看一组数字的二进制表示
    4　　　　0100
    8　　　　1000
    12      1100
    16     10000
    20     10100
由此可见 4 的倍数的二进制表示的后 2 为一定为 0。

从另外一个角度来看，4 的二进制表示是 0100，任何 4 的倍数一定是在此基础上增加 n 个 0100
由此也可得 4 的倍数的二进制表示的后 2 为一定为 0。
*/
const fn nim_game_bitwise(n: i32) -> bool {
    // (n % 4) != 0
    (n & 3) != 0
}
