use std::env;
use std::net::IpAddr;
use std::str::FromStr;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) => Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let flag = args[1].clone();
        let ipaddr = args[2].clone();

        let threads = if args.len() == 4 {
            match args[3].parse::<u16>() {
                Ok(threads) => threads,
                Err(_) => return Err("Invalid number of threads"),
            }
        } else {
            4
        };

        Ok(Arguments { flag, ipaddr, threads })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

}
