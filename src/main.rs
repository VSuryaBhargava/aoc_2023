use utils::{
    args::Args,
    parser::ParseInput,
    problem::{SolvePart1, SolvePart2},
};

fn main() {
    let args = utils::args::get_args();
    let input = utils::parser::read_file(args.my_input, args.day);

    let value = match args.day {
        1 => run(day01::Problem, args, input),
        2 => run(day02::Problem, args, input),
        3 => run(day03::Problem, args, input),
        4 => run(day04::Problem, args, input),
        5 => run(day05::Problem, args, input),
        6 => run(day06::Problem, args, input),
        7 => run(day07::Problem, args, input),
        8 => run(day08::Problem, args, input),
        9 => run(day09::Problem, args, input),
        10 => run(day10::Problem, args, input),
        11 => run(day11::Problem, args, input),
        12 => run(day12::Problem, args, input),
        13 => run(day13::Problem, args, input),
        14 => run(day14::Problem, args, input),
        15 => run(day15::Problem, args, input),
        16 => run(day16::Problem, args, input),
        17 => run(day17::Problem, args, input),
        18 => run(day18::Problem, args, input),
        19 => run(day19::Problem, args, input),
        20 => run(day20::Problem, args, input),
        21 => run(day21::Problem, args, input),
        22 => run(day22::Problem, args, input),
        23 => run(day23::Problem, args, input),
        24 => run(day24::Problem, args, input),
        25 => run(day25::Problem, args, input),
        _ => panic!("Invalid day."),
    };

    println!("Solution: {}", value);
}

fn run<T>(
    mut problem: impl ParseInput<ParsedType = T>
        + SolvePart1<ParsedType = T>
        + SolvePart2<ParsedType = T>,
    args: Args,
    input: String,
) -> String {
    let parsed_input = problem.parse(input, args.part2);
    if args.part2 {
        problem.solve_part_two(parsed_input)
    } else {
        problem.solve_part_one(parsed_input)
    }
}
