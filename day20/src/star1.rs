use std::collections::HashMap;
use std::collections::VecDeque;

pub fn star1() {
    let mut graph = HashMap::new();
    let mut state = HashMap::new();
    
    for l in include_str!("data.in").lines() {
        let (src, rest) = l.split_once(" -> ").unwrap();
        let connections = rest.split(", ").collect::<Vec<_>>();
        let (kind, label) = src.split_at(1); 
        let (node, state_type) = match kind {
            "%" => (label, Node::FlipFlop(false)),
            "&" => (label, Node::Conjunction(HashMap::new())),
            "b" => (label, Node::Broadcaster),
            _ => unreachable!(),
        };
        graph.insert(node, connections);
        state.insert(node, state_type);
    }

    for (&node, connections) in &graph{
        for &n in connections {
            match state.get_mut(n) {
                Some(Node::Conjunction(m)) => {
                    m.insert(node, false);
                }
                _ => {}
            }
        }
    }

    let mut count = [0, 0];
    let mut q = VecDeque::new();
    for _ in 0..1000 {
        //Sexy hardcoded values
        q.push_back(("roadcaster", "button", false));

        //Straight outa kösystem
        while let Some((node, prev, high)) = q.pop_front() {
            count[high as usize] += 1;
            
            let pulse = match state.get_mut(node) {
                Some(Node::FlipFlop(_)) if high => continue,
                Some(Node::FlipFlop(on)) => {
                    *on = !*on;
                    *on
                }
                Some(Node::Conjunction(m)) => {
                    m.insert(prev, high);
                    m.values().any(|&b| !b)
                }
                Some(Node::Broadcaster) => false,
                None => continue,
            };
            q.extend(graph[node].iter().map(|&n| (n, node, pulse)));
        }
    }
    print!("{:?}\n",count[0] * count[1]);
}

enum Node<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
    Broadcaster,
}
