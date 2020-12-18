#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_string_rows;
use std::fmt;

enum NodeType {
    Plus,
    Mult,
    LPar,
    RPar,
    Operand(i64),
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::Plus => f.write_str("+"),
            NodeType::Mult => f.write_str("*"),
            NodeType::LPar => f.write_str("("),
            NodeType::RPar => f.write_str(")"),
            NodeType::Operand(v) => f.write_fmt(format_args!("{}", v)),
        }
    }
}

struct EvaluationNode {
    left: Option<Box<EvaluationNode>>,
    right: Option<Box<EvaluationNode>>,
    value: NodeType,
}

impl EvaluationNode {
    fn new(value: NodeType) -> Self {
        Self {
            left: None,
            right: None,
            value,
        }
    }

    fn evaluate(&self) -> i64 {
        match self.value {
            NodeType::Operand(v) => v,
            NodeType::Plus => {
                (*self.left.as_ref().unwrap()).evaluate()
                    + (*self.right.as_ref().unwrap()).evaluate()
            }
            NodeType::Mult => {
                (*self.left.as_ref().unwrap()).evaluate()
                    * (*self.right.as_ref().unwrap()).evaluate()
            }
            _ => panic!("Not implemented"),
        }
    }
}

impl fmt::Display for EvaluationNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            NodeType::Operand(v) => write!(f, "{}", v),
            _ => write!(
                f,
                "{} {} {}",
                self.left.as_ref().unwrap(),
                self.value,
                self.right.as_ref().unwrap()
            ),
        }
    }
}

fn parse_string(s: String) -> EvaluationNode {
    let terms = s.split(" ");
    let mut open_subs = 0;
    let mut sub_expr = String::new();
    let mut current_node: Option<EvaluationNode> = None;
    for t in terms {
        if open_subs > 0 {
            current_node = if t.contains(")") {
                open_subs -= t.chars().filter(|c| *c == ')').count();
                if open_subs == 0 {
                    sub_expr = format!("{} {}", sub_expr, t.strip_suffix(")").unwrap());
                    let new_node = parse_string(sub_expr);
                    sub_expr = String::new();
                    match current_node {
                        Some(mut c) => {
                            c.right = Some(Box::new(new_node));
                            Some(c)
                        }
                        _ => Some(new_node),
                    }
                } else {
                    sub_expr = format!("{} {}", sub_expr, t);
                    current_node
                }
            } else {
                open_subs += t.chars().filter(|c| *c == '(').count();
                sub_expr = format!("{} {}", sub_expr, t);
                current_node
            }
        } else {
            current_node = match t {
                "+" => {
                    let mut new_node = EvaluationNode::new(NodeType::Plus);
                    match current_node {
                        Some(c) => {
                            new_node.left = Some(Box::new(c));
                        }
                        _ => {}
                    }
                    Some(new_node)
                }
                "*" => {
                    let mut new_node = EvaluationNode::new(NodeType::Mult);
                    match current_node {
                        Some(c) => {
                            new_node.left = Some(Box::new(c));
                        }
                        _ => {}
                    }
                    Some(new_node)
                }
                _ => {
                    let mut o_it = t.chars();
                    match o_it.next().unwrap() {
                        '(' => {
                            open_subs += t.chars().filter(|c| *c == '(').count();
                            sub_expr += o_it.collect::<String>().as_str();
                            current_node
                        }
                        _ => match t.parse::<i64>() {
                            Ok(val) => {
                                let new_node = EvaluationNode::new(NodeType::Operand(val));

                                match current_node {
                                    Some(ref mut c) => match c.value {
                                        NodeType::Operand(_) => panic!("unexpected operand"),
                                        _ => {
                                            c.right = Some(Box::new(new_node));
                                            current_node
                                        }
                                    },
                                    None => Some(new_node),
                                }
                            }
                            Err(e) => panic!("failed to parse: {} ({})", e, t),
                        },
                    }
                }
            }
        }
    }
    current_node.unwrap()
}

pub fn main() {
    let input: Vec<String> = get_row_input!("18");

    let solution: i64 = input.into_iter().map(|r| parse_string(r).evaluate()).sum();
    println!("Day 18 Part 1: {}", solution);
}
