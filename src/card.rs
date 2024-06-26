use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DragonCard {
    Green,
    White,
    Red,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NumberCard {
    Bamboo,
    Characters,
    Coin,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Card {
    Flower,
    Dragon(DragonCard),
    Number(NumberCard, u8),
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Flower => write!(f, "FL"),
            Card::Dragon(d) => match d {
                DragonCard::Green => write!(f, "DG"),
                DragonCard::White => write!(f, "DW"),
                DragonCard::Red => write!(f, "DR"),
            },
            Card::Number(c, n) => match c {
                NumberCard::Bamboo => write!(f, "G{}", n),
                NumberCard::Characters => write!(f, "B{}", n),
                NumberCard::Coin => write!(f, "R{}", n),
            },
        }
    }
}

impl Card {
    pub fn can_stack_onto(&self, dest: &Card) -> bool {
        let Card::Number(c1, n1) = self else { return false; };
        let Card::Number(c2, n2) = dest else { return false; };
        if c1 == c2 {
            return false;
        }
        n1 + 1 == *n2
    }
}