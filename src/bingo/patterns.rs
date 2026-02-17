use crate::bingo::game::GameType;

type Pattern = Vec<u8>;
type RoundWinningPatterns = Vec<Pattern>;
pub type GamePatterns = Vec<RoundWinningPatterns>;

//classic
const CLASSIC_COLUMN_1: [u8; 5] = [1, 2, 3, 4, 5];
const CLASSIC_COLUMN_2: [u8; 5] = [6, 7, 8, 9, 10];
const CLASSIC_COLUMN_3: [u8; 5] = [11, 12, 13, 14, 15];
const CLASSIC_COLUMN_4: [u8; 5] = [16, 17, 18, 19, 20];
const CLASSIC_COLUMN_5: [u8; 5] = [21, 22, 23, 24, 25];
const CLASSIC_ROW_1: [u8; 5] = [1, 6, 11, 16, 21];
const CLASSIC_ROW_2: [u8; 5] = [2, 7, 12, 17, 22];
const CLASSIC_ROW_3: [u8; 5] = [3, 8, 13, 18, 23];
const CLASSIC_ROW_4: [u8; 5] = [4, 9, 14, 19, 24];
const CLASSIC_ROW_5: [u8; 5] = [5, 10, 15, 20, 25];
const DIAGONAL_1: [u8; 5] = [1, 7, 13, 19, 25];
const DIAGONAL_2: [u8; 5] = [5, 9, 13, 17, 21];
const SHAPE_PLUS: [u8; 9] = [3, 8, 11, 12, 13, 14, 15, 18, 23];
const FULLHOUSE: [u8; 25] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];

//british
const BRITISH_ROW_1: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
const BRITISH_ROW_2: [u8; 9] = [10, 11, 12, 13, 14, 15, 16, 17, 18];
const BRITISH_ROW_3: [u8; 9] = [19, 20, 21, 22, 23, 24, 25, 26, 27];
const BRITISH_ROW_1_AND_2: [u8; 18] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18];
const BRITISH_ROW_1_AND_3: [u8; 18] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 19, 20, 21, 22, 23, 24, 25, 26, 27];
const BRITISH_ROW_2_AND_3: [u8; 18] = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27];
const BRITISH_FULLHOUSE: [u8; 27] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27,
];

//picture
const TOP_LEFT_2X2: [u8; 4] = [1, 2, 6, 7];
const TOP_RIGHT_2X2: [u8; 4] = [21, 22, 16, 17];
const BOTTOM_LEFT_2X2: [u8; 4] = [4, 5, 9, 10];
const BOTTOM_RIGHT_2X2: [u8; 4] = [24, 25, 19, 20];
const TOP_LEFT_3X3: [u8; 9] = [1, 2, 3, 6, 7, 8, 11, 12, 13];
const TOP_RIGHT_3X3: [u8; 9] = [21, 22, 23, 16, 17, 18, 11, 12, 13];
const BOTTOM_LEFT_3X3: [u8; 9] = [3, 4, 5, 8, 9, 10, 13, 14, 15];
const BOTTOM_RIGHT_3X3: [u8; 9] = [23, 24, 25, 18, 19, 20, 13, 14, 15];

pub fn get_game_patterns(game_type: &GameType) -> GamePatterns {
    match game_type {
        GameType::Classic => { 
            vec![
                vec![
                    CLASSIC_COLUMN_1.to_vec(),
                    CLASSIC_COLUMN_2.to_vec(),
                    CLASSIC_COLUMN_3.to_vec(),
                    CLASSIC_COLUMN_4.to_vec(),
                    CLASSIC_COLUMN_5.to_vec(),
                    CLASSIC_ROW_1.to_vec(),
                    CLASSIC_ROW_2.to_vec(),
                    CLASSIC_ROW_3.to_vec(),
                    CLASSIC_ROW_4.to_vec(),
                    CLASSIC_ROW_5.to_vec(),
                    DIAGONAL_1.to_vec(),
                    DIAGONAL_2.to_vec(),
                ],
                vec![
                    match rand::random_range(0..1) {
                        0 => SHAPE_PLUS,
                        _ => SHAPE_PLUS
                    }.to_vec()

                ],
                vec![
                    FULLHOUSE.to_vec(),
                ]
            ]
        },
        GameType::British => {
            vec![
                vec![
                    BRITISH_ROW_1.to_vec(),
                    BRITISH_ROW_2.to_vec(),
                    BRITISH_ROW_3.to_vec()
                ],
                vec![
                    BRITISH_ROW_1_AND_2.to_vec(),
                    BRITISH_ROW_1_AND_3.to_vec(),
                    BRITISH_ROW_2_AND_3.to_vec()
                ],
                vec![
                    BRITISH_FULLHOUSE.to_vec()
                ],

            ]
        },
        GameType::Picture => {
            vec![
                vec![
                    TOP_LEFT_2X2.to_vec(),
                    TOP_RIGHT_2X2.to_vec(),
                    BOTTOM_LEFT_2X2.to_vec(),
                    BOTTOM_RIGHT_2X2.to_vec()
                ],
                vec![
                    TOP_LEFT_3X3.to_vec(),
                    TOP_RIGHT_3X3.to_vec(),
                    BOTTOM_LEFT_3X3.to_vec(),
                    BOTTOM_RIGHT_3X3.to_vec()
                ],
                vec![
                    FULLHOUSE.to_vec()
                ]
            ]
        }
    }

}






