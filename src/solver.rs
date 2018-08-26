extern crate rand;
use rand::thread_rng;
use rand::Rng;


fn singleton(v: u16) -> u16 {
    return 1 << (v - 1);
}

#[test]
fn test_singleton() {
    assert_eq!(singleton(3) == 4);
}

struct Sofa {
    grid: Problem,
    freedom: Freedom,
    best: [u16; 9],
    best_size: u16,
    best_value: u16
}

fn sofa_set(s: Sofa) -> Sofa {

}

fn sofa(problem: &Problem, freedom: &Freedom, ) -> Sofa {

}

struct Solver {
    problem: Problem,
    count: u64,
    branch_score: u64,
}


fn pick_value(set: u16) -> u16 {
    let mut rng = thread_rng();
    let mut x = rng.gen_range(0, set.count_ones());
    for i in 0..9 {
        match (x, set & (1 << i)){
            (0, 1) => return i + 1,
            (_, _) => x -= 1
        }
    }
    return 0;
}

fn choose_b1(problem: &Problem) -> Problem {
    let mut vals = problem.values;
    let mut opts = ALL_VALUES;
    for i in 0..3 {
        for j in 0..3 {
            let value = pick_value(opts);
            vals[i * DIM as usize + j] = value;
            opts &= !singleton(value);
        }
    }
    return Problem {values: vals};
}

fn choose_col1(problem: &Problem) -> Problem {
    let mut vals = problem.values;
    let mut opts = ALL_VALUES;
    for i in 0..3 {
        opts &= !singleton(vals[i as usize * DIM]);
    }
    for j in 3..9 {
        let value = pick_value(j);
        vals[j as usize * DIM as usize] = value;
        opts &= !singleton(value);
    }
    return Problem { values: vals };
}

fn choose_rest(problem: &Problem, freedom: &Freedom) -> Option<Problem> {

}
