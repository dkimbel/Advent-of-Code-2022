use std::fs;

fn main() {
    let comparer = PacketComparer::new("resources/input_1");
    let solution_1 = comparer.ordering_score();
    println!("Part 1 solution: {}", solution_1);
}

#[derive(Debug, Clone)]
enum Packet {
    List(Vec<Packet>),
    Val(u32),
}

#[derive(PartialEq, Eq)]
enum Token {
    ListStart { depth: u32 },
    ListEnd { depth: u32 },
    Val(u32),
}

impl Packet {
    fn new(input: &str) -> Self {
        let tokens = Self::tokenize(input);
        Self::parse_list(&tokens[1..tokens.len() - 1])
    }

    fn parse_list(tokens: &[Token]) -> Self {
        let mut curr_list: Vec<Packet> = Vec::new();

        let mut i = 0;
        while i < tokens.len() {
            let token = &tokens[i];
            let (packet, new_i) = match token {
                Token::Val(n) => (Self::Val(*n), i + 1),
                Token::ListStart { depth } => {
                    let matching_end_i_relative = tokens[i..tokens.len()]
                        .iter()
                        .position(|token| token == &Token::ListEnd { depth: *depth })
                        .unwrap();
                    let matching_end_i = matching_end_i_relative + i;
                    (
                        Self::parse_list(&tokens[i + 1..matching_end_i]),
                        matching_end_i + 1,
                    )
                }
                Token::ListEnd { .. } => panic!("Unexpected end-of-list token!"),
            };
            curr_list.push(packet);
            i = new_i;
        }

        Self::List(curr_list)
    }

    fn tokenize(input: &str) -> Vec<Token> {
        let mut depth: u32 = 0;
        let mut tokens = Vec::new();
        let mut curr_str_num = String::new();

        for char in input.chars() {
            if char == '[' {
                depth += 1;
                tokens.push(Token::ListStart { depth });
            } else if char == ']' {
                if !curr_str_num.is_empty() {
                    tokens.push(Token::Val(curr_str_num.parse::<u32>().unwrap()));
                    curr_str_num.clear();
                }
                tokens.push(Token::ListEnd { depth });
                depth -= 1;
            } else if char == ',' {
                if !curr_str_num.is_empty() {
                    tokens.push(Token::Val(curr_str_num.parse::<u32>().unwrap()));
                    curr_str_num.clear();
                }
            } else if char.is_ascii_digit() {
                curr_str_num.push(char);
            } else {
                panic!("Cannot tokenize char {}", char);
            }
        }

        tokens
    }
}

struct PacketComparer {
    pairs: Vec<(Packet, Packet)>,
}

impl PacketComparer {
    fn new(file_path: &str) -> Self {
        let file_content = fs::read_to_string(file_path).unwrap();
        let unparsed_pairs = file_content.split("\n\n");

        let mut pairs = Vec::new();
        for unparsed_pair in unparsed_pairs {
            let (unparsed_left, unparsed_right) = unparsed_pair.split_once('\n').unwrap();
            pairs.push((Packet::new(unparsed_left), Packet::new(unparsed_right)));
        }

        Self { pairs }
    }

    // todo refactor to use Ord trait?
    fn pair_correctly_ordered(pair: &(Packet, Packet)) -> bool {
        let (Packet::List(v_left), Packet::List(v_right)) = pair else {panic!("Can only compare two list packets!")};
        match Self::packet_lists_correctly_ordered(v_left, v_right) {
            Some(b) => b,
            None => panic!("Packets are completely equal!"),
        }
    }

    fn packet_lists_correctly_ordered(left_list: &[Packet], right_list: &[Packet]) -> Option<bool> {
        let max_len = std::cmp::max(left_list.len(), right_list.len());
        for i in 0..max_len {
            if i >= left_list.len() {
                return Some(true);
            } else if i >= right_list.len() {
                return Some(false);
            }

            let left = &left_list[i];
            let right = &right_list[i];

            use Packet::*;
            match (left, right) {
                (Val(l), Val(r)) => {
                    if l == r {
                        continue;
                    } else if l < r {
                        return Some(true);
                    } else if l > r {
                        return Some(false);
                    }
                }
                // TODO reduce repetition
                (List(ll), List(rl)) => {
                    let maybe_result = Self::packet_lists_correctly_ordered(ll, rl);
                    match maybe_result {
                        res @ Some(_) => return res,
                        None => continue,
                    }
                }
                (List(ll), rval @ Val(_)) => {
                    let maybe_result = Self::packet_lists_correctly_ordered(ll, &[rval.clone()]);
                    match maybe_result {
                        res @ Some(_) => return res,
                        None => continue,
                    }
                }
                (lval @ Val(l), List(rl)) => {
                    let maybe_result = Self::packet_lists_correctly_ordered(&[lval.clone()], rl);
                    match maybe_result {
                        res @ Some(_) => return res,
                        None => continue,
                    }
                }
            }
        }
        None
    }

    fn ordering_score(&self) -> usize {
        self.pairs
            .iter()
            .enumerate()
            .map(|(i, pair)| {
                if Self::pair_correctly_ordered(pair) {
                    i + 1
                } else {
                    0
                }
            })
            .sum()
    }
}
