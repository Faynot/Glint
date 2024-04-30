use pest::prec_climber::{PrecClimber, Assoc, Operator};
use pest::Parser;
use pest_derive::Parser as PestParser;
use pest::iterators::Pair;

#[derive(PestParser)]
#[grammar = "grammar.pest"]
struct MyParser;

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    FloorDivide,
    Modulo,
}

fn parse_operation(pair: Pair<Rule>) -> Operation {
    match pair.as_rule() {
        Rule::operation => {
            match pair.as_str() {
                "+" => Operation::Add,
                "-" => Operation::Subtract,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                "^" => Operation::Power,
                "//" => Operation::FloorDivide,
                "%" => Operation::Modulo,
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn parse_expr(pair: Pair<Rule>, climber: &PrecClimber<Rule>) -> f64 {
    let term = |pair: Pair<Rule>| {
        if pair.as_rule() == Rule::term {
            parse_term(pair, climber)
        } else {
            panic!("Unexpected rule: {:?}", pair.as_rule());
        }
    };
    climber.climb(pair.into_inner(), term, |lhs, op, rhs| {
        match op.as_rule() {
            Rule::operation => {
                let op = parse_operation(op);
                match op {
                    Operation::Add => lhs + rhs,
                    Operation::Subtract => lhs - rhs,
                    Operation::Multiply => lhs * rhs,
                    Operation::Divide => lhs / rhs,
                    Operation::Power => lhs.powf(rhs),
                    Operation::FloorDivide => (lhs / rhs).floor(),
                    Operation::Modulo => lhs % rhs,
                }
            }
            _ => unreachable!(),
        }
    })
}

fn parse_term(pair: Pair<Rule>, climber: &PrecClimber<Rule>) -> f64 {
    match pair.as_rule() {
        Rule::term => {
            let inner_pair = pair.into_inner().next().unwrap();
            match inner_pair.as_rule() {
                Rule::num => {
                    inner_pair.as_str().parse().unwrap()
                },
                Rule::expr => {
                    parse_expr(inner_pair, climber)
                },
                _ => unreachable!(),
            }
        }
        Rule::num => {
            pair.as_str().parse().unwrap()
        }
        _ => unreachable!(),
    }
}


fn parse_calculation(pair: Pair<Rule>) -> f64 {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::operation, Assoc::Left),
    ]);

    parse_expr(pair, &climber)
}

fn main() {
    let input = "2 * (3 + 4)";
    let pairs = MyParser::parse(Rule::calculation, input).unwrap_or_else(|e| panic!("{}", e));
    let result = parse_calculation(pairs.into_iter().next().unwrap());
    println!("Result: {}", result);
}
