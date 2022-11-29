use std::fs;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Operator {
    Sum,
    Mul,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Parenthesis {
    Open,
    Close,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    Term(i64),
    Op(Operator),
    Flow(Parenthesis),
}

fn lexer(s: &str) -> Vec<Token> {
    let mut tokens = vec![];
    for c in s.replace(" ", "").chars() {
        match c {
            '+' => tokens.push(Token::Op(Operator::Sum)),
            '*' => tokens.push(Token::Op(Operator::Mul)),
            '(' => tokens.push(Token::Flow(Parenthesis::Open)),
            ')' => tokens.push(Token::Flow(Parenthesis::Close)),
            num => tokens.push(Token::Term(num.to_string().parse().unwrap())),
        }
    }
    tokens
}

trait Node {
    fn value(&self) -> Option<i64>;
    fn op(&self, rhs: Self, lhs: Self) -> Self;
    fn flow(&self) -> Option<Parenthesis>;
    fn priority(&self) -> Option<Operator>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Tree<T: Node> {
    nodes: Vec<Box<Tree<T>>>,
    value: Option<T>,
}

impl Node for Token {
    fn value(&self) -> Option<i64> {
        match self {
            Token::Term(num) => Some(*num),
            _ => None,
        }
    }
    fn op(&self, lhs: Self, rhs: Self) -> Self {
        let v = lhs.value().unwrap();
        let w = rhs.value().unwrap();
        match self {
            Token::Op(Operator::Sum) => Token::Term(v + w),
            Token::Op(Operator::Mul) => Token::Term(v * w),
            _ => self.clone(),
        }
    }
    fn flow(&self) -> Option<Parenthesis> {
        match self {
            Token::Flow(paren) => Some(paren.clone()),
            _ => None,
        }
    }
    fn priority(&self) -> Option<Operator> {
        match self {
            Token::Op(op) => Some(op.clone()),
            _ => None,
        }
    }
}

fn parse_rec<T>(tokens: Vec<T>) -> (Tree<T>, Vec<T>)
where
    T: Clone,
    T: Node,
    T: std::fmt::Debug,
{
    let mut tree = Tree::<T> {
        value: None,
        nodes: vec![],
    };
    let mut parsed = tokens.clone();
    while parsed.len() > 0 {
        if Some(Parenthesis::Close) == parsed[0].flow() {
            parsed.remove(0);
            break;
        } else if Some(Parenthesis::Open) == parsed[0].flow() {
            let (subtree, remaining) = parse_rec::<T>(parsed.iter().skip(1).cloned().collect());
            tree.nodes.push(Box::new(subtree));
            parsed = remaining;
        } else {
            tree.nodes.push(Box::new(Tree::<T> {
                value: Some(parsed[0].clone()),
                nodes: vec![],
            }));
            parsed.remove(0);
        }
    }
    (tree, parsed)
}

fn parser<T>(tokens: Vec<T>) -> Tree<T>
where
    T: Clone,
    T: Node,
    T: std::fmt::Debug,
{
    parse_rec(tokens).0
}

fn exec<T>(tree: Tree<T>, precedence: &Option<Operator>) -> Tree<T>
where
    T: Clone,
    T: Node,
    T: std::fmt::Debug,
{
    if tree.value.is_some() {
        tree
    } else {
        let mut prec = precedence.clone();
        let mut parsed = tree.nodes.clone();
        let mut i: usize = 1;
        loop {
            if i >= parsed.len() {
                if parsed.len() == 1 {
                    break *parsed[0].clone();
                } else {
                    i = 1;
                    prec = None;
                }
            }
            if prec.is_none() || prec == parsed[i].clone().value.unwrap().priority() {
                let prevvalue = exec(*parsed[i - 1].clone(), precedence).value.unwrap();
                let opvalue = parsed[i].clone().value.unwrap();
                let nextvalue = exec(*parsed[i + 1].clone(), precedence).value.unwrap();
                parsed[i] = Box::new(Tree::<T> {
                    value: Some(opvalue.op(prevvalue, nextvalue)),
                    nodes: vec![],
                });
                parsed.remove(i + 1);
                parsed.remove(i - 1);
            } else {
                i += 2;
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let summer = |op| {
        input
            .split("\n")
            .filter(|line| line.len() > 0)
            .map(|line| {
                exec(parser(lexer(line)), &op)
                    .value
                    .unwrap()
                    .value()
                    .unwrap()
            })
            .sum::<i64>()
    };
    println!("The sum of all the lines is {}", summer(None));
    println!(
        "The sum of all the lines with + over * is {}",
        summer(Some(Operator::Sum))
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    const TEST_0: &str = "2 * 3 + (4 * 5)";
    const TEST_1: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    const TEST_2: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    const TEST_3: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn lexer_gen() {
        assert_eq!(
            lexer(TEST_0),
            vec![
                Token::Term(2),
                Token::Op(Operator::Mul),
                Token::Term(3),
                Token::Op(Operator::Sum),
                Token::Flow(Parenthesis::Open),
                Token::Term(4),
                Token::Op(Operator::Mul),
                Token::Term(5),
                Token::Flow(Parenthesis::Close),
            ]
        );
    }

    #[test]
    fn tree_gen() {
        assert_eq!(
            parser(lexer(TEST_0)),
            Tree::<Token> {
                value: None,
                nodes: vec![
                    Box::new(Tree::<Token> {
                        value: Some(Token::Term(2)),
                        nodes: vec![],
                    }),
                    Box::new(Tree::<Token> {
                        value: Some(Token::Op(Operator::Mul)),
                        nodes: vec![],
                    }),
                    Box::new(Tree::<Token> {
                        value: Some(Token::Term(3)),
                        nodes: vec![],
                    }),
                    Box::new(Tree::<Token> {
                        value: Some(Token::Op(Operator::Sum)),
                        nodes: vec![],
                    }),
                    Box::new(Tree::<Token> {
                        value: None,
                        nodes: vec![
                            Box::new(Tree::<Token> {
                                value: Some(Token::Term(4)),
                                nodes: vec![],
                            }),
                            Box::new(Tree::<Token> {
                                value: Some(Token::Op(Operator::Mul)),
                                nodes: vec![],
                            }),
                            Box::new(Tree::<Token> {
                                value: Some(Token::Term(5)),
                                nodes: vec![],
                            }),
                        ],
                    }),
                ]
            }
        );
    }

    #[test]
    fn exec_test() {
        assert_eq!(
            exec(parser(lexer(TEST_0)), &None),
            Tree::<Token> {
                value: Some(Token::Term(26)),
                nodes: vec![]
            }
        );
        assert_eq!(
            exec(parser(lexer(TEST_1)), &None),
            Tree::<Token> {
                value: Some(Token::Term(437)),
                nodes: vec![]
            }
        );
        assert_eq!(
            exec(parser(lexer(TEST_2)), &None),
            Tree::<Token> {
                value: Some(Token::Term(12240)),
                nodes: vec![]
            }
        );
        assert_eq!(
            exec(parser(lexer(TEST_3)), &None),
            Tree::<Token> {
                value: Some(Token::Term(13632)),
                nodes: vec![]
            }
        );
    }

    #[test]
    fn exec_test_precedence() {
        assert_eq!(
            exec(parser(lexer(TEST_0)), &Some(Operator::Sum)),
            Tree::<Token> {
                value: Some(Token::Term(46)),
                nodes: vec![]
            }
        );
        assert_eq!(
            exec(parser(lexer(TEST_1)), &Some(Operator::Sum)),
            Tree::<Token> {
                value: Some(Token::Term(1445)),
                nodes: vec![]
            }
        );
        assert_eq!(
            exec(parser(lexer(TEST_2)), &Some(Operator::Sum)),
            Tree::<Token> {
                value: Some(Token::Term(669060)),
                nodes: vec![]
            }
        );
        assert_eq!(
            exec(parser(lexer(TEST_3)), &Some(Operator::Sum)),
            Tree::<Token> {
                value: Some(Token::Term(23340)),
                nodes: vec![]
            }
        );
    }
}
