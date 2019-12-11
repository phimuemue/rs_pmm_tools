#[macro_export]
macro_rules! remember_cond{
    (if $($t:tt)*) => {
        remember_cond!(() $($t)*)
    };
    (($($cond:tt)*) $blk_if:block) => {
        remember_cond!(($($cond)*) $blk_if else {})
    };
    (($($cond:tt)*) $blk_if:block else $blk_else:block) => {
        if $($cond)* {
            let () = {$blk_if};
            true
        } else {
            let () = {$blk_else};
            false
        }
    };
    (($($cond_acc:tt)*) $t_cond:tt $($t_rest:tt)*) => {
        remember_cond!(($($cond_acc)* $t_cond) $($t_rest)*)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_remember_cond() {
        fn some_fn(_b: bool, _n: usize) -> bool {
            true
        }
        macro_rules! verify {
            ($e: expr) => {
                $e
            };
        }
        let mut n = 0;
        let b = remember_cond!(if verify!(some_fn(verify!(some_fn(true, 0)), 4)) {
            n+=1;
        } else {
            n-=1;
            1;
        });
        assert!(b);
        assert_eq!(n, 1);
        let b = remember_cond!(if false&&!verify!(some_fn(verify!(some_fn(true, 0)), 4)) {
            n+=1;
        } else {
            n-=1;
        });
        assert!(!b);
        assert_eq!(n, 0);
        let b = remember_cond!(if !!verify!(some_fn(verify!(some_fn(true, 0+if some_fn(false, 3){4} else {3})), 4)) {
            n+=1;
        });
        assert!(b);
        assert_eq!(n, 1);
    }
}
