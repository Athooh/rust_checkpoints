#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        match self {
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive } => 
                matches!(other.antigen, Antigen::A | Antigen::O) && other.rh_factor == RhFactor::Positive || other.rh_factor == RhFactor::Negative,
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive } => 
                other.antigen == Antigen::O && (other.rh_factor == RhFactor::Positive || other.rh_factor == RhFactor::Negative),
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive } => 
                matches!(other.antigen, Antigen::B | Antigen::O) && other.rh_factor == RhFactor::Positive || other.rh_factor == RhFactor::Negative,
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive } => true,
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative } => 
                matches!(other.antigen, Antigen::A | Antigen::O) && other.rh_factor == RhFactor::Negative,
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative } => 
                other.antigen == Antigen::O && other.rh_factor == RhFactor::Negative,
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative } => 
                matches!(other.antigen, Antigen::B | Antigen::O) && other.rh_factor == RhFactor::Negative,
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative } => 
                other.rh_factor == RhFactor::Negative,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let all_blood_types = vec![
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
        ];

        all_blood_types.into_iter()
            .filter(|bt| self.can_receive_from(bt))
            .collect()
    }

    pub fn recipients(&self) -> Vec<Self> {
        let all_blood_types = vec![
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
        ];

        all_blood_types.into_iter()
            .filter(|bt| bt.can_receive_from(self))
            .collect()
    }
}
