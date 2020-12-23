
const PARENS: [u8; 2] = [b'(', b')'];

#[derive(Debug, Copy, Clone, PartialEq)]
enum Token {
    Literal(i64),
    Op(u8),
    Paren(u8),
}

struct Parser<'a> {
    expr: &'a str
}

impl Parser<'_> {
    fn advance(&mut self) -> Token {
        let bytes = self.expr.as_bytes();
        let mut start: usize = 0;
        while bytes[start] == b' ' {
            start += 1;
        }
        let mut end = start + 1;
        match bytes[start] {
            x @ b'(' | x @ b')' => {
                self.expr = &self.expr[end..];
                return Token::Paren(x);
            }
            x @ b'+' | x @ b'*' => {
                self.expr = &self.expr[end..];
                return Token::Op(x);
            }
            b'0'..=b'9' => {
                while end < self.expr.len() && bytes[end] != b' ' && !PARENS.contains(&bytes[end]) {
                    end += 1;
                }
                let ret = Token::Literal(self.expr[start..end].parse::<i64>().unwrap());
                self.expr = &self.expr[end..];
                return ret;
            }
            x => panic!("Can't parse token {}", x),
        }
    }

    fn peek(&self) -> Token {
        let bytes = self.expr.as_bytes();
        let mut start: usize = 0;
        while bytes[start] == b' ' {
            start += 1;
        }
        let mut end = start + 1;
        match bytes[start] {
            x @ b'(' | x @ b')' => {
                return Token::Paren(x);
            }
            x @ b'+' | x @ b'*' => {
                return Token::Op(x);
            }
            b'0'..=b'9' => {
                while end < self.expr.len() && bytes[end] != b' ' && !PARENS.contains(&bytes[end]) {
                    end += 1;
                }
                return Token::Literal(self.expr[start..end].parse::<i64>().unwrap());
            }
            x => panic!("Can't parse token {}", x),
        }
    }

    fn eval1(&mut self) -> i64 {
        let lhs = self.advance();
        let mut res = match lhs {
            Token::Paren(b'(') => self.eval1(),
            Token::Literal(x) => x,
            token => panic!("Unexpected LHS token {:?}", token),
        };

        while !self.expr.is_empty() {
            if self.peek() == Token::Paren(b')') {
                self.advance();
                break;
            }

            let op = self.advance();
            let rhs = self.advance();

            let rhs_val = match rhs {
                Token::Paren(b'(') => self.eval1(),
                Token::Literal(x) => x,
                token => panic!("Unexpected RHS token {:?}", token),
            };

            res = match op {
                Token::Op(b'+') => res + rhs_val,
                Token::Op(b'*') => res * rhs_val,
                op => panic!("Unexpected op {:?}", op),
            };
        }
        return res 
    }

    fn eval2(&mut self) -> i64 {
        let lhs = self.advance();
        let mut res = match lhs {
            Token::Paren(b'(') => self.eval2(),
            Token::Literal(x) => x,
            token => panic!("Unexpected LHS token {:?}", token),
        };

        while !self.expr.is_empty() {
            if self.peek() == Token::Paren(b')') {
                self.advance();
                break;
            }

            let op = self.advance();
            let rhs_val = match op {
                Token::Op(b'*') => self.eval2(),
                _ => {
                    let rhs = self.advance();
                    match rhs {
                        Token::Paren(b'(') => self.eval2(),
                        Token::Literal(x) => x,
                        token => panic!("Unexpected RHS token {:?}", token),
                    }
                }
            };

            res = match op {
                Token::Op(b'+') => res + rhs_val,
                Token::Op(b'*') => res * rhs_val,
                op => panic!("Unexpected op {:?}", op),
            };

            if op == Token::Op(b'*') {
                break;
            }
        }
        return res 
    }
}

fn main() {
    let mut sum:i64 = 0;
    let mut sum2:i64 = 0;
    for line in aoc::file_lines_iter("./day18.txt") {
        let mut p = Parser{expr: &line};
        sum += p.eval1();
        let mut p2 = Parser{expr: &line};
        sum2 += p2.eval2();
    }
    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum2);
}
