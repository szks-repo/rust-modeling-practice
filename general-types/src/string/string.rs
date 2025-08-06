use anyhow::anyhow;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MinDefinedString<const N: usize> {
    value: String,
}

pub type NonEmptyString = MinDefinedString<1>;

impl <const N: usize> TryFrom<&str> for MinDefinedString<N> {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.chars().count() < N {
            Err(anyhow!("String must have at least {} characters", N))
        } else {
            Ok(MinDefinedString{value: s.to_string()})
        }
    }
}
impl<const N: usize> MinDefinedString<N> {
    pub fn into_string(self) -> String {
        self.value
    }
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MaxDefinedString<const N: usize> {
    value: String,
}

impl <const N: usize> TryFrom<&str> for MaxDefinedString<N> {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.chars().count() > N {
            Err(anyhow!("String exceeds maximum length {} characters", N))
        } else {
            Ok(MaxDefinedString{value: s.to_string()})
        }
    }
}
impl<const N: usize> MaxDefinedString<N> {
    pub fn into_string(self) -> String {
        self.value
    }
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MinMaxDefinedString<const MIN: usize, const MAX: usize> {
    value: String,
}

impl <const MIN: usize, const MAX: usize> TryFrom<&str> for MinMaxDefinedString<MIN, MAX> {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let count = s.chars().count();
        if count < MIN {
            Err(anyhow!("String must have at least {} characters", MIN))
        } else if count > MAX {
            Err(anyhow!("String exceeds maximum length {} characters", MAX))
        } else {
            Ok(MinMaxDefinedString{value: s.to_string()})
        }
    }
}

impl<const MIN: usize, const MAX: usize> MinMaxDefinedString<MIN, MAX> {
    pub fn into_string(self) -> String {
        self.value
    }
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LenDefinedString<const N: usize> {
    value: String
}

impl <const N: usize> TryFrom<&str> for LenDefinedString<N> {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.chars().count() != N {
            Err(anyhow!("String expected length {} characters", N))
        } else {
            Ok(Self{value: s.to_string()})
        }
    }
}

impl<const N: usize> LenDefinedString<N> {
    pub fn into_string(self) -> String {
        self.value
    }
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use anyhow::Error;

    #[test]
    fn test_non_empty_string_success() {
        let result = NonEmptyString::try_from("a");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "a")
    }

    #[test]
    fn test_non_empty_string_failure () {
        let result = NonEmptyString::try_from("");
        assert!(result.is_err());
    }

    #[test]
    fn test_max_string_success() {
        let result: Result<MinDefinedString<5>, Error> = MinDefinedString::try_from("12345");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "12345");
    }

    #[test]
    fn test_max_string_failure() {
        let result: Result<MaxDefinedString<5>, Error> = MaxDefinedString::try_from("123456");
        assert!(result.is_err());
    }

    #[test]
    #[should_panic]
    fn test_max_string_failure_panic() {
        let _: MaxDefinedString<5> = MaxDefinedString::try_from("123456").unwrap();
    }
}