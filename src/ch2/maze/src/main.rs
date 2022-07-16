use rand::Rng;

const MAP_N: usize = 25;

fn main() {
    // 棒倒し法で迷路を生成する
    let mut rng = rand::thread_rng();
    let mut maze = [[0; MAP_N]; MAP_N];

    // 1.) 外周を壁にする
    for n in 0..MAP_N {
        maze[n][0] = 1;// 上壁
        maze[0][n] = 1;// 左壁
        maze[n][MAP_N-1] = 1;// 右壁
        maze[MAP_N-1][n] = 1;// 下壁
    }

    // 2.) 2マスに1つ壁を配置
    for y in 2..MAP_N-2 {
        for x in 2..MAP_N-2 {
            if x%2==1 || y%2==1 {continue;}
            maze[y][x] = 1;
            // 3. 上下左右のいずれかを壁にする
            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[y-1][x] = 1, // 上
                1 => maze[y+1][x] = 1, // 下
                2 => maze[y][x-1] = 1, // 左
                3 => maze[y][x+1] = 1, // 右
                _ => {}
            }
        }
    }

    // 画面に表示
    let tiles = ["  ", "ZZ"];
    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}", tiles[maze[y][x]]);
        }
        println!();
    }
}
