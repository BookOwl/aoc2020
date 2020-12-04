use crate::Part;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Space {
    Open,
    Filled,
}

struct Map {
    data:   Vec<Vec<Space>>,
    width:  usize,
    height: usize,
}

impl Map {
    fn new(map: &str) -> Self {
        let data: Vec<_> = map.split('\n').filter_map(|l| {
            if l.trim() != "" {
                Some(l.chars().map(|c| {
                    match c {
                        '.' => Space::Open,
                        '#' => Space::Filled,
                        _ => panic!("Unexpected input {:?}", c),
                    }
                }).collect::<Vec<_>>())
            } else {
                None
            }
        }).collect();
        let height = data.len();
        let width = data[0].len();
        Map {
            data,
            width,
            height
        }
    }
    fn read(&self, x: usize, y: usize) -> Option<Space> {
        if y >= self.height {
            None
        } else {
            let idx = x % self.width;
            Some(self.data[y][idx])
        }
    }
}

pub fn run(part: Part, input_str: &str) {
    let input = Map::new(&input_str);
    match part {
        Part::First => part1(&input),
        Part::Second => part2(&input),
        Part::All => {
            part1(&input);
            part2(&input);
        },
    }
}

fn trees_encountered(map: &Map, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut num_trees = 0;
    while let Some(space) = map.read(x, y) {
        if space == Space::Filled {
            num_trees += 1;
        }
        x += dx;
        y += dy;
    }
    num_trees
}

fn part1(map: &Map) {
    let num_trees = trees_encountered(&map, 3, 1);
    println!("{}", num_trees);
}

fn part2(map: &Map) {
    let dirs = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let total = dirs.iter().map(|(x, y)| trees_encountered(&map, *x, *y) as u64).fold(1, |a, b| a*b);
    println!("{}", total);
}
