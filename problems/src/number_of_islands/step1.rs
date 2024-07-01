// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  -

  何を考えて解いていたか
  - これが新しいしまなのかを見ていく
  - まず、どこを歩いたかわからないと辛いのでseenを準備する
  - もちろん地図も必要なのでgridももつ
  - 自分がどこにいるかの情報も必要なのでx, y
  - 今立ってる座標にすでに訪れていたらここは見ないでOK。文字通り戻る
  - 島が続く限り探索する
  - 四方八方を見ていく
  - しらみ潰しに地図をなぞって、陸ににたどりついたら探索開始。
  - これが新しい島なのかどうかをたどっていく

  想定ユースケース
  - ゲームでこのフィールド上の島の数をしめしたいときに使えるかも
    ユーザーが自分でフィールドを改変するタイプのゲームとか


  正解してから気づいたこと
  - is_new_island()の抜ける条件はまとめられた方がよさそう

  - 訪れた地点を0に書き換えていくという方法もあるが、訪れたとこをマークするなら'0'ではなく'x'とかにするべきでは？
  - BFSを使うという手もあった
    BFSは性質上,有向グラフの最短経路探索につかうイメージがあり、選択肢にあがらなかった
    DFSの方が無向グラフの探索に使えるイメージ
    メモリの節約をするならBFSのほうが良さそう
  - Union Findは思いつかなかった。
    このデータ構造の存在は知っているが、あまり使ったことがなくできなかった
*/

pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut island_count = 0;

        let mut seen = grid
            .clone()
            .into_iter()
            .map(|row| row.into_iter().map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                // 探索開始は島のときだけでOK
                if grid[y][x] == '1' {
                    if Self::is_new_island(&mut seen, &grid, x, y) {
                        island_count += 1
                    }
                }
            }
        }

        island_count as i32
    }

    fn is_new_island(seen: &mut [Vec<bool>], grid: &[Vec<char>], x: usize, y: usize) -> bool {
        if y >= grid.len() {
            return false;
        }

        if x >= grid[y].len() {
            return false;
        }

        if seen[y][x] {
            return false;
        }

        seen[y][x] = true;

        if grid[y][x] == '0' {
            return false;
        }

        let (left, right, up, down) = ((y, x - 1), (y, x + 1), (y - 1, x), (y + 1, x));

        for (next_y, next_x) in [left, right, up, down] {
            if !Self::is_new_island(seen, grid, next_x, next_y) {
                continue;
            }
        }

        true
    }
}
