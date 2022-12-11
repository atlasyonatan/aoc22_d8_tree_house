use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(path).unwrap();
    let grid = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|s| s.chars().map(|c| c as u8 - '0' as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let height = grid.len();
    let width = grid[0].len();

    //part 1
    let outside = 2 * (width + height - 2);
    let inside = (1..width - 1)
        .flat_map(|x| (1..height - 1).map(move |y| (x, y)))
        .filter(|p| is_visible(&grid, p))
        .count();

    println!("part 1: {}", outside + inside);

    //part 2
    let max_view = (1..width - 1)
        .flat_map(|x| (1..height - 1).map(move |y| (x, y)))
        .map(|p| viewing_distances(&grid, &p))
        .map(|views| views.iter().fold(1, |a, i| a * i))
        .max()
        .unwrap();

    println!("part 2: {}", max_view);
}

fn is_visible<H: PartialOrd>(grid: &Vec<Vec<H>>, point: &(usize, usize)) -> bool {
    let (x, y) = *point;
    let h = &grid[x][y];

    let x_len = grid.len();
    let y_len = grid[0].len();

    [0..y, y + 1..y_len]
        .into_iter()
        .any(|r| r.map(|y| &grid[x][y]).all(|v| *v < *h))
        || [0..x, x + 1..x_len]
            .into_iter()
            .any(|r| r.map(|x| &grid[x][y]).all(|v| *v < *h))
}

fn viewing_distances<H: PartialOrd>(grid: &Vec<Vec<H>>, point: &(usize, usize)) -> [usize; 4] {
    let (x, y) = *point;
    let h = &grid[x][y];

    let x_len = grid.len();
    let y_len = grid[0].len();

    let mut a = [0; 4];
    let mut i = (0..y).rev().map(|y: usize| &grid[x][y]);
    a[0] = i.by_ref().take_while(|v| **v < *h).count()
        + match i.next() {
            Some(_) => 1,
            None => 0,
        };
    let mut i = (y + 1..y_len).map(|y: usize| &grid[x][y]);
    a[1] = i.by_ref().take_while(|v| **v < *h).count()
        + match i.next() {
            Some(_) => 1,
            None => 0,
        };
    let mut i = (0..x).rev().map(|x: usize| &grid[x][y]);
    a[2] = i.by_ref().take_while(|v| **v < *h).count()
        + match i.next() {
            Some(_) => 1,
            None => 0,
        };
    let mut i = (x + 1..x_len).map(|x: usize| &grid[x][y]);
    a[3] = i.by_ref().take_while(|v| **v < *h).count()
        + match i.next() {
            Some(_) => 1,
            None => 0,
        };
    return a;
}

// fn viewing_distance<H: PartialOrd>(
//     grid: &Vec<Vec<H>>,
//     point: &(usize, usize),
//     step: &(isize, isize),
// ) -> bool {
//     let (x, y) = *point;
//     let h = &grid[x][y];
//     let (x, y) = (x as isize, y as isize);

//     let x_len = grid.len();
//     let y_len = grid[0].len();
//     let x_range = 0..x_len as isize;
//     let y_range = 0..y_len as isize;

//     let (dx, dy) = *step;
//     let mut scale = 0;
//     loop {
//         scale += 1;
//         let x = x + dx * scale;
//         let y = y + dy * scale;
//         if !x_range.contains(&x) || !y_range.contains(&y) {
//             break false;
//         }
//     }

//     // [0..y, y + 1..y_len]
//     //     .into_iter()
//     //     .any(|r| r.map(|y| &grid[x][y]).all(|v| *v < *h))
//     //     || [0..x, x + 1..x_len]
//     //         .into_iter()
//     //         .any(|r| r.map(|x| &grid[x][y]).all(|v| *v < *h))
// }
