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
  - 陸の数を数えながら島を探索していく
  - 探索済みの島にはもういかないようにする
  - 最大値を更新し続ければOK



  想定ユースケース
  -

  正解してから気づいたこと
  - Union Findでも面倒だけどできそう
  - 今回は再帰の深さは高々5000くらい
    書く再帰に積まれるstackフレームは
    ポインタ(4byte)3つ、usize変数(8byte)２つ, ローカル変数(usize)2つ
    12 + 32 = 44 bytes
    ここにリターンアドレスとベースポインタの２つが加算されて
    44 + 16 = 60bytes
    最大まで再起した場合、5000 * 60 = 300000 bytes = 0.2 MBなので問題なし
  - とはいえ再帰を避けるやり方があるならそうしてみる
*/

pub struct Solution;
impl Solution {
    const LAND: i32 = 1;
    const WATER: i32 = 0;
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut seen = grid
            .iter()
            .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut max_land_count = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == Self::LAND {
                    let mut land_count = 0;
                    Self::count_lands(&grid, &mut seen, y, x, &mut land_count);
                    max_land_count = max_land_count.max(land_count);
                }
            }
        }

        max_land_count as i32
    }

    fn is_within_grid(grid: &[Vec<i32>], y: &usize, x: &usize) -> bool {
        *y < grid.len() && grid.get(*y).is_some_and(|row| *x < row.len())
    }

    pub fn count_lands(
        grid: &[Vec<i32>],
        seen: &mut [Vec<bool>],
        y: usize,
        x: usize,
        land_count: &mut usize,
    ) {
        if !Self::is_within_grid(grid, &y, &x) {
            return;
        }

        if seen[y][x] || grid[y][x] == Self::WATER {
            return;
        }

        seen[y][x] = true;

        *land_count += 1;

        for (next_y, next_x) in [(y, x + 1), (y, x - 1), (y + 1, x), (y - 1, x)] {
            Self::count_lands(grid, seen, next_y, next_x, land_count)
        }
    }
}
