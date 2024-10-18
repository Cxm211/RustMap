use std::f64;

type SeqFunc = Box<dyn FnMut() -> i32>;

struct Gen {
    f: SeqFunc,
    pub output: i32,
}

fn seq_next(state: &mut Gen) -> i32 {
    state.output = (state.f)();
    state.output
}

struct PowerGen {
    f: SeqFunc,
    output: i32,
    pos: i32,
    n: i32,
}

fn power_next(state: &mut PowerGen) -> i32 {
    state.pos += 1;
    (f64::from(state.pos).powi(state.n)) as i32
}

fn power_seq(n: i32) -> Box<PowerGen> {
    Box::new(PowerGen {
        f: Box::new(move || power_next(&mut *power_seq(n))),
        output: -1,
        pos: -1,
        n,
    })
}

struct FilterGen {
    f: SeqFunc,
    output: i32,
    input: Box<Gen>,
    without: Box<Gen>,
}

fn filter_next(state: &mut FilterGen) -> i32 {
    loop {
        seq_next(&mut *state.input);
        while state.without.output < state.input.output {
            seq_next(&mut *state.without);
        }
        if state.without.output != state.input.output {
            break;
        }
    }
    state.input.output
}

fn filter_seq(input: Box<Gen>, without: Box<Gen>) -> Box<FilterGen> {
    Box::new(FilterGen {
        f: Box::new(move || filter_next(&mut *filter_seq(input, without))),
        output: -1,
        input,
        without,
    })
}

fn main() {
    let mut s = filter_seq(power_seq(2), power_seq(3));

    for _ in 0..20 {
        seq_next(&mut *s);
    }