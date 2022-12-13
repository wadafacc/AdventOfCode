use std::vec;

#[derive(Clone,Debug, Default, PartialEq)]
struct Node {
    height: char,
    x: usize,
    y:usize,
}




fn main() {
    let input:&str = include_str!("../data.txt");
    let (map,node_map) = parse_map(input);
    setup(map.clone(), node_map);

    for l in map {
        println!("{:?}", l);
    }
}

fn setup(map:Vec<Vec<char>>, node_map:Vec<Vec<Node>>) {
    let start_coords :(usize,usize) = get_point('S', map.clone());
    let target_coords :(usize,usize) = get_point('E', map);
    let start_node = Node{
        height: 'S',
        x: start_coords.0,
        y: start_coords.1
    };
    let target_node = Node{
        height: 'E',
        x: target_coords.0,
        y: target_coords.1
    };

    // some_pathfinding_algo(node_map, start_node, target_node);
    get_neighbors(target_node, node_map);
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
    return ((target.x - x) as i32, (target.y - y) as i32);
}

fn get_neighbors(n:Node, map:Vec<Vec<Node>>) -> Vec<Node>{
    let mut neighbors:Vec<Node> = Vec::new();
    println!("Node to check neighbors: {:?}", n);
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if x == n.x -1  && y == n.y|| x == n.x +1  && y == n.y {
                println!("{:?}", map[y][x]);
            }
            if y == n.y -1 && x == n.x || y == n.y +1  && x == n.x{
                println!("{:?}", map[y][x]);
            }
        } 
    }

    neighbors
}

fn parse_map(input: &str) -> (Vec<Vec<char>>, Vec<Vec<Node>>){
    let line:Vec<&str> = input.lines().collect();

    let mut map:Vec<Vec<char>> = Vec::new();
    let mut filler:Vec<char> = Vec::new(); 
    for i in 0..line[0].len() + 2 {
        filler.push('#');
    }
    map.push(filler.clone());

    input.lines().into_iter().for_each(|l|{
        let mut new:Vec<char> = Vec::new();
        new.push('#');
        l.chars().for_each(|c|{
            new.push(c);
        });
        new.push('#');
        map.push(new);
    });
    map.push(filler);

    // return node map
    let mut node_map: Vec<Vec<Node>> = Vec::new();
    for l in &map{
        let mut row:Vec<Node> = Vec::new();
        for c in l {
            let pos: (usize,usize) = get_point(*c, map.clone());
            let new = Node{
                height: *c,
                x: pos.0,
                y:pos.1
            };
            row.push(new);

        }
        node_map.push(row);
    }
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