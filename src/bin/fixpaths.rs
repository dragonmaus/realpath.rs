use getopt::Opt;
use realpath::realpaths;
use std::{env, io};

program::main!("fixpaths");

fn print_usage(program_name: &str) {
    println!("Usage: {} [-h] [-e VAR] [string ...]", program_name);
    println!("  -e VAR  operate on the value of environment variable VAR");
    println!();
    println!("  -h      display this help");
}

fn program(name: &str) -> program::Result {
    let mut args = program::args();
    let mut opts = getopt::Parser::new(&args, "e:h");

    let mut pathss: Vec<String> = Vec::new();
    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('e', Some(key)) => match env::var_os(&key) {
                    None => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("{} not set", key),
                        )
                        .into());
                    },
                    Some(value) => pathss.push(value.to_string_lossy().into_owned()),
                },
                Opt('h', None) => {
                    print_usage(name);
                    return Ok(0);
                },
                _ => unreachable!(),
            },
        }
    }

    pathss.append(args.split_off(opts.index()).as_mut());

    if pathss.is_empty() {
        match env::var_os("PATH") {
            None => return Err(io::Error::new(io::ErrorKind::InvalidData, "PATH not set").into()),
            Some(value) => pathss.push(value.to_string_lossy().into_owned()),
        }
    }

    for paths in pathss {
        println!("{}", realpaths(&paths)?.to_string_lossy());
    }

    Ok(0)
}
