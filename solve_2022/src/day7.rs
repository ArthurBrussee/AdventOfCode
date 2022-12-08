use aoc_lib::AocSolution;
use itertools::Itertools;

pub struct Solution;

enum FS {
    Dir { name: String, children: Vec<usize> },
    File { size: u32 },
}

fn node_size(tree: &[FS], node: &FS) -> u32 {
    match node {
        FS::Dir { children, .. } => children.iter().map(|&c| node_size(tree, &tree[c])).sum(),
        FS::File { size, .. } => *size,
    }
}

fn get_or_insert_dir(cur_path: &[usize], tree: &mut Vec<FS>, dir_name: &str) -> usize {
    if let Some(last) = cur_path.last() {
        if let FS::Dir { children, .. } = &tree[*last] {
            if let Some(&ind) = children
                .iter()
                .find(|&c| matches!(&tree[*c], FS::Dir { name, .. } if name == dir_name))
            {
                return ind;
            }
        }
    }

    tree.push(FS::Dir {
        name: dir_name.to_string(),
        children: Vec::new(),
    });
    tree.len() - 1
}

impl AocSolution for Solution {
    const DATE: (u32, u32) = (2022, 7);

    fn calc(input: &str) -> (u32, u32) {
        let mut tree: Vec<FS> = Vec::new();
        let mut cur_path: Vec<usize> = Vec::new();

        for line in input.lines() {
            if line.starts_with("$ cd") {
                let (_, _, folder_name) = line.split(' ').collect_tuple().unwrap();

                if folder_name == ".." {
                    cur_path.pop();
                } else {
                    let ind = get_or_insert_dir(&cur_path, &mut tree, folder_name);
                    cur_path.push(ind);
                }
            } else if line.starts_with("$ ls") {
                continue;
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

        let dir_sizes: Vec<_> = tree
            .iter()
            .filter(|n| matches!(n, FS::Dir { .. }))
            .map(|n| node_size(&tree, n))
            .sorted()
            .collect();

        let p1 = dir_sizes.iter().filter(|&&n| n < 100000).sum();

        let free_space = 70000000 - dir_sizes.last().unwrap();

        let p2 = dir_sizes
            .iter()
            .copied()
            .find(|x| free_space + x > 30000000)
            .unwrap();
        (p1, p2)
    }
}

#[test]
fn test() {
    Solution::test(95437, 24933642);
}
