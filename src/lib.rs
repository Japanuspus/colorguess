use std::fmt;
use rand;
use rand::prelude::*;

const N_PEG: usize = 4;
const N_COLOR: usize = 8;
const N_SCORE: usize = (N_PEG+2)*(N_PEG+1)/2;


type RawPegs =  [u8; N_PEG];

#[derive(Debug, Clone)]
pub struct Pegs {
    ordered: RawPegs, 
    sorted: RawPegs,
}

pub type ScoreHistogram = [u16; N_SCORE]; //u16 can hold N_COLOR ^ N_PEG

#[derive(Debug, Clone, Copy, PartialEq)]
/// A colored or black key peg is placed for each code peg 
/// from the guess which is correct in both color and position. 
/// A white key peg indicates the existence of a correct color 
/// code peg placed in the wrong position
pub struct Score {
    /// Black/Colored: correct position and color
    pub b: u8,
    /// White: correct color, wrong position
    pub w: u8,
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", "o".repeat(self.b as usize), "w".repeat(self.w as usize))
    }
}


impl Pegs {
    pub fn new(vals: &RawPegs) -> Self 
    {
        let mut p = Pegs {
            ordered: vals.clone(),
            sorted: vals.clone(),
        };
        p.sorted.sort();
        p
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut pegs: RawPegs = [0; N_PEG];
        for i in 0..pegs.len() {
            pegs[i] = rng.gen_range(0 as u8, N_COLOR as u8);
        }
        Pegs::new(&pegs)
    }

    pub fn values<'a>(&'a self) -> &'a RawPegs {
        &self.ordered
    }

    fn count_matches_exact(&self, b: &Pegs) -> u8 {
        self.ordered.iter().zip(b.ordered.iter()).filter(|(ia,ib)| ia==ib).count() as u8
    }

    fn count_matches_color(&self, other: &Pegs) -> u8 {
        let mut ia = 0;
        let mut ib = 0;
        let mut color = 0;
        let a = self.sorted;
        let b = other.sorted;
        while (ia<N_PEG) & (ib<N_PEG) {
            let va = a[ia];
            let vb = b[ib];
            if va==vb {color += 1;}
            if va<=vb {ia+=1;}
            if vb<=va {ib+=1;}
        }
        color
    }

    pub fn score_against(self: &Pegs, b: &Pegs) -> Score {
        let exact = self.count_matches_exact(b);
        let color = self.count_matches_color(b);
        Score {b: exact, w: color-exact}
    }
}

impl fmt::Display for Pegs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.ordered)
    }
}


#[test]
fn test_score() {
    let s = Pegs::new(&[1,2,3,4]).score_against(&Pegs::new(&[1,4,3,3])); 
    assert_eq!((s.b, s.w), (2, 1));

    let s = Pegs::new(&[1,2,3,4]).score_against(&Pegs::new(&[4,3,2,1])); 
    assert_eq!((s.b, s.w), (0, 4));

    let s = Pegs::new(&[1,1,1,1]).score_against(&Pegs::new(&[1,4,3,3])); 
    assert_eq!((s.b, s.w), (1, 0));
}



pub struct BoardRow {
    pub guess: Pegs,
    pub score: Score,
}

pub struct Board {
    pub guesses: Vec<BoardRow>,
    pub possible: Vec<Pegs>,
}




//fn triangle(n: u8) -> u8 { n*(n+1)/2 }

/// Convert score to unique index in range(N_SCORE)
///   |  0  1  2  3  4 (b)
///  -+-------------------
///  0| 10 11 12 13 14 
///  1|  6  7  8  9
///  2|  3  4  5
///  3|  1  2
///  4|  0
/// (w)
fn score_index(s: &Score) -> usize {
    let n = N_PEG - s.w as usize;
    let t = n*(n+1)/2;
    // triangle(N_PEG - s.w as usize);
    (s.b as usize) + t
}

fn list_scores() -> Vec<Score> {
    (0..=N_PEG as u8).flat_map(|n_total| 
        (0..=   n_total).map(move |n_w| Score {b: n_total-n_w,  w: n_w}) 
    ).collect()
}

const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }
fn log_2(x: u16) -> usize {
    num_bits::<u16>() as usize - (x.leading_zeros() as usize)
}

pub fn print_score_histogram(histogram: & ScoreHistogram) {
    for score in list_scores() {
        let ct = histogram[score_index(&score)];
        let nhash = log_2(ct);
        println!("{: >4}: {: <4}  {}", &score, ct, "#".repeat(nhash));
    }
}

#[test]
fn test_score_index() {
    assert_eq!(score_index(&Score {b: 2, w: 1}), 8);
}

pub fn count_outcomes(guess: &Pegs, possible: &[Pegs]) -> ScoreHistogram {
    let mut s = [0; N_SCORE];
    for p in possible {
        s[score_index(&guess.score_against(p)) as usize]+=1;
    }
    s
}

/// Expand peg list by replacing each entry with N_COLOR entries
/// obtained by setting value of Peg[i] to all in 0..N_COLOR
fn expand_peg_list(pegs: &[RawPegs], index: usize) -> Vec<RawPegs> {
    pegs.iter()
    .map(|p: &RawPegs| {
        (0..(N_COLOR as u8)).map(move |v| {let mut pp=p.clone(); pp[index]=v; pp})
    }).flatten().collect()
}

pub fn build_all_configs() -> Vec<Pegs> {
    (0..N_PEG)
    .fold(vec![[0;4]], |p, index| expand_peg_list(&p, index))
    .iter().map(|p| Pegs::new(p)).collect()
}


impl fmt::Display for BoardRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}  > {:?}", self.guess, self.score)
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            guesses: Vec::new(),
            possible: build_all_configs()
        }
    }
    
    pub fn add_guess(&mut self, r: BoardRow) {
        self.possible = self.possible.iter()
            .filter(|p| p.score_against(&r.guess)==r.score)
            .cloned().collect();
        self.guesses.push(r);
    }

    pub fn is_complete(&self) -> bool {
        self.possible.len()<=1 &&
        self.guesses.last().unwrap().score.b == (N_PEG as u8)
    }
}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n *** Board state ***\n")?;
        for g in &self.guesses {
            write!(f, "{}\n", g)?;
        }
        write!(f, "Remaining possible solutions: {}\n", self.possible.len())
    }
}
