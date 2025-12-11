use std::borrow::Cow;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use aoc::parse_lines;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        parse_lines(&input)
            .map(|machine: Machine| machine.solve_part_1())
            .sum::<i32>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        parse_lines(&input)
            .map(|machine: Machine| machine.solve_part_2())
            .sum::<i64>()
            .to_string()
    }
}

struct Machine {
    lights_target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_target: Vec<i32>,
}

impl Machine {
    fn press_button_lights(&self, button_idx: usize, lights: Vec<bool>) -> Vec<bool> {
        let mut new_lights = lights.clone();
        for &light_idx in &self.buttons[button_idx] {
            new_lights[light_idx] = !new_lights[light_idx];
        }
        new_lights
    }

    fn press_button_joltage(&self, button_idx: usize, times: i32, joltage: Vec<i32>) -> Vec<i32> {
        let mut new_joltage = joltage.clone();
        for &joltage_idx in &self.buttons[button_idx] {
            new_joltage[joltage_idx] += times;
        }
        new_joltage
    }

    fn solve_part_1(&self) -> i32 {
        let mut q = VecDeque::new();
        q.push_back((vec![false; self.lights_target.len()], 0));
        let mut seen = HashSet::new();
        loop {
            let (lights, presses) = q.pop_front().unwrap();
            if !seen.insert(lights.clone()) {
                continue;
            }
            if lights == *self.lights_target {
                return presses;
            }
            for button_idx in 0..self.buttons.len() {
                let new_lights = self.press_button_lights(button_idx, lights.clone());
                q.push_back((new_lights, presses + 1));
            }
        }
    }

    fn solve_part_2(&self) -> i64 {
        log::info!("starting solve_part_2 for joltage target {:?}", self.joltage_target);
        let mut m = vec![vec![0i64; self.buttons.len() + 1]; self.joltage_target.len()];
        for y in 0..self.joltage_target.len() {
            for (x, button) in self.buttons.iter().enumerate() {
                if button.contains(&y) {
                    m[y][x] = 1;
                }
            }
        }
        for (y, &target) in self.joltage_target.iter().enumerate() {
            m[y][self.buttons.len()] = target as i64;
        }

        log::info!("matrix before elimination: {:?}", m);

        gaussian_elimination(&mut m);

        log::info!("matrix after elimination: {:?}", m);

        // remove empty rows if present
        m.retain(|row| row[..self.buttons.len()].iter().any(|&val| val != 0));

        let mut pivot_cols: Vec<usize> = vec![];
        let mut free_vars: Vec<usize> = vec![];
        for c in 0..self.buttons.len() {
            if m.iter().filter(|row| row[c] == 1).count() == 1 {
                pivot_cols.push(c);
            } else {
                free_vars.push(c);
            }
        }

        let presses: Vec<i64> = if free_vars.is_empty() {
            log::info!("Unique solution found");
            m.iter().map(|row| row[self.buttons.len()]).collect()
        } else {
            log::info!("Free variables found at columns {:?}", free_vars);
            if free_vars.len() + m.len() != self.buttons.len() {
                panic!("not enough free variables to solve the system");
            }

            let mut var_ranges = vec![];
            for var in free_vars {
                // determine possible range of var
                let mut min_val = 0;
                let mut max_val = i64::MAX;
                for row in &m {
                    let v = row[var];
                    if v == 0 {
                        continue
                    }
                    let last_col = row[self.buttons.len()];
                    if last_col > 0 {
                        max_val = max_val.min(last_col / v);
                    } else {
                        min_val = min_val.max(last_col / v);
                    }
                }
                var_ranges.push((var, min_val, max_val));
            }

            // try combinations of free vars within their ranges to find minimal solution
            let mut best_solution: Option<Vec<i64>> = None;
            let mut best_total = i64::MAX;
            let mut vars_values = var_ranges.iter().map(|(i, min, _)| (*i, *min)).collect::<Vec<_>>();
            loop {
                let mut candidate_solution = vec![0i64; self.buttons.len()];
                for (i, b) in pivot_cols.iter()
                    .copied()
                    .zip(apply_free_vars(&m, &vars_values))
                    .chain(vars_values.iter().copied()) {
                    candidate_solution[i] = b;
                }

                let valid = candidate_solution.iter().all(|p| *p >= 0);
                if valid {
                    let total: i64 = candidate_solution.iter().sum();
                    if total < best_total {
                        best_total = total;
                        best_solution = Some(candidate_solution);
                    }
                }

                // increment free vars
                let mut incremented = false;
                for i in 0..vars_values.len() {
                    let (_, min_val, max_val) = var_ranges[i];
                    if vars_values[i].1 < max_val {
                        vars_values[i].1 += 1;
                        incremented = true;
                        break;
                    } else {
                        vars_values[i].1 = min_val;
                    }
                }
                if !incremented {
                    break;
                }
            }

            best_solution.unwrap()
        };

        // simulate presses to verify
        let mut joltage = vec![0i32; self.joltage_target.len()];
        for (b, &n) in presses.iter().enumerate() {
            joltage = self.press_button_joltage(b, n as i32, joltage);
        }
        if joltage != self.joltage_target {
            panic!("simulated joltage {:?} does not match target {:?}", joltage, self.joltage_target);
        }

        let total = presses.into_iter().sum();
        log::info!("finished solve_part_2 with presses {}", total);
        total
    }
}

fn gaussian_elimination(m: &mut Vec<Vec<i64>>) {
    let rows = m.len();
    let cols = m[0].len();
    let mut r = 0;
    for c in 0..cols {
        if r >= rows {
            break;
        }
        let mut pivot = r;
        while pivot < rows && m[pivot][c] == 0 {
            pivot += 1;
        }
        if pivot == rows {
            continue;
        }
        if pivot != r {
            log::debug!("swapping row {} with pivot row {}", r, pivot);
        }
        m.swap(r, pivot);
        log::debug!("intermediate matrix at row {}, col {}: {:?}", r, c, m);
        if m[r][c] != 1 {
            let div = m[r][c];
            log::debug!("dividing row {} by {}", r, div);
            for j in c..cols {
                if m[r][j] % div != 0 {
                    panic!("cannot perform elimination with non-integer result");
                }
                m[r][j] /= div;
            }
            log::debug!("intermediate matrix after div: {:?}", m);
        }
        for i in 0..rows {
            if i != r && m[i][c] != 0 {
                log::debug!("eliminating row {} using row {}", i, r);
                let mul = m[i][c];
                for j in c..cols {
                    m[i][j] -= mul * m[r][j];
                }
            }
        }
        r += 1;
    }
}

// returns adjusted last column after applying free vars
fn apply_free_vars(m: &Vec<Vec<i64>>, free_vars: &[(usize, i64)]) -> Vec<i64> {
    let width = m[0].len();
    let mut result = vec![];

    for row in m {
        let mut adjustment = 0;
        for &(var_idx, var_value) in free_vars {
            adjustment += row[var_idx] * var_value;
        }
        result.push(row[width - 1] - adjustment);
    }

    result
}

impl FromStr for Machine {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // [#.###] (0,1) (0,2,3,4) (0,1,4) (3,4) {37,29,8,20,35}
        let parts: Vec<_> = s.split_whitespace().collect();

        let lights_str = parts.get(0).ok_or("missing lights")?;
        let lights_target: Vec<bool> = lights_str
            .trim_matches(&['[', ']'][..])
            .chars()
            .map(|c| match c {
                '#' => Ok(true),
                '.' => Ok(false),
                _ => Err(format!("invalid light character {c}")),
            })
            .collect::<Result<_, _>>()?;

        let buttons_str = parts.iter().skip(1).take_while(|p| p.starts_with('(') && p.ends_with(')'));
        let buttons: Vec<Vec<usize>> = buttons_str
            .map(|b_str| {
                b_str
                    .trim_matches(&['(', ')'][..])
                    .split(',')
                    .map(|num_str| num_str.parse::<usize>().map_err(|e| format!("invalid button index: {e:?}")))
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;

        let joltage_str = parts.last().ok_or("missing joltage")?;
        let joltage_target: Vec<i32> = joltage_str
            .trim_matches(&['{', '}'][..])
            .split(',')
            .map(|num_str| num_str.parse::<i32>().map_err(|e| format!("invalid joltage value: {e:?}")))
            .collect::<Result<_, _>>()?;

        Ok(Machine {
            lights_target,
            buttons,
            joltage_target,
        })
    }
}

#[cfg(test)]
mod tests {
    use aoc::init_test_logging;
    use super::*;

    #[test]
    fn test_gaussian_elimination() {
        init_test_logging();

        let mut m = vec![
            vec![1, 1, 1, 1, 0, 1, 0, 46],
            vec![0, 1, 1, 1, 0, 0, 0, 29],
            vec![1, 1, 1, 0, 1, 1, 0, 59],
            vec![1, 0, 0, 0, 0, 0, 1, 22],
            vec![0, 1, 0, 1, 1, 1, 1, 64],
        ];
        gaussian_elimination(&mut m);
        let expected = vec![
            vec![1,  0,  0,  0,  0,  0,  1,  22],
            vec![0,  1,  0,  0,  2,  0,  2,  82],
            vec![0,  0,  1,  0, -1,  0, -2, -40],
            vec![0,  0,  0,  1, -1,  0,  0, -13],
            vec![0,  0,  0,  0,  0,  1, -1,  -5],
        ];
        assert_eq!(m, expected);
    }
}