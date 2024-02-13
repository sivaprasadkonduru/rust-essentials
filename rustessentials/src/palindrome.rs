impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut rev = 0;
        let mut orig = x;
        while orig > 0 {
            rev = (rev * 10) + (orig % 10);
            orig /= 10;
        }
        if rev == x {
            return true
        }
        else {
            return false
        }
    }
}