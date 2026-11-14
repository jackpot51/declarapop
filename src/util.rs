use std::process;

use crate::Error;

pub fn command_stdout(mut command: process::Command) -> Result<String, Error> {
    let output = command.stderr(process::Stdio::inherit()).output()?;
    if !output.status.success() {
        return Err(Error::ExitStatus(
            command.get_program().to_string_lossy().into(),
            output.status,
        ));
    }
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
}
