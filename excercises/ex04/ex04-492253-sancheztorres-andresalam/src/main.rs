// Andres Alam Sanchez Torres
use std::collections::HashSet;
use std::time::Instant;
use std::{self, cmp, vec};

struct Item {
    id: usize,
    value: u32,
    weight: usize,
}

fn solve_knapsack(solutions: &mut [Vec<u32>], items: &[Item], item_subset: usize, capacity: usize) {
    let n = item_subset;

    for i in 0..=n {
        for w in 0..=capacity {
            if i == 0 || w == 0 {
                solutions[i][w] = 0;
            } else if items[i - 1].weight <= w {
                let item = &items[i - 1];
                let value_w_next_item = solutions[i - 1][w - item.weight] + item.value;

                solutions[i][w] = cmp::max(solutions[i - 1][w], value_w_next_item);
            } else {
                solutions[i][w] = solutions[i - 1][w];
            }
        }
    }
}

fn get_solution(solutions: &mut [Vec<u32>], items: &[Item], i: usize, w: usize) -> HashSet<usize> {
    if i == 0 {
        return HashSet::new();
    }

    if solutions[i][w] > solutions[i - 1][w] {
        let mut solution_set = HashSet::from([i - 1]);

        solution_set.extend(&get_solution(
            solutions,
            items,
            i - 1,
            w - items[i - 1].weight,
        ));

        solution_set
    } else {
        get_solution(solutions, items, i - 1, w)
    }
}

fn read_next_line() -> String {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Can't read next line");

    line.trim().to_string()
}

fn main() {
    let start_time = Instant::now();

    let n = read_next_line()
        .trim()
        .parse::<usize>()
        .expect("Cannot parse N.");

    // parse next n item data lines into Item instances
    let items = (0..n)
        .map(|_| {
            let curr_item_line = read_next_line();
            let raw_item_meta = curr_item_line.split(' ').map(ToString::to_string);

            let item_meta = raw_item_meta
                .map(|elem| elem.parse::<usize>().expect("Can't parse number"))
                .collect::<Vec<_>>();

            assert!(item_meta.len() == 3, "Can't parse item data");

            Item {
                id: item_meta[0],
                value: item_meta[1] as u32,
                weight: item_meta[2],
            }
        })
        .collect::<Vec<_>>();

    let capacity = read_next_line()
        .parse::<usize>()
        .expect("Last line is not a number");

    let mut solutions: Vec<Vec<u32>> = vec![vec![0; capacity + 1]; n + 1];

    solve_knapsack(&mut solutions, &items, n, capacity);

    let solution = get_solution(&mut solutions, &items, n, capacity);

    let ids: Vec<String> = solution
        .iter()
        .map(|key| items[*key].id.to_string())
        .collect();

    let value = solutions[n][capacity];

    println!("Items: {}", ids.join(" "));
    println!("Total: {}", value);

    let duration = start_time.elapsed();
    println!("Time : {:5} s", duration.as_secs_f32());
}
