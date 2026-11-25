struct ZobristKeys {
    pieces: [[u64; 64]; 12],
    side_to_move: u64,
    castling: [u64; 16],
    en_passant: [u64; 8]
}
const SEED: u64 = 94498698040556907;
const KEYS: ZobristKeys = generate_keys(SEED);
const PIECES_KEY: [[u64; 64]; 12] = KEYS.pieces;
const SIDE_TO_MOVE_KEY: u64 = KEYS.side_to_move;
const CASTLING_KEY: [u64; 16] = KEYS.castling;
const EN_PASSANT_KEY: [u64; 8] = KEYS.en_passant;

const fn generate_key(seed: &mut u64) -> u64 {
    let mut res = *seed;
    res ^= res << 5;
    res ^= res >> 13;
    res ^= res << 10;
    res ^= *seed;
    res
}

const fn generate_keys(seed: u64) -> ZobristKeys {
    let mut res = ZobristKeys{
        pieces: [[0; 64]; 12],
        side_to_move: 0,
        castling: [0; 16],
        en_passant: [0; 8]
    };
    let mut state: u64 = seed;
    let mut i = 0;
    while i < res.pieces.len() {
        let mut j = 0;
        while j < res.pieces[i].len(){
            res.pieces[i][j] = generate_key(&mut state);
            change_seed_state(&mut state);
            j += 1;
        }
        i += 1
    }

    res.side_to_move = generate_key(&mut state);
    change_seed_state(&mut state);

    i = 0;
    while i < res.castling.len(){
        res.castling[i] = generate_key(&mut state);
        change_seed_state(&mut state);
        i += 1;
    }

    i = 0;
    while i < res.en_passant.len(){
        res.en_passant[i] = generate_key(&mut state);
        change_seed_state(&mut state);
        i += 1;
    }

    res
}

const fn change_seed_state(seed: &mut u64) {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
}