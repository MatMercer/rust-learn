fn main() {
    println!("Hello, world!");
}

const T_MAGIC: f64 = 5.0 / 9.0;
const DIFFERENCE: f64 = 32.0;

fn f_to_c(f: f64) -> f64 {
    (f - DIFFERENCE) * T_MAGIC
}

fn c_to_f(c: f64) -> f64 {
    (c / T_MAGIC) + DIFFERENCE
}

mod tests {
    use super::*;

    #[test]
    fn test_f_to_c() {
        assert_eq!(f_to_c(0.0), -17.77777777777778);
        assert_eq!(f_to_c(32.0), 0.0);
        assert_eq!(f_to_c(64.0), 17.77777777777778);
        assert_eq!(f_to_c(100.0), 37.77777777777778);
        assert_eq!(f_to_c(-40.0), -40.0);
        assert_eq!(f_to_c(-1000.0), -573.33333333333333);
    }

    #[test]
    fn test_c_to_f() {
        assert_eq!(c_to_f(-17.77777777777778), 0.0);
        assert_eq!(c_to_f(0.0), 32.0);
        assert_eq!(c_to_f(17.77777777777778), 64.0);
        assert_eq!(c_to_f(37.77777777777778), 100.0);
        assert_eq!(c_to_f(-40.0), -40.0);
        assert_eq!(c_to_f(-573.33333333333333), -1000.0);
    }
}


