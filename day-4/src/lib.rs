#[derive(Debug)]
pub struct IdRange {
    high: u32,
    low: u32,
}

pub trait Contains {
    fn contains_other(&self, other: &Self) -> bool;
}

impl Contains for IdRange {
    fn contains_other(&self, other: &Self) -> bool {
        return self.high >= other.high && self.low <= other.low;
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
    fn a_contains_b_should_be_true() {
        // arrange
        let a = IdRange::from_str("1-3").unwrap();
        let b = IdRange::from_str("2-2").unwrap();

        // act
        let result = a.contains_other(&b);

        //assert
        assert!(result, "test a_contains_b_should_be_true should be true");
    }
}
