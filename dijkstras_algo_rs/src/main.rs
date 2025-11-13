struct Node {
    name: char,
    distance_from_source: i32,
    is_visited: bool,
}

struct Edge<'a> {
    source: &'a Node,
    weight: i32,
    destination: &'a mut Node,
}

impl<'a> Edge<'a> {
    fn new(source: &'a Node, weight: i32, destination: &'a mut Node) -> Self {
        Self {
            source,
            weight,
            destination,
        }
    }
}

impl Node {
    fn new(name: char) -> Self {
        Self {
            name,
            distance_from_source: i32::MAX,
            is_visited: false,
        }
    }
}

fn main() {
    let mut a = Node::new('A');
    let mut b = Node::new('B');
    let mut c = Node::new('C');
    let mut d = Node::new('D');
    let mut e = Node::new('E');

    let mut edge1 = Edge::new(&a, 17, &mut b);
    let mut edge2 = Edge::new(&a, 7, &mut c);
    let mut edge3 = Edge::new(&b, 11, &mut c);
    let mut edge4 = Edge::new(&b, 4, &mut d);
    let mut edge5 = Edge::new(&c, 15, &mut d);
    let mut edge6 = Edge::new(&d, 13, &mut e);
    let mut edge7 = Edge::new(&c, 5, &mut e);

    let graph = vec![
        &mut edge1, &mut edge2, &mut edge3, &mut edge4, &mut edge5, &mut edge6, &mut edge7,
    ];

    // Let the starting vertex be A.
    a.distance_from_source = 0;

    for mut edge in graph {
        if edge.source.name == a.name {
            edge.destination.distance_from_source = edge.weight;
        }
    }
}
