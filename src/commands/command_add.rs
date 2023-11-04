use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn create(venv_path: &Path, name: &String, version: Option<String>, force: bool) -> i32 {
    let path = venv_path.join(&name);
    if path.is_file() || (path.is_dir() && !force) {
        eprintln!("Env with the same name `{name}` exists.");
        return 1;
    }

    let venv_exe = "virtualenv";
    let mut cmd = Command::new(venv_exe);
    let mut cmd = cmd.arg(path.as_os_str());
    if let Some(ver) = version {
        cmd = cmd.arg("--python").arg(format!("{ver}"));
    }

    let ok = match cmd
        // The following lines are customized settings
        .args(["--activators", "batch,powershell"])
        .args(["--no-setuptools", "--no-wheel", "--no-vcs-ignore"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
    {
        Ok(output) => match output.status.code() {
            Some(code) => code == 0,
            None => false,
        },
        Err(e) => {
            eprintln!("Failed to create env `{name}`: {e}\nMaybe `{venv_exe}` is not in PATH?");
            return 1;
        }
    };

    if ok && path.exists() && create_idle(&path) {
        0
    } else {
        1
    }
}

fn create_idle(path: &Path) -> bool {
    let idle = path.join("Scripts/idle.bat");
    match fs::File::create(idle) {
        Ok(mut file) => {
            let result = file.write_all(b"@call %~dp0python.exe -m idlelib %*");
            match result {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}
