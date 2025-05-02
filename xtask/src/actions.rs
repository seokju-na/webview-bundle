use crate::Error;
use crate::exec::Exec;
use biome_console::{Console, ConsoleExt, markup};
use relative_path::RelativePathBuf;
use similar::{ChangeTag, TextDiff};
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub enum Actions {
  Write {
    path: RelativePathBuf,
    content: String,
    prev_content: Option<String>,
  },
  Command {
    cmd: String,
    args: Vec<String>,
    path: RelativePathBuf,
  },
}

pub fn run_actions<C>(
  name: &str,
  root_dir: &Path,
  console: Arc<Mutex<Box<C>>>,
  actions: Vec<Actions>,
  dry_run: bool,
) -> Result<(), Error>
where
  C: Console + Send + Sync + 'static,
{
  if dry_run {
    dry_run_actions(name, console, actions);
    return Ok(());
  }

  let prefix = format!("[{name}] ");

  for action in actions.iter() {
    let mut cons = console.lock().unwrap();
    match action {
      Actions::Write {
        path,
        content,
        prev_content: _,
      } => {
        let filepath = path.to_path(root_dir);
        match std::fs::write(filepath, content) {
          Ok(()) => cons.log(markup! {
            <Success>{prefix}</Success>"Write file: "{path.to_string()}
          }),
          Err(e) => {
            cons.error(markup! {
              <Error>{prefix}</Error>"Write file error: "{e.to_string()}
              "  "{path.to_string()}
            });
            return Err(Error::ActionFailed {
              action: action.clone(),
            });
          }
        }
      }
      Actions::Command { cmd, args, path } => {
        let mut exec = Exec::new(cmd);
        for arg in args {
          exec.arg(arg);
        }
        exec.cwd(&path.to_path(root_dir));
        exec.line_prefix(&prefix);
        cons.log(markup! {
          <Info>{prefix}</Info>"Run command: "{exec.format()}
        });
        let status = exec.run(console.clone())?;
        if !status.success() {
          cons.error(markup! {
            <Error>{prefix}</Error>"Command error with status: "{status.to_string()}
          });
          return Err(Error::ActionFailed {
            action: action.clone(),
          });
        }
      }
    }
  }
  Ok(())
}

fn dry_run_actions<C>(name: &str, console: Arc<Mutex<Box<C>>>, actions: Vec<Actions>)
where
  C: Console + Send + Sync + 'static,
{
  let prefix = format!("[{name}] ");
  for action in actions {
    let mut cons = console.lock().unwrap();
    match action {
      Actions::Write {
        path,
        content,
        prev_content,
      } => {
        cons.log(markup! {
          <Info>{prefix}</Info>"Will write file: "{path.to_string()}
        });
        if let Some(prev_content) = prev_content {
          let diff = TextDiff::from_lines(&prev_content, &content);
          for change in diff.iter_all_changes() {
            let line = match change.missing_newline() {
              true => change.to_string(),
              false => change.to_string().trim_end().to_string(),
            };
            match change.tag() {
              ChangeTag::Insert => {
                let line_no = format!(
                  "{:>3}",
                  change
                    .new_index()
                    .map(|x| x.to_string())
                    .unwrap_or_default()
                );
                cons.log(markup! {"  "<Dim>{line_no}</Dim>"|"<Success>"+"{line}</Success>});
              }
              ChangeTag::Delete => {
                let line_no = format!(
                  "{:>3}",
                  change
                    .old_index()
                    .map(|x| x.to_string())
                    .unwrap_or_default()
                );
                cons.log(markup! {"  "<Dim>{line_no}</Dim>"|"<Error>"-"{line}</Error>});
              }
              _ => {}
            }
          }
        }
      }
      Actions::Command { cmd, args, path } => {
        let mut exec = Exec::new(&cmd);
        for arg in args {
          exec.arg(&arg);
        }
        cons.log(markup! {
          <Info>{prefix}</Info>"Will run command: "{exec.format()}
        });
        cons.log(markup! {
          "  "<Dim>"at "{path.to_string()}</Dim>
        });
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use biome_console::{ColorMode, EnvConsole};

  #[test]
  fn dry_run() {
    let mut console = EnvConsole::default();
    console.set_color(ColorMode::Enabled);
    let console = Arc::new(Mutex::new(Box::new(console)));
    dry_run_actions(
      "test",
      console.clone(),
      vec![Actions::Write {
        path: RelativePathBuf::from("test.txt"),
        content: r#"{
  "version": "1.1.0"
}
"#
        .to_string(),
        prev_content: Some(
          r#"{
  "version": "1.2.0"
}
"#
          .to_string(),
        ),
      }],
    )
  }
}
