use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Symbol {
    Empty,
    PlayerO,
    PlayerX,
}

#[derive(Debug, Clone, Copy)]
struct Board {
    position: [Symbol; 9],
}

impl Board {
    fn new() -> Board {
        return Board {
            position: [Symbol::Empty; 9],
        };
    }

    fn winner(&self) -> Symbol {
        for r in 0..3 {
            if self.position[r * 3] != Symbol::Empty
                && self.position[r * 3] == self.position[r * 3 + 1]
                && self.position[r * 3] == self.position[r * 3 + 2]
            {
                return self.position[r * 3];
            }
        }
        for c in 0..3 {
            if self.position[c] != Symbol::Empty
                && self.position[c] == self.position[c + 3]
                && self.position[c] == self.position[c + 6]
            {
                return self.position[c];
            }
        }
        if self.position[0] != Symbol::Empty
            && self.position[0] == self.position[4]
            && self.position[0] == self.position[8]
        {
            return self.position[0];
        }
        if self.position[2] != Symbol::Empty
            && self.position[2] == self.position[4]
            && self.position[2] == self.position[6]
        {
            return self.position[0];
        }
        Symbol::Empty
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+---+---+---+")?;
        for r in 0..3 {
            write!(f, "|")?;
            for c in 0..3 {
                let sym = match self.position[r * 3 + c] {
                    Symbol::Empty => "   ",
                    Symbol::PlayerO => " O ",
                    Symbol::PlayerX => " X ",
                };
                write!(f, "{}|", sym)?;
            }
            writeln!(f)?;
            writeln!(f, "+---+---+---+")?;
        }
        Ok(())
    }
}

fn main() {
    let mut b = Board::new();
    b.position[0] = Symbol::PlayerX;
    b.position[2] = Symbol::PlayerO;
    b.position[2] = Symbol::PlayerX;
    b.position[4] = Symbol::PlayerX;
    b.position[6] = Symbol::PlayerX;
    println!("{}\nWinner = {:?}", b, b.winner());
}
