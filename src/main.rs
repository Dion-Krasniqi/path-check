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
    let mut und_mat: Vec<Vec<usize>> = vec![vec![0usize;num_nodes];num_nodes];

    for (u,v) in und_edges.iter() {
        und_mat[*u-1][*v-1] = 1;
        und_mat[*v-1][*u-1] = 1;
    }
    println!("{:?}", und_mat);

    naive_djikstra(&und_mat, 2, 3);
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
 let mut unvisited: Vec<(usize,usize)> = nodes[0].iter_mut()
     .enumerate()
     .map(|(idx,node)| (idx,usize::MAX)).collect();
 unvisited[start] = (start,0);
 let mut cur_node = start;
 loop {
    for i in 0..nodes[0].len() {
        if nodes[cur_node][i] == 1 {
            nodes[i][cur_node] = 0;
            if unvisited[cur_node].1 < unvisited[i].1 
            {
                unvisited[i].1 = unvisited[cur_node].1 + 1;
            }
        }
    } 
    println!("{:?}", unvisited);
    unvisited[cur_node].1 = usize::MAX;
    if let Some((idx,_)) = unvisited.clone().iter_mut()
        .min_by(|(_,a),(_,b)| a.cmp(b)) 
            && unvisited[*idx].1 < usize::MAX {
            if *idx == target { 
                println!("Shortest path from start to target is {}", 
                    unvisited[*idx].1);
                return; 
            };
            cur_node = *idx;
    } else { break;};
 }
 if unvisited[target].1 < usize::MAX { 
                println!("Shortest path from start to target is {}", 
                    unvisited[target].1);
                return; 
            };
 

 println!("{:?}", unvisited);
 println!("No path");
}
