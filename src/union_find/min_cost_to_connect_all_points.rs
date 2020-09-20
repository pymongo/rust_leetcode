struct Solution;

struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind { parents: (0..n).collect() }
    }

    fn find_root(&self, node: usize) -> usize {
        let mut curr_node = node;
        let mut curr_node_parent = self.parents[curr_node];
        while curr_node_parent != curr_node {
            curr_node = curr_node_parent;
            curr_node_parent = self.parents[curr_node];
        }
        return curr_node_parent;
    }

    // 如果a和b不相连，则添加一条node_a连向node_b边
    fn union(&mut self, node_a: usize, node_b: usize) {
        // 路径压缩: 不要直接将b连到a上，而是将b的祖先连向a的祖先，以此压缩路径减少连边
        let root_a = Self::find_root(self, node_a);
        let root_b = Self::find_root(self, node_b);
        if root_a != root_b {
            // 将b的祖先挂载到a的祖先下
            self.parents[root_b] = root_a;
        }
    }
}

// 平面上有若干点，找出能连接所有点的最短边的总和
// 除了并查集排除重复连边，还能用「最小生成树的模板」的Prim或Kruskal算法
impl Solution {
    fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut edges = Vec::with_capacity(n * (n - 1) / 2);
        for start in 0..n {
            for end in start + 1..n {
                edges.push((
                    (points[start][0] - points[end][0]).abs()
                        + (points[start][1] - points[end][1]).abs(),
                    start,
                    end,
                ));
            }
        }
        edges.sort_unstable();
        let mut total_cost = 0;
        let mut union_find = UnionFind::new(n);
        let mut used_edges = 0;
        for (cost, node_a, node_b) in edges {
            let root_a = union_find.find_root(node_a);
            let root_b = union_find.find_root(node_b);
            if root_a == root_b {
                continue;
            }
            total_cost += cost;
            union_find.parents[root_b] = root_a;
            used_edges += 1;
            if used_edges == n - 1 {
                break;
            }
        }
        return total_cost;
    }
}

#[cfg(test)]
fn test_cases() -> Vec<(Vec<Vec<i32>>, i32)> {
    vec![
        (
            vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]],
            20,
        ),
        (
            vec![vec![2, -3], vec![-17, -8], vec![13, 8], vec![-17, -15]],
            53,
        ),
        (vec![vec![3, 12], vec![-2, 5], vec![-4, 1]], 18),
        (vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![-1, 1]], 4),
    ]
}

#[test]
fn test() {
    for (points, min_cost) in test_cases() {
        assert_eq!(Solution::min_cost_connect_points(points), min_cost);
    }
}