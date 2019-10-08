/// 素集合データ構造(disjoint-set)
/// 重みなし
/// くっつけるのは速いが、切り離すのは不可能
pub struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>, // 属するグループの頂点数
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect::<Vec<usize>>(), // はじめはすべての頂点が根
            rank: vec![0; n],
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let px = self.par[x];
            self.par[x] = self.root(px);
            self.par[x]
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let rx = self.root(x);
        let ry = self.root(y);
        if rx == ry {
            return false;
        }

        let (large, small) = if self.rank[rx] < self.rank[ry] {
            (ry, rx)
        } else {
            (rx, ry)
        };

        self.par[small] = large;
        self.rank[large] += self.rank[small];
        self.rank[small] = 0;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::UnionFind;

    fn atcoder_abc097_d(n: usize, m: usize, ps: &Vec<usize>, xy: &Vec<(usize, usize)>) -> usize {
        let mut uf = UnionFind::new(n);
        for &(x, y) in xy.iter() {
            uf.unite(x - 1, y - 1);
        }

        let mut count = 0;
        for (i, p) in ps.iter().enumerate() {
            if uf.same(i, *p - 1) {
                count += 1;
            }
        }
        count
    }

    #[test]
    fn test_union_find() {
        let n = 5;
        let m = 2;
        let ps = vec![5, 3, 1, 4, 2];
        let xy = vec![(1, 3), (5, 4)];
        assert_eq!(2, atcoder_abc097_d(n, m, &ps, &xy));

        let n = 3;
        let m = 2;
        let ps = vec![3, 2, 1];
        let xy = vec![(1, 2), (2, 3)];
        assert_eq!(3, atcoder_abc097_d(n, m, &ps, &xy));

        let n = 10;
        let m = 8;
        let ps = vec![5, 3, 6, 8, 7, 10, 9, 1, 2, 4];
        let xy = vec![
            (3, 1),
            (4, 1),
            (5, 9),
            (2, 5),
            (6, 5),
            (3, 5),
            (8, 9),
            (7, 9),
        ];
        assert_eq!(8, atcoder_abc097_d(n, m, &ps, &xy));

        let n = 5;
        let m = 1;
        let ps = vec![1, 2, 3, 4, 5];
        let xy = vec![(1, 5)];
        assert_eq!(5, atcoder_abc097_d(n, m, &ps, &xy));
    }
}
