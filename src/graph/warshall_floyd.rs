/// 隣接行列で表されるグラフの全点間最短路を求める。負辺があっても動作する。
/// 負閉路が存在する場合はそれも検出する(ある頂点 k から k への最短路が負ならば負閉路が存在)。
/// 到達できない要素には、infが格納される。
/// O(V^3)
pub fn warshall_floyd(matrix: &mut Vec<Vec<i64>>, inf: i64) {
    let n = matrix.len();
    assert_eq!(n, matrix[0].len());
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if matrix[i][k] == inf || matrix[k][j] == inf {
                    continue;
                }
                matrix[i][j] = std::cmp::min(matrix[i][j], matrix[i][k] + matrix[k][j]);
            }
        }
    }
}
