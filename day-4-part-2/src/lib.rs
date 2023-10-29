#[derive(Debug)]
pub struct IdRange {
    high: u32,
    low: u32,
}

pub trait Overlaps {
    fn overlaps_with_other(&self, other: &Self) -> bool;
}

impl Overlaps for IdRange {
    fn overlaps_with_other(&self, other: &Self) -> bool {
        if self.high > other.high && self.low > other.high {
            return false;
        }
        if self.high < other.high && self.high < other.low {
            return false;
        }
        return true;
    }
}

pub trait FromStr {
    fn from_str(range: &str) -> Result<IdRange, &str>;
}

impl FromStr for IdRange {
    fn from_str(range: &str) -> Result<IdRange, &str> {
        let high_and_low = range.split("-").collect::<Vec<_>>();
        let low_char = high_and_low[0];
        let high_char = high_and_low[1];

        let (Ok(high), Ok(low)) = (high_char.parse::<u32>(), low_char.parse::<u32>()) else {
            return Err("Error parsing str");
        };

        Ok(IdRange { high, low })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_does_not_overlap() {
        // arrange
        let a = IdRange::from_str("1-3").unwrap();
        let b = IdRange::from_str("4-6").unwrap();

        // act
        let result = a.overlaps_with_other(&b);

        //assert
        assert!(!result, "test overlaps should be false");
    }

    #[test]
    fn higher_does_not_overlap() {
        // arrange
        // 10 > 4,
        let a = IdRange::from_str("10-30").unwrap();
        let b = IdRange::from_str("4-6").unwrap();

        // act
        let result = a.overlaps_with_other(&b);

        //assert
        assert!(!result, "test overlaps should be false");
    }
}
