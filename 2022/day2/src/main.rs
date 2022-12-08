// strategy guide format:
// column 1 is A,B,C for R,P,S
// column 2 is X,Y,Z for R,P,S

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input missing");

    let input_split = input.lines();

    let mut total_score: u32 = 0;

    for move_line in input_split.clone() {
        let mut move_chars = move_line.chars();
        let opponent_move = parse_move_char(move_chars.nth(0).unwrap());
        let my_move = parse_move_char(move_chars.last().unwrap());

        let mut score = my_move.value();

        match MoveSet(my_move, opponent_move) {
            MoveSet(Move::Rock, Move::Rock)
            | MoveSet(Move::Paper, Move::Paper)
            | MoveSet(Move::Scissors, Move::Scissors) => score += 3,

            MoveSet(Move::Paper, Move::Rock)
            | MoveSet(Move::Scissors, Move::Paper)
            | MoveSet(Move::Rock, Move::Scissors) => score += 6,

            _ => {}
        }

        total_score += score as u32;
    }

    println!("Total score: {}", total_score);

    total_score = 0;

    for move_line in input_split {
        let mut move_chars = move_line.chars();
        let opponent_move = parse_move_char(move_chars.nth(0).unwrap());
        let response = move_chars.last().unwrap();

        let mut score = 0;

        match (opponent_move, response) {
            // lose
            (Move::Paper, 'X') => score += Move::Rock.value(),
            (Move::Scissors, 'X') => score += Move::Paper.value(),
            (Move::Rock, 'X') => score += Move::Scissors.value(),

            // draw
            (Move::Rock, 'Y') => score += 3 + Move::Rock.value(),
            (Move::Paper, 'Y') => score += 3 + Move::Paper.value(),
            (Move::Scissors, 'Y') => score += 3 + Move::Scissors.value(),

            // win
            (Move::Scissors, 'Z') => score += 6 + Move::Rock.value(),
            (Move::Rock, 'Z') => score += 6 + Move::Paper.value(),
            (Move::Paper, 'Z') => score += 6 + Move::Scissors.value(),

            _ => unreachable!(),
        }

        total_score += score as u32;
    }

    println!("Total score part 2: {}", total_score);
}

fn parse_move_char(move_char: char) -> Move {
    match move_char {
        'A' | 'X' => Move::Rock,
        'B' | 'Y' => Move::Paper,
        'C' | 'Z' => Move::Scissors,
        _ => unreachable!(),
    }
}

struct MoveSet(Move, Move);

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn value(&self) -> u8 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}
