use pest::Parser;
use avdl_serde_code_generator::{Rule, AVDLParser, to_rust::{self, build_rust_code_from_avdl}};
use std::fs::{self, File, DirEntry};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::ffi::{OsStr};
use std::io::Write;

fn create_rust_version(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
  let input = fs::read_to_string(input_path)?;
  let parsed = AVDLParser::parse(Rule::avdl_protocol, &input)?;
  let mut output = File::create(output_path)?;
  // let derive_okay_paths = ["keybase1/common.rs","keybase1/upk.rs", "keybase1/teams.rs", "keybase1/reset.rs", "keybase1/tlf_keys", "stellar1/common"];
  let derive_okay_paths = ["gregor1","keybase1/teams.rs","chat1/gregor.rs","keybase1/kbfs_common.rs","keybase1/notify_runtimestats.rs","keybase1/simple_fs.rs","keybase1/identify_common.rs","keybase1/identify_ui.rs","keybase1/git.rs","keybase1/upk.rs", "chat1/chat_ui.rs","chat1/local.rs","keybase1/tlf_keys.rs","chat1/unfurl.rs",  "chat1/api.rs","chat1/common.rs","stellar1/common.rs","chat1/commands.rs","chat1/remote.rs","keybase1/common.rs","keybase1/reset.rs", "kebayse1/teams.rs", "keybase1/favorite.rs","keybase1/identify.rs", "stellar1/local.rs"];
  if derive_okay_paths.iter().any(|p| output_path.contains(p)) {
    to_rust::set_add_derive(true);
  }
  to_rust::set_derive_hash_for_ident(vec!["UserVersion"].into_iter().map(|s| s.into()).collect());
  build_rust_code_from_avdl(parsed, &mut output)?;
    to_rust::set_add_derive(false);
  Ok(())
}

fn visit_dirs<F>(dir: &Path, cb: &F) -> Result<(), Box<dyn Error>> where F: Fn(&DirEntry) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                cb(&entry)?;
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
    Ok(())
}

fn map_to_output_file(input: &PathBuf) -> PathBuf {
  let file_name = input.file_name().unwrap();
    let original_path = input.as_path().iter().skip_while(|part| {
      part != &OsStr::new("keybase-protocol")
    }).skip(1).collect::<Vec<&OsStr>>();
    let mut output_path = Path::new("src/protocol/").to_path_buf();
    for part in original_path.iter() {
      output_path.push(part);
    }
    output_path.pop();
    let output_filename = Path::new(file_name).with_extension("rs");
    output_path.push(output_filename);
    output_path
}

fn create_mod_file(path: &PathBuf) -> Result<(), Box<dyn Error>> {
  println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
  let inner_files = fs::read_dir(path).unwrap();
  let inner_files: Vec<String> = inner_files.map(|f| {
    let file_name = f.unwrap().file_name();
    let file_name_str = file_name.to_str().unwrap();
    let mod_use = format!("pub mod {};\npub use {}::*;", file_name_str, file_name_str);
    mod_use.replace(".avdl", "")
  }).collect();
  let mut output_filename = Path::new(path).to_path_buf();
  output_filename.push("mod.rs");
  let output_filename = map_to_output_file(&output_filename);
  fs::create_dir_all(output_filename.parent().unwrap())?;
  let mut file = File::create(output_filename)?;
  write!(&mut file, "{}", inner_files.join("\n"))?;


  Ok(())
}

fn main() {
  create_mod_file(&Path::new("keybase-protocol/").to_path_buf()).unwrap();
  to_rust::set_derive_hash_for_ident(vec!["UserVersion"].into_iter().map(|s| s.into()).collect());
  visit_dirs(Path::new("keybase-protocol/"), &|entry: &DirEntry| -> Result<(), Box<dyn Error>> {
    let entry_path = entry.path();
    if entry_path.is_dir() {
      create_mod_file(&entry_path).unwrap();
    } else {
      let output_path = map_to_output_file(&entry_path);
      fs::create_dir_all(output_path.parent().unwrap())?;
      create_rust_version(entry.path().to_str().unwrap(), output_path.to_str().unwrap())?;
    }
    Ok(())
  }).unwrap();
}