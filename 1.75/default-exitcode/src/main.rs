use std::process::ExitCode;

fn cmd_ops() -> Result<ExitCode, ()> {
    if true {
        Ok(ExitCode::SUCCESS)
    } else {
        Err(())
    }
}

fn main() -> Result<ExitCode, ()> {
    let exit_code: ExitCode = cmd_ops().unwrap_or_default();

    Ok(exit_code)
}
