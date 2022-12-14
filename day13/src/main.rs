use std::fs;

fn main() {
    let comparer = PacketComparer::new("resources/input_1");
    println!("{:#?}", comparer.pairs)
}

#[derive(Debug)]
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
        Self::parse_list(&tokens[1..tokens.len()])
    }

    fn parse_list(tokens: &[Token]) -> Self {
        let mut curr_list: Vec<Packet> = Vec::new();

        let mut i = 0;
        while i < tokens.len() {
            let token = &tokens[i];
            let (packet, new_i) = match token {
                Token::Val(n) => (Self::Val(*n), i + 1),
                Token::ListStart { depth } => {
                    let matching_end_i = tokens[i..tokens.len()]
                        .iter()
                        .position(|token| token == &Token::ListEnd { depth: *depth })
                        .unwrap();
                    (
                        Self::parse_list(&tokens[i..matching_end_i]),
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
}
