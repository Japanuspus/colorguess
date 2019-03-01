use itertools::Itertools;
use std::fmt;
use rand;
use rand::prelude::*;

const N_PEG: usize = 4;
const N_COLOR: usize = 8;
const N_SCORE: usize = (N_PEG+2)*(N_PEG+1)/2;

pub type Pegs = [u8; N_PEG];
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

pub struct BoardRow {
    pub guess: Pegs,
    pub score: Score,
}

pub struct Board {
    pub guesses: Vec<BoardRow>,
    pub possible: Vec<Pegs>,
}

fn count_matches_exact(a: &Pegs, b: &Pegs) -> u8 {
    a.iter().zip(b.iter()).filter(|(ia,ib)| ia==ib).count() as u8
}

#[test]
fn test_count_matches_exact() {
    assert_eq!(2, count_matches_exact(&[1,2,3,4], &[1,4,3,3]));
    assert_eq!(0, count_matches_exact(&[1,2,3,4], &[4,3,2,1]));
    assert_eq!(1, count_matches_exact(&[1,1,1,1], &[4,3,2,1]));
}

pub fn count_matches_color(a: &Pegs, b: &Pegs) -> u8 {
    let mut ia = a.into_iter().sorted().peekable();
    let mut ib = b.into_iter().sorted().peekable();
    let mut color: u8 = 0;
    loop {
        let adv = match (ia.peek(), ib.peek()) {
            (Some(ca), Some(cb)) => {
                //if ca==cb {color+=1;}
                (ca<=cb, cb<=ca)
            },
            _ => break,
        };
        if adv.0 {ia.next();}
        if adv.1 {ib.next();}
        if adv.0 && adv.1 {color+=1;}
    };
    color
}

// 3 times faster than sorting...
pub fn count_matches_color_nosort(a: &Pegs, b: &Pegs) -> u8 {
    let mut counts = [0u8; N_COLOR];
    // Only count for those colors present in a
    for va in a {
        counts[*va as usize] = a
        .into_iter()
        .filter(|v| **v==*va)
        .zip(
            b.into_iter().filter(|v| **v==*va)
        ).count() as u8;
    }
    counts.iter().sum()
}


pub fn get_random_pegs() -> Pegs {
    let mut rng = rand::thread_rng();
    let mut pegs: Pegs = [0; N_PEG];
    for i in 0..pegs.len() {
        pegs[i] = rng.gen_range(0 as u8, N_COLOR as u8);
    }
    pegs
}

#[test]
fn test_count_matches_color() {

    let f = count_matches_color;
    assert_eq!(3, f(&[1,2,3,4], &[1,4,3,3]));
    assert_eq!(4, f(&[1,2,3,4], &[4,3,2,1]));
    assert_eq!(1, f(&[1,1,1,1], &[4,3,2,1]));
}

#[test]
fn test_count_matches_color_nosort() {

    let f = count_matches_color_nosort;
    assert_eq!(3, f(&[1,2,3,4], &[1,4,3,3]));
    assert_eq!(4, f(&[1,2,3,4], &[4,3,2,1]));
    assert_eq!(1, f(&[1,1,1,1], &[4,3,2,1]));
}

pub fn get_score(a: &Pegs, b: &Pegs) -> Score {
    let exact = count_matches_exact(a, b);
    let color = count_matches_color(a, b);
    Score {b: exact, w: color-exact}
}

#[test]
fn test_score() {
    let s = get_score(&[1,2,3,4], &[1,4,3,3]); 
    assert_eq!((s.b, s.w), (2, 1));
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
        s[score_index(&get_score(guess, p)) as usize]+=1;
    }
    s
}

/// Expand peg list by replacing each entry with N_COLOR entries
/// obtained by setting value of Peg[i] to all in 0..N_COLOR
fn expand_peg_list(pegs: &[Pegs], index: usize) -> Vec<Pegs> {
    pegs.iter()
    .map(|p: &Pegs| {
        (0..(N_COLOR as u8)).map(move |v| {let mut pp=p.clone(); pp[index]=v; pp})
    }).flatten().collect()
}

pub fn build_all_configs() -> Vec<Pegs> {
    (0..N_PEG).fold(vec![[0;4]], |p, index| expand_peg_list(&p, index))
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
            .filter(|p| get_score(p, &r.guess)==r.score)
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
