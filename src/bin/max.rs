use std::process::ExitCode;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// list of space separated numbers to compare
    list: Vec<f64>,
}

fn main() -> ExitCode {
    let args: Args = Args::parse();
    match max(args.list) {
        Some(x) => {
            println!("{}", x);
            ExitCode::SUCCESS
        }
        None => ExitCode::FAILURE,
    }
}

fn max(list: Vec<f64>) -> Option<f64> {
    if list.is_empty() {
        return None;
    }
    let mut clonelist: Vec<f64> = list.clone();
    let mut y: f64 = clonelist.pop().unwrap();
    clonelist.into_iter().for_each(|x: f64| y = y.max(x));
    Some(y)
}

#[cfg(test)]
mod max {
    use super::*;

    #[test]
    fn two_unequal_values() {
        let result: Option<f64> = max(vec![2_f64, 1_f64]);
        assert_eq!(result, Some(2_f64));
    }

    #[test]
    fn two_equal_values() {
        let result: Option<f64> = max(vec![1_f64, 1_f64]);
        assert_eq!(result, Some(1_f64));
    }

    #[test]
    fn no_values() {
        let result: Option<f64> = max(vec![]);
        assert_eq!(result, None);
    }
}
