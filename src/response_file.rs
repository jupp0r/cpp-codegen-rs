use std::io::Read;
use std::iter::Iterator;
use std::option::Option;
use std::collections::VecDeque;
use std::fs::File;

pub struct ParseResponseFiles<I>
    where I: Iterator<Item = String>
{
    iter: I,
    response_file_stack: VecDeque<String>,
}

impl<I> ParseResponseFiles<I>
    where I: Iterator<Item = String>
{
    fn next_from_iter(&mut self) -> Option<I::Item> {
        match self.iter.next() {
            None => None,
            Some(item) => {
                if item.starts_with("@") {
                    let mut f = File::open(&item[1..]).unwrap();
                    let mut s = String::new();
                    f.read_to_string(&mut s).unwrap();
                    self.response_file_stack = s.split(";").map(String::from).collect();
                    println!("read from response file: {:?}", self.response_file_stack);
                    self.response_file_stack.pop_front()
                } else {
                    Some(item)
                }
            }
        }
    }

    fn next_from_stack(&mut self) -> Option<I::Item> {
        self.response_file_stack.pop_front()
    }
}

impl<I> Iterator for ParseResponseFiles<I>
    where I: Iterator<Item = String>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.response_file_stack.is_empty() {
            self.next_from_iter()
        } else {
            self.next_from_stack()
        }
    }
}

pub trait ExtendWithResponseFile: Iterator {
    fn extend_with_response_file(self) -> ParseResponseFiles<Self>
        where Self: Iterator<Item = String> + Sized
    {
        ParseResponseFiles {
            iter: self,
            response_file_stack: VecDeque::new(),
        }
    }
}

impl<I> ExtendWithResponseFile for I where I: Iterator {}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;
    use std::io::{Result, Write};
    use std::fs::File;
    use super::ExtendWithResponseFile;

    fn write_response_file(content: &String) -> Result<(String, TempDir)> {
        let tmp_dir = try!(TempDir::new("cpp-codegen"));
        let response_file_path = tmp_dir.path().join("interface.rsp");
        let mut response_file = try!(File::create(&response_file_path));
        try!(response_file.write(content.as_bytes()));
        Ok((response_file_path.to_string_lossy().into_owned().to_string(), tmp_dir))
    }

    #[test]
    fn should_leave_sequence_without_file_unaltered() {
        let s = vec!["-DFOO", "-DBAR", "-WBAZ"];
        assert_eq!(s.clone(),
                   s.into_iter().map(String::from).extend_with_response_file().collect::<Vec<_>>());
    }

    #[test]
    fn should_read_single_file() {
        let (path, _dir) = write_response_file(&"-x;c++;-std=c++14;-DFLAG".to_string()).unwrap();
        let s = vec!["-x", "c++", "-std=c++14", "-DFLAG"];
        let args = vec!["@".to_string() + &path];
        assert_eq!(s.clone(),
                   args.into_iter()
                       .extend_with_response_file()
                       .collect::<Vec<_>>());
    }

    #[test]
    fn should_read_multiple_files_with_intermixed_args() {
        let (path1, _dir1) = write_response_file(&"-x;c++".to_string()).unwrap();
        let (path2, _dir2) = write_response_file(&"-DFOO;-DBAR".to_string()).unwrap();
        let args = vec!["-DPRE".to_string(),
                        "@".to_string() + &path1,
                        "-DMID1".to_string(),
                        "-DMID2".to_string(),
                        "@".to_string() + &path2,
                        "-DPOST".to_string()];


        let s = vec!["-DPRE".to_string(),
                     "-x".to_string(),
                     "c++".to_string(),
                     "-DMID1".to_string(),
                     "-DMID2".to_string(),
                     "-DFOO".to_string(),
                     "-DBAR".to_string(),
                     "-DPOST".to_string()];

        assert_eq!(s.clone(), args.into_iter()
                   .extend_with_response_file()
                   .collect::<Vec<_>>())
    }
}
