/**************************************
- Author: Clement Poncelet
- Desc: Contains:
    - Type Truth: (true, false or unknown)
***************************************/

/**************************************
            Type Truth
***************************************/

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Truth {
    True,
    False,
    Unknown,
}

impl Truth {
    pub fn is_true(self) -> bool {
        matches!(self, Truth::True)
    }

    pub fn is_false(self) -> bool {
        matches!(self, Truth::False)
    }

    pub fn is_unknown(self) -> bool {
        matches!(self, Truth::Unknown)
    }

    pub fn to_bool(self) -> Option<bool> {
        match self {
            Truth::True => Some(true),
            Truth::False => Some(false),
            Truth::Unknown => None,
        }
    }
}

impl Truth {
    pub fn or(self, other: Truth) -> Truth {
        match (self, other) {
            (Truth::True, _)    | (_, Truth::True)      => Truth::True,
            (Truth::Unknown, _) | (_, Truth::Unknown)   => Truth::Unknown,
            _ => Truth::False,
        }
    }

    pub fn and(self, other: Truth) -> Truth {
        match (self, other) {
            (Truth::False, _)   | (_, Truth::False)     => Truth::False,
            (Truth::Unknown, _) | (_, Truth::Unknown)   => Truth::Unknown,
            _ => Truth::True,
        }
    }
}

impl std::ops::Not for Truth {
    type Output = Truth;

    fn not(self) -> Truth {
        match self {
            Truth::True => Truth::False,
            Truth::False => Truth::True,
            Truth::Unknown => Truth::Unknown,
        }
    }
}

impl std::ops::BitAnd for Truth {
    type Output = Truth;

    fn bitand(self, rhs: Truth) -> Truth {
        match (self, rhs) {
            (Truth::False, _) | (_, Truth::False) => Truth::False,
            (Truth::True, Truth::True) => Truth::True,
            _ => Truth::Unknown,
        }
    }
}

impl std::ops::BitOr for Truth {
    type Output = Truth;

    fn bitor(self, rhs: Truth) -> Truth {
        match (self, rhs) {
            (Truth::True, _) | (_, Truth::True) => Truth::True,
            (Truth::False, Truth::False) => Truth::False,
            _ => Truth::Unknown,
        }
    }
}

impl From<bool> for Truth {
    fn from(b: bool) -> Self {
        if b { Truth::True } else { Truth::False }
    }
}

/**************************************
            Unit Tests
***************************************/

#[cfg(test)]
mod tests {
    use crate::csp::truth::Truth;

    #[test]
    fn truth_equality() {
        assert_eq!(Truth::True, Truth::True);
        assert_eq!(Truth::False, Truth::False);
        assert_eq!(Truth::Unknown, Truth::Unknown);
        assert_ne!(Truth::True, Truth::False);
    }

    #[test]
    fn truth_helpers() {
        assert!(Truth::True.is_true());
        assert!(!Truth::True.is_false());
        assert!(!Truth::True.is_unknown());

        assert!(Truth::False.is_false());
        assert!(Truth::Unknown.is_unknown());
    }

    #[test]
    fn truth_not() {
        assert_eq!(!Truth::True, Truth::False);
        assert_eq!(!Truth::False, Truth::True);
        assert_eq!(!Truth::Unknown, Truth::Unknown);
    }
}
