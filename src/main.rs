use std::env::args_os;
use std::process::ExitCode;
use configparser::ini::Ini;

fn main() -> ExitCode {
    let args: Vec<_> = args_os().collect();
    if args.len() != 4 {
        eprintln!("Usage: ini_patch input.ini patch.ini output.ini");
        eprintln!("New keys and values will be appended in corresponding sections");
        eprintln!("Keys that exist in input and patch will have the value from patch");
        return ExitCode::from(2);
    }

    let mut base = Ini::new();
    match base.load(&args[1]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading input ini: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let mut patch = Ini::new();
    match patch.load(&args[2]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading patch ini: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let mut out = base.clone();
    for (section, entries) in patch.get_mut_map() {
        for (key, value) in entries {
            let val = Some(String::from(value.as_ref().unwrap()));
            out.set(section, key, val);
        }
    }
    out.write(&args[3]).unwrap();

    return ExitCode::SUCCESS;
}
