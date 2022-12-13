#[derive(Clone,Debug, Default, PartialEq)]
struct Node {
    height: char,
    x_pos: usize,
    y_pos:usize,
}




fn main() {
    let input:&str = include_str!("../data.txt");
    let (map,node_map) = parse_map(input);
    setup(map, node_map);
}

fn setup(map:Vec<Vec<char>>, node_map:Vec<Vec<Node>>) {
    let start_coords :(usize,usize) = get_point('S', map.clone());
    let target_coords :(usize,usize) = get_point('E', map);
    let start_node = Node{
        height: 'a',
        x_pos: start_coords.0,
        y_pos: start_coords.1
    };
    let target_node = Node{
        height: 'a',
        x_pos: start_coords.0,
        y_pos: start_coords.1
    };

    some_pathfinding_algo(node_map, start_node, target_node);

    
    println!("{:?} | {:?}", start_coords,target_coords);
}

fn some_pathfinding_algo(map: Vec<Vec<Node>>, start: Node, target: Node) {
    let mut visited: Vec<Node> = Vec::new();
    let mut unvisited: Vec<Node> = Vec::new();

    unvisited.push(start);
    while unvisited.len() != 0 {
        let current = unvisited[0].clone();
        for i in 0..unvisited.len() {
            // get distance for each neighbor
            let neighbors = get_neighbors(current.clone(), map.to_vec());
            println!("{:?}", neighbors);
        }
    }

    /*
        grab a node from unvisited
        check its neighbors -> whichever has the shortest theoretical distance AND is walkable, take it 
        take that neighbor and take previous node in visited
        do until you get node.

    */ 
}

fn theoretical_dist(x:usize, y:usize, target: Node) -> (i32,i32) {
    return ((target.x_pos - x) as i32, (target.y_pos - y) as i32);
}

fn get_neighbors(n:Node, map:Vec<Vec<Node>>) -> Vec<Node>{
    let mut neighbors:Vec<Node> = Vec::new();

    for ln in map {
        for node in ln {

        }
    }

    neighbors
}

// gettin good at those ;)
fn parse_map(input: &str) -> (Vec<Vec<char>>, Vec<Vec<Node>>){
    let mut map:Vec<Vec<char>> = Vec::new();
    input.lines().into_iter().for_each(|l|{
        let mut new:Vec<char> = Vec::new();
        l.chars().for_each(|c|{
            new.push(c);
        });
        map.push(new);
    });

    // return node map
    let mut node_map: Vec<Vec<Node>> = Vec::new();
    for l in &map{
        let mut row:Vec<Node> = Vec::new();
        for c in l {
            let pos: (usize,usize) = get_point(*c, map.clone());
            let new = Node{
                height: *c,
                x_pos: pos.0,
                y_pos:pos.1
            };
            row.push(new);

        }
        node_map.push(row);
    }
    println!("{:?}", node_map);
    println!("{:?}", node_map);

    return (map, node_map);
}

fn get_point(c:char, map:Vec<Vec<char>>) -> (usize,usize) {
    let mut pos:(usize,usize) = (0,0);
    for ln in 0..map.len() {
        if map[ln].contains(&c) {
            pos = (map[ln].iter().position(|&p| p == c).unwrap() as usize,ln as usize);
        }
    }
    pos
}