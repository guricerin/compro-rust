use std::cmp::Ordering;

// 優先度付きキューに格納するためのタプル構造体。
// Rev((cost, node)) のように扱う。
// コストの低いものから優先的に取り出されるようにする。
#[derive(Eq, PartialEq, Clone, Debug)]
struct Rev<T>(pub T);

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Rev;
    use std::collections::BinaryHeap;

    // wupc-2012 E
    // 拡張(グラフでの)ダイクストラ
    #[test]
    fn extended_graph() {
        let n = 4;
        // let m = 3;
        let ftc = vec![(0, 1, 1), (1, 2, 1), (2, 3, 1)];

        let mut edges = vec![vec![]; n];
        for &(f, t, c) in ftc.iter() {
            edges[f].push((t, c));
            edges[t].push((f, c));
        }

        const INF: usize = 1e9 as usize;
        let mut dist4 = vec![vec![INF; 4]; n];
        let mut dist7 = vec![vec![INF; 7]; n];

        let mut heap = BinaryHeap::new();
        // 優先度付きキューにタプルを格納した場合、取り出されるときはタプルの第0要素が基準となる。
        heap.push(Rev((0, 0)));
        while let Some(Rev((cost, from))) = heap.pop() {
            let step4 = cost % 4;
            let step7 = cost % 7;
            if dist4[from][step4] <= cost && dist7[from][step7] <= cost {
                continue;
            }

            if dist4[from][step4] > cost {
                dist4[from][step4] = cost;
            }
            if dist7[from][step7] > cost {
                dist7[from][step7] = cost;
            }

            if from == n - 1 {
                continue;
            }

            for &(to, c) in edges[from].iter() {
                let next_cost = cost + c;
                if dist4[to][next_cost % 4] > next_cost || dist7[to][next_cost % 7] > next_cost {
                    heap.push(Rev((next_cost, to)));
                }
            }
        }

        assert_eq!(std::cmp::min(dist4[n - 1][0], dist7[n - 1][0]), 7);
    }

}
