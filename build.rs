use clap::{Command, CommandFactory};
use clap_complete::Shell;
use std::fs::File;
use std::path::Path;

include!("src/cli.rs");
const APP_NAME: &str = "unicom";

fn generate(s: Shell, app: &mut Command, outdir: &Path, file: &str) {
    let destfile = outdir.join(file);
    std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
    let mut dest = File::create(destfile).unwrap();
    clap_complete::generate(s, app, APP_NAME, &mut dest);
}

fn main() {
    let mut app = CliArgs::command();
    app.set_bin_name(APP_NAME);

    let outdir = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/completions/");
    generate(Shell::Bash, &mut app, &outdir, "unicom-completion.bash");
    generate(Shell::Zsh, &mut app, &outdir, "_unicom");
    generate(Shell::Fish, &mut app, &outdir, "unicom-completion.fish");
    generate(
        Shell::PowerShell,
        &mut app,
        &outdir,
        "unicom-completion.ps1",
    );
    generate(Shell::Elvish, &mut app, &outdir, "unicom-completion.elv");
}
