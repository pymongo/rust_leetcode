/// https://leetcode.com/problems/reverse-bits/
fn reverse_bits(mut n: u32) -> u32 {
    let (mut ret, mut power) = (0u32, 0u32);
    while n != 0 {
        ret += (n & 1) << power;
        n >>= 1;
        power -= 1;
    }
    ret
    // n.reverse_bits()
}
