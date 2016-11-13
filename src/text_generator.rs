use rand::{Rng, StdRng, SeedableRng};

const BASE_16_CHARSET: &'static [u8] = b"abdhjklmnprstvw_";

const BASE_28_CHARSET: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz .";

const BASE_38_CHARSET: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz .0123456789";

const BASE_62_CHARSET: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                          abcdefghijklmnopqrstuvwxyz\
                                          0123456789";

pub struct TextGenerator<'a, R: 'a + Rng> {
    rng: &'a mut R,
    chars: &'static [u8],
}

pub trait TextGen {
    fn gen_text<'a>(&'a mut self, chars: &'static [u8]) -> TextGenerator<'a, Self>
        where Self: Sized + Rng
    {
        TextGenerator {
            rng: self,
            chars: chars,
        }
    }
}

impl TextGen for StdRng {}

impl<'a, R> TextGenerator<'a, R>
    where R: Sized + Rng
{
    pub fn new(rng: &'a mut R, chars: &'static [u8]) -> TextGenerator<'a, R> {
        TextGenerator {
            rng: rng,
            chars: chars,
        }
    }

    pub fn base_16(rng: &'a mut R) -> TextGenerator<'a, R> {
        Self::new(rng, BASE_16_CHARSET)
    }

    pub fn base_28(rng: &'a mut R) -> TextGenerator<'a, R> {
        Self::new(rng, BASE_28_CHARSET)
    }

    pub fn base_38(rng: &'a mut R) -> TextGenerator<'a, R> {
        Self::new(rng, BASE_38_CHARSET)
    }

    pub fn base_62(rng: &'a mut R) -> TextGenerator<'a, R> {
        Self::new(rng, BASE_62_CHARSET)
    }
}

impl<'a, R: Rng> Iterator for TextGenerator<'a, R> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        Some(*self.rng.choose(self.chars).unwrap() as char)
    }
}

#[test]
fn new_should_generate_text() {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let gen = TextGenerator::new(&mut rng, b"ab");
    let actual: String = gen.take(20).collect();
    let expected = "abbaaabaaabaaaabbaaa";
    assert_eq!(actual, expected);
}

#[test]
fn base28_should_generate_text() {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let gen = TextGenerator::base_28(&mut rng);
    let actual: String = gen.take(56).collect();
    let expected = "kbd eovoymnqik hjw uj mxgceyeilcef yxdf.mognzk.yv v emud";
    assert_eq!(actual, expected);
}

#[test]
fn base38_should_generate_text() {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let gen = TextGenerator::base_38(&mut rng);
    let actual: String = gen.take(76).collect();
    let expected = "cthg06zq8 xisqunz8i4dakfwcy8eqzeu3m6fl7ngi81lq5knkn4wiyprff l f0x9xtui0igtov";
    assert_eq!(actual, expected);
}

#[test]
fn base62_should_generate_text() {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let gen = TextGenerator::base_62(&mut rng);
    let actual: String = gen.take(80).collect();
    let expected = "qjjak8j2KCZmMY6Nxaqo7SWPWqG6eE3UkpQuLjPPoCYLZWnuVQjEOke3rPrMraD8ZjNVEOGaklcVyKqL";
    assert_eq!(actual, expected);
}

#[test]
fn gen_text_should_generate_text() {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let gen = rng.gen_text(b"ab");
    let actual: String = gen.take(20).collect();
    let expected = "abbaaabaaabaaaabbaaa";
    assert_eq!(actual, expected);
}
