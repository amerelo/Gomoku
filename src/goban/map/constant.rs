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

pub const IS_CAPTURABLE_P1: [(i128, i128, i128); 8] = [
                                             (0o3333, 0o2110, 3),
                                             (0o3333, 0o2110, 6),
                                             (0o3333, 0o0112, 3),
                                             (0o3333, 0o0112, 6),
                                             (0o3333333, 0o2010100, 6),
                                             (0o3333333, 0o2010100, 12),
                                             (0o3333333, 0o0010102, 6),
                                             (0o3333333, 0o0010102, 12),
                                            ];

pub const IS_CAPTURABLE_P2: [(i128, i128, i128); 8] = [
                                             (0o3333, 0o1220, 3),
                                             (0o3333, 0o1220, 6),
                                             (0o3333, 0o0221, 3),
                                             (0o3333, 0o0221, 6),
                                             (0o3333333, 0o1020200, 6),
                                             (0o3333333, 0o1020200, 12),
                                             (0o3333333, 0o0020201, 6),
                                             (0o3333333, 0o0020201, 12),
                                            ];


pub const THREE_MOVE_P1: [(i128, i128, i128); 9] = [
                                             (0o33333, 0o01100, 3),
                                             (0o33333, 0o01010, 6),
                                             (0o33333, 0o00110, 9),
                                             (0o333333, 0o010100, 3),
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
                                             (0o333333, 0o020200, 3),
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

const THREE_ALIGN: i128 = 3;
const THREE_ALIGN_CUT: i128 = 2;
pub const CAPTURE: i128 = 6;
const ENEMY_CAPTURE: i128 = -1;
const ENEMY_FOUR_ALIGN: i128 = 8;
const ENEMY_THREE_ALIGN: i128 = 7;
const ENEMY_TWO_ALIGN: i128 = 2;
const TWO_ALIGN: i128 = 1;
const FOUR_ALIGN: i128 = 7;
const FIVE_ALIGN: i128 = 40;
const ENEMY_FIVE_ALIGN: i128 = 9;

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
**  /*N D*/ = /*08 03*/
**
**  N = number in the array
**  D = shift checked for x/y
**
*/

pub const MOVE_P1: [(i128, i128, i128, i128); 43] = [
                            /*00 00*/        (0o3333, 0o0110, 0, THREE_ALIGN),        // |.11X
                            /*01 01*/        (0o3333, 0o0101, 3, THREE_ALIGN),        // |.1X1
                            /*02 02*/        (0o3333, 0o0011, 6, THREE_ALIGN + 2),        // |.X11
                            /*03 00*/        (0o33333, 0o01010, 0, THREE_ALIGN_CUT),  // .1.1X
                            /*04 01*/        (0o33333, 0o01001, 3, THREE_ALIGN_CUT),  // .1.X1
                            /*05 03*/        (0o33333, 0o0011, 9, THREE_ALIGN_CUT),   // .X.11
                            /*06 00*/        (0o33333, 0o01100, 0, THREE_ALIGN_CUT),  // .11.X
                            /*07 02*/        (0o33333, 0o01001, 6, THREE_ALIGN_CUT),  // .1X.1
                            /*08 03*/        (0o33333, 0o00101, 9, THREE_ALIGN_CUT),  // .X1.1
                            /*09 00*/        (0o3333, 0o1220, 0, CAPTURE),            // |122X
                            /*10 03*/        (0o3333, 0o0221, 9, CAPTURE),            // |X221
                            /*11 00*/        (0o33333, 0o22220, 0, ENEMY_FOUR_ALIGN), // 2222X
                            /*12 04*/        (0o33333, 0o02222, 12, ENEMY_FOUR_ALIGN),// X2222
                            /*13 00*/        (0o3333, 0o2220, 0, ENEMY_THREE_ALIGN),  // |222X
                            /*14 03*/        (0o3333, 0o0222, 9, ENEMY_THREE_ALIGN),  // |X222
                            /*15 01*/        (0o3333, 0o2202, 3, ENEMY_THREE_ALIGN),  // |22X2
                            /*16 02*/        (0o3333, 0o2022, 6, ENEMY_THREE_ALIGN),  // |2X22
                            /*17 01*/        (0o33333, 0o11100, 3, FOUR_ALIGN),       // 111X.
                            /*18 02*/        (0o33333, 0o11010, 6, FOUR_ALIGN),       // 11X1.
                            /*19 03*/        (0o33333, 0o10110, 9, FOUR_ALIGN),       // 1X11.
                            /*20 04*/        (0o33333, 0o01110, 12, FOUR_ALIGN),      // X111.
                            /*21 00*/        (0o33333, 0o01110, 0, FOUR_ALIGN),       // .111X
                            /*22 01*/        (0o33333, 0o01101, 3, FOUR_ALIGN),       // .11X1
                            /*23 02*/        (0o33333, 0o01011, 6, FOUR_ALIGN),       // .1X11
                            /*24 03*/        (0o33333, 0o00111, 9, FOUR_ALIGN),       // .X111
                            /*25 00*/        (0o33333, 0o11110, 0, FIVE_ALIGN),       // 1111X
                            /*26 01*/        (0o33333, 0o11101, 3, FIVE_ALIGN),       // 111X1
                            /*27 02*/        (0o33333, 0o11011, 6, FIVE_ALIGN),       // 11X11
                            /*28 03*/        (0o33333, 0o10111, 9, FIVE_ALIGN),       // 1X111
                            /*29 04*/        (0o33333, 0o01111, 12, FIVE_ALIGN),      // X1111
                            /*30 00*/        (0o33333, 0o22220, 0, ENEMY_FIVE_ALIGN), // 2222X
                            /*31 01*/        (0o33333, 0o22202, 3, ENEMY_FIVE_ALIGN), // 222X2
                            /*32 02*/        (0o33333, 0o22022, 6, ENEMY_FIVE_ALIGN), // 22X22
                            /*33 03*/        (0o33333, 0o20222, 9, ENEMY_FIVE_ALIGN), // 2X222
                            /*34 04*/        (0o33333, 0o02222, 12, ENEMY_FIVE_ALIGN),// X2222
                            /*35 01*/        (0o3333, 0o0102, 3, ENEMY_CAPTURE),      // |01X2
                            /*36 02*/        (0o3333, 0o0012, 6, ENEMY_CAPTURE),      // |0X12
                            /*37 01*/        (0o3333, 0o2100, 3, ENEMY_CAPTURE),      // |2X10
                            /*38 02*/        (0o3333, 0o2010, 6, ENEMY_CAPTURE),      // |21X0
                            /*39 02*/        (0o3333, 0o0022, 6, ENEMY_TWO_ALIGN),    // |0X22
                            /*40 01*/        (0o3333, 0o2200, 3, ENEMY_TWO_ALIGN),    // |22X0
                            /*41 01*/        (0o3333, 0o0100, 3, TWO_ALIGN),          // |01X0
                            /*42 02*/        (0o3333, 0o0010, 6, TWO_ALIGN),          // |0X10

                                            ];

pub const MOVE_P2: [(i128, i128, i128, i128); 43] = [
                            /*00 00*/        (0o3333, 0o0220, 0, THREE_ALIGN),        // |.22X
                            /*01 01*/        (0o3333, 0o0202, 3, THREE_ALIGN),        // |.2X2
                            /*02 02*/        (0o3333, 0o0022, 6, THREE_ALIGN  + 2),        // |.X22
                            /*03 00*/        (0o33333, 0o02020, 0, THREE_ALIGN_CUT),  // .2.2X
                            /*04 01*/        (0o33333, 0o02002, 3, THREE_ALIGN_CUT),  // .2.X2
                            /*05 03*/        (0o3333, 0o0022, 9, THREE_ALIGN_CUT),    // .X.22
                            /*06 00*/        (0o33333, 0o02200, 0, THREE_ALIGN_CUT),  // .22.X
                            /*07 02*/        (0o33333, 0o02002, 6, THREE_ALIGN_CUT),  // .2.X2
                            /*08 03*/        (0o33333, 0o00202, 9, THREE_ALIGN_CUT),  // .X2.2
                            /*09 00*/        (0o3333, 0o2110, 0, CAPTURE),            // |211X
                            /*10 03*/        (0o3333, 0o0112, 9, CAPTURE),            // |X112
                            /*11 00*/        (0o33333, 0o11110, 0, ENEMY_FOUR_ALIGN), // 1111X
                            /*12 04*/        (0o33333, 0o01111, 12, ENEMY_FOUR_ALIGN),// X1111
                            /*13 00*/        (0o3333, 0o1110, 0, ENEMY_THREE_ALIGN),  // |111X
                            /*14 03*/        (0o3333, 0o0111, 9, ENEMY_THREE_ALIGN),  // |X111
                            /*15 01*/        (0o3333, 0o1101, 3, ENEMY_THREE_ALIGN),  // |11X1
                            /*16 02*/        (0o3333, 0o1011, 6, ENEMY_THREE_ALIGN),  // |1X11
                            /*17 01*/        (0o33333, 0o22200, 3, FOUR_ALIGN),       // 222X.
                            /*18 02*/        (0o33333, 0o22020, 6, FOUR_ALIGN),       // 22X2.
                            /*19 03*/        (0o33333, 0o20220, 9, FOUR_ALIGN),       // 2X22.
                            /*20 04*/        (0o33333, 0o02220, 12, FOUR_ALIGN),      // X222.
                            /*21 00*/        (0o33333, 0o02220, 0, FOUR_ALIGN),       // .222X
                            /*22 01*/        (0o33333, 0o02202, 3, FOUR_ALIGN),       // .22X2
                            /*23 02*/        (0o33333, 0o02022, 6, FOUR_ALIGN),       // .2X22
                            /*24 03*/        (0o33333, 0o00222, 9, FOUR_ALIGN),       // .X222
                            /*25 00*/        (0o33333, 0o22220, 0, FIVE_ALIGN),       // 2222X
                            /*26 01*/        (0o33333, 0o22202, 3, FIVE_ALIGN),       // 222X2
                            /*27 02*/        (0o33333, 0o22022, 6, FIVE_ALIGN),       // 22X22
                            /*28 03*/        (0o33333, 0o20222, 9, FIVE_ALIGN),       // 2X222
                            /*29 04*/        (0o33333, 0o02222, 12, FIVE_ALIGN),      // X2222
                            /*30 00*/        (0o33333, 0o11110, 0, ENEMY_FIVE_ALIGN), // 1111X
                            /*31 01*/        (0o33333, 0o11101, 3, ENEMY_FIVE_ALIGN), // 111X1
                            /*32 02*/        (0o33333, 0o11011, 6, ENEMY_FIVE_ALIGN), // 11X11
                            /*33 03*/        (0o33333, 0o10111, 9, ENEMY_FIVE_ALIGN), // 1X111
                            /*34 04*/        (0o33333, 0o01111, 12, ENEMY_FIVE_ALIGN),// X1111
                            /*35 01*/        (0o3333, 0o0201, 3, ENEMY_CAPTURE),      // |02X1
                            /*36 02*/        (0o3333, 0o0021, 6, ENEMY_CAPTURE),      // |0X21
                            /*37 01*/        (0o3333, 0o1200, 3, ENEMY_CAPTURE),      // |1X20
                            /*38 02*/        (0o3333, 0o1020, 6, ENEMY_CAPTURE),      // |12X0
                            /*39 02*/        (0o3333, 0o0011, 6, ENEMY_TWO_ALIGN),    // |0X11
                            /*40 01*/        (0o3333, 0o1100, 3, ENEMY_TWO_ALIGN),    // |11X0
                            /*41 01*/        (0o3333, 0o0200, 3, TWO_ALIGN),          // |02X0
                            /*42 02*/        (0o3333, 0o0020, 6, TWO_ALIGN),          // |0X20

                                            ];

pub const DMOVE_P1: [(i128, i128, i128, i128); 43] = [
                            /*00 00*/        (0o3333333, 0o0010100, 0, THREE_ALIGN),          // ||.'1'1'X
                            /*01 02*/        (0o3333333, 0o0010001, 6, THREE_ALIGN),          // ||.'1'X'1
                            /*02 04*/        (0o3333333, 0o0000101, 12, THREE_ALIGN + 2),         // ||.'X'1'1
                            /*03 00*/        (0o333333333, 0o001000100, 0, THREE_ALIGN_CUT),  // .'1'.'1'X
                            /*04 02*/        (0o333333333, 0o001000001, 6, THREE_ALIGN_CUT),  // .'1'.'X'1
                            /*05 06*/        (0o333333333, 0o000000101, 18, THREE_ALIGN_CUT), // .'X'.'1'1
                            /*06 00*/        (0o333333333, 0o001010000, 0, THREE_ALIGN_CUT),  // .'1'1'.'X
                            /*07 04*/        (0o333333333, 0o001000001, 12, THREE_ALIGN_CUT), // .'1'X'.'1
                            /*08 06*/        (0o333333333, 0o000010001, 18, THREE_ALIGN_CUT), // .'X'1'.'1
                            /*09 00*/        (0o3333333, 0o1020200, 0, CAPTURE),              // ||1'2'2'X
                            /*10 06*/        (0o3333333, 0o0020201, 18, CAPTURE),             // ||X'2'2'1
                            /*11 00*/        (0o333333333, 0o202020200, 0, ENEMY_FOUR_ALIGN), // 2'2'2'2'X
                            /*12 08*/        (0o333333333, 0o002020202, 24, ENEMY_FOUR_ALIGN),// X'2'2'2'2
                            /*13 00*/        (0o3333333, 0o2020200, 0, ENEMY_THREE_ALIGN),    // ||2'2'2'X
                            /*14 06*/        (0o3333333, 0o0020202, 18, ENEMY_THREE_ALIGN),   // ||X'2'2'2
                            /*15 02*/        (0o3333333, 0o2020002, 6, ENEMY_THREE_ALIGN),    // |2'2'X'2
                            /*16 03*/        (0o3333333, 0o2000202, 12, ENEMY_THREE_ALIGN),    // |2'X'2'2
                            /*17 02*/        (0o333333333, 0o101010000, 6, FOUR_ALIGN),       // 1'1'1'X'.
                            /*18 04*/        (0o333333333, 0o101000100, 12, FOUR_ALIGN),      // 1'1'X'1'.
                            /*19 06*/        (0o333333333, 0o100010100, 18, FOUR_ALIGN),      // 1'X'1'1'.
                            /*20 08*/        (0o333333333, 0o001010100, 24, FOUR_ALIGN),      // X'1'1'1'.
                            /*21 00*/        (0o333333333, 0o001010100, 0, FOUR_ALIGN),       // .'1'1'1'X
                            /*22 02*/        (0o333333333, 0o001010001, 6, FOUR_ALIGN),       // .'1'1'X'1
                            /*23 04*/        (0o333333333, 0o001000101, 12, FOUR_ALIGN),      // .'1'X'1'1
                            /*24 06*/        (0o333333333, 0o000010101, 18, FOUR_ALIGN),      // .'X'1'1'1
                            /*25 00*/        (0o333333333, 0o101010100, 0, FIVE_ALIGN),       // 1'1'1'1'X
                            /*26 02*/        (0o333333333, 0o101010001, 6, FIVE_ALIGN),       // 1'1'1'X'1
                            /*27 04*/        (0o333333333, 0o101000101, 12, FIVE_ALIGN),      // 1'1'X'1'1
                            /*28 06*/        (0o333333333, 0o100010101, 18, FIVE_ALIGN),      // 1'X'1'1'1
                            /*29 08*/        (0o333333333, 0o001010101, 24, FIVE_ALIGN),      // X'1'1'1'1
                            /*30 00*/        (0o333333333, 0o202020200, 0, ENEMY_FIVE_ALIGN), // 2'2'2'2'X
                            /*31 02*/        (0o333333333, 0o202020002, 6, ENEMY_FIVE_ALIGN), // 2'2'2'X'2
                            /*32 04*/        (0o333333333, 0o202000202, 12, ENEMY_FIVE_ALIGN),// 2'2'X'2'2
                            /*33 06*/        (0o333333333, 0o200020202, 18, ENEMY_FIVE_ALIGN),// 2'X'2'2'2
                            /*34 08*/        (0o333333333, 0o002020202, 24, ENEMY_FIVE_ALIGN),// X'2'2'2'2
                            /*35 02*/        (0o3333333, 0o0010002, 6, ENEMY_CAPTURE),        // ||0'1'X'2
                            /*36 04*/        (0o3333333, 0o0000102, 12, ENEMY_CAPTURE),       // ||0'X'1'2
                            /*37 02*/        (0o3333333, 0o2010000, 6, ENEMY_CAPTURE),        // ||2'X'1'0
                            /*38 04*/        (0o3333333, 0o2000100, 12, ENEMY_CAPTURE),       // ||2'1'X'0
                            /*39 04*/        (0o3333333, 0o0000202, 12, ENEMY_TWO_ALIGN),     // |0'X'2'2
                            /*40 02*/        (0o3333333, 0o2020000, 6, ENEMY_TWO_ALIGN),      // |2'2'X'0
                            /*41 02*/        (0o3333333, 0o0010000, 6, TWO_ALIGN),            // |0'1'X'0
                            /*42 04*/        (0o3333333, 0o0000100, 12, TWO_ALIGN),           // |0'X'1'0

                                            ];

pub const DMOVE_P2: [(i128, i128, i128, i128); 43] = [
                            /*00 00*/        (0o3333333, 0o0020200, 0, THREE_ALIGN),          // ||.'2'2'X
                            /*01 02*/        (0o3333333, 0o0020002, 6, THREE_ALIGN),          // ||.'2'X'2
                            /*02 04*/        (0o3333333, 0o0000202, 12, THREE_ALIGN + 2),         // ||.'X'2'2
                            /*03 00*/        (0o333333333, 0o002000200, 0, THREE_ALIGN_CUT),  // .'2'.'2'X
                            /*04 02*/        (0o333333333, 0o002000002, 6, THREE_ALIGN_CUT),  // .'2'.'X'2
                            /*05 06*/        (0o333333333, 0o000000202, 18, THREE_ALIGN_CUT), // .'X'.'2'2
                            /*06 00*/        (0o333333333, 0o002020000, 0, THREE_ALIGN_CUT),  // .'2'2'.'X
                            /*07 04*/        (0o333333333, 0o002000002, 12, THREE_ALIGN_CUT), // .'2'X'.'2
                            /*08 06*/        (0o333333333, 0o000020002, 18, THREE_ALIGN_CUT), // .'X'2'.'2
                            /*09 00*/        (0o3333333, 0o2010100, 0, CAPTURE),              // ||2'1'1'X
                            /*10 06*/        (0o3333333, 0o0010102, 18, CAPTURE),             // ||X'1'1'2
                            /*11 00*/        (0o333333333, 0o101010100, 0, ENEMY_FOUR_ALIGN), // 1'1'1'1'X
                            /*12 08*/        (0o333333333, 0o001010101, 24, ENEMY_FOUR_ALIGN),// X'1'1'1'1
                            /*13 00*/        (0o3333333, 0o1010100, 0, ENEMY_THREE_ALIGN),    // ||1'1'1'X
                            /*14 06*/        (0o3333333, 0o0010101, 18, ENEMY_THREE_ALIGN),   // ||X'1'1'1
                            /*15 02*/        (0o3333333, 0o1010001, 6, ENEMY_THREE_ALIGN),    // |1'1'X'1
                            /*16 03*/        (0o3333333, 0o1000101, 12, ENEMY_THREE_ALIGN),   // |1'X'1'1
                            /*17 02*/        (0o333333333, 0o202020000, 6, FOUR_ALIGN),       // 2'2'2'X'.
                            /*18 04*/        (0o333333333, 0o202000200, 12, FOUR_ALIGN),      // 2'2'X'2'.
                            /*19 06*/        (0o333333333, 0o200020200, 18, FOUR_ALIGN),      // 2'X'2'2'.
                            /*20 08*/        (0o333333333, 0o002020200, 24, FOUR_ALIGN),      // X'2'2'2'.
                            /*21 00*/        (0o333333333, 0o002020200, 0, FOUR_ALIGN),       // .'2'2'2'X
                            /*22 02*/        (0o333333333, 0o002020002, 6, FOUR_ALIGN),       // .'2'2'X'2
                            /*23 04*/        (0o333333333, 0o002000202, 12, FOUR_ALIGN),      // .'2'X'2'2
                            /*24 06*/        (0o333333333, 0o000020202, 18, FOUR_ALIGN),      // .'X'2'2'2
                            /*25 00*/        (0o333333333, 0o202020200, 0, FIVE_ALIGN),       // 2'2'2'2'X
                            /*26 02*/        (0o333333333, 0o202020002, 6, FIVE_ALIGN),       // 2'2'2'X'2
                            /*27 04*/        (0o333333333, 0o202000202, 12, FIVE_ALIGN),      // 2'2'X'2'2
                            /*28 06*/        (0o333333333, 0o200020202, 18, FIVE_ALIGN),      // 2'X'2'2'2
                            /*29 08*/        (0o333333333, 0o002020202, 24, FIVE_ALIGN),      // X'2'2'2'2
                            /*30 00*/        (0o333333333, 0o101010100, 0, ENEMY_FIVE_ALIGN), // 1'1'1'1'X
                            /*31 02*/        (0o333333333, 0o101010001, 6, ENEMY_FIVE_ALIGN), // 1'1'1'X'1
                            /*32 04*/        (0o333333333, 0o101000101, 12, ENEMY_FIVE_ALIGN),// 1'1'X'1'1
                            /*33 06*/        (0o333333333, 0o100010101, 18, ENEMY_FIVE_ALIGN),// 1'X'1'1'1
                            /*34 08*/        (0o333333333, 0o001010101, 24, ENEMY_FIVE_ALIGN),// X'1'1'1'1
                            /*35 02*/        (0o3333333, 0o0020001, 6, ENEMY_CAPTURE),        // ||0'2'X'1
                            /*36 04*/        (0o3333333, 0o0000201, 12, ENEMY_CAPTURE),       // ||0'X'2'1
                            /*37 02*/        (0o3333333, 0o1020000, 6, ENEMY_CAPTURE),        // ||1'X'2'0
                            /*38 04*/        (0o3333333, 0o1000200, 12, ENEMY_CAPTURE),       // ||1'2'X'0
                            /*39 04*/        (0o3333333, 0o0000101, 12, ENEMY_TWO_ALIGN),     // |0'X'1'1
                            /*40 02*/        (0o3333333, 0o1010000, 6, ENEMY_TWO_ALIGN),      // |1'1'X'0
                            /*41 02*/        (0o3333333, 0o0020000, 6, TWO_ALIGN),            // |0'2'X'0
                            /*42 04*/        (0o3333333, 0o0000200, 12, TWO_ALIGN),           // |0'X'2'0

                                            ];

