use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Reverse;

fn main() {
    let num_nodes: usize = 4;
    let mut adj_mat: Vec<Vec<usize>> = vec![vec![0usize;num_nodes];num_nodes];
    let edges: Vec<(usize,usize)> = vec![(1,2),(2,1),(3,4),(4,1)];
    for (u,v) in edges.iter() {
        adj_mat[*u-1][*v-1] = 1;
    }
    println!("{:?}", adj_mat);
    // nodes of interest
    let u = 3;
    let v = 1;
    let result = exponent_with_checking(&adj_mat, u, v);
    println!("Shortest path from, {} to {} is {}", u, v, result); 
    let und_edges: Vec<(usize,usize)> = vec![(1,2),(2,5),(1,5),(2,4),(4,3)];
    let num_nodes: usize = 5;
    let mut und_mat: Vec<Vec<usize>> = vec![vec![usize::MAX;num_nodes];num_nodes];

    for (u,v) in und_edges.iter() {
        und_mat[*u-1][*v-1] = 1;
        und_mat[*v-1][*u-1] = 1;
    }
    println!("{:?}", und_mat);

    naive_djikstra(&und_mat, 1, 2);
    heap_djikstra(&und_mat, 1, 2);
}
fn mat_mul(a: &Vec<Vec<usize>>,
           b: &Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    let mut result = vec![vec![0usize;a.len()];a.len()];
    for i in 0..a.len() {
        for j in 0..a.len() {
            for k in 0..a.len() {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}
fn exponent_with_checking(a: &Vec<Vec<usize>>,
            u: usize,
            v: usize,
) -> usize /*Vec<Vec<usize>>*/ {
    if a[u-1][v-1] > 0 {
        return 1
    }
    let mut dummy_res = mat_mul(a, a);
    if dummy_res[u-1][v-1] > 0 {
        return 2
    }
    for i in 0..(a.len()-3) {
        dummy_res = mat_mul(&dummy_res, a);
        if dummy_res[u-1][v-1] > 0 {
            return i + 3 
        }
    }
    0
}

fn naive_djikstra(a: &Vec<Vec<usize>>,
                  start: usize,
                  target: usize,
) {
 let mut nodes = a.clone();
 // nodes[start] as unvisited
 let mut cur_node = start;
 nodes[start][cur_node] = 0;
 let nodes_len = nodes[0].len();
 loop {
    for i in 0..nodes_len {
        if i == cur_node { continue;};
        if nodes[cur_node][i] == 1 {
            if nodes[start][cur_node] < nodes[start][i] 
            {
                nodes[start][i] = nodes[start][cur_node] + 1;
            }
        }
    }
    // mark visited
    nodes[start][cur_node] = usize::MAX;
    if let Some((idx,_)) = nodes[start].iter()
        .enumerate()
        .min_by(|(_,a),(_,b)| a.cmp(b)) 
            && nodes[start][idx] < usize::MAX {
            if idx == target { 
                println!("Shortest path from start to target is {}", 
                    nodes[start][idx]);
                return; 
            };
            cur_node = idx;
    } else { break;};
 }
 if nodes[start][target] > 0 
    &&  nodes[start][target] < usize::MAX { 
            println!("Shortest path from start to target is {}", 
                     nodes[start][target]);
            return; 
 };
 println!("No path");
}
#[derive(Eq,PartialEq,Clone,Debug)]
// tag, distance
struct Node(pub usize, pub usize);
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(other.1.cmp(&self.1))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.1.cmp(&self.1)
    }
}
fn heap_djikstra(a: &Vec<Vec<usize>>,
                  start: usize,
                  target: usize,
) {
    let mut dummy_start: Vec<Node> = (0..a[0].len())
        .map(|e| Node(e,usize::MAX)).collect();
    dummy_start[start] = Node(start, 0);
    let mut unvisited:  BinaryHeap<Node> = BinaryHeap::
        from(dummy_start);
    let mut cur_node: Node = Node(start, 0);
    loop {
        let mut temp = unvisited.clone().into_vec();
        temp.iter_mut().for_each(|n| {
            if a[cur_node.0][n.0] == 1 {
                if cur_node.1 < n.1 {
                    n.1 = cur_node.1 + 1;
                }
            }
        });
        println!("{:?}", temp);
        unvisited = BinaryHeap::from(temp);
        cur_node = unvisited.pop().unwrap();
        if cur_node.0 == target && cur_node.1 < usize::MAX { 
            println!("Path is {}", cur_node.1);
            return;
        };
        if unvisited.is_empty() { break; };
        if cur_node.1 == usize::MAX { break; };
    }
    println!("Nada");
}
