use crate::{
    board::bitboard::BitBoard,
    error::{Square64ConversionError, SquareConversionError},
    util::{Color, File, Rank, FILES_BOARD, RANKS_BOARD, SQUARE_120_TO_64, SQUARE_64_TO_120},
};
use num::Integer;
use std::{
    fmt,
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumIter, EnumString};

#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumString, EnumCountMacro)]
#[rustfmt::skip]
#[strum(use_phf)]
pub enum Square64 {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl From<Square> for Square64 {
    fn from(square_120: Square) -> Self {
        SQUARE_120_TO_64[square_120 as usize]
            .expect("10x12 Square should have a corresponding 8x8 Square64")
    }
}

impl Add<usize> for Square64 {
    type Output = Result<Self, Square64ConversionError>;
    fn add(self, rhs: usize) -> Self::Output {
        let result: Result<Self, Square64ConversionError> = (self as usize + rhs).try_into();
        result
    }
}

impl TryFrom<u8> for Square64 {
    type Error = Square64ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::iter()
            .find(|s| *s as u8 == value)
            .ok_or(Square64ConversionError::FromU8(value))
    }
}

impl TryFrom<u32> for Square64 {
    type Error = Square64ConversionError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::iter()
            .find(|s| *s as u32 == value)
            .ok_or(Square64ConversionError::FromU32(value))
    }
}

impl TryFrom<usize> for Square64 {
    type Error = Square64ConversionError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::iter()
            .find(|s| *s as usize == value)
            .ok_or(Square64ConversionError::FromUsize(value))
    }
}

impl Square64 {
    pub fn from_file_and_rank(file: File, rank: Rank) -> Self {
        let index_64 = (file as u8) + (rank as u8) * 8;
        index_64.try_into().expect(
            "index_64 should map to valid Square64 since rank and file must be in range 0..=7",
        )
    }

    pub fn get_file(&self) -> File {
        FILES_BOARD[*self as usize].expect("should return valid File since every Square64 has one")
    }

    pub fn get_rank(&self) -> Rank {
        RANKS_BOARD[*self as usize].expect("should return valid File since every Square64 has one")
    }

    pub fn get_color(&self) -> Color {
        // NOTE: Rank1's value as u8 is 0
        // XOR relationship:
        // (rank_even, index_even) -> black
        // (rank_even, index_odd) -> white
        // (rank_odd, index_even) -> white
        // (rank_odd, index_odd) -> black
        let parity_xor = (self.get_rank() as u8).is_even() ^ (*self as u8).is_even();
        match parity_xor {
            true => Color::White,
            false => Color::Black,
        }
    }
}

#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumString, EnumCountMacro)]
#[rustfmt::skip]
#[strum(use_phf)]
pub enum Square {
    A1 = 21, B1, C1, D1, E1, F1, G1, H1,
    A2 = 31, B2, C2, D2, E2, F2, G2, H2,
    A3 = 41, B3, C3, D3, E3, F3, G3, H3,
    A4 = 51, B4, C4, D4, E4, F4, G4, H4,
    A5 = 61, B5, C5, D5, E5, F5, G5, H5,
    A6 = 71, B6, C6, D6, E6, F6, G6, H6,
    A7 = 81, B7, C7, D7, E7, F7, G7, H7,
    A8 = 91, B8, C8, D8, E8, F8, G8, H8,
}

impl From<Square64> for Square {
    fn from(square_64: Square64) -> Self {
        SQUARE_64_TO_120[square_64 as usize]
            .expect("8x8 Square64 should have a corresponding 10x12 Square")
    }
}

impl Add<i8> for Square {
    type Output = Result<Self, SquareConversionError>;
    fn add(self, rhs: i8) -> Self::Output {
        let result = (self as i8 + rhs).try_into();
        result
    }
}

impl Sub<i8> for Square {
    type Output = Result<Self, SquareConversionError>;
    fn sub(self, rhs: i8) -> Self::Output {
        let result = (self as i8 - rhs).try_into();
        result
    }
}

impl TryFrom<i8> for Square {
    type Error = SquareConversionError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::iter()
            .find(|s| *s as i8 == value)
            .ok_or(SquareConversionError::FromI8(value))
    }
}

impl TryFrom<u8> for Square {
    type Error = SquareConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::iter()
            .find(|s| *s as u8 == value)
            .ok_or(SquareConversionError::FromU8(value))
    }
}

impl TryFrom<u32> for Square {
    type Error = SquareConversionError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::iter()
            .find(|s| *s as u32 == value)
            .ok_or(SquareConversionError::FromU32(value))
    }
}

impl TryFrom<usize> for Square {
    type Error = SquareConversionError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::iter()
            .find(|s| *s as usize == value)
            .ok_or(SquareConversionError::FromUsize(value))
    }
}

impl Square {
    pub fn from_file_and_rank(file: File, rank: Rank) -> Self {
        let index_120 = (21 + (file as u8) + (10 * (rank as u8)));
        index_120.try_into().expect(
            "index_64 should map to valid Square since rank and file must be in range 0..=7",
        )
    }

    pub fn get_file(&self) -> File {
        FILES_BOARD[*self as usize].expect("should return valid File since every Square has one")
    }

    pub fn get_rank(&self) -> Rank {
        RANKS_BOARD[*self as usize].expect("should return valid Rank since every Square has one")
    }

    pub fn get_color(&self) -> Color {
        // NOTE: Rank1's value as u8 is 0
        // XOR relationship:
        // (rank_even, index_even) -> white
        // (rank_even, index_odd) -> black
        // (rank_odd, index_even) -> black
        // (rank_odd, index_odd) -> white
        let parity_xor = (self.get_rank() as u8).is_even() ^ (*self as u8).is_even();
        match parity_xor {
            true => Color::Black,
            false => Color::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gamestate::NUM_BOARD_SQUARES;

    // Conversions
    #[test]
    fn test_square_120_to_square_64() {
        let input = Square::A6;
        let output: Square64 = input.into();
        let expected = Square64::A6;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_64_to_square_120() {
        let input = Square64::A6;
        let output: Square = input.into();
        let expected = Square::A6;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_64_addition_usize_valid() {
        let lhs = Square64::B1;
        let rhs: usize = 8;
        let output = lhs + rhs;
        let expected = Ok(Square64::B2);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_64_addition_usize_valid_zero() {
        let lhs = Square64::B1;
        let rhs: usize = 0;
        let output = lhs + rhs;
        let expected = Ok(Square64::B1);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_64_addition_usize_invalid() {
        let lhs = Square64::B1;
        let rhs: usize = 63;
        let output = lhs + rhs;
        let expected = Err(Square64ConversionError::FromUsize(lhs as usize + rhs));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_120_try_from_str_valid() {
        let input = "D2";
        let output = Square::try_from(input);
        let expected = Ok(Square::D2);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_120_try_from_str_lowercase_invalid() {
        let input = "d2";
        let output = Square::try_from(input).map_err(Into::into);
        let expected = Err(SquareConversionError::FromStr(
            strum::ParseError::VariantNotFound,
        ));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_120_try_from_str_invalid() {
        let input = "INVALID";
        let output = Square::try_from(input).map_err(Into::into);
        let expected = Err(SquareConversionError::FromStr(
            strum::ParseError::VariantNotFound,
        ));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_120_to_string() {
        let input = Square::D2;
        let output: String = input.to_string();
        let expected = "D2".to_owned();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_64_try_from_str_valid() {
        let input = "D2";
        let output = Square64::try_from(input);
        let expected = Ok(Square64::D2);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_64_try_from_str_lowercase_invalid() {
        let input = "d2";
        let output = Square64::try_from(input).map_err(Into::into);
        let expected = Err(Square64ConversionError::FromStr(
            strum::ParseError::VariantNotFound,
        ));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_64_try_from_str_invalid() {
        let input = "INVALID";
        let output = Square64::try_from(input).map_err(Into::into);
        let expected = Err(Square64ConversionError::FromStr(
            strum::ParseError::VariantNotFound,
        ));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_64_to_string() {
        let input = Square64::D2;
        let output: String = input.to_string();
        let expected = "D2".to_owned();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_120_try_from_u8_valid() {
        let input: u8 = 34;
        let output = Square::try_from(input);
        let expected = Ok(Square::D2);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_120_try_from_u8_invalid() {
        let input: u8 = 11;
        let output = Square::try_from(input);
        let expected = Err(SquareConversionError::FromU8(11));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_64_try_from_u8_valid() {
        let input: u8 = 34;
        let output = Square64::try_from(input);
        let expected = Ok(Square64::C5);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_64_try_from_u8_invalid() {
        let input: u8 = 64;
        let output = Square64::try_from(input);
        let expected = Err(Square64ConversionError::FromU8(64));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_120_try_from_u32_valid() {
        let input: u32 = 34;
        let output = Square::try_from(input);
        let expected = Ok(Square::D2);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_120_try_from_u32_invalid() {
        let input: u32 = 11;
        let output = Square::try_from(input);
        let expected = Err(SquareConversionError::FromU32(11));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_64_try_from_u32_valid() {
        let input: u32 = 34;
        let output = Square64::try_from(input);
        let expected = Ok(Square64::C5);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_64_try_from_u32_invalid() {
        let input: u32 = 64;
        let output = Square64::try_from(input);
        let expected = Err(Square64ConversionError::FromU32(64));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_120_try_from_usize_valid() {
        let input: usize = 34;
        let output = Square::try_from(input);
        let expected = Ok(Square::D2);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_120_try_from_usize_invalid() {
        let input: usize = 11;
        let output = Square::try_from(input);
        let expected = Err(SquareConversionError::FromUsize(11));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_64_try_from_usize_valid() {
        let input: usize = 34;
        let output = Square64::try_from(input);
        let expected = Ok(Square64::C5);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_square_64_try_from_usize_invalid() {
        let input: usize = 64;
        let output = Square64::try_from(input);
        let expected = Err(Square64ConversionError::FromUsize(64));
        assert_eq!(output, expected);
    }

    // Other methods
    #[test]
    fn test_from_file_and_rank() {
        let square = Square::from_file_and_rank(File::FileB, Rank::Rank3);
        assert_eq!(square, Square::B3);
    }

    #[test]
    fn test_get_file() {
        let output = Square::H7.get_file();
        let expected = File::FileH;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_get_rank() {
        let output = Square::H7.get_rank();
        let expected = Rank::Rank7;
        assert_eq!(output, expected);
    }
}
