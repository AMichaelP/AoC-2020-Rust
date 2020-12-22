/*
--- Day 3: Toboggan Trajectory ---

With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.

Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For example:

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

These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:

..##.........##.........##.........##.........##.........##.......  --->
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->

You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).

The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by counting all the trees you would encounter for the slope right 3, down 1:

From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.

The locations you'd check in the above example are marked here with O where there was an open square and X where there was a tree:

..##.........##.........##.........##.........##.........##.......  --->
#..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........X.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...#X....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->

In this example, traversing the map using this slope would cause you to encounter 7 trees.

Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?

--- Part Two ---

Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.

Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:

    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked.)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.

In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.

What do you get if you multiply together the number of trees encountered on each of the listed slopes?

 */

use std::fs::read_to_string;
use std::path::PathBuf;

const INPUT_PATH: &str = "inputs/day_03/input";
const PRINT_SLOPE: bool = true;
const HIT_TARGET_CHAR: &str = "#";
const PART_ONE_PARAMS: (usize, usize) = (1, 3);
const PART_TWO_PARAMS: [(usize, usize); 5] = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

fn main() {
    let part_one_answer = get_hit_count_for_slope(PART_ONE_PARAMS.0, PART_ONE_PARAMS.1);

    let mut p2_buf = 1;
    let part_two_answer = PART_TWO_PARAMS
        .iter()
        .map(|x| get_hit_count_for_slope(x.0, x.1))
        .map(|x| {
            p2_buf *= x;
            p2_buf
        })
        .last()
        .unwrap();

    dbg!(part_one_answer);
    dbg!(part_two_answer);
}

fn get_hit_count_for_slope(row_step_by: usize, col_step_by: usize) -> usize {
    let s = read_to_string(PathBuf::from(INPUT_PATH)).unwrap();
    let rows: Vec<&str> = s.split("\r\n").collect();
    let mut tree_hit_count: usize = 0;

    for (i, &r) in rows.iter().step_by(row_step_by).enumerate() {
        let step = match i {
            0 => 0,
            _ => (i * col_step_by) % r.len(),
        };

        let hit_result_char = match &r[step..step + 1] {
            HIT_TARGET_CHAR => {
                tree_hit_count += 1;
                "X"
            }
            _ => "O",
        };

        let end = &r[step + 1..];

        if PRINT_SLOPE {
            match step {
                0 => println!("|{}|{}", hit_result_char, end),
                _ => println!("{}|{}|{}", &r[..step], hit_result_char, end),
            };
        }
    }
    tree_hit_count
}
