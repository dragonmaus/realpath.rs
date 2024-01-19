use getopt::Opt;
use realpath::realpath;

program::main!("realpath");

fn print_usage(program_name: &str) {
    println!("Usage: {} [-0h] path [path ...]", program_name);
    println!("  -0   terminate output lines with NUL, not newline");
    println!();
    println!("  -h   display this help");
}

fn program(name: &str) -> program::Result {
    let mut args = program::args();
    let mut opts = getopt::Parser::new(&args, "0h");

    let mut eol = '\n';

    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('0', None) => eol = '\0',
                Opt('h', None) => {
                    print_usage(name);
                    return Ok(0);
                },
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
