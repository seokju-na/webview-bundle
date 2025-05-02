use biome_console::{Console, ConsoleExt, markup};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Exec {
  cmd: String,
  args: Vec<String>,
  cwd: Option<PathBuf>,
  line_prefix: Option<String>,
}

impl Exec {
  pub fn new(cmd: &str) -> Self {
    Self {
      cmd: cmd.to_string(),
      args: vec![],
      cwd: None,
      line_prefix: None,
    }
  }

  pub fn arg(&mut self, arg: &str) -> &mut Self {
    self.args.push(arg.to_string());
    self
  }

  pub fn cwd(&mut self, cwd: &Path) -> &mut Self {
    self.cwd = Some(cwd.to_path_buf());
    self
  }

  pub fn line_prefix(&mut self, line_prefix: &str) -> &mut Self {
    self.line_prefix = Some(line_prefix.to_string());
    self
  }

  pub fn format(&self) -> String {
    let args = self.args.join(" ");
    format!("{} {}", self.cmd, args)
  }

  pub fn run<C>(&self, console: Arc<Mutex<Box<C>>>) -> Result<ExitStatus, crate::Error>
  where
    C: Console + Send + Sync + 'static,
  {
    let mut cmd = Command::new(&self.cmd);
    for arg in &self.args {
      cmd.arg(arg);
    }
    if let Some(cwd) = &self.cwd {
      cmd.current_dir(cwd);
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let prefix = self.line_prefix.to_owned().unwrap_or_default();

    let out_task = {
      let prefix = prefix.clone();
      let console = console.clone();
      thread::spawn(move || -> Result<(), std::io::Error> {
        let mut cons = console.lock().unwrap();
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
          cons.log(markup! {
            <Info>{prefix}</Info>{line}
          });
        }
        Ok(())
      })
    };
    let err_task = {
      let prefix = prefix.clone();
      let console = console.clone();
      thread::spawn(move || -> Result<(), std::io::Error> {
        let mut cons = console.lock().unwrap();
        let reader = BufReader::new(stderr);
        for line in reader.lines().map_while(Result::ok) {
          cons.error(markup! {
            <Error>{prefix}</Error>{line}
          });
        }
        Ok(())
      })
    };

    let status = child.wait()?;
    out_task.join().unwrap()?;
    err_task.join().unwrap()?;

    Ok(status)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use biome_console::BufferConsole;

  #[test]
  fn echo() {
    let console = Arc::new(Mutex::new(Box::new(BufferConsole::default())));
    let status = Exec::new("echo")
      .arg("hello")
      .line_prefix("[prefixed] ")
      .run(console.clone())
      .unwrap();
    assert!(status.success());
    assert_eq!(console.lock().unwrap().out_buffer.len(), 1);
  }
}
