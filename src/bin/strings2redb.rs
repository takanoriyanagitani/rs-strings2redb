use std::io;
use std::process::ExitCode;

use rs_strings2redb::stdin2strings2path2sink_default;

fn io_envkey2val(key: &'static str) -> impl Fn() -> String {
    move || std::env::var(key).unwrap_or_default()
}

fn io_path() -> impl Fn() -> String {
    io_envkey2val("ENV_OUTPUT_PATH")
}

fn io_main() -> impl FnOnce() -> Result<(), io::Error> {
    || {
        let out_path: String = io_path()();
        stdin2strings2path2sink_default(out_path)()
    }
}

fn sub() -> Result<(), io::Error> {
    io_main()()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        let p: String = io_path()();
        eprintln!("ENV_OUTPUT_PATH: {p}");
        ExitCode::FAILURE
    })
}
