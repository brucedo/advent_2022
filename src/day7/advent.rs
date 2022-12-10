use std::{cell::RefCell, rc::Rc};


pub struct File
{
    pub name: String,
    pub size: usize,
}


pub struct Directory
{
    name: String,
    children: Vec<usize>,
    parent: Option<usize>,
    files: Vec<File>,
    size: usize
}

impl Directory
{
    pub fn new(name: String) -> Directory
    {
        Directory {name, children: Vec::new(), parent: None, files: Vec::new(), size: 0 }
    }

    pub fn size(&self) -> usize
    {
        return self.size;
    }

    pub fn name(&self) -> &str
    {
        return self.name.as_str();
    }

    // fn recalculate_size(&self)
    // {
    //     let mut size = 0;
    //     for child in &self.children
    //     {
    //         size += child.borrow().size(); 
    //     }

    //     for file in &self.files
    //     {
    //         size += file.size;
    //     }

    //     if self.parent.is_some()
    //     {
    //         self.parent.as_ref().unwrap().as_ref().borrow().recalculate_size();
    //     }
    // }

    pub fn add_file(&mut self, file: File)
    {

        self.size += file.size;
        self.files.push(file);

        // self.recalculate_size();
    }

}

pub struct Filesystem
{
    directories: Vec<Rc<RefCell<Directory>>>,
    cwd: Rc<RefCell<Directory>>,
    cwd_index: usize
}

impl Filesystem 
{
    pub fn new() -> Filesystem
    {
        let root = Rc::new(RefCell::new(Directory::new(String::from("/"))));
        let mut dirs = Vec::new();
        dirs.push(root.clone());
        Filesystem { directories: dirs, cwd: root , cwd_index: 0}
    }

    pub fn is_dir(&self, name: &str) -> bool
    {
        if let Some(cwd) = self.directories.get(self.cwd_index)
        {
            for child_index in &cwd.as_ref().borrow().children
            {
                if let Some(dir) = self.directories.get(*child_index)
                {
                    if dir.as_ref().borrow().name == name { return  true; }
                }
            }
        }

        false
    }

    pub fn create_dir(&mut self, mut child: Directory)
    {
        child.parent = Some(self.cwd_index);
        self.directories.push(Rc::new(RefCell::new(child)));

        if let Some(cwd) = self.directories.get(self.cwd_index)
        {
            cwd.borrow_mut().children.push(self.directories.len() - 1);
        }
    }

    pub fn update_sizes(&self)
    {
        self.recalculate_size(self.cwd_index);

        
    }

    fn recalculate_size(&self, index: usize)
    {
        let mut new_size: usize = 0;
        if let Some(dir) = self.directories.get(index)
        {
            for file in &dir.as_ref().borrow().files
            {
                new_size += file.size;
            }

            for child_index in &dir.as_ref().borrow().children
            {
                new_size += self.directories.get(*child_index).unwrap().as_ref().borrow().size;
            }

            dir.borrow_mut().size = new_size;
        }
    }

    pub fn cwd(&mut self, to: &str)
    {
        match to 
        {
            "/" => 
            {
                self.cwd_index = 0;
                // self.cwd = self.directories.get(0).unwrap().clone();
            },
            ".." => 
            {
                if let Some(cwd) = self.directories.get(self.cwd_index)
                {
                    if let Some(parent_index) = cwd.as_ref().borrow().parent
                    {
                        // temp = self.directories.get(parent_index).unwrap_or(&self.directories.get(0).unwrap().clone()).clone()
                        self.cwd_index = parent_index;
                    }
                    else
                    {
                        return;
                    }                    
                }
            },
            _ => 
            {
                if let Some(cwd) = self.directories.get(self.cwd_index)
                {
                    for child_index in &cwd.as_ref().borrow().children
                    {
                        if let Some(dir) = self.directories.get(*child_index)
                        {
                            if dir.as_ref().borrow().name == to
                            {
                                self.cwd_index = *child_index;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_cwd(&self) -> Rc<RefCell<Directory>>
    {
        self.cwd.clone()
    }
}

pub struct Shell
{

}

impl Shell
{
    pub fn line_processor(line: &str) -> LineType
    {
        if line.trim().is_empty()
        {
            return LineType::Noop;
        }

        let mut split = line.split(" ");

        if line.starts_with("$")
        {
            split.next(); // discard prompt
            let command = split.next().unwrap();
            let dir_name = split.next();
            
            match command
            {
                "cd" => 
                {
                    return LineType::ChangeDirectory(String::from(dir_name.unwrap()));
                },
                "ls" => 
                {
                    return LineType::List
                },
                _ => 
                {
                    return LineType::UnknownToken
                }
            }
        }
        else
        {
            let part_a = split.next().unwrap();
            let part_b = split.next().unwrap();

            if part_a == "dir"
            {
                return LineType::DirectoryEntry(String::from(part_b));
            }
            else
            {
                if let Ok(file_size) = usize::from_str_radix(part_a, 10)
                {
                    return LineType::FileEntry((String::from(part_b), file_size));
                }
            }
        }

        return LineType::UnknownToken;
    }
}

#[derive(Debug)]
pub enum LineType 
{
    ChangeDirectory(String),
    List,
    DirectoryEntry(String),
    FileEntry((String, usize)),
    Noop,
    UnknownToken,
}

#[cfg(test)]

pub mod test
{
    use crate::day7::advent::{LineType, Shell, File, Directory};

    use super::Filesystem;

    #[test]
    pub fn when_a_filesystem_adds_a_directory_it_should_be_the_child_of_the_current_working_directory()
    {
        let mut fs = Filesystem::new();

        let a = Directory::new(String::from("a"));

        assert!(!fs.is_dir("a"));
        fs.create_dir(a);
        assert!(fs.is_dir("a"));
    }

    #[test]
    pub fn when_a_filesystem_is_asked_to_change_to_the_dot_dot_directory_it_moves_cwd_to_the_current_dirs_parent()
    {
        let mut fs = Filesystem::new();

        let a = Directory::new(String::from("a"));

        fs.create_dir(a);
        fs.cwd("a");
        fs.cwd("..");

        assert_eq!(fs.get_cwd().borrow().name(), "/");
    }

    #[test]
    pub fn when_a_filesystem_is_asked_to_change_to_the_forward_slash_directory_it_moves_cwd_to_the_root()
    {
        let mut fs = Filesystem::new();

        let a = Directory::new(String::from("a"));
        let b = Directory::new(String::from("b"));

        fs.create_dir(a);
        fs.cwd("a");
        fs.create_dir(b);
        fs.cwd("b");

        fs.cwd("/");

        assert_eq!(fs.get_cwd().borrow().name(), "/");
    }

    #[test]
    pub fn when_a_filesystem_is_updated_the_cwd_and_all_parent_directories_should_update_their_size()
    {
        let mut fs = Filesystem::new();
        
        let a = Directory::new(String::from("a"));
        let mut b = Directory::new(String::from("b"));

        let file = File {name: String::from("filename"), size: 4321};

        let root_size = fs.get_cwd().borrow().size();
        let a_size = a.size();
        let b_size = b.size();

        fs.create_dir(a);
        fs.cwd("a");
        b.add_file(file);
        fs.create_dir(b);

        fs.cwd("b");

        fs.update_sizes();

        assert_eq!(b_size + 4321, fs.get_cwd().borrow().size());
        fs.cwd("..");
        assert_eq!(a_size + 4321, fs.get_cwd().borrow().size());
        fs.cwd("..");
        assert_eq!(root_size + 4321, fs.get_cwd().borrow().size());
    }

    #[test]
    pub fn when_add_file_is_called_a_directory_should_append_the_file_to_its_list_and_size_should_update()
    {
        let mut dir = Directory::new(String::from("a"));

        let file = File{name: String::from("file"), size: 4321};

        let old_size = dir.size();
        dir.add_file(file);

        assert_eq!(old_size + 4321, dir.size());
    }

    #[test]
    pub fn when_line_processor_receives_a_cd_command_it_should_return_command_type()
    {
        let line = "$ cd /";

        match Shell::line_processor(line)
        {
            LineType::ChangeDirectory(dir_name) => 
            {
                assert_eq!(dir_name, String::from("/"));
            },
            _=> {panic!("The wrong LineType was returned.  Should have received ChangeDirectory token.")}
        }
        // assert_eq!(LineType::ChangeDirectory, );
    }

    #[test]
    pub fn when_line_processor_receives_an_ls_command_it_should_return_ls_command_type()
    {
        let line = "$ ls";

        match Shell::line_processor(line)
        {
            LineType::List => {},
            _ => {panic!("The wrong LineType was returned.  Should have received ChangeDirectory token.")}
        }
    }

    #[test]
    pub fn when_line_processor_receives_a_directory_definition_line_it_should_return_directory_type()
    {
        let line = "dir anything";

        match Shell::line_processor(line)
        {
            LineType::DirectoryEntry(dir_name) =>
            {
                assert_eq!(dir_name, String::from("anything"));
            },
            _ => {panic!("The wrong LineType was returned.  Should have received ChangeDirectory token.")}
        }
    }

    #[test]
    pub fn when_line_processor_receives_a_file_definition_line_it_should_return_file_entry_type()
    {
        let line = "124124155 alpha.omega";

        match Shell::line_processor(line)
        {
            LineType::FileEntry((file_name, file_size)) => {},
            _ => {panic!("The wrong LineType was returned.  Should have received ChangeDirectory token.")}
        }
    }

    #[test]
    pub fn when_line_processor_receives_a_blank_line_noop_type_is_returned()
    {
    
        let line = "";
        match Shell::line_processor(line)
        {
            LineType::Noop => {},
            _ => {panic!("The wrong LineType was returned.  Should have received ChangeDirectory token.")}
        }

        let line = "                        ";
        match Shell::line_processor(line)
        {
            LineType::Noop => {},
            _ => {panic!("The wrong LineType was returned.  Should have received ChangeDirectory token.")}
        }
    }

    #[test]
    pub fn when_unknown_text_is_passed_to_line_processor_an_unknown_token_type_is_returned()
    {
        let line = "doir 11515151111ddd.txt";

        match Shell::line_processor(line)
        {
            LineType::UnknownToken => {},
            _ => {panic!("The wrong LineType was returned.  Should have received ChangeDirectory token.")}
        }
    }

}