// Include the utils module in this directory
mod utils;

// Import the necessary crates/packages
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use std::cmp::Ordering;

// Libraries needed for the inode generation:
// use std::fs;
// use std::io;

// Command line args are handled by Structs in Rust, an entry for each argument implemented
#[derive(StructOpt)]
pub struct CmdArgs 
{
  /// Takes the directory to be listed from cmd line
  #[structopt(parse(from_os_str), default_value = ".")]
  pub dir: std::path::PathBuf, 

  /// Prints almsot all entries, except those beginning with . or ..
  #[structopt(short = "A")]
  pub almost_all_entries: bool,

  /// Prints all entries, doesn't ignore those beginning with .
  #[structopt(short = "a")]
  pub all_entries: bool,

  /// Prints tha author of each file, set by default
  #[structopt(long = "author")]
  pub author: bool,

  /// Sort the files by the date created
  #[structopt(short = "c")]
  pub created: bool,
      
  /// Only list directories
  #[structopt(short = "d", long = "directory")]
  pub only_dir: bool,

  /// Don not sort entries
  #[structopt(short = "f")]
  pub no_sort_f: bool,

  /// Human readable format, set by default
  #[structopt(short = "h", long = "human-readable")]
  pub human: bool,

  /// Shows the time of creation instead of last modified time
  #[structopt(short = "i", long = "inode")]
  pub inode: bool,

  /// List all the details of the file
  #[structopt(short = "l")]
  pub long: bool,  

  /// Prints list as comma separated values
  #[structopt(short = "m")]
  pub comma: bool,

  /// Sorts files alphabetically by filename
  #[structopt(short = "n", long = "name")]
  pub name: bool,

  /// Prints file names in quotes
  #[structopt(short = "Q", long = "quote-name")]
  pub quote: bool,

  /// Print files in the reverse order - best used with a sort option
  #[structopt(short = "r", long = "reverse")]
  pub reverse: bool,

  /// Print files recursively
  #[structopt(short = "R", long = "recursive")]
  pub recur: bool,
  
  /// Prints the allocated size of each file
  #[structopt(short = "s", long = "size")]
  pub display_size: bool,

  /// Sorts files by file size
  #[structopt(short = "S")]
  pub size: bool,

  /// Sorts files by the last-modified time
  #[structopt(short = "t")]
  pub modified: bool,

  /// Prints file names one per line
  #[structopt(short = "1")]
  pub one: bool,

  /// Doesn't sort, prints in the directory order (default)
  #[structopt(short = "U")]
  pub no_sort: bool,

  /// Formats the time output, options are Rust time formats like %Y-%m-%d %H:%M:%S (default), %b %e %H:%M , %a %b %e %T %Y and rearranged variants 
  #[structopt(long = "time-style", default_value = "%Y-%m-%d %H:%M:%S")]
  pub time_style: String,

  /// Sort by access time
  #[structopt(long = "atime")]
  pub accessed: bool,

  /// Takes as input the criteria to sort based on
  #[structopt(long = "sort", default_value = "create")]
  pub sort: String,
}

// Deal with files and folders seperately
struct Directory 
{
  paths: Vec<File>,
  args: CmdArgs,
}

// A Struct for the file details
#[derive(Clone)]
struct File 
{
  path:      std::path::PathBuf,
  file_type: Vec<PathDetails>,
  group:     String,
  user:      String,
  modified:  String,
  accessed:  String,
  created:   String,
  size:      String,
  perms:     String,
}

// Store the sorting criteria chosen
enum SortCriteria 
{
  Name,
  Created,
  Modified,
  Accessed,
  Size,
  Not,
}

// Store the path details
#[derive(Copy, Clone, Debug)]
enum PathDetails 
{
  Directory,
  Symlink,
  Path,
  Pipe,
  CharDevice,
  BlockDevice,
  Socket,
}

// An implementation on the above enumeration
impl PathDetails 
{
  fn new(file: &Path) -> Result<Vec<Self>, Box<dyn std::error::Error>> 
  {
    let mut return_val = Vec::new();

    if file.symlink_metadata()?.is_dir() {return_val.push(Self::Directory) }
    if file.symlink_metadata()?.file_type().is_symlink() {return_val.push(Self::Symlink)}
    if file.symlink_metadata()?.file_type().is_fifo() {return_val.push(Self::Pipe)}
    if file.symlink_metadata()?.file_type().is_char_device() {return_val.push(Self::CharDevice)}
    if file.symlink_metadata()?.file_type().is_block_device() {return_val.push(Self::BlockDevice)}
    if file.symlink_metadata()?.file_type().is_socket() {return_val.push(Self::Socket)}
    if return_val.is_empty() {return_val.push(Self::Path)}

    Ok(return_val)
  }

  fn format_char(&self, letter: &str) -> String 
  {
    format!("{}", letter)
  }

  // Map the file type to character  
  fn type_to_char_map(&self) -> String 
  {
    match self 
    {
      Self::Directory     => self.format_char("d"),
      Self::Symlink       => self.format_char("l"),
      Self::CharDevice    => self.format_char("c"),
      Self::BlockDevice   => self.format_char("b"),
      Self::Socket        => self.format_char("s"),
      _                   => self.format_char("-"),
    }
  }
}

// Implementation on the File struct
impl File 
{
  fn new(file: std::path::PathBuf, time_format: String) -> Self 
  {
    Self 
    {
      group:     utils::get_user(file.to_path_buf(), "group".to_string()),
      user:      utils::get_user(file.to_path_buf(), "user".to_string()),
      modified:  utils::get_time(file.to_path_buf(), time_format.to_owned(), "modified".to_string()),
      created:   utils::get_time(file.to_path_buf(), time_format.to_owned(), "created".to_string()),
      accessed:  utils::get_time(file.to_path_buf(), time_format.to_owned(), "accessed".to_string()),
      size:      utils::get_size(file.to_path_buf()),
      perms:     utils::get_perms(file.to_path_buf()),
      file_type: PathDetails::new(&file).unwrap(),
      path: file,
    }
  }
}

// Translate the sorting type selected to a compatible format
fn get_sort_type(sort_type: [bool; 5]) -> SortCriteria 
{
  for (i, t) in sort_type.iter().enumerate() {
    if *t {
      match i {
        0 => return SortCriteria::Name,
        1 => return SortCriteria::Created,
        2 => return SortCriteria::Modified,
        3 => return SortCriteria::Accessed,
        4 => return SortCriteria::Size,
        _ => (),
      }
    }
  }
  SortCriteria::Not
}

// Implementation on the Directory struct
impl Directory 
{
  fn new(args: CmdArgs) -> Result<Self, Box<dyn std::error::Error>> 
  {
    let dir = &args.dir;

    if !std::path::Path::new(&dir).exists() 
    {
	    return Err(Box::new(std::io::Error::from_raw_os_error(2)))
    }

    if !std::path::Path::new(&dir).is_dir() 
    {
      let f = File::new(dir.to_owned(), args.time_style);
      match args.long 
      {
        true => print!("{:?}", f),
        _ => print!("{}", f)
      }
      std::process::exit(0)
    }

    let paths = std::fs::read_dir(dir)?.map(|output| output.map(|e| File::new(e.path(), args.time_style.to_owned()))).collect::<Result<Vec<File>, std::io::Error>>()?;
    Ok(Self { paths, args })
  }

  // Deals with the -d argument here
  fn only_directory(&mut self) 
  {
    let new = &self.paths;
    let mut newer = Vec::new();
    let mut directories = Vec::new();
    for (i, f) in new.iter().enumerate() 
    {
      if f.path.symlink_metadata().unwrap().is_dir() 
      {
        directories.push(new[i].to_owned());
      } 
    }

    match get_sort_type
    ([
      self.args.name,
      self.args.created,
      self.args.modified,
      self.args.accessed,
      self.args.size,
    ]) 
    {
      SortCriteria::Name => 
      {
        name_sort(&mut directories);
        name_sort(&mut newer)
      }
      SortCriteria::Created => 
      {
        create_sort(&mut directories);
        create_sort(&mut newer)
      }
      SortCriteria::Modified => 
      {
        modified_sort(&mut directories);
        modified_sort(&mut newer)
      }
      SortCriteria::Accessed => 
      {
        accessed_sort(&mut directories);
        accessed_sort(&mut newer)
      }
      SortCriteria::Size => 
      {
        size_sort(&mut directories);
        size_sort(&mut newer)
      }
      SortCriteria::Not => (),
    }
    directories.append(&mut newer);
    self.paths = directories;
  }

  fn sort_paths(&mut self) 
  {
    match get_sort_type
    ([
      self.args.name,
      self.args.created,
      self.args.modified,
      self.args.accessed,
      self.args.size,
    ]) 
    {
      SortCriteria::Name     => sort_as(&mut self.paths, |a, b| {
        a.path
          .file_name()
          .unwrap()
          .to_str()
          .unwrap()
          .to_lowercase()
          .cmp(&b.path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_lowercase()
          )
      }),
      SortCriteria::Created  => sort_as(&mut self.paths, |a, b| {
        a.path
          .symlink_metadata()
          .unwrap()
          .created()
          .unwrap()
          .cmp(&b.path
            .symlink_metadata()
            .unwrap()
            .created()
            .unwrap()
          )
      }),
      SortCriteria::Modified => sort_as(&mut self.paths,  |a, b| {
        a.path
          .symlink_metadata()
          .unwrap()
          .modified()
          .unwrap()
          .cmp(&b.path
            .symlink_metadata()
            .unwrap()
            .modified()
            .unwrap()
          )
      }),
      SortCriteria::Accessed => sort_as(&mut self.paths,  |a, b| {
        a.path
          .symlink_metadata()
          .unwrap()
          .accessed()
          .unwrap()
          .cmp(&b.path
            .symlink_metadata()
            .unwrap()
            .modified()
            .unwrap()
          )
      }),

      SortCriteria::Size => sort_as(&mut self.paths,  |a, b| {
        a.path
          .symlink_metadata()
          .unwrap()
          .size()
          .cmp(&b.path.
            symlink_metadata()
            .unwrap()
            .size()
          )
      }),
      SortCriteria::Not => (),
    }
  }

  // Sort the entries
  fn sort(&mut self) 
  {
    if self.args.only_dir 
    {
      self.only_directory();
    }
    else
    {
      self.sort_paths();
    }  
  }

// Format the output to match the Unix command's output format
  fn format_output(&mut self) 
  {
    let (mut group, mut user, mut size) = (0, 0, 0);
    
    for path in self.paths.iter() 
    {
      if path.group.len() > group 
      {
        group = path.group.len()
      }
      if path.user.len() > user 
      {
        user = path.user.len()
      }
      if path.size.len() > size 
      {
        size = path.size.len()
      }
    }

    for p in 0..self.paths.iter().len() 
    {
      let g_copy = self.paths[p].group.to_owned();
      let u_copy = self.paths[p].user.to_owned();
      let s_copy = self.paths[p].size.to_owned();
      let mut g_spaces = String::new();
      for _ in 0..(group - g_copy.len() + 1) 
      {
        g_spaces.push(' ');
      }
      let mut u_spaces = String::new();
      for _ in 0..(user - u_copy.len() + 1) 
      {
        u_spaces.push(' ')
      }
      let mut s_spaces = String::new();
      for _ in 0..(size - s_copy.len() + 1) 
      {
        s_spaces.push(' ')
      }
      self.paths[p].group = format!("{}{}", g_copy, g_spaces);
      self.paths[p].user  = format!("{}{}", u_copy, u_spaces);
      self.paths[p].size  = format!("{}{}", s_spaces, s_copy);
    }
  }

  fn setup(&mut self) -> &mut Directory 
  {
    self.sort();
    self.format_output();
	  self
  }
}

// Sorting of the entries happens here:
fn sort_as<T>(files: &mut Vec<File>, sort_method: T)
where T: Fn(&File, &File) -> Ordering 
{
  files.sort_by(sort_method);
  if CmdArgs::from_args().reverse
  {
    files.reverse()
  }
}

fn name_sort(dir: &mut Vec<File>) 
{
  dir.sort_by(|a, b| {
    a.path
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
      .to_lowercase()
      .cmp(&b.path.file_name().unwrap().to_str().unwrap().to_lowercase())
  })
}

fn create_sort(dir: &mut Vec<File>) 
{
  dir.sort_by(|a, b| {
    a.path
      .symlink_metadata()
      .unwrap()
      .created()
      .unwrap()
      .cmp(&b.path.symlink_metadata().unwrap().created().unwrap())
  })
}

fn modified_sort(dir: &mut Vec<File>) 
{
  dir.sort_by(|a, b| {
    a.path
      .symlink_metadata()
      .unwrap()
      .modified()
      .unwrap()
      .cmp(&b.path.symlink_metadata().unwrap().modified().unwrap())
  })
}

fn accessed_sort(dir: &mut Vec<File>) 
{
  dir.sort_by(|a, b| {
    a.path
      .symlink_metadata()
      .unwrap()
      .accessed()
      .unwrap()
      .cmp(&b.path.symlink_metadata().unwrap().modified().unwrap())
  })
}

fn size_sort(dir: &mut Vec<File>) 
{
  dir.sort_by(|a, b| {
    a.path
      .symlink_metadata()
      .unwrap()
      .size()
      .cmp(&b.path.symlink_metadata().unwrap().size())
  })
}

// Display format options, according to the -ls command options
impl std::fmt::Display for File 
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
  {
    let output = String::new();
    let file_name = self.path.components().next_back().unwrap().as_os_str().to_string_lossy().to_string();
    let ch = file_name.chars().next().unwrap();
    let ch2 = file_name.chars().nth(1).unwrap();

    let new = &self.path;

    for (_i, v) in new.iter().enumerate() 
    {

      let path = PathBuf::from(v);
      
      if path.is_dir() && CmdArgs::from_args().recur
      {
        // Recursive function to come here - unable to figure out this part.
        // Pseudocode:
        // Print the directory name
        // Call the main function with the new directory's name
        // The base case - if an entry is a file, will automatically terminate the recursion, because of the enclosing 'if' condition
      }
    } 
    

    if ch != '.' || (ch == '.' && CmdArgs::from_args().all_entries) || ( CmdArgs::from_args().almost_all_entries && ( (ch != '.' && ch2 != '.') || (ch != '.') ) ) 
    {
      // The following code is the pseudocode to print the inode of an entry
      // Code snippet borrowed from: https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html#tymethod.ino
      // Error message: ^^^ method not found in `Result<Metadata, std::io::Error>`
 
      // if CmdArgs::from_args().inode
      // {
      //   let meta = fs::metadata(&file_name);
      //   let inode_num = meta.ino();
      //   write!(f,"{} {}", inode_num, file_name);
      // }

      if CmdArgs::from_args().display_size
      {
        write!(f, "{} {}", self.size, file_name)
      }
      else if CmdArgs::from_args().quote
      {
        write!(f, "\"{}{}\"", output, file_name)
      }
      else if CmdArgs::from_args().one
      {
        write!(f, "{}\n{}", output, file_name)
      }
      else if CmdArgs::from_args().comma
      {
        write!(f, "{}{}, ", output, file_name)
      }
      else
      {
        write!(f, "{}{}", output, self.path.components().next_back().unwrap().as_os_str().to_string_lossy().to_string())
      }
    }
    else
    {
      write!(f,"")
    }
  }
}

impl std::fmt::Debug for File 
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
  {
    let mut output = String::new();
    for (i, _v) in self.file_type.iter().enumerate() 
    {

      if i == 0 
      {  
        output = format!("{}", self.path
        .components()
        .next_back()
        .unwrap()
        .as_os_str()
        .to_string_lossy()
        .to_string());
        continue;
      }
      output = format!("{}", output);
    }

    let ch = output.chars().next().unwrap();

    if ch != '.' || (ch == '.' && CmdArgs::from_args().all_entries)
    {

	  let time = if CmdArgs::from_args().created { &self.created } else if CmdArgs::from_args().modified { &self.modified } else { &self.accessed};
    
    writeln!(f, "{} {} {}  {}{} {}", self.perms, self.size, self.user, self.group, time, output)
    }

    else
    {
      write!(f,"")
    }
  }
}

impl std::fmt::Display for Directory 
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
  {
    Ok(for i in self.paths.iter() 
    {
      match self.args.long 
      {
        true => write!(f, "{:?}", i)?,
        _    => write!(f, "{} ", i)?,
      }
    })
  }
}


fn main() 
{
  println!
  (
    "{}",
    match Directory::new(CmdArgs::from_args()) 
    {
      Ok(mut output) => format!("{}", output.setup()),
      Err(err) => format!("{}", err)
    }
  );
}
