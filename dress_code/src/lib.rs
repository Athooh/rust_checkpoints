// #[derive(Debug, PartialEq, Eq)]
// pub enum Jacket {
//     Black,
//     White,
//     Flowers,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub enum Hat {
//     Snapback,
//     Baseball,
//     Fedora,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub struct Outfit {
//     pub jacket: Jacket,
//     pub hat: Hat,
// }

// pub fn choose_outfit(formality_level: Option<u32>, invitation_message: Result<&str, &str>) -> Outfit {
//     let jacket = match formality_level {
//         Some(level) if level > 0 => Jacket::White,
//         Some(_) => Jacket::Black,
//         None => Jacket::Flowers,
//     };

//     let hat = match invitation_message {
//         Ok(_) => Hat::Fedora,
//         Err(_) => Hat::Snapback,
//     };

//     if formality_level.is_none() && invitation_message.is_err() {
//         return Outfit {
//             jacket: Jacket::Flowers,
//             hat: Hat::Baseball,
//         };
//     }

//     Outfit { jacket, hat }
// }

#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black,
    White,
    Flowers,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Hat {
    Snapback,
    Baseball,
    Fedora,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}

pub fn choose_outfit(formality_level: Option<u32>, invitation_message: Result<&str, &str>) -> Outfit {
    let jacket = if let Some(level) = formality_level {
        if level > 0 {
            Jacket::White
        } else {
            Jacket::Black
        }
    } else {
        Jacket::Flowers
    };

    let hat = if invitation_message.is_ok() {
        Hat::Fedora
    } else {
        Hat::Snapback
    };

    if formality_level.is_none() && invitation_message.is_err() {
        return Outfit {
            jacket: Jacket::Flowers,
            hat: Hat::Baseball,
        };
    }

    Outfit { jacket, hat }
}