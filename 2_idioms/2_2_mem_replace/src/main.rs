use std::mem;

fn main() {}

/// Структура, що представляє три значення одного типу.
#[derive(Clone, Debug, PartialEq)]
struct Trinity<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Trinity<T> {
    /// Повертає новий екземпляр структури, який містить елементи у зворотньому порядку.
    fn rotate(&mut self) {
        mem::swap(&mut self.a, &mut self.b);
        mem::swap(&mut self.b, &mut self.c);
    }
}

/// Структура Solver вирішує завдання знайдення конкретного значення серед набору Trinity.
#[derive(Debug)]
struct Solver<T> {
    expected: Trinity<T>,
    unsolved: Vec<Trinity<T>>,
}

impl<T: PartialEq> Solver<T> {
    /// Здійснює розв'язання задачі: видаляє з unsolved ті Trinity, які дорівнюють expected після обертання.
    fn solve(&mut self) {
        let mut unsolved = Vec::with_capacity(self.unsolved.len());
        'outer: for mut t in mem::take(&mut self.unsolved) {
            for _ in 0..3 {
                if t == self.expected {
                    continue 'outer;
                }
                t.rotate();
            }
            unsolved.push(t)
        }
        self.unsolved = unsolved;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Перевіряє, чи Solver видаляє вірні Trinity зі списку unsolved.
    fn should_solve_three_of_four_cases() {
        let mut solver = Solver {
            expected: Trinity { a: 1, b: 2, c: 3 },
            unsolved: vec![
                Trinity { a: 1, b: 2, c: 3 },
                Trinity { a: 2, b: 1, c: 3 },
                Trinity { a: 2, b: 3, c: 1 },
                Trinity { a: 3, b: 1, c: 2 },
            ],
        };

        solver.solve();

        assert_eq!(solver.unsolved, vec![Trinity { a: 2, b: 1, c: 3 }]);
    }
}
