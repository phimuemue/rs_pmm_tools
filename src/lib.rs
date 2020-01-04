pub trait TMutateReturn<I, T> : Sized {
    type T;
    fn mutate_return(self) -> Self::T;
}

macro_rules! impl_mutate_return{($s_mutate_return:ident, $($t:ident,)*) => {
    // TODORUST: have a single SMutateReturn that implements call operator variadically?
    // TODO? Do we need Fn/FnOnce/FnMut
    pub struct $s_mutate_return<F>{f: F}
    impl<I, $($t,)* F: FnMut(&mut I, $($t,)*)> TMutateReturn<I, ($($t,)*)> for F {
        type T = $s_mutate_return<Self>;
        fn mutate_return(self) -> Self::T {
            Self::T{f: self}
        }
    }
    impl<F> $s_mutate_return<F> {
        #[allow(non_snake_case)]
        pub fn into_fn<I, $($t,)*>(mut self) -> impl FnMut(I, $($t,)*)->I
            where
                F: FnMut(&mut I, $($t,)*)
        {
            move |mut i, $($t,)*| {
                (self.f)(&mut i, $($t,)*);
                i
            }
        }
    }
}}
impl_mutate_return!(SMutateReturn0,);
impl_mutate_return!(SMutateReturn1, T0,);
impl_mutate_return!(SMutateReturn2, T0, T1,);

#[macro_export]
macro_rules! mutate_return{($f: expr) => {
    TMutateReturn::mutate_return($f).into_fn()
}}

#[test]
fn test_mutate_return() {
    assert_eq!([1usize,2,3].iter().copied().fold(vec![], mutate_return!(Vec::push)), vec![1, 2, 3]);
    assert_eq!(mutate_return!(Vec::push)(vec![], 6), vec![6]);
    assert_eq!(mutate_return!(|x: &mut usize| {*x = 5;})(7usize), 5);
    assert_eq!(mutate_return!(|x: &mut usize, a, b| {*x = 5 + if a {2} else {b};})(7usize, true, 3), 7);
    let mut vecn = Vec::<usize>::new();
    {
        let mut fn_twice = mutate_return!(|n_mut: &mut usize, n_in| {
            vecn.push(n_in);
            *n_mut = *n_mut + n_in;
        });
        dbg!(fn_twice(5, 3));
        dbg!(fn_twice(19, 9));
    }
    assert_eq!(vecn, vec![3, 9]);
    fn app1(f: impl FnOnce(usize, usize)->usize) -> usize {
        dbg!(f(4,3))
    }
    assert_eq!(app1(mutate_return!(|x: &mut usize, y| {*x=y;})), 3);
}

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
            n += 1;
        } else {
            n -= 1;
            1;
        });
        assert!(b);
        assert_eq!(n, 1);
        let b = remember_cond!(
            if false && !verify!(some_fn(verify!(some_fn(true, 0)), 4)) {
                n += 1;
            } else {
                n -= 1;
            }
        );
        assert!(!b);
        assert_eq!(n, 0);
        let b = remember_cond!(if !!verify!(some_fn(
            verify!(some_fn(true, 0 + if some_fn(false, 3) { 4 } else { 3 })),
            4
        )) {
            n += 1;
        });
        assert!(b);
        assert_eq!(n, 1);
    }
}
