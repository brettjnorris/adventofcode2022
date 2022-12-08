#[derive(Clone)]
enum FileType {
    File,
    Directory,
}

enum CommandType {
    CD,
    List,
    Unknown,
}

struct Command {
    command_type: CommandType,
    target: Option<String>,
}

impl Command {
    fn from_string(input: &str) -> Command {
        let parts = input.split_whitespace().collect::<Vec<&str>>();
        match parts[1] {
            "cd" => Command {
                command_type: CommandType::CD,
                target: Some(parts[2].to_string()),
            },
            "ls" => Command {
                command_type: CommandType::List,
                target: None,
            },
            _ => Command {
                command_type: CommandType::Unknown,
                target: None,
            },
        }
    }
}

#[derive(Clone)]
struct File {
    file_type: FileType,
    path: String,
    size: Option<u32>,
}

impl File {
    fn from_string(current_path: &str, input: &str) -> Option<File> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts[0] != "dir" {
            let path = format!("{}{}", current_path, parts[1]);
            Some(File {
                path,
                file_type: FileType::File,
                size: Some(parts[0].parse::<u32>().unwrap()),
            })
        } else {
            Some(File {
                path: File::parse_directory(current_path.to_string(), parts[1]),
                file_type: FileType::Directory,
                size: None,
            })
        }
    }

    fn parse_directory(current_path: String, target: &str) -> String {
        match target {
            ".." => {
                let parts: Vec<&str> = current_path.split("/").collect();
                parts[..(parts.len() - 1)].join("/")
            }
            _ => {
                format!("{}{}", current_path, target)
            }
        }
    }
}

fn calculate_directory_size(directory: File, files: Vec<File>) -> u32 {
    files
        .clone()
        .into_iter()
        .filter(|f| f.path.starts_with(&directory.path[..]))
        .map(|f| f.size.unwrap_or_else(|| 0))
        .sum::<u32>()
}

fn change_directory(current_directory: &str, command: Command) -> String {
    match command.target {
        Some(target) => {
            if target == ".." {
                let parts = current_directory.split("/").collect::<Vec<&str>>();
                let new_path = parts[..(parts.len() - 2)].join("/");
                format!("{}/", new_path)
            } else {
                if current_directory != "" {
                    format!("{}{}/", current_directory, target)
                } else {
                    format!("{}/", current_directory)
                }
            }
        }
        _ => current_directory.to_string(),
    }
}

fn parse_files(input: &str) -> Vec<File> {
    let mut current_directory = "".to_string();
    let root = File {
        path: "/".to_string(),
        file_type: FileType::Directory,
        size: None,
    };
    let mut files: Vec<File> = vec![root];

    for line in input.lines() {
        if line.starts_with("$") {
            let command = Command::from_string(line);
            if let CommandType::CD = command.command_type {
                let new_directory = change_directory(current_directory.as_str(), command);
                current_directory = new_directory;
            }
        } else {
            let file = File::from_string(current_directory.as_str(), line).unwrap();
            files.push(file);
        }
    }

    files
}

fn cumulative_directory_size_by_max_size(files: Vec<File>, max_size: u32) -> u32 {
    let inner_files = files.clone();
    files
        .into_iter()
        .filter_map(|f| {
            let is_directory = match f.file_type {
                FileType::Directory => true,
                _ => false,
            };

            match is_directory {
                false => None,
                true => {
                    let directory_size = calculate_directory_size(f, inner_files.clone());
                    match directory_size > max_size {
                        true => None,
                        false => Some(directory_size),
                    }
                }
            }
        })
        .sum::<u32>()
}

fn directories_by_size(files: Vec<File>) -> Vec<u32> {
    let inner_files = files.clone();

    let mut sizes = files
        .into_iter()
        .filter_map(|file| match file.file_type {
            FileType::Directory => Some(calculate_directory_size(file, inner_files.clone())),
            _ => None,
        })
        .collect::<Vec<u32>>();

    sizes.sort();

    sizes
}

pub fn part_one(input: &str) -> Option<u32> {
    let files = parse_files(input);
    Some(cumulative_directory_size_by_max_size(files, 100000))
}

pub fn part_two(input: &str) -> Option<u32> {
    let files = parse_files(input);
    let fs_size: u32 = 70000000;
    let required_free_space: u32 = 30000000;

    let sizes = directories_by_size(files);
    let used_space: &u32 = sizes.last().unwrap();
    let unused_space = fs_size - used_space;

    let directory = sizes
        .into_iter()
        .find(|size| unused_space + size > required_free_space)
        .unwrap();

    Some(directory)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }

    // #[test]
    // fn test_file_from_string_dir() {
    //     let input = "dir a";
    //     let file = File::from_string(input);
    //     assert_eq!(file.size, None);
    //     assert_eq!(file.name, "a".to_string());
    // }

    // #[test]
    // fn test_file_from_string_file() {
    //     let input = "14848514 b.txt";
    //     let file = File::from_string(input);
    //     assert_eq!(file.size, Some(14848514));
    //     assert_eq!(file.name, "b.txt".to_string());
    // }
}
