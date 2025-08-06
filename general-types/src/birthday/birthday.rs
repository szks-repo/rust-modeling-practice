use std::convert::TryFrom;
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Birthday {
    value: DateTime<Utc>,
}

impl TryFrom<&str> for Birthday {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self, Self::Error> {
        Ok(Self {
            value: DateTime::parse_from_rfc3339(s)?.to_utc()
        })
    }
}

impl<Tz: TimeZone> From<DateTime<Tz>> for Birthday {
    fn from(datetime: DateTime<Tz>) -> Self {
        Self {
            value: datetime.to_utc()
        }
    }
}

impl Birthday {
    pub fn get_age(&self, now: DateTime<Utc>) -> i32 {
        let age = now.year() - self.value.year();
        let birthday_not_reached = now.month() < self.value.month()
            || (now.month() == self.value.month() && now.day() < self.value.day());

        if birthday_not_reached {
            age - 1
        } else {
            age
        }
    }
    pub fn get_age_with_timezone<Tz: TimeZone>(&self, now: DateTime<Tz>) -> i32 {
        self.get_age(now.to_utc())
    }

    pub fn get_value(&self) -> DateTime<Utc> {
        self.value
    }
    pub fn get_value_with_timezone<Tz: TimeZone>(self, timezone: Tz) -> DateTime<Tz> {
        self.value.with_timezone(&timezone)
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use crate::birthday::Birthday;

    #[test]
    fn test_birthday_try_from() {
        let today = DateTime::parse_from_rfc3339("2025-08-02T20:00:00Z");
        let result = Birthday::try_from("2001-08-03T00:00:00Z");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get_age(today.unwrap().to_utc()), 23);
    }

}
