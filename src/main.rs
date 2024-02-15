use std::{env, fs::File, io::{self, BufRead, BufReader, Write}, path::{Path, PathBuf}};
use zip::{write::FileOptions, ZipWriter};

#[derive(Debug, Clone)]
struct Cli {
  initial_fsize: f64,
  layers: i32,
  file_name: String
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
  let args: Vec<String> = env::args().collect();
  
  load_logo();

  // store args in struct
  let cli = Cli {
    initial_fsize: args[1].parse::<f64>().ok().or(Some(10.0)).unwrap(),
    layers: args[2].parse::<i32>().ok().or(Some(4)).unwrap(),
    file_name: args[3].parse::<String>().ok().or(Some("YAZZ".to_string())).unwrap()
  };

  //print settings
  print_args(cli.clone());

  // CONFIG
  let config = FileOptions::large_file(FileOptions::default(), if cli.initial_fsize > 4000.0 {true} else {false});
  
  // INIT STUFF
  let full_name = format!("{}.zip",cli.file_name);
  let path = Path::new(&full_name);
  let file = File::create(&path)?;

  // file contents
  let f_contents = "0".repeat((cli.initial_fsize * 1048576.0) as usize);


  // initial file creation
  let mut zip = ZipWriter::new(file);
  zip.start_file(cli.file_name, config)?;
  zip.write_all(f_contents.as_bytes())?;
  zip.finish()?;

  for i in 0..cli.layers {
    print_progress(i as f64, cli.layers as f64);
    copy_contents(PathBuf::from(full_name.clone()),full_name.clone(), cli.layers, config);  
  }

  println!("> \x1b[92mDONE.\x1b[0m");
  Ok(())
}

fn copy_contents(path:PathBuf, name:String, amount: i32, config: FileOptions) -> File {
  // create new buffer and read old file into it
  let mut file = File::open(path).expect("no filé");
  let mut buf = Vec::new();
  io::copy(&mut file, &mut buf).expect("wda error handling hehe");
  
  let temp = File::create(&Path::new(&name)).expect("waoidajdoawdjiaw Error hadnalign");
  let mut zip = ZipWriter::new(temp);

  // Add the old content into a new one i times
  for i in 0..amount {
    zip.start_file(format!("{}.zip", i.to_string()), config).expect("Err");
    zip.write_all(&buf).expect("Err 2");
  }
  zip.finish().expect("wadijdaodaw error handling hehe")
}

fn load_logo() {
  let f = File::open("./assets/title.txt").unwrap();
  let lines:Vec<String> = BufReader::new(f).lines().map(|l|l.expect("cant read lad")).collect();

  for l in lines {
    println!("{}",l);
  }
}

fn print_args(cli:Cli) {
  println!("> \x1b[96mCONFIGURATION LOADED \x1b[0m");
  println!("> INITIAL FILE SIZE: \x1b[91m{} MB\x1b[0m", cli.initial_fsize);
  println!("> LAYER COUNT: \x1b[91m{} \x1b[0m", cli.layers);
  println!("> EFFECTIVE FILE COUNT: \x1b[91m{}^{} \x1b[0m (calc it yourself you lazy ass)", cli.layers, cli.layers);
  println!("> FILE NAME: \x1b[91m{}.zip \x1b[0m", cli.file_name);
  println!("---------------------------------------------");
}

fn print_progress(current: f64, max: f64) {
  let percent = current / max;
  println!("> PROGRESS: \x1b[91m{:.2}% \x1b[0m", 100.0 * percent);
}