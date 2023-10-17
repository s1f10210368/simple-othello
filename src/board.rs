pub struct Board {
    black: u64, // 黒い石の配置を表すためのビットボード
    white: u64, // 白い石の配置を表すためのビットボード
    turns: usize, // ターン数
}

impl Board {
    pub fn new() -> Self {
        // 初期配置設定
        let black = 0x0000000810000000; // 黒い石の初期配置
        let white = 0x0000001008000000; // 白い石の初期配置
        let n_moves = 0; // ターン数の初期値

        // ここで構造体を生成し、初期値を返している
        Board { black, white, turns: n_moves }
    }
}

pub fn legal_moves(&self) -> u64 {
    #[inline]
    const fn calc(my_turn: u64, enemy_turn: u64, mask:u64, shift: u32) -> u64 {
        let leftline = line!(my_turn, enemy_turn & mask, shift_leftline, shift);
        let rightline = line!(my_turn, enemy_turn & mask, shift_rightline, shift);
        shift_leftline(leftline, shift) | shift_rightline(rightline, shift);
    }

    let players = [self.black, self.white];
    let my_turn = players[self.turns % 2];
    let enemy_turn = players[(self.turns+1) % 2];
    let blank_board = !(my_turn | enemy_turn);
    let mut possible = 0;
    for (shift, mask) in Board::SHIFT_MASK_LIST {
        possible |= calc(my_turn, enemy_turn, mask, shift);
    }
    possible & blank_board
}