use std::collections::{HashMap, HashSet};

mod utils;

type Tile = Vec<Vec<bool>>;

fn main() {
    let lines = utils::read_input();
    let input = parse_input(&lines);

    let tile_variants = input
        .iter()
        .map(|(id, tile)| (*id, generate_variations(tile.to_vec())))
        .collect::<HashMap<i32, Vec<Tile>>>();

    let result = solve(&tile_variants);
    println!("Result: {}", result);
}

fn parse_input(lines: &Vec<String>) -> HashMap<i32, Tile> {
    let mut tiles = HashMap::new();
    let mut iter = lines.iter();

    while let Some(line) = iter.next() {
        // "Tile 2311:"
        let tile_id = line["Tile ".len()..].trim_end_matches(':').parse().unwrap();

        let mut tile = Vec::new();
        while let Some(line) = iter.next() {
            if line.is_empty() {
                break;
            }
            // #.#.#####.
            tile.push(line.chars().map(|c| c == '#').collect());
        }
        tiles.insert(tile_id, tile);
    }

    tiles
}

fn generate_variations(tile: Tile) -> Vec<Tile> {
    let mut variations = Vec::new();
    let mut current = tile;

    // Base rotations
    for _ in 0..4 {
        variations.push(current.clone());
        current = rotate_90(current);
    }

    // Flipped variations
    let flipped_horizontal = flip_horizontal(current.clone());
    variations.push(flipped_horizontal.clone());

    let flipped_vertical = flip_vertical(current.clone());
    variations.push(flipped_vertical.clone());

    // Flipped rotations
    let mut flipped_rotated = flipped_horizontal;
    for _ in 0..4 {
        variations.push(flipped_rotated.clone());
        flipped_rotated = rotate_90(flipped_rotated);
    }

    variations.sort();
    variations.dedup();

    variations
}

fn rotate_90(tile: Tile) -> Tile {
    let rows = tile.len();
    let cols = tile[0].len();
    let mut rotated: Tile = vec![vec![false; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            rotated[j][rows - 1 - i] = tile[i][j];
        }
    }

    rotated
}

fn flip_horizontal(tile: Tile) -> Tile {
    tile.iter().cloned().rev().collect()
}

fn flip_vertical(tile: Tile) -> Tile {
    tile
        .iter()
        .map(|row| row.iter().cloned().rev().collect())
        .collect()
}

fn possible_separations(n: i32) -> impl Iterator<Item = (i32, i32)> {
    let max_y = n;

    (1..=max_y)
        .filter_map(move |x| {
            if max_y % x == 0 {
                Some((x, max_y / x))
            } else {
                None
            }
        })
}

fn build_valid_placement(x_tiles: i32, y_tiles: i32, tile_variants: &HashMap<i32, Vec<Tile>>) -> Option<Vec<Vec<i32>>> {
    let mut placement_id: Vec<Vec<i32>> = vec![vec![-1; x_tiles as usize]; y_tiles as usize];
    let mut placement_variant_id: Vec<Vec<i32>> = vec![vec![-1; x_tiles as usize]; y_tiles as usize];
    let mut used_tiles = HashSet::new();

    if place_tile(0, 0, x_tiles, y_tiles, tile_variants, &mut placement_id, &mut placement_variant_id, &mut used_tiles) {
        Some(placement_id)
    } else {
        None
    }
}

fn place_tile(x: i32, y: i32, x_tiles: i32, y_tiles: i32, tile_variants: &HashMap<i32, Vec<Tile>>, placement_id: &mut Vec<Vec<i32>>, placement_variant_id: &mut Vec<Vec<i32>>, used_tiles: &mut HashSet<i32>) -> bool {
    if y >= y_tiles {
        return true;
    }

    let get_neighbour_variant = |x_diff: i32, y_diff: i32, placement_id: &Vec<Vec<i32>>, placement_variant_id: &Vec<Vec<i32>>| -> &Tile {
        let new_x = (x + x_diff) as usize;
        let new_y = (y + y_diff) as usize;

        let neighbour_id = placement_id[new_y][new_x];
        let neighbour_variant_id = placement_variant_id[new_y][new_x];
        &tile_variants.get(&neighbour_id).unwrap()[neighbour_variant_id as usize]
    };

    for (tile_id, variants) in tile_variants.iter() {
        if used_tiles.contains(tile_id) {
            continue;
        }
        
        for (variant_id, variant) in variants.iter().enumerate() {
            if y > 0 {
                let neighbour_variant = get_neighbour_variant(0, -1, placement_id, placement_variant_id);
                if !up_down_match(variant, neighbour_variant) {
                    continue;
                }
            }
            if x > 0 {
                let neighbour_variant = get_neighbour_variant(-1, 0, placement_id, placement_variant_id);
                if !left_right_match(neighbour_variant, variant) {
                    continue;
                }
            }

            placement_id[y as usize][x as usize] = *tile_id;
            placement_variant_id[y as usize][x as usize] = variant_id as i32;
            used_tiles.insert(*tile_id);

            let new_x = if x + 1 < x_tiles { x + 1 } else { 0 };
            let new_y = if new_x == 0 { y + 1 } else { y };
            if place_tile(new_x, new_y, x_tiles, y_tiles, tile_variants, placement_id, placement_variant_id, used_tiles) {
                return true;
            }

            used_tiles.remove(tile_id);
        }
    }

    false
}

fn left_right_match(left_tile: &Tile, right_tile: &Tile) -> bool {
    let y_max = left_tile.len();
    let x_max: usize = left_tile[0].len();

    for y in 0..y_max {
        if left_tile[y][x_max - 1] != right_tile[y][0] {
            return false;
        }
    }

    true
}

fn up_down_match(upper_tile: &Tile, lower_tile: &Tile) -> bool {
    let y_max = upper_tile.len();
    let x_max = upper_tile[0].len();

    for x in 0..x_max {
        if upper_tile[y_max - 1][x] != lower_tile[0][x] {
            return false;
        }
    }

    true
}

fn solve(tile_variants: &HashMap<i32, Vec<Tile>>) -> u64 {
    for (x_tiles, y_tiles) in possible_separations(tile_variants.len() as i32) {
        if let Some(valid_placement) = build_valid_placement(x_tiles, y_tiles, &tile_variants) {
            return 
                *valid_placement.first().unwrap().first().unwrap() as u64 *
                *valid_placement.first().unwrap().last().unwrap() as u64 *
                *valid_placement.last().unwrap().first().unwrap() as u64 *
                *valid_placement.last().unwrap().last().unwrap() as u64;
        }
    }

    panic!("No valid placement found")
}
