pub const  SIZEMAP: i128 = 19;
pub const  RSIZEMAP: i128 = 18;

pub const CAPTURE_P1: [(i128, i128, i128); 4] = [
                                             (0o3333, 0o1220, 0),
                                             (0o3333, 0o0221, 9),
                                             (0o3333333, 0o1020200, 0),
                                             (0o3333333, 0o0020201, 18)
                                            ];

pub const CAPTURE_P2: [(i128, i128, i128); 4] = [
                                             (0o3333, 0o2110, 0),
                                             (0o3333, 0o0112, 9),
                                             (0o3333333, 0o2010100, 0),
                                             (0o3333333, 0o0010102, 18)
                                            ];


pub const THREE_MOVE_P1: [(i128, i128, i128); 9] = [
                                             (0o33333, 0o01100, 3),
                                             (0o33333, 0o01010, 6),
                                             (0o33333, 0o00110, 9),
                                             (0o333333, 0o10100, 3),
                                             (0o333333, 0o010010, 6),
                                             (0o333333, 0o000110, 12),
                                             (0o333333, 0o011000, 3),
                                             (0o333333, 0o010010, 9),
                                             (0o333333, 0o001010, 12)
                                            ];

pub const THREE_MOVE_P2: [(i128, i128, i128); 9] = [
                                             (0o33333, 0o02200, 3),
                                             (0o33333, 0o02020, 6),
                                             (0o33333, 0o00220, 9),
                                             (0o333333, 0o20200, 3),
                                             (0o333333, 0o020020, 6),
                                             (0o333333, 0o000220, 12),
                                             (0o333333, 0o022000, 3),
                                             (0o333333, 0o020020, 9),
                                             (0o333333, 0o002020, 12)
                                            ];

pub const DTHREE_MOVE_P1: [(i128, i128, i128); 9] = [
                                             (0o3333333333, 0o001010000, 6),
                                             (0o3333333333, 0o001000100, 12),
                                             (0o3333333333, 0o000010100, 18),
                                             (0o33333333333, 0o00100010000, 6),
                                             (0o33333333333, 0o00100000100, 12),
                                             (0o33333333333, 0o00000010100, 24),
                                             (0o33333333333, 0o00101000000, 6),
                                             (0o33333333333, 0o00100000100, 18),
                                             (0o33333333333, 0o00001000100, 24)
                                            ];

pub const DTHREE_MOVE_P2: [(i128, i128, i128); 9] = [
                                             (0o3333333333, 0o002020000, 6),
                                             (0o3333333333, 0o002000200, 12),
                                             (0o3333333333, 0o000020200, 18),
                                             (0o33333333333, 0o00200020000, 6),
                                             (0o33333333333, 0o00200000200, 12),
                                             (0o33333333333, 0o00000020200, 24),
                                             (0o33333333333, 0o00202000000, 6),
                                             (0o33333333333, 0o00200000200, 18),
                                             (0o33333333333, 0o00002000200, 24)
                                            ];

/*
** Value for heurestic
*/

// done
const THREE_ALIGN: i128 = 3;
const THREE_ALIGN_CUT: i128 = 2;
const THREE_ALIGN_FREE: i128 = 6;
const CAPTURE: i128 = 4;
const ENEMY_FOUR_ALIGN: i128 = 8;

// to do
const ENEMY_THREE_ALIGN_FREE: i128 = 8;
const FOUR_ALIGN: i128 = 7;
const FOUR_ALIGN_CUT: i128 = 6;
const FIVE_ALIGN: i128 = 10;

/*
**
**  (a, b, c, d) = (0o3333, 0o0110, 6, 3)
**  a = the mask
**  b = the patern
**  c = the shift ( << ) ; can overflow if not checked before calling the macro
**  d = value of the patern
**
**
**  1 = player one
**  2 = player two
**  . = empty slot
**  | = not checked (used for alignement of the comment)
**  ' = will never be set (used for diagonal check)
**  X = empty slot where we want to check the patern
**
*/

pub const MOVE_P1: [(i128, i128, i128, i128); 13] = [
                                             (0o3333, 0o0110, 0, THREE_ALIGN),        // |.11X
                                             (0o3333, 0o0101, 3, THREE_ALIGN),        // |.1X1
                                             (0o3333, 0o0011, 6, THREE_ALIGN),        // |.X11
                                             (0o33333, 0o01010, 0, THREE_ALIGN_CUT),  // .1.1X
                                             (0o333333, 0o01001, 3, THREE_ALIGN_CUT), // .1.X1
                                             (0o33333, 0o0011, 9, THREE_ALIGN_CUT),   // .X.11
                                             (0o33333, 0o01100, 0, THREE_ALIGN_CUT),  // .11.X
                                             (0o33333, 0o01001, 6, THREE_ALIGN_CUT),  // .1.X1
                                             (0o33333, 0o00101, 9, THREE_ALIGN_CUT),  // .X1.1
                                             (0o3333, 0o1220, 0, CAPTURE),            // |122X
                                             (0o3333, 0o0221, 9, CAPTURE),            // |X221
                                             (0o33333, 0o22220, 0, ENEMY_FOUR_ALIGN), // 2222X
                                             (0o33333, 0o02222, 12, ENEMY_FOUR_ALIGN),// X2222
                                            ];

pub const MOVE_P2: [(i128, i128, i128, i128); 13] = [
                                             (0o3333, 0o0220, 0, THREE_ALIGN),        // |.22X
                                             (0o3333, 0o0202, 3, THREE_ALIGN),        // |.2X2
                                             (0o3333, 0o0022, 6, THREE_ALIGN),        // |.X22
                                             (0o33333, 0o02020, 0, THREE_ALIGN_CUT),  // .2.2X
                                             (0o333333, 0o02002, 3, THREE_ALIGN_CUT), // .2.X2
                                             (0o3333, 0o0022, 9, THREE_ALIGN_CUT),    // .X.22
                                             (0o33333, 0o02200, 0, THREE_ALIGN_CUT),  // .22.X
                                             (0o33333, 0o02002, 6, THREE_ALIGN_CUT),  // .2.X2
                                             (0o33333, 0o00202, 9, THREE_ALIGN_CUT),  // .X2.2
                                             (0o3333, 0o2110, 0, CAPTURE),            // |211X
                                             (0o3333, 0o0112, 9, CAPTURE),            // |X112
                                             (0o33333, 0o11110, 0, ENEMY_FOUR_ALIGN), // 1111X
                                             (0o33333, 0o01111, 12, ENEMY_FOUR_ALIGN),// X1111
                                            ];

pub const DMOVE_P1: [(i128, i128, i128, i128); 13] = [
                                             (0o3333333, 0o0010100, 0, THREE_ALIGN),          // ||.'1'1'X
                                             (0o3333333, 0o0010001, 6, THREE_ALIGN),          // ||.'1'X'1
                                             (0o3333333, 0o0000101, 12, THREE_ALIGN),         // ||.'X'1'1
                                             (0o333333333, 0o001000100, 0, THREE_ALIGN_CUT),  // .'1'.'1'X
                                             (0o333333333, 0o001000001, 6, THREE_ALIGN_CUT),  // .'1'.'X'1
                                             (0o333333333, 0o000000101, 18, THREE_ALIGN_CUT), // .'X'.'1'1
                                             (0o333333333, 0o001010000, 0, THREE_ALIGN_CUT),  // .'1'1'.'X
                                             (0o333333333, 0o001000001, 12, THREE_ALIGN_CUT), // .'1'X'.'1
                                             (0o333333333, 0o000010001, 18, THREE_ALIGN_CUT), // .'X'1'.'1
                                             (0o3333333, 0o1020200, 0, CAPTURE),              // ||1'2'2'X
                                             (0o3333333, 0o0020201, 18, CAPTURE),             // ||X'2'2'1
                                             (0o333333333, 0o202020200, 0, ENEMY_FOUR_ALIGN), // 2'2'2'2'X
                                             (0o333333333, 0o002020202, 24, ENEMY_FOUR_ALIGN),// X'2'2'2'2
                                            ];

pub const DMOVE_P2: [(i128, i128, i128, i128); 13] = [
                                             (0o3333333, 0o0020200, 0, THREE_ALIGN),          // ||.'2'2'X
                                             (0o3333333, 0o0020002, 6, THREE_ALIGN),          // ||.'2'X'2
                                             (0o3333333, 0o0000202, 12, THREE_ALIGN),         // ||.'X'2'2
                                             (0o333333333, 0o002000200, 0, THREE_ALIGN_CUT),  // .'2'.'2'X
                                             (0o333333333, 0o002000002, 6, THREE_ALIGN_CUT),  // .'2'.'X'2
                                             (0o333333333, 0o000000202, 18, THREE_ALIGN_CUT), // .'X'.'2'2
                                             (0o333333333, 0o002020000, 0, THREE_ALIGN_CUT),  // .'2'2'.'X
                                             (0o333333333, 0o002000002, 12, THREE_ALIGN_CUT), // .'2'X'.'2
                                             (0o333333333, 0o000020002, 18, THREE_ALIGN_CUT), // .'X'2'.'2
                                             (0o3333333, 0o2010100, 0, CAPTURE),              // ||2'1'1'X
                                             (0o3333333, 0o0010102, 18, CAPTURE),             // ||X'1'1'2
                                             (0o333333333, 0o101010100, 0, ENEMY_FOUR_ALIGN), // 1'1'1'1'X
                                             (0o333333333, 0o001010101, 24, ENEMY_FOUR_ALIGN),// X'1'1'1'1
                                            ];

