use getopts::Options;
use std::env;

fn print_usage(opts: Options) {
    print!("{}", opts.usage(&"Usage:\n [options]"));
}

pub fn parse() -> Option<String> {
    let flag_name = "db";
    println!("Before");
    let mut opts = Options::new();
    println!("2");
    opts.optopt("", flag_name, "The path to the compilation database", "");
    println!("3");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let matches = match opts.parse(&args) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let result = matches.opt_str(flag_name);
    if result.is_none() {
        print_usage(opts);
    }
    println!("after: {:?}", result);
    
    return result;
}
