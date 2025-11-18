impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut n = n; //making n mutable to right shift
        let mut res : i32 = 0;
        for i in 0..32{

            //res = res<<1;// left shift res to make up space for next bit
            //res = res | (n&1); // getting last bit of n and doing and with last bit of res
            res = res<<1 | (n&1); // one liner for above 2 lines
            
            n>>=1; // right shifting n so that new bit can come
        }
        res
    }
}
