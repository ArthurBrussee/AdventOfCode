use aoc_lib::AocSolution;
use itertools::Itertools;

pub struct Solution;

#[derive(Debug)]
enum FS {
    Dir { name: String, children: Vec<usize> },
    File { size: u32 },
}

fn calculate_directory_size(tree: &[FS], dir: usize) -> u32 {
    match &tree[dir] {
        FS::Dir { children, .. } => children
            .iter()
            .map(|&c| calculate_directory_size(tree, c))
            .sum(),
        FS::File { size, .. } => *size,
    }
}

fn get_or_insert_dir(cur_path: &[usize], tree: &mut Vec<FS>, dir_name: &str) -> usize {
    let folder_index = cur_path.last().and_then(|last| {
        let cur_node = &tree[*last];

        match cur_node {
            FS::Dir { children, .. } => children.iter().find(|&c| match &tree[*c] {
                FS::Dir { name, .. } => name == dir_name,
                FS::File { .. } => false,
            }),
            FS::File { .. } => unreachable!(),
        }
    });
    if let Some(ind) = folder_index {
        *ind
    } else {
        tree.push(FS::Dir {
            name: dir_name.to_string(),
            children: Vec::new(),
        });
        tree.len() - 1
    }
}

impl AocSolution for Solution {
    const DATE: (u32, u32) = (2022, 7);

    fn calc(input: &str) -> (u32, u32) {
        let mut tree: Vec<FS> = Vec::new();
        let mut cur_path: Vec<usize> = Vec::new();

        for line in input.lines() {
            if line.starts_with('$') {
                let commands = line.split(' ').collect::<Vec<_>>();

                if commands[1] == "cd" {
                    let folder_name = commands[2];

                    if folder_name == ".." {
                        cur_path.pop();
                    } else {
                        let ind = get_or_insert_dir(&cur_path, &mut tree, folder_name);
                        cur_path.push(ind);
                    }
                }
            } else {
                let index = tree.len();

                if line.starts_with("dir") {
                    let (_, folder_name) = line.split_once(' ').unwrap();
                    get_or_insert_dir(&cur_path, &mut tree, folder_name);
                } else {
                    let (size, _) = line.split_once(' ').unwrap();
                    tree.push(FS::File {
                        size: size.parse().unwrap(),
                    });
                }

                match tree[*cur_path.last().unwrap()] {
                    FS::Dir {
                        ref mut children, ..
                    } => children.push(index),
                    _ => unreachable!(),
                }
            }
        }

        let dir_sizes: Vec<_> = (0..tree.len())
            .filter(|n| matches!(tree[*n], FS::Dir { .. }))
            .map(|n| calculate_directory_size(&tree, n))
            .sorted()
            .collect();

        let p1 = dir_sizes.iter().filter(|&&n| n < 100000).sum();

        let free_space = 70000000 - dir_sizes.last().unwrap();

        let p2 = dir_sizes
            .iter()
            .find(|&x| free_space + x > 30000000)
            .copied()
            .unwrap();
        (p1, p2)
    }
}

#[test]
fn test() {
    Solution::test(95437, 24933642);
}
