use std::collections::HashMap;
use std::collections::VecDeque;

pub fn star2() {
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

    let mut end = "";
    for (&node, connections) in &graph{
        for &n in connections {
            match state.get_mut(n) {
                Some(Node::Conjunction(m)) => {
                    m.insert(node, false);
                }
                Some(_) => {}
                None => end = node,
            }
        }
    }

    let mut cycles = match &state[end] {
        Node::Conjunction(m) => m
            .iter()
            .map(|(&node, _)| (node, None))
            .collect::<HashMap<_, _>>(),
        _ => unreachable!(),
    };

    let mut q = VecDeque::new();
    'outer: for t in 1.. {
        //Sexy hardcoded values
        q.push_back(("roadcaster", "button", false));
        
        //Straight outa kösystem
        while let Some((node, prev, high)) = q.pop_front() {
            if high && node == end {
                let v = cycles.get_mut(prev).unwrap();
                if v.is_none() {
                    *v = Some(t);
                    if cycles.values().all(|o| o.is_some()) {
                        break 'outer;
                    }
                }
            }
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

    //Assuming independent coprime cycles
    print!("{:?}\n", cycles.values().map(|o| o.unwrap()).product::<u64>());
}

enum Node<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
    Broadcaster,
}