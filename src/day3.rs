use crate::err;
use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let mut file = match File::open("data/03/input.txt") {
        Ok(file) => file,
        Err(e) => panic!(e),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(e) => panic!(e),
        _ => {}
    };

    let map = match Map::parse(&contents) {
        Ok(map) => map,
        Err(e) => panic!(e),
    };

    let tree_counts = vec![
        map.encounters(
            Movement::with_moves(vec![Move::Right(1), Move::Down(1)]),
            MapElement::Tree,
        ),
        map.encounters(
            Movement::with_moves(vec![Move::Right(3), Move::Down(1)]),
            MapElement::Tree,
        ),
        map.encounters(
            Movement::with_moves(vec![Move::Right(5), Move::Down(1)]),
            MapElement::Tree,
        ),
        map.encounters(
            Movement::with_moves(vec![Move::Right(7), Move::Down(1)]),
            MapElement::Tree,
        ),
        map.encounters(
            Movement::with_moves(vec![Move::Right(1), Move::Down(2)]),
            MapElement::Tree,
        ),
    ];

    println!(
        "tree product: {}",
        tree_counts.iter().fold(1, |acc, c| acc * c)
    );
}

enum Move {
    Up(u32),
    Right(u32),
    Down(u32),
    Left(u32),
}

struct Movement {
    moves: Vec<Move>,
}

impl Movement {
    fn new() -> Movement {
        Movement { moves: Vec::new() }
    }
    fn with_moves(moves: Vec<Move>) -> Movement {
        Movement { moves: moves }
    }

    fn moves(&self) -> &Vec<Move> {
        &self.moves
    }
}

struct Traversal<'a> {
    map: &'a Map,
    movement: Movement,

    loc_x: u32,
    loc_y: u32,
}

impl<'a> Traversal<'a> {
    fn new(map: &'a Map, movement: Movement) -> Traversal<'a> {
        Traversal {
            map: map,
            movement: movement,

            loc_x: 0,
            loc_y: 0,
        }
    }

    fn x(&self) -> u32 {
        self.loc_x
    }
    fn y(&self) -> u32 {
        self.loc_y
    }
}

impl<'a> Iterator for Traversal<'a> {
    type Item = &'a MapElement;

    fn next(&mut self) -> Option<Self::Item> {
        let map_width = self.map.width() as i64;
        let map_height = self.map.height() as i64;

        let mut x = self.loc_x as i64;
        let mut y = self.loc_y as i64;
        for mov in self.movement.moves() {
            match mov {
                Move::Up(delta) => {
                    y = match y - (*delta as i64) {
                        next_y if next_y < 0 => return None,
                        next_y => next_y,
                    }
                }
                Move::Down(delta) => {
                    y = match y + (*delta as i64) {
                        next_y if next_y > map_height => return None,
                        next_y => next_y,
                    }
                }
                Move::Left(delta) => {
                    x = match x - (*delta as i64) {
                        next_x if next_x < 0 => map_width - next_x.abs(),
                        next_x => next_x,
                    }
                }
                Move::Right(delta) => {
                    x = match x + (*delta as i64) {
                        next_x if next_x >= map_width => next_x - map_width,
                        next_x => next_x,
                    }
                }
            };
        }

        self.loc_x = x as u32;
        self.loc_y = y as u32;

        let res = self.map.get(self.loc_x as usize, self.loc_y as usize);
        res
    }
}

#[derive(PartialEq, Debug)]
enum MapElement {
    Open,
    Tree,
}

impl MapElement {
    fn parse(d: char) -> Result<MapElement, err::ParseError> {
        match d {
            '.' => Ok(MapElement::Open),
            '#' => Ok(MapElement::Tree),
            _ => Err(err::ParseError::new("unknown element", &d.to_string())),
        }
    }
}

impl std::fmt::Display for MapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MapElement::Tree => "#",
                MapElement::Open => ".",
            }
        )
    }
}

struct Map {
    map: Vec<Vec<MapElement>>,
}

impl Map {
    fn parse(d: &str) -> Result<Map, err::ParseError> {
        let lines: Vec<&str> = d
            .split("\n")
            .into_iter()
            .map(|l| l.trim())
            .filter(|l| *l != "")
            .collect();
        let mut map: Vec<Vec<MapElement>> = Vec::new();
        for line in lines {
            let mut map_line = Vec::new();
            for c in line.chars() {
                map_line.push(MapElement::parse(c)?)
            }
            map.push(map_line);
        }

        Ok(Map { map: map })
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn width(&self) -> usize {
        match self.map.first() {
            Some(l) => l.len(),
            None => 0,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&MapElement> {
        match self.map.get(y) {
            Some(l) => match l.get(x) {
                Some(me) => Some(me),
                _ => None,
            },
            _ => None,
        }
    }

    fn traverse(&self, movement: Movement) -> Traversal {
        Traversal::new(self, movement)
    }

    fn encounters(&self, movement: Movement, element: MapElement) -> u32 {
        self.traverse(movement).filter(|e| **e == element).count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_map_11x11() -> Map {
        Map::parse(
            r"
        ..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#
        ",
        )
        .unwrap()
    }
    fn make_test_map_12x12() -> Map {
        Map::parse(
            r"
        ..##........
        #...#...#..#
        .#....#..#.#
        ..#.#...#.#.
        .#...##..#.#
        ..#.##......
        ##.#.#....#.
        .#........#.
        #.##...#...#
        #...##....##
        .#..#...#.#.
        ",
        )
        .unwrap()
    }

    mod traversal {
        use super::*;

        #[test]
        fn move_right_in_bounds() {
            let map = make_test_map_11x11();
            let mut t = Traversal::new(&map, Movement::with_moves(vec![Move::Right(3)]));

            let next = t.next();
            assert_eq!(t.x(), 3);
            assert_eq!(t.y(), 0);
            assert_eq!(next, Some(&MapElement::Tree));
        }
        #[test]
        fn move_down_in_bounds() {
            let map = make_test_map_11x11();
            let mut t = Traversal::new(&map, Movement::with_moves(vec![Move::Down(1)]));

            let next = t.next();
            assert_eq!(t.x(), 0);
            assert_eq!(t.y(), 1);
            assert_eq!(next, Some(&MapElement::Tree));
        }
        #[test]
        fn multiple_moves_in_bounds() {
            let map = make_test_map_11x11();
            let mut t = Traversal::new(
                &map,
                Movement::with_moves(vec![Move::Right(3), Move::Down(1)]),
            );

            assert_eq!(t.next(), Some(&MapElement::Open));
        }
        #[test]
        fn multiple_nexts_in_bounds() {
            let map = make_test_map_11x11();
            let mut t = Traversal::new(
                &map,
                Movement::with_moves(vec![Move::Right(3), Move::Down(1)]),
            );

            let next = t.next();
            assert_eq!(t.x(), 3);
            assert_eq!(t.y(), 1);
            assert_eq!(next, Some(&MapElement::Open));
            let next = t.next();
            assert_eq!(t.x(), 6);
            assert_eq!(t.y(), 2);
            assert_eq!(next, Some(&MapElement::Tree));
            let next = t.next();
            assert_eq!(t.x(), 9);
            assert_eq!(t.y(), 3);
            assert_eq!(next, Some(&MapElement::Open));
        }
        #[test]
        fn multiple_nexts_wrap_x_bounds() {
            let map = make_test_map_11x11();
            let mut t = Traversal::new(
                &map,
                Movement::with_moves(vec![Move::Right(3), Move::Down(1)]),
            );

            let next = t.next();
            assert_eq!(t.x(), 3);
            assert_eq!(t.y(), 1);
            assert_eq!(next, Some(&MapElement::Open));
            let next = t.next();
            assert_eq!(t.x(), 6);
            assert_eq!(t.y(), 2);
            assert_eq!(next, Some(&MapElement::Tree));
            let next = t.next();
            assert_eq!(t.x(), 9);
            assert_eq!(t.y(), 3);
            assert_eq!(next, Some(&MapElement::Open));
            let next = t.next();
            assert_eq!(t.x(), 1);
            assert_eq!(t.y(), 4);
            assert_eq!(next, Some(&MapElement::Tree));
        }
        #[test]
        fn wrap_exactly_on_column() {
            let map = make_test_map_12x12();
            let mut t = Traversal::new(
                &map,
                Movement::with_moves(vec![Move::Right(2), Move::Down(1)]),
            );

            assert_eq!(t.next(), Some(&MapElement::Open));
            assert_eq!(t.next(), Some(&MapElement::Open));
            assert_eq!(t.next(), Some(&MapElement::Open));
            assert_eq!(t.next(), Some(&MapElement::Open));
            assert_eq!(t.next(), Some(&MapElement::Open)); // 5
            assert_eq!(t.next(), Some(&MapElement::Tree));
            assert_eq!(t.next(), Some(&MapElement::Open));
            assert_eq!(t.next(), Some(&MapElement::Open));
            assert_eq!(t.next(), Some(&MapElement::Open));
            assert_eq!(t.next(), Some(&MapElement::Tree)); // 10
            assert_eq!(t.next(), None);
        }
    }
}
