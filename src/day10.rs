use std::borrow::Cow;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use aoc::parse_lines;
use num_rational::Rational32;

const ZERO: Rational32 = Rational32::ZERO;
const ONE: Rational32 = Rational32::ONE;

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
            .sum::<i32>()
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

    fn solve_part_2(&self) -> i32 {
        log::info!("starting solve_part_2 for joltage target {:?}", self.joltage_target);
        let mut m = vec![vec![ZERO; self.buttons.len() + 1]; self.joltage_target.len()];
        for y in 0..self.joltage_target.len() {
            for (x, button) in self.buttons.iter().enumerate() {
                if button.contains(&y) {
                    m[y][x] = ONE;
                }
            }
        }
        for (y, &target) in self.joltage_target.iter().enumerate() {
            m[y][self.buttons.len()] = target.into();
        }

        log::info!("matrix before elimination:");
        pretty_print_matrix(&m);

        gaussian_elimination(&mut m);

        log::info!("matrix after elimination:");
        pretty_print_matrix(&m);

        // remove empty rows if present
        m.retain(|row| row[..self.buttons.len()].iter().any(|&val| val != ZERO));

        let mut pivot_cols: Vec<usize> = vec![];
        let mut free_vars: Vec<usize> = vec![];
        for c in 0..self.buttons.len() {
            if is_pivot_col(&m, c) {
                pivot_cols.push(c);
            } else {
                free_vars.push(c);
            }
        }

        let presses: Vec<i32> = if free_vars.is_empty() {
            log::info!("Unique solution found");
            m.iter()
                .map(|row| try_into_i32(row[self.buttons.len()]).unwrap())
                .collect()
        } else {
            log::info!("Free variables found at columns {:?}", free_vars);
            if free_vars.len() + m.len() != self.buttons.len() {
                panic!("not enough free variables to solve the system");
            }

            let mut var_ranges = vec![];
            for var in free_vars {
                // determine possible range of var
                let mut max_val = i32::MAX;
                for row in &m {
                    let v = row[var];
                    if v == ZERO {
                        continue
                    }
                    let row_nonzero_count = row[..self.buttons.len()]
                        .iter()
                        .filter(|&&val| val != ZERO)
                        .count();
                    if row_nonzero_count > 2 {
                        continue; // cannot determine bound from this row
                    }
                    let last_col = row[self.buttons.len()];
                    if last_col > ZERO && v > ZERO {
                        max_val = max_val.min(try_into_i32((last_col / v).floor()).unwrap());
                    }
                }
                if max_val == i32::MAX {
                    log::warn!("unable to determine range for free variable at column {}", var);
                    max_val = 100; // arbitrary limit to avoid infinite search
                }
                var_ranges.push((var, 0, max_val));
            }
            log::info!("Free variable ranges: {var_ranges:?}");

            // try combinations of free vars within their ranges to find minimal solution
            let mut best_solution: Option<Vec<i32>> = None;
            let mut best_total = i32::MAX;
            let mut vars_values = var_ranges.iter().map(|(i, min, _)| (*i, *min)).collect::<Vec<_>>();
            loop {
                let mut candidate_solution = vec![0i32; self.buttons.len()];
                if let Some(adjusted_last_col) = apply_free_vars(&m, &vars_values) {
                    for (i, b) in pivot_cols.iter()
                        .copied()
                        .zip(adjusted_last_col)
                        .chain(vars_values.iter().copied()) {
                        candidate_solution[i] = b;
                    }

                    let valid = candidate_solution.iter().all(|p| *p >= 0);
                    if valid {
                        let total: i32 = candidate_solution.iter().sum();
                        if total < best_total {
                            best_total = total;
                            best_solution = Some(candidate_solution);
                        }
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

            best_solution.expect("did not find any valid solution")
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

type Matrix = Vec<Vec<Rational32>>;

fn gaussian_elimination(m: &mut Matrix) {
    let rows = m.len();
    let cols = m[0].len();
    let mut r = 0;
    for c in 0..cols {
        if r >= rows {
            break;
        }
        let mut pivot = r;
        while pivot < rows && m[pivot][c] == ZERO {
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
        if m[r][c] != ONE {
            let div = m[r][c];
            log::debug!("dividing row {} by {}", r, div);
            for j in c..cols {
                m[r][j] /= div;
            }
            log::debug!("intermediate matrix after div: {:?}", m);
        }
        for i in 0..rows {
            if i != r && m[i][c] != ZERO {
                log::debug!("eliminating row {} using row {}", i, r);
                let mul = m[i][c];
                for j in c..cols {
                    let mrj = m[r][j];
                    m[i][j] -= mul * mrj;
                }
            }
        }
        r += 1;
    }
}

fn is_pivot_col(m: &Matrix, col: usize) -> bool {
    let num_ones = m.iter().filter(|row| row[col] == ONE).count();
    if num_ones != 1 {
        return false;
    }
    let num_nonzeros = m.iter().filter(|row| row[col] != ZERO).count();
    num_nonzeros == 1
}

// returns adjusted last column after applying free vars
fn apply_free_vars(m: &Matrix, free_vars: &[(usize, i32)]) -> Option<Vec<i32>> {
    //log::debug!("applying free vars {:?} to matrix {:?}", free_vars, m);
    let width = m[0].len();
    let mut result = vec![];

    for row in m {
        let mut adjustment = ZERO;
        for &(var_idx, var_value) in free_vars {
            adjustment += row[var_idx] * Rational32::from(var_value);
        }
        result.push(try_into_i32(row[width - 1] - adjustment).ok()?);
    }

    Some(result)
}

fn pretty_print_matrix(m: &Matrix) {
    if log::log_enabled!(log::Level::Info) {
        for row in m {
            let row_str: Vec<String> = row.iter().map(|val| format!("{:>8}", val)).collect();
            log::info!("{}", row_str.join(" "));
        }
    }
}

fn try_into_i32(f: Rational32) -> Result<i32, Cow<'static, str>> {
    if !f.is_integer() {
        return Err(format!("value {} is not an integer", f).into());
    }
    Ok(f.to_integer())
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
            vec![1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 46.0],
            vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 29.0],
            vec![1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 59.0],
            vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 22.0],
            vec![0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 64.0],
        ];
        gaussian_elimination(&mut m);
        let expected = vec![
            vec![1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  1.0,  22.0],
            vec![0.0,  1.0,  0.0,  0.0,  2.0,  0.0,  2.0,  82.0],
            vec![0.0,  0.0,  1.0,  0.0, -1.0,  0.0, -2.0, -40.0],
            vec![0.0,  0.0,  0.0,  1.0, -1.0,  0.0,  0.0, -13.0],
            vec![0.0,  0.0,  0.0,  0.0,  0.0,  1.0, -1.0,  -5.0],
        ];
        assert_eq!(m, expected);
    }
}