
use aoc::multi_for;
use std::collections::HashMap;
use std::cell::RefCell;

const SIZE: usize = 10; 

struct Image {
    size: usize,
    data: Vec<char>,
    edges: Vec<Vec<char>>,
}

impl Image {
    fn new(size: usize, data: Vec<char>) -> Image {
        let mut image = Image{size, data, edges:Vec::new()};
        image.gen_edges();
        image
    }

    fn pixel(&self, x: usize, y: usize) -> char {
        self.data[y * self.size + x]
    }

    fn pixel_mut(&mut self, x: usize, y: usize) -> &mut char {
        &mut self.data[y * self.size + x]
    }
    
    fn gen_edges(&mut self) {
        let mut edges: Vec<Vec<char>> = Vec::with_capacity(4);

        let mut top_edge: Vec<char> = Vec::with_capacity(self.size);
        for x in 0..self.size {
            top_edge.push(self.pixel(x, 0));
        }
        edges.push(top_edge);

        let mut bot_edge: Vec<char> = Vec::with_capacity(self.size);
        for x in 0..self.size {
            bot_edge.push(self.pixel(x, self.size-1));
        }
        edges.push(bot_edge);

        let mut left_edge: Vec<char> = Vec::with_capacity(self.size);
        for y in 0..self.size {
            left_edge.push(self.pixel(0, y));
        }
        edges.push(left_edge);

        let mut right_edge: Vec<char> = Vec::with_capacity(self.size);
        for y in 0..self.size {
            right_edge.push(self.pixel(self.size-1, y));
        }
        edges.push(right_edge);

        self.edges = edges;
    }
    
    fn top_edge(&self) -> &Vec<char> {
        &self.edges[0]
    }

    fn bot_edge(&self) -> &Vec<char> {
        &self.edges[1]
    }
    
    fn left_edge(&self) -> &Vec<char> {
        &self.edges[2]
    }

    fn right_edge(&self) -> &Vec<char> {
        &self.edges[3]
    }
    
    fn rotate(&mut self) {
        let mut new_data: Vec<char> = Vec::new();
        new_data.resize(self.data.len(), 'X');
        multi_for! { [x in 0..self.size, y in 0..self.size] 
            new_data[x * self.size + (self.size-1-y)] = self.pixel(x, y);
        }
        self.data = new_data;
        self.gen_edges();
    }

    fn flip_x(&mut self) {
        multi_for! { [x in 0..(self.size / 2), y in 0..self.size]
            self.data.swap(y * self.size + x, y * self.size + (self.size-1-x));
        }
        self.gen_edges();
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                print!("{}", self.pixel(x, y));
            }
            println!("");
        }
    }
}

struct Tile {
    id: i64,
    image: Image,
}

fn main() {
    let mut tiles: HashMap<i64, RefCell<Tile>> = HashMap::new();
    let mut lines = aoc::file_lines_iter("./day20.txt");

    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let id = line.split(" ").nth(1).unwrap().trim_end_matches(":").parse::<i64>().unwrap();

        let mut image: Vec<char> = Vec::with_capacity(SIZE * SIZE);
        for _ in 0..10 {
            if let Some(line) = lines.next() {
                image.extend(line.chars());
            }
        }
        tiles.insert(id, RefCell::new(Tile{id:id, image:Image::new(SIZE, image)}));
    }
    let final_size = (tiles.len() as f64).sqrt() as usize;

    // Map all edges and tile_ids with each other
    let mut all_edges: HashMap<Vec<char>, Vec<i64>> = HashMap::new();
    for tile in tiles.values() {
        for edge in &tile.borrow().image.edges {
            all_edges.entry(edge.clone()).or_insert(Vec::new()).push(tile.borrow().id);
            let mut rev_edge = edge.clone();
            rev_edge.reverse();
            all_edges.entry(rev_edge).or_insert(Vec::new()).push(tile.borrow().id);
        }
    }

    let mut id_edge_count: HashMap<i64, i64> = HashMap::new();
    // Filter out the tile ids with outermost edges that have no pairing (edge -> ids.len() == 1)
    for id in all_edges.iter().filter(|(_edge, ids)| ids.len() == 1).map(|(_edge, ids)| ids[0]) {
        // Map the tile_id this edge belongs to and maintain a count of how many times we see this
        // tile_id
        *id_edge_count.entry(id).or_insert(0) += 1; 
    }

    // Every tile has 8 edges (because we "flipped" each edge)
    // ids with an edge count of 4 unpaired edges have 1/2 their edges unpaied, i.e. corner pieces
    let corners: Vec<i64> = id_edge_count.iter().filter(|(_id, edge_count)| **edge_count == 4).map(|(id, _edge_count)| *id).collect();

    // Assemble the final image
    let mut tile_locations: HashMap<(usize, usize), i64> = HashMap::new();
    let mut final_image_data: Vec<char> = Vec::new();
    final_image_data.resize(8 * 8 * final_size * final_size, 'X');
    let mut final_image = Image::new(8 * final_size, final_image_data);
    
    // Place a random corner tile
    let tile = tiles.get(&corners[0]).unwrap();

    while !(all_edges.get(tile.borrow().image.top_edge()).unwrap().len() == 1 &&
            all_edges.get(tile.borrow().image.left_edge()).unwrap().len() == 1) {
        tile.borrow_mut().image.rotate();
    }
    assert!(all_edges.get(tile.borrow().image.top_edge()).unwrap().len() == 1);
    assert!(all_edges.get(tile.borrow().image.left_edge()).unwrap().len() == 1);
    assert!(all_edges.get(tile.borrow().image.right_edge()).unwrap().len() == 2);
    assert!(all_edges.get(tile.borrow().image.bot_edge()).unwrap().len() == 2);

    tile_locations.insert((0, 0), tile.borrow().id);
    multi_for! { [x in 1..9, y in 1..9]
        *final_image.pixel_mut(x-1, y-1) = tile.borrow().image.pixel(x, y);
    }

    multi_for! { [tile_x in 0..final_size, tile_y in 0..final_size]
        if tile_x == 0 && tile_y == 0 {
            continue;
        }
        if tile_x == 0 {
            let tile_above = tiles.get(tile_locations.get(&(tile_x, tile_y-1)).unwrap()).unwrap();
            let this_tile_id = all_edges.get(tile_above.borrow().image.bot_edge()).unwrap().iter()
                                    .filter(|id| **id != tile_above.borrow().id).nth(0).unwrap();
            let this_tile = tiles.get(this_tile_id).unwrap();

            let mut count = 0;
            while tile_above.borrow().image.bot_edge() != this_tile.borrow().image.top_edge() {
                this_tile.borrow_mut().image.rotate();
                count += 1;
                if count == 4 {
                    this_tile.borrow_mut().image.flip_x();
                }
            }
            assert!(all_edges.get(this_tile.borrow().image.left_edge()).unwrap().len() == 1);
                
            tile_locations.insert((tile_x, tile_y), this_tile.borrow().id);
            multi_for! { [x in 1..9, y in 1..9]
                let final_x = 8 * tile_x + (x-1);
                let final_y = 8 * tile_y + (y-1);
                *final_image.pixel_mut(final_x, final_y) = this_tile.borrow().image.pixel(x, y);
            }
        } else if tile_x != 0 {
            let tile_to_left = tiles.get(tile_locations.get(&(tile_x-1, tile_y)).unwrap()).unwrap();
            let this_tile_id = all_edges.get(tile_to_left.borrow().image.right_edge()).unwrap().iter()
                                    .filter(|id| **id != tile_to_left.borrow().id).nth(0).unwrap();
            let this_tile = tiles.get(this_tile_id).unwrap();

            let mut count = 0;
            while tile_to_left.borrow().image.right_edge() != this_tile.borrow().image.left_edge() {
                this_tile.borrow_mut().image.rotate();
                count += 1;
                if count == 4 {
                    this_tile.borrow_mut().image.flip_x();
                }
            }

            tile_locations.insert((tile_x, tile_y), this_tile.borrow().id);
            multi_for! {[x in 1..9, y in 1..9]
                let final_x = 8 * tile_x + (x-1);
                let final_y = 8 * tile_y + (y-1);
                *final_image.pixel_mut(final_x, final_y) = this_tile.borrow().image.pixel(x, y);
            }
        }
    }

    let dragon_mask = vec![(18, 0),
                           (0, 1), (5, 1), (6, 1), (11, 1), (12, 1), (17, 1), (18, 1), (19, 1),
                           (1, 2), (4, 2), (7, 2), (10, 2), (13, 2), (16, 2)];

    let mut dragons_spotted;
    let mut count = 0;
    loop {
        dragons_spotted = 0;

        multi_for! { [x in 0..((8 * final_size) -20), y in 0..((8 * final_size) - 3)]
            if dragon_mask.iter().all(|(x_offset, y_offset)| final_image.pixel(x + x_offset, y + y_offset) == '#') {
                dragon_mask.iter().for_each(|(x_offset, y_offset)| *final_image.pixel_mut(x + x_offset, y + y_offset) = 'O');
                dragons_spotted += 1;
            }
        }
        if dragons_spotted > 0 {
            break;
        }
        final_image.rotate();
        count += 1;
        if count == 4 {
            final_image.flip_x();
        }
    }
    final_image.print();

    let wave_count = final_image.data.iter().filter(|c| **c == '#').count(); 
    println!("Part 1: {}", corners.iter().product::<i64>());
    println!("Part 2: {}", wave_count);
}

