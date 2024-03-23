use clap::CommandFactory;
use clap::Parser;
use normalize_path::NormalizePath;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
struct Commands {
  /// The directory to search within
  #[arg(default_value = ".")]
  target_dir: PathBuf,
  /// A file name or folder name to look for
  #[arg(short = 'f', long = "find")]
  find_names: Vec<String>,
  /// A file name to look for
  #[arg(short = 'F', long = "file")]
  file_names: Vec<String>,
  /// A directory name to look for
  #[arg(short = 'D', long = "dir")]
  dir_names: Vec<String>,
  /// Exclude directories with name matching
  #[arg(short = 'e', long = "exclude")]
  exclude_names: Vec<String>,
  /// Delete without asking
  #[arg(long = "force")]
  force: bool,
}

fn main() {
  let mut cmd = Commands::parse();
  if cmd.target_dir.is_relative() {
    cmd.target_dir = std::env::current_dir()
      .unwrap()
      .join(cmd.target_dir)
      .normalize();
  }

  if cmd.find_names.len() == 0 && cmd.dir_names.len() == 0 && cmd.file_names.len() == 0 {
    Commands::command().print_help().unwrap();
    return;
  }

  let lookup_exclude = HashSet::<String>::from_iter(cmd.exclude_names);
  let mut lookup_any = HashSet::<String>::new();
  let mut lookup_dirs = HashSet::<String>::new();
  let mut lookup_files = HashSet::<String>::new();

  println!("Looking within:");
  println!("  {}", cmd.target_dir.to_str().unwrap());

  println!("");
  println!("Looking for:");
  for item in &cmd.dir_names {
    println!("  [DIR] {}", item);
    lookup_dirs.insert(item.clone());
  }
  for item in &cmd.file_names {
    println!("  [FILE] {}", item);
    lookup_files.insert(item.clone());
  }
  for item in &cmd.find_names {
    println!("  {}", item);
    lookup_any.insert(item.clone());
  }

  if lookup_exclude.len() != 0 {
    println!("Excluding");
    for item in &lookup_exclude {
      println!("  {}", item);
    }
  }

  let mut matches = HashSet::<PathBuf>::new();

  println!("");
  println!("Found:");

  WalkDir::new(&cmd.target_dir)
    .into_iter()
    .filter_entry(|entry| {
      let entry_path = entry.path();
      let entry_name = entry.file_name().to_str().unwrap();
      
      if lookup_exclude.contains(entry_name) {
        return false;
      }
      if entry_path.is_dir()
        && (lookup_any.contains(entry_name) || lookup_dirs.contains(entry_name))
      {
        println!("  {}", entry_path.to_str().unwrap());
        matches.insert(entry_path.to_path_buf());
        return false;
      }
      if entry_path.is_file()
        && (lookup_any.contains(entry_name) || lookup_files.contains(entry_name))
      {
        println!("  {}", entry_path.to_str().unwrap());
        matches.insert(entry_path.to_path_buf());
      }
      return true;
    })
    .for_each(|_| {});

  println!("");
  print!("Delete matches? ({} found) [y/N] ", matches.len());
  let mut line = String::new();
  let _ = std::io::stdout().flush();
  std::io::stdin().read_line(&mut line).unwrap();
  line = line.trim().to_string();

  if line != "y" && line != "Y" {
    println!("Nothing deleted");
    return;
  }

  for item in matches.iter() {
    println!("  {}", item.to_str().unwrap());
    if item.is_dir() {
      fs::remove_dir_all(&item).unwrap();
    }
    if item.is_file() {
      fs::remove_file(&item).unwrap();
    }
  }
}
