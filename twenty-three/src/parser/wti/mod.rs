pub enum Int {
    Hit(u32),
    Miss,
}

pub fn parse<Iter: Iterator<Item = char>>(iter: &mut std::iter::Peekable<Iter>) -> Int {
    _parse(iter, Word::Start)
}

enum Word {
    // Start ////////////////////////////////////
    Start,
    // O ////////////////////////////////////////
    /////////////////////////////////////////////
    // one
    O,
    On,
    // two reversed
    Ow,
    // T ////////////////////////////////////////
    /////////////////////////////////////////////
    // two
    T,
    Tw,
    // three
    Th,
    Thr,
    Thre,
    // eight reversed
    Thg,
    Thgi,
    // F ////////////////////////////////////////
    /////////////////////////////////////////////
    // four
    F,
    Fo,
    Fou,
    // five
    Fi,
    Fiv,
    // S ////////////////////////////////////////
    /////////////////////////////////////////////
    // six
    S,
    Si,
    // seven
    Se,
    Sev,
    Seve,
    // E ////////////////////////////////////////
    /////////////////////////////////////////////
    // eight
    E,
    Ei,
    Eig,
    Eigh,
    // one reversed
    En,
    // three reversed
    Ee,
    Eer,
    Eerh,
    // five reversed
    Ev,
    Evi,
    // nine reversed
    Eni,
    // N ////////////////////////////////////////
    /////////////////////////////////////////////
    // nine
    N,
    Ni,
    Nin,
    // seven reversed
    Ne,
    Nev,
    Neve,
    // R ////////////////////////////////////////
    /////////////////////////////////////////////
    // four reversed
    R,
    Ru,
    Ruo,
    // X ////////////////////////////////////////
    /////////////////////////////////////////////
    // six reversed
    X,
    Xi,
}

enum Consume {
    ConsumeWord(Word),
    ConsumeInt(Int),
    Exit(Int),
}

fn _parse<Iter: Iterator<Item = char>>(iter: &mut std::iter::Peekable<Iter>, word: Word) -> Int {
    let next = iter.peek();
    let consume = match next {
        Some(char) => match word {
            Word::Start => {
                // parsing number literals first
                // before invoking this method
                // so always consume first el
                match char {
                    'o' => Consume::ConsumeWord(Word::O),
                    't' => Consume::ConsumeWord(Word::T),
                    'f' => Consume::ConsumeWord(Word::F),
                    's' => Consume::ConsumeWord(Word::S),
                    'e' => Consume::ConsumeWord(Word::E),
                    'n' => Consume::ConsumeWord(Word::N),
                    'r' => Consume::ConsumeWord(Word::R),
                    'x' => Consume::ConsumeWord(Word::X),
                    _ => Consume::ConsumeInt(Int::Miss),
                }
            }
            Word::O => match char {
                'n' => Consume::ConsumeWord(Word::On),
                'w' => Consume::ConsumeWord(Word::Ow),
                _ => Consume::Exit(Int::Miss),
            },
            Word::On => match char {
                'e' => Consume::ConsumeInt(Int::Hit(1)),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Ow => match char {
                't' => Consume::ConsumeInt(Int::Hit(2)),
                _ => Consume::Exit(Int::Miss),
            },
            Word::T => match char {
                'w' => Consume::ConsumeWord(Word::Tw),
                'h' => Consume::ConsumeWord(Word::Th),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Tw => match char {
                'o' => Consume::ConsumeInt(Int::Hit(2)),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Th => match char {
                'r' => Consume::ConsumeWord(Word::Thr),
                'g' => Consume::ConsumeWord(Word::Thg),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Thr => match char {
                'e' => Consume::ConsumeWord(Word::Thre),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Thg => match char {
                'i' => Consume::ConsumeWord(Word::Thgi),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Thre => match char {
                'e' => Consume::ConsumeInt(Int::Hit(3)),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Thgi => match char {
                'e' => Consume::ConsumeInt(Int::Hit(8)),
                _ => Consume::Exit(Int::Miss),
            },
            Word::F => match char {
                'o' => Consume::ConsumeWord(Word::Fo),
                'i' => Consume::ConsumeWord(Word::Fi),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Fo => match char {
                'u' => Consume::ConsumeWord(Word::Fou),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Fi => match char {
                'v' => Consume::ConsumeWord(Word::Fiv),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Fou => match char {
                'r' => Consume::ConsumeInt(Int::Hit(4)),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Fiv => match char {
                'e' => Consume::ConsumeInt(Int::Hit(5)),
                _ => Consume::Exit(Int::Miss),
            },
            Word::S => match char {
                'i' => Consume::ConsumeWord(Word::Si),
                'e' => Consume::ConsumeWord(Word::Se),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Si => match char {
                'x' => Consume::ConsumeInt(Int::Hit(6)),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Se => match char {
                'v' => Consume::ConsumeWord(Word::Sev),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Sev => match char {
                'e' => Consume::ConsumeWord(Word::Seve),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Seve => match char {
                'n' => Consume::ConsumeInt(Int::Hit(7)),
                _ => Consume::Exit(Int::Miss),
            },
            Word::E => match char {
                'i' => Consume::ConsumeWord(Word::Ei),
                'n' => Consume::ConsumeWord(Word::En),
                'e' => Consume::ConsumeWord(Word::Ee),
                'v' => Consume::ConsumeWord(Word::Ev),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Ei => match char {
                'g' => Consume::ConsumeWord(Word::Eig),
                _ => Consume::Exit(Int::Miss),
            },
            Word::En => match char {
                'o' => Consume::ConsumeInt(Int::Hit(1)),
                'i' => Consume::ConsumeWord(Word::Eni),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Ee => match char {
                'r' => Consume::ConsumeWord(Word::Eer),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Ev => match char {
                'i' => Consume::ConsumeWord(Word::Evi),
                _ => Consume::Exit(Int::Miss),
            },
            //////////////////////////////////////////////////////
            Word::Eig => match char {
                'h' => Consume::ConsumeWord(Word::Eigh),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Eni => match char {
                'n' => Consume::ConsumeInt(Int::Hit(9)),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Eer => match char {
                'h' => Consume::ConsumeWord(Word::Eerh),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Evi => match char {
                'f' => Consume::ConsumeInt(Int::Hit(5)),
                _ => Consume::Exit(Int::Miss),
            },
            //////////////////////////////////////////////////////
            Word::Eigh => match char {
                't' => Consume::ConsumeInt(Int::Hit(8)),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Eerh => match char {
                't' => Consume::ConsumeInt(Int::Hit(3)),
                _ => Consume::Exit(Int::Miss),
            },
            Word::N => match char {
                'i' => Consume::ConsumeWord(Word::Ni),
                'e' => Consume::ConsumeWord(Word::Ne),
                _ => Consume::Exit(Int::Miss),
            },
            //////////////////////////////////////////////////////
            Word::Ni => match char {
                'n' => Consume::ConsumeWord(Word::Nin),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Ne => match char {
                'v' => Consume::ConsumeWord(Word::Nev),
                _ => Consume::Exit(Int::Miss),
            },
            //////////////////////////////////////////////////////
            Word::Nin => match char {
                'e' => Consume::ConsumeInt(Int::Hit(9)),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Nev => match char {
                'e' => Consume::ConsumeWord(Word::Neve),
                _ => Consume::Exit(Int::Miss),
            },
            //////////////////////////////////////////////////////
            Word::Neve => match char {
                's' => Consume::ConsumeInt(Int::Hit(7)),
                _ => Consume::Exit(Int::Miss),
            },
            //////////////////////////////////////////////////////
            Word::R => match char {
                'u' => Consume::ConsumeWord(Word::Ru),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Ru => match char {
                'o' => Consume::ConsumeWord(Word::Ruo),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Ruo => match char {
                'f' => Consume::ConsumeInt(Int::Hit(4)),
                _ => Consume::Exit(Int::Miss),
            },
            //////////////////////////////////////////////////////
            Word::X => match char {
                'i' => Consume::ConsumeWord(Word::Xi),
                _ => Consume::Exit(Int::Miss),
            },
            Word::Xi => match char {
                's' => Consume::ConsumeInt(Int::Hit(6)),
                _ => Consume::Exit(Int::Miss),
            },
        },
        None => Consume::Exit(Int::Miss),
    };
    match consume {
        Consume::ConsumeInt(int) => {
            iter.next();
            int
        }
        Consume::ConsumeWord(word) => {
            iter.next();
            _parse(iter, word)
        }
        Consume::Exit(int) => int,
    }
}
