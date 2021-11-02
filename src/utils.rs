use std::os::unix::fs::{MetadataExt, PermissionsExt};
use humansize::{file_size_opts as options, FileSize};
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};


pub fn get_user(path: std::path::PathBuf, desc: String) -> String 
{
  if desc == "group"
  {
    let id = users::get_group_by_gid(path.symlink_metadata().unwrap().gid());
    if  let Some(u) = id 
    {
      String::from(u.name().to_string_lossy())
    } else 
    {
      String::from(" ")
    }
  }
  else if desc == "user"
  {
    let id = users::get_user_by_uid(path.symlink_metadata().unwrap().uid());
    if let Some(u) = id 
    {
      String::from(u.name().to_string_lossy())
    } else 
    {
      String::from(" ")
    }
  }
  else
  {
    String::from(" ")
  }
}

pub fn get_size(file: std::path::PathBuf) -> String 
{
  std::fs::symlink_metadata(file)
    .unwrap()
    .size()
    .file_size(options::CONVENTIONAL)
    .unwrap()
}

pub fn get_perms(file: std::path::PathBuf) -> String 
{
  let mode = file.symlink_metadata().unwrap().permissions().mode() as u16;
  let user = mode_to_output_map(mode, S_IRUSR as u16, S_IWUSR as u16, S_IXUSR as u16);
  let group = mode_to_output_map(mode, S_IRGRP as u16, S_IWGRP as u16, S_IXGRP as u16);
  let other = mode_to_output_map(mode, S_IROTH as u16, S_IWOTH as u16, S_IXOTH as u16);
  let f = crate::PathDetails::new(&file).unwrap()[0].type_to_char_map();
  [f, user, group, other].join("")
}

pub fn get_time(file: std::path::PathBuf, format: String, desc: String) -> String 
{
  let time;
  
  if desc == "modified"
  {
    time = chrono::NaiveDateTime::from_timestamp(
    filetime::FileTime::from_last_modification_time(&file.symlink_metadata().unwrap()).seconds()
    as i64, 0, );
  }
  else if desc == "created"
  {
    time = chrono::NaiveDateTime::from_timestamp(
      filetime::FileTime::from_creation_time(&file.symlink_metadata().unwrap())
      .unwrap()
      .seconds() as i64,
      0,
      );
  }
  else // if desc == "accessed"
  {
    time = chrono::NaiveDateTime::from_timestamp(
      filetime::FileTime::from_last_access_time(&file.symlink_metadata().unwrap()).seconds()
      as i64, 0, );
  }

  let datetime: chrono::DateTime<chrono::Local> = chrono::DateTime::from_utc(time, *chrono::Local::now().offset());
  datetime.format(&format).to_string()
}



fn mode_to_output_map(mode: u16, read: u16, write: u16, execute: u16) -> String 
{
  match (mode & read, mode & write, mode & execute) {
    (0, 0, 0) => format!(
      "---",
    ),
    (_, 0, 0) => format!(
      "r--",
    ),
    (0, _, 0) => format!(
      "-w-",
    ),
    (0, 0, _) => format!(
      "--x",
    ),
    (_, 0, _) => format!(
      "r-x",
    ),
    (_, _, 0) => format!(
      "rw-",
    ),
    (0, _, _) => format!(
      "-wx",
    ),
    (_, _, _) => format!(
      "rwx",
    ),
  }
}
