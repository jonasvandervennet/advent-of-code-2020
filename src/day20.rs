use crate::util::{print_part_1, print_part_2};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(PartialEq, Debug, Clone, Copy)]
enum TileType {
    DOT,
    HASH,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Tile {
    grid: [[TileType; 10]; 10],
    id: usize,
    neighbours: [usize; 4],
}

impl Tile {
    fn empty() -> Self {
        Self {
            grid: [[TileType::DOT; 10]; 10],
            id: 0,
            neighbours: [0; 4],
        }
    }
    fn from_lines(lines: &str) -> Self {
        let mut grid = [[TileType::DOT; 10]; 10];
        let mut lines = lines.lines();
        let id = lines.next().unwrap().split(" ").collect::<Vec<&str>>()[1]
            .split(":")
            .collect::<Vec<&str>>()[0]
            .parse::<usize>()
            .unwrap();
        for (i, line) in lines.enumerate() {
            for (j, c) in line.chars().enumerate() {
                grid[i][j] = match c {
                    '.' => TileType::DOT,
                    '#' => TileType::HASH,
                    _ => unreachable!(),
                };
            }
        }
        Self {
            grid: grid,
            id: id,
            neighbours: [0; 4],
        }
    }

    // (bool, bool, usize, usize) == (has match, was flipped, index self, index other)
    fn can_neighbour(&self, other: &Tile) -> (bool, bool, usize, usize) {
        for (j, o_edge) in other.get_edges(false).iter().enumerate() {
            for (i, edge) in self.get_edges(false).iter().enumerate() {
                if edge == o_edge {
                    return (true, false, i, j);
                }
            }

            for (i, edge) in self.get_edges(true).iter().enumerate() {
                if edge == o_edge {
                    return (true, true, i, j);
                }
            }
        }
        (false, false, 0, 0)
    }

    fn get_edges(&self, flipped: bool) -> Vec<Vec<TileType>> {
        let mut edges = vec![vec![], vec![], vec![], vec![]];

        for i in 0..self.grid.len() {
            edges[0].push(self.grid[0][i]); // top
            edges[1].push(self.grid[self.grid.len() - 1][i]); // bot
            edges[2].push(self.grid[i][0]); // left
            edges[3].push(self.grid[i][self.grid.len() - 1]); // right
        }
        if flipped {
            edges[0].reverse();
            edges[1].reverse();
            edges[2].reverse();
            edges[3].reverse();
        }
        edges
    }

    fn flip_horizontal_axis(&mut self) {
        let mut grid = self.grid.clone();
        for i in 0..self.grid.len() {
            for j in 0..self.grid.len() {
                grid[i][j] = self.grid[self.grid.len() - (i + 1)][j];
            }
        }
        self.grid = grid;
    }

    fn flip_vertical_axis(&mut self) {
        let mut grid = self.grid.clone();
        for i in 0..grid.len() {
            grid[i].reverse();
        }
        self.grid = grid;
    }

    // rotates grid 90° to the left
    fn rotate(&mut self) {
        let mut grid = self.grid.clone();
        for i in 0..self.grid.len() {
            for j in 0..self.grid.len() {
                grid[i][j] = self.grid[j][self.grid.len() - (i + 1)];
            }
        }
        self.grid = grid;
    }
}

fn rotate_grid(grid: &mut Vec<Vec<&TileType>>) {
    let old_grid = grid.clone();  // use old values when calculating new ones
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            grid[i][j] = old_grid[j][grid.len() - (i + 1)];
        }
    }
}

fn find_tile(tiles: &Vec<Tile>, tile_id: usize) -> &Tile {
    for tile in tiles {
        if tile.id == tile_id {
            return tile;
        }
    }
    println!("Asked to find tile: {}", tile_id);
    unreachable!();
}

fn edge_unique(tiles: &Vec<Tile>, edge: &Vec<TileType>, owner: usize) -> bool {
    for tile in tiles {
        if tile.id == owner {
            continue;
        }
        for tile_edge in tile.get_edges(false) {
            if tile_edge == *edge {
                return false;
            }
        }
        for tile_edge in tile.get_edges(true) {
            if tile_edge == *edge {
                return false;
            }
        }
    }
    true
}

fn rearrange_tiles(input: &str, part: usize) -> usize {
    // have to find the corner pieces,
    // a.k.a. the ones with only two possible neighbours

    let mut tiles = input
        .split("\r\n\r\n")
        .map(|tile_lines| Tile::from_lines(tile_lines))
        .collect::<Vec<Tile>>();

    let mut corners = Vec::new();
    let mut neighbours: HashMap<usize, Vec<usize>> = HashMap::new();

    for tile1 in tiles.iter() {
        for tile2 in tiles.iter() {
            if tile1.id == tile2.id {
                continue;
            }
            let neighbour_match = tile1.can_neighbour(tile2);
            if neighbour_match.0 {
                neighbours.entry(tile1.id).or_default().push(tile2.id);
            }
        }
    }

    for tile in tiles.iter_mut() {
        let mut i = 0;
        for &id in neighbours[&tile.id].iter() {
            tile.neighbours[i] = id;
            i += 1;
        }
        if i == 2 {
            corners.push(tile.id);
        }
    }
    if part == 1 {
        let mut corners_id_mult = 1;
        for tile_id in corners.iter() {
            corners_id_mult *= tile_id;
        }
        return corners_id_mult;
    }

    /* IDEA:
     * - pick some corner piece
     * - it has to have 2 unique edges
     * - orient such that they are top and left edge
     */
    let tile_grid_size = (tiles.len() as f64).sqrt() as usize;
    let mut oriented_tiles = vec![vec![Tile::empty(); tile_grid_size]; tile_grid_size];
    let mut all_fixed_tiles = Vec::new();
    let mut top_left = find_tile(&tiles, corners[0]).to_owned();

    loop {
        let mut i = 0;
        let mut edge_orientation = [0; 2];
        for (side, corner_edge) in top_left.get_edges(false).iter().enumerate() {
            if edge_unique(&tiles, &corner_edge, top_left.id) {
                edge_orientation[i] = side;
                i += 1;
            }
        }
        if [[0, 2], [2, 0]].contains(&edge_orientation) {
            break;
        }
        top_left.rotate();
    }
    oriented_tiles[0][0] = top_left;
    all_fixed_tiles.push(top_left);

    let mut tiles_todo: Vec<(usize, usize, (usize, usize))> = Vec::new();
    for &n in &top_left.neighbours {
        if n != 0 {
            tiles_todo.push((n, top_left.id, (0, 0)));
        }
    }

    loop {
        // exit condition
        if tiles_todo.len() == 0 {
            break;
        }

        // main loop
        let (tile_id, parent_id, (x, y)) = tiles_todo.pop().unwrap();
        let mut tile = find_tile(&tiles, tile_id).to_owned();
        let parent: &Tile = find_tile(&all_fixed_tiles, parent_id);

        loop {
            let match_description = tile.can_neighbour(parent);
            assert_eq!(match_description.0, true);

            if match_description.1 {
                // some flip needs to occur
                if [2, 3].contains(&match_description.2) {
                    // matched on a side, so flip horizontal
                    tile.flip_horizontal_axis();
                } else {
                    tile.flip_vertical_axis();
                }
            } else {
                // no flips occured
                if [[2, 3], [3, 2], [1, 0], [0, 1]]
                    .contains(&[match_description.2, match_description.3])
                {
                    // valid match
                    break;
                } else {
                    // still invalid, give it a turn
                    tile.rotate();
                }
            }
        }

        // Structure tiles back together in one big grid
        let match_description = tile.can_neighbour(parent);
        let new_coords;
        match match_description.3 {
            // where is this tile located compared to the parent
            0 => {
                // top
                new_coords = (x - 1, y);
            }
            1 => {
                // bottom
                new_coords = (x + 1, y);
            }
            2 => {
                // left
                new_coords = (x, y - 1);
            }
            3 => {
                // right
                new_coords = (x, y + 1);
            }
            _ => unreachable!(),
        }
        oriented_tiles[new_coords.0][new_coords.1] = tile;
        all_fixed_tiles.push(tile);

        // Add new tiles to the queue
        'outer: for &to_check in &tile.neighbours {
            for seen_tile in all_fixed_tiles.iter() {
                if to_check == 0 || to_check == seen_tile.id {
                    continue 'outer;
                }
            }
            tiles_todo.push((to_check, tile.id, new_coords));
        }
    }

    // Collect all tiles
    let mut full_grid: Vec<Vec<TileType>> =
        vec![vec![TileType::DOT; oriented_tiles.len() * 10]; oriented_tiles.len() * 10];
    let mut full_grid_ids: Vec<Vec<usize>> =
        vec![vec![0; oriented_tiles.len()]; oriented_tiles.len()];
    for (i, tile_row) in oriented_tiles.iter().enumerate() {
        for (j, tile) in tile_row.iter().enumerate() {
            for (k, grid_row) in tile.grid.iter().enumerate() {
                for (l, point) in grid_row.iter().enumerate() {
                    full_grid[k + 10 * i][l + 10 * j] = *point;
                    full_grid_ids[i][j] = tile.id;
                }
            }
        }
    }

    // Remove border tiles
    let mut borderless_grid = Vec::new();
    for (i, row) in full_grid.iter().enumerate() {
        if i % 10 == 0 || (i + 1) % 10 == 0 {
            continue;
        }
        let mut new_row = vec![];
        for (j, tile) in row.iter().enumerate() {
            if j % 10 == 0 || (j + 1) % 10 == 0 {
                continue;
            }
            new_row.push(tile);
        }
        borderless_grid.push(new_row);
    }

    let monster = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
        [1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1],
        [0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0],
    ];

    let monster_size = 15; // monster occupies 15 HASHes
    let mut num_monsters = 0;
    let grid_size = borderless_grid.len();

    for _ in 0..4 {
        // 4 rotations, might have to add flipping too if solution is not reached
        for (i, row) in borderless_grid[..grid_size - 3].iter().enumerate() {
            'monster_pos: for (j, _) in row[..grid_size - 20].iter().enumerate() {
                for m_i in 0..3 {
                    for m_j in 0..20 {
                        if monster[m_i][m_j] == 1
                            && *borderless_grid[i + m_i][j + m_j] != TileType::HASH
                        {
                            continue 'monster_pos;
                        }
                    }
                }
                num_monsters += 1;
            }
        }

        if num_monsters > 0 {
            break;
        }
        rotate_grid(&mut borderless_grid);
    }

    borderless_grid
        .iter()
        .flatten()
        .filter(|&&&x| x == TileType::HASH)
        .count()
        - monster_size * num_monsters
}

pub fn main() {
    let input = read_to_string("inputs/day20.txt").expect("Input not found..");

    // PART 1
    let start = Instant::now();
    let known_answer = "27803643063307";
    let part_1: usize = rearrange_tiles(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "1644";
    let part_2: usize = rearrange_tiles(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = read_to_string("inputs/day20_test.txt").expect("Input not found..");
        let answer: usize = rearrange_tiles(&input, 1);
        assert_eq!(answer, 20899048083289);
    }

    #[test]
    fn test_example_2() {
        let input = read_to_string("inputs/day20_test.txt").expect("Input not found..");
        let answer: usize = rearrange_tiles(&input, 2);
        assert_eq!(answer, 273);
    }
}
