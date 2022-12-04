use aoc_lib::AocSolution;

struct Tree {
    nodes: Vec<Node>,
    root: usize,
}

#[derive(Clone, Copy)]
enum Node {
    Pair { left: usize, right: usize },
    Val(usize),
}

impl Tree {
    fn make_node(&mut self) -> usize {
        // Uninit sentiel
        self.nodes.push(Node::Val(123456));
        self.nodes.len() - 1
    }

    fn split(&mut self, node: usize) {
        match self.nodes[node] {
            Node::Val(v) => {
                let childl = self.make_node();
                let childr = self.make_node();

                self.nodes[childl] = Node::Val(v / 2);
                self.nodes[childr] = Node::Val((v + 1) / 2);

                self.nodes[node] = Node::Pair {
                    left: childl,
                    right: childr,
                }
            }
            _ => panic!("Can't split pair."),
        }
    }

    fn is_value_pair(&self, node: usize) -> bool {
        match self.nodes[node] {
            Node::Pair { left, right } => {
                matches!(
                    (self.nodes[left], self.nodes[right]),
                    (Node::Val(_), Node::Val(_))
                )
            }
            _ => false,
        }
    }

    fn visit_mut(&self, mut f: impl FnMut(usize, usize) -> bool) -> Option<usize> {
        let mut stack = vec![(self.root, 0)];

        loop {
            if let Some((v, depth)) = stack.pop() {
                if f(v, depth) {
                    break Some(v);
                }

                if let Node::Pair { left, right } = self.nodes[v] {
                    stack.push((right, depth + 1));
                    stack.push((left, depth + 1));
                }
            } else {
                break None;
            }
        }
    }

    fn visit(&self, f: impl Fn(usize, usize) -> bool) -> Option<usize> {
        self.visit_mut(f)
    }

    // TODO(arthurkb): Make parent node fixed.
    fn reduce(&mut self) {
        loop {
            if let Some(node) = self.visit(|node, depth| depth >= 4 && self.is_value_pair(node)) {
                self.explode(node);
                continue;
            }

            if let Some(node) =
                self.visit(|node, _| matches!(self.nodes[node], Node::Val(_val @ 10..)))
            {
                self.split(node);
                continue;
            }

            break;
        }
    }

    fn explode(&mut self, node: usize) {
        if let Node::Pair { left, right } = self.nodes[node] {
            let val_left = match self.nodes[left] {
                Node::Pair { left: _, right: _ } => unreachable!(),
                Node::Val(v) => v,
            };
            let val_right = match self.nodes[right] {
                Node::Pair { left: _, right: _ } => unreachable!(),
                Node::Val(v) => v,
            };

            let mut last_val = None;
            let mut right_found = false;
            let mut ln = None;

            let rn = self.visit_mut(|node, _| {
                if node == left {
                    ln = last_val;
                }

                if let Node::Val(_) = self.nodes[node] {
                    last_val = Some(node);

                    if right_found {
                        return true;
                    }
                }

                if node == right {
                    right_found = true;
                }

                false
            });

            if let Some(ln) = ln {
                if let Node::Val(v) = self.nodes[ln] {
                    self.nodes[ln] = Node::Val(v + val_left);
                }
            }

            if let Some(rn) = rn {
                if let Node::Val(v) = self.nodes[rn] {
                    self.nodes[rn] = Node::Val(v + val_right);
                }
            }

            // We just 'leak' the child nodes, no recycling lol.
            self.nodes[node] = Node::Val(0);
        } else {
            panic!("Can't explode value!")
        }
    }

    fn score(&self, node: usize) -> usize {
        match self.nodes[node] {
            Node::Pair { left, right } => 3 * self.score(left) + 2 * self.score(right),
            Node::Val(v) => v,
        }
    }

    fn add(&mut self, input: &str) {
        let new_tree = self.nodes.is_empty();

        let start_node = if !new_tree {
            let root = self.make_node();
            let rparent = self.make_node();

            self.nodes[root] = Node::Pair {
                left: self.root,
                right: rparent,
            };

            self.root = root;
            rparent
        } else {
            self.root = self.make_node();
            self.root
        };

        let mut stack = vec![start_node];

        for c in input.chars() {
            match c {
                '[' => {
                    let parent = stack.last().copied().unwrap();
                    let left_node = self.make_node();
                    let right_node = self.make_node();
                    self.nodes[parent] = Node::Pair {
                        left: left_node,
                        right: right_node,
                    };
                    stack.push(left_node);
                }
                ',' => {
                    stack.pop();
                    let parent = stack.last().copied().unwrap();

                    match self.nodes[parent] {
                        Node::Pair { left: _, right } => stack.push(right),
                        Node::Val(_) => unreachable!(),
                    }
                }
                ']' => {
                    let _ = stack.pop();
                }
                _ => {
                    let node = stack.last().copied().unwrap();
                    self.nodes[node] = Node::Val(c.to_digit(10).unwrap() as usize);
                }
            };
        }

        if !new_tree {
            self.reduce();
        }
    }
}

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2021, 18);

    fn calc(input: &str) -> (usize, usize) {
        let mut tree = Tree {
            nodes: Vec::new(),
            root: 0,
        };

        let all_lines: Vec<_> = input.lines().collect();

        for l in all_lines.iter() {
            tree.add(l);
        }

        let p2 = all_lines
            .clone()
            .iter()
            .flat_map(|l1| {
                all_lines.iter().map(|l2| {
                    let mut tree = Tree {
                        nodes: Vec::new(),
                        root: 0,
                    };

                    tree.add(l1);
                    tree.add(l2);
                    tree.score(tree.root)
                })
            })
            .max()
            .unwrap();

        (tree.score(tree.root), p2)
    }
}

#[test]
fn test() {
    Solution::test(4140, 3993);
}
