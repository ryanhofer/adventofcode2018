const INPUT: &'static str = include_str!("input.txt");

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> u32 {
    let root = parse_tree(INPUT);
    root.recursive_metadata_sum()
}

fn part_two() -> u32 {
    let root = parse_tree(INPUT);
    root.recursive_value_sum()
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<u8>,
}

impl Node {
    fn metadata_sum(&self) -> u32 {
        self.metadata.iter().map(|&v| v as u32).sum()
    }

    fn recursive_metadata_sum(&self) -> u32 {
        let mut sum = self.metadata_sum();

        for child in &self.children {
            sum += child.recursive_metadata_sum();
        }

        sum
    }

    fn recursive_value_sum(&self) -> u32 {
        let mut sum: u32 = 0;

        if self.children.is_empty() {
            return self.metadata_sum();
        }

        for &index in &self.metadata {
            let index = index as usize;
            if let Some(child) = self.children.get(index - 1) {
                sum += child.recursive_value_sum();
            }
        }

        sum
    }
}

fn parse_tree(s: &str) -> Node {
    let mut input = s.trim().split(" ").filter_map(|s| s.parse().ok());
    let root = parse_node(&mut input);

    root
}

fn parse_node<I>(input: &mut I) -> Node
where
    I: Iterator<Item = u8>,
{
    let num_children = input.next().unwrap();
    let num_metadata = input.next().unwrap();
    let mut children = vec![];
    let mut metadata = vec![];

    for _ in 0..num_children {
        children.push(parse_node(input));
    }

    for _ in 0..num_metadata {
        let v = input.next().unwrap();
        metadata.push(v);
    }

    Node { children, metadata }
}
