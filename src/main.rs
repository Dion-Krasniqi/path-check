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
