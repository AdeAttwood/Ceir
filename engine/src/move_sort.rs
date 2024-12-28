use common::ResolvedMovement;

#[rustfmt::skip]
pub const MVV_LVA: [[i32; 6]; 6] = [
    [0, 0, 0, 0, 0, 0],       // victim K, attacker K, Q, R, B, N, P
    [50, 51, 52, 53, 54, 55], // victim Q, attacker K, Q, R, B, N, P
    [40, 41, 42, 43, 44, 45], // victim R, attacker K, Q, R, B, N, P
    [30, 31, 32, 33, 34, 35], // victim B, attacker K, Q, R, B, N, P
    [20, 21, 22, 23, 24, 25], // victim N, attacker K, Q, R, B, N, P
    [10, 11, 12, 13, 14, 15], // victim P, attacker K, Q, R, B, N, P
];

pub fn sort_key(movement: &ResolvedMovement) -> i32 {
    match movement.capture {
        None => 0,
        Some(victim) => -MVV_LVA[victim as usize][movement.piece as usize],
    }
}
