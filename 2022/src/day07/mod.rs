#[derive(Debug)]
struct File {
    _name: String,
    size: u64,
}

#[derive(Debug)]
struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<usize>,
    parent: usize,
}

const FILESYSTEM_ROOT_ID: usize = 0;
const FILESYSTEM_SIZE: u64 = 70_000_000;
const SIZE_NEEDED_FOR_UPDATE: u64 = 30_000_000;

#[derive(Debug)]
struct Filesystem {
    dirs: Vec<Dir>,
}

impl Filesystem {
    fn new() -> Self {
        let mut new_fs = Filesystem { dirs: vec![] };
        new_fs.dirs.push(Dir::new("/", FILESYSTEM_ROOT_ID));
        new_fs
    }
    fn mkdir<S: Into<String>>(&mut self, name: S, parent: usize) -> usize {
        let new_dir_id = self.dirs.len();
        self.dirs.push(Dir::new(name, parent));
        self.get_dir_mut(parent).dirs.push(new_dir_id);
        new_dir_id
    }
    fn get_dir(&self, id: usize) -> &Dir {
        &self.dirs[id]
    }
    fn get_dir_mut(&mut self, id: usize) -> &mut Dir {
        &mut self.dirs[id]
    }
}

impl Dir {
    fn new<S: Into<String>>(name: S, parent: usize) -> Self {
        Dir { name: name.into(), files: Vec::new(), dirs: Vec::new(), parent }
    }
    fn total_size(&self, fs: &Filesystem) -> u64 {
        self.files.iter().map(|f| f.size).sum::<u64>()
            + self
                .dirs
                .iter()
                .map(|d| fs.get_dir(*d).total_size(fs))
                .sum::<u64>()
    }
    fn child<'a, S: AsRef<str>>(&'a self, name: S, fs: &'a Filesystem) -> Option<usize> {
        self.dirs
            .iter()
            .copied()
            .find(|id| fs.get_dir(*id).name.as_str() == name.as_ref())
    }
    fn mkfile<S: Into<String>>(&mut self, name: S, size: u64) {
        self.files.push(File { _name: name.into(), size })
    }
}

fn parse(s: &str) -> Filesystem {
    let mut fs = Filesystem::new();
    let mut cwd = FILESYSTEM_ROOT_ID;
    for l in s.lines() {
        let mut tokens = l.split_whitespace();
        match tokens.next() {
            None => continue, // Empty line - go to next line
            Some("$") => {
                // Command
                match tokens.next() {
                    Some("ls") => continue,
                    Some("cd") => match tokens.next() {
                        Some("/") => cwd = FILESYSTEM_ROOT_ID,
                        Some("..") => cwd = fs.get_dir(cwd).parent,
                        Some(dir_name) => {
                            cwd = fs
                                .get_dir(cwd)
                                .child(dir_name, &fs)
                                .unwrap_or_else(|| panic!("Found subdirectory {dir_name}"));
                        }
                        None => panic!("Expect directory name in {l}"),
                    },
                    Some(_) => panic!("Bad command in {l}"),
                    None => panic!("Expect command in {l}"),
                }
            }
            Some("dir") => {
                // Directory entry
                let child_name = tokens
                    .next()
                    .unwrap_or_else(|| panic!("dir has a name in {l}"));
                fs.mkdir(child_name, cwd);
            }
            Some(maybe_size) => {
                // File entry
                match maybe_size.parse::<u64>() {
                    Ok(size) => {
                        fs.get_dir_mut(cwd).mkfile(
                            tokens
                                .next()
                                .unwrap_or_else(|| panic!("file has a name in {l}")),
                            size,
                        );
                    }
                    _ => panic!("Bad input line {l}"),
                }
            }
        }
    }
    fs
}

fn part1_evaluate(fs: &Filesystem) -> u64 {
    fs.dirs
        .iter()
        .filter_map(|d| {
            let this_size = d.total_size(fs);
            (this_size <= 100_000).then_some(this_size)
        })
        .sum::<u64>()
}

fn part2_evaluate(fs: &Filesystem) -> u64 {
    let space_used = fs.get_dir(FILESYSTEM_ROOT_ID).total_size(fs);
    let space_available = FILESYSTEM_SIZE - space_used;
    let space_to_free = SIZE_NEEDED_FOR_UPDATE.max(space_available) - space_available;
    fs.dirs
        .iter()
        .filter_map(|d| {
            let this_size = d.total_size(fs);
            (this_size >= space_to_free).then_some(this_size)
        })
        .min()
        .expect("Minimum directory size found")
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
#[cfg(test)]
#[test]
fn test_part1() {
    let fs = parse(TEST_INPUT_STRING);
    assert_eq!(part1_evaluate(&fs), 95437);
}

#[test]
fn test_part2() {
    let fs = parse(TEST_INPUT_STRING);
    assert_eq!(part2_evaluate(&fs), 24933642);
}

pub fn run() -> Option<(u64, bool, u64, bool)> {
    let input_string = include_str!("./input.txt");
    let fs = parse(input_string);
    let part1_answer = part1_evaluate(&fs);
    let part2_answer = part2_evaluate(&fs);
    Some((part1_answer, part1_answer == 1648397, part2_answer, part2_answer == 1815525))
}
