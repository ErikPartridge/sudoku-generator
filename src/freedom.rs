const ORDER: u8 = 3;
const DIM: u8 = ORDER * ORDER;
const ELEMENTS: u8 = DIM * DIM;
const ALL_VALUES: u16 = (1 << DIM) - 1;
use problem::Problem;

pub struct Freedom {
    pub values: [u16; 81]
}

fn singleton(v: u16) -> u16 {
    return 1 << (v - 1);
}

/// Here, we compute all the possible values of other cells, once value is set in location (col, rows)
fn freedom_eliminate(f: &Freedom, col: u16, row: u16, value: u16) -> Freedom {
    let mut freedom: Freedom = Freedom { values: f.values };
    let mask: u16 = !singleton(value);

    // Remove it as a contender in the row and the column
    for i in 0..9 {
        let col_index = col as usize + i * 9;
        freedom.values[col_index] &= mask;
        let row_index = row as usize / 9 + i;
        freedom.values[row_index] &= mask;
    }
    let mut index = (row - row % ORDER as u16) * DIM as u16 + col - col % ORDER as u16;
    for j in 0..3 {
        for k in 0..3 {
            freedom.values[index as usize + k] &= mask;
        }
        index += DIM as u16;
    }
    return freedom;
}

/// This will generate the freedom matrix from the problem set
fn create_freedom(problem: &Problem) -> Freedom {
    let vals = [ALL_VALUES; ELEMENTS as usize];
    let mut freedom = Freedom{ values: vals };
    for row in 0..9 {
        for col in 0..9 {
            match problem.values[(row * (DIM as u16) + col) as usize] {
                0 => (),
                v => freedom = freedom_eliminate(&freedom, col, row, v)
            }
        }
    }
    return freedom
}

fn sanity_check(problem: &Problem, freedom: &Freedom) -> bool {
    for i in 0..81 {
        if problem.values[i] != 0 {
            if ! (freedom.values[i] & singleton(problem.values[i])) == 0 {
                return false;
            }
        }
    }
    return true;
}

fn search_least_free(problem: &Problem, freedom: &Freedom) -> u16 {
    let mut best_index = 81;
    let mut best_value = <u16>::max_value();
    for i in 0..81 {
        if problem.values[i] == 0 {
            let score = freedom.values[i].count_ones() as u16;
            if score < best_value {
                best_index = 1;
                best_value = score;
            }
        }
    }
    return best_index;
}
