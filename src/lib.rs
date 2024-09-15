use std::cmp::max;
use std::fmt::{Display, Formatter};
use nalgebra::DMatrix;
use bon::builder;

#[builder]
pub fn knapsack(
    capacity: usize,
    items: Vec<KnapsackItem>,
    print_subproblems: Option<bool>,
    print_table: Option<bool>
) -> KnapsackSolution {
    let n = items.len();
    let mut dp = DMatrix::repeat(n + 1, capacity + 1, 0);

    for i in 1..=n {
        for w in 0..=capacity {
            if i == 0 || w == 0 {
                dp[(i, w)] = 0;
            } else if items.get(i - 1).unwrap().weight <= w {
                dp[(i, w)] = max(items.get(i - 1).unwrap().value +
                                     dp[(i - 1, w - items.get(i - 1).unwrap().weight)],
                                 dp[(i - 1, w)]);
            } else {
                dp[(i, w)] = dp[(i - 1, w)];
            }

            if let Some(p) = print_subproblems {
                if p {
                    println!(
                        "Subproblem: First {} items, Capacity {} - Optimal value: {}",
                        i, w, dp[(i, w)]
                    );
                }
            }

            if let Some(p) = print_table {
                if p {
                    print!("{}", dp);
                }
            }
        }
    }

    let mut item_indices = Vec::new();
    let mut total_weight = 0;
    let mut remaining_capacity = capacity;
    for i in (1..=n).rev() {
        if dp[(i, remaining_capacity)] != dp[(i - 1, remaining_capacity)] {
            item_indices.push(i - 1);
            total_weight += items[i - 1].weight;
            remaining_capacity -= items[i - 1].weight;
        }
    }

    KnapsackSolution {
        optimal_profit: dp[(n, capacity)],
        total_weight,
        item_indices,
    }
}

pub struct KnapsackItem {
    pub value: usize,
    pub weight: usize,
}

pub struct KnapsackSolution {
    optimal_profit: usize,
    total_weight: usize,
    item_indices: Vec<usize>,
}

impl Display for KnapsackSolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "Optimal profit: {}\nTotal weight: {}\nItems indexes: {:?}",
               self.optimal_profit,
               self.total_weight,
               self.item_indices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let items = vec![
            KnapsackItem { value: 60, weight: 10 },
            KnapsackItem { value: 100, weight: 20 },
            KnapsackItem { value: 120, weight: 30 },
        ];

        let solution = knapsack().capacity(50).items(items).call();
        assert_eq!(solution.optimal_profit, 220);
        assert_eq!(solution.total_weight, 50);
        assert_eq!(solution.item_indices, vec![2, 1]);
    }
}
