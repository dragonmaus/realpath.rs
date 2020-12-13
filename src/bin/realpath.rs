use getopt::prelude::*;
use realpath::realpath;

program::main!("realpath");

fn usage_line() -> String {
    format!("Usage: {} [-0h] path [path ...]", program::name("realpath"))
}

fn print_usage() {
    println!("{}", usage_line());
    println!("  -0   terminate output lines with NUL, not newline");
    println!();
    println!("  -h   display this help");
}

fn program() -> program::Result {
    let mut args = program::args();
    let mut opts = Parser::new(&args, "0h");

    let mut eol = '\n';

    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('0', None) => eol = '\0',
                Opt('h', None) => {
                    print_usage();
                    return Ok(0);
                }
                _ => unreachable!(),
            },
        }
    }

    let mut args = args.split_off(opts.index());

    if args.is_empty() {
        args.push(String::from("."));
    }

    for arg in args {
        print!("{}{}", realpath(&arg)?.to_string_lossy(), eol);
    }

    Ok(0)
}
