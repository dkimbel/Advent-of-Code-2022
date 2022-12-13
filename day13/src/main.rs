use std::fs;

fn main() {
    let comparer = PacketComparer::new("resources/input_1");
}

enum Packet {
    List(Vec<Packet>),
    Val(u32),
}

enum Token {
    ListStart { depth: u32 },
    ListEnd { depth: u32 },
    Val(u32),
}

impl Packet {
    fn new(input: &str) -> Self {
        // TODO first make Vec<Token>, where Token can be ListStart(depth), Val(n), or ListEnd(depth)
        let tokens = Self::tokenize(input);
        // Self::parse_list(&input[1..input.len()])
        todo!()
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

    fn parse_list(input: &str) -> Self {
        let mut curr_list: Vec<Packet> = Vec::new();

        let mut chunks = input.split(',');
        for chunk in input.split(',') {
            if chunk.starts_with('[') {
                // TODO
                //     excise chunks, ideally with takeWhile, until we find counterbalancing ends-in-]
                //       first find index of chunk with final ']', then use take on mut iter
                //     call parse_list on taken set of chunks (as &str?), minus braces
                //     push the resulting list to own curr_list
                //     continue processing add'l chunks
                todo!()
            } else {
                curr_list.push(Packet::Val(chunk.parse::<u32>().unwrap()))
            }
        }
        Packet::List(curr_list)
    }
}

struct PacketComparer {
    pairs: Vec<(Vec<Token>, Vec<Token>)>,
}

impl PacketComparer {
    fn new(file_path: &str) -> Self {
        let file_content = fs::read_to_string(file_path).unwrap();
        let unparsed_pairs = file_content.split("\n\n");

        let mut pairs = Vec::new();
        for unparsed_pair in unparsed_pairs {
            let (unparsed_left, unparsed_right) = unparsed_pair.split_once('\n').unwrap();
            // pairs.push((Packet::new(unparsed_left), Packet::new(unparsed_right)));
            pairs.push((
                Packet::tokenize(unparsed_left),
                Packet::tokenize(unparsed_right),
            ));
        }

        Self { pairs }
    }
}
