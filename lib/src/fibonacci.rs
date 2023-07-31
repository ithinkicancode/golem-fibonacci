pub struct FibState {
    previous: usize,
    current: usize,
}

pub const FIB_0: FibState = FibState {
    previous: 0,
    current: 0,
};

impl FibState {
    pub fn value(&self) -> usize {
        self.current
    }

    pub fn calc_next(
        &mut self,
    ) -> &Self {
        match self.current {
            0 => {
                self.current = 1;
            }
            _ => {
                self.current +=
                    self.previous;
                self.previous = self
                    .current
                    - self.previous;
            }
        }

        self
    }

    pub fn reset(&mut self) -> &Self {
        *self = FIB_0;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use proptest::prelude::proptest;

    fn expected_fib(n: usize) -> usize {
        if n < 2 {
            n
        } else {
            let (
                mut previous,
                mut current,
            ) = (0, 1);

            for _ in 2..=n {
                (previous, current) = (
                    current,
                    previous + current,
                );
            }
            current
        }
    }

    #[test]
    fn fib_state_works_from_zero_to_15()
    {
        let mut fib = FIB_0;

        let actual: Vec<_> = (0..=15)
            .map(|_| {
                let value = fib.value();
                fib.calc_next();
                value
            })
            .collect();

        assert_eq!(
            actual,
            vec![
                0, 1, 1, 2, 3, 5, 8,
                13, 21, 34, 55, 89,
                144, 233, 377, 610
            ]
        );
    }

    proptest! {
        #[test]
        fn fib_state_produces_the_correct_number(
            n in 0_usize..=93
        ) {
            let mut fib = FIB_0;

            for _ in 0..n {
                fib.calc_next();
            }

            assert_eq!(
                fib.value(),
                expected_fib(n)
            );
        }
    }
}
