fn main() {
    println!("Hello, world!");
}

fn fib(n: u128) -> u128 {
    if n == 0 {
        return n
    } else if n <= 2 {
        return 1
    } else {
        let mut sum: u128 = 0;
        let mut last: u128 = 0;
        let mut curr: u128 = 1;

        for _i in 1..n {
            sum = last + curr;
            last = curr;
            curr = sum;
        }

        sum
    }
    // recursive, slow
    //if n <= 1 {
    //    n * 1
    //} else {
    //    fib(n - 1) + fib(n - 2)
    //}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_eq_0() {
        assert_eq!(fib(0), 0);
    }

    #[test]
    fn test_n_eq_1() {
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn test_n_other_integer() {
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(14), 377);
        assert_eq!(fib(100), 354224848179261915075);
    }
}

