use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd)]
enum CardType {
    J, /* move to after T for part 1 */
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

const CHAR_TO_CARD_TYPE: [(char, CardType); 13] = [
    ('2', CardType::Two),
    ('3', CardType::Three),
    ('4', CardType::Four),
    ('5', CardType::Five),
    ('6', CardType::Six),
    ('7', CardType::Seven),
    ('8', CardType::Eight),
    ('9', CardType::Nine),
    ('T', CardType::T),
    ('J', CardType::J),
    ('Q', CardType::Q),
    ('K', CardType::K),
    ('A', CardType::A),
];

fn main() {
    let input: Vec<String> = read_to_string("./input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut card_pairs: Vec<(Vec<CardType>, u64)> = vec![];
    for line in input.into_iter() {
        let (cards, bid) = line.split_once(' ').unwrap();

        let mut cards_vec: Vec<CardType> = Vec::new();
        for c in cards.chars() {
            for (chr, card_type) in CHAR_TO_CARD_TYPE {
                if c == chr {
                    cards_vec.push(card_type);
                }
            }
        }

        let bid = bid.parse::<u64>().unwrap();

        card_pairs.push((cards_vec, bid));
    }

    /* part 1 */
    card_pairs.sort_by(|(lhs, _), (rhs, _)| {
        let mut lmap = HashMap::new();
        let mut rmap = HashMap::new();
        for c in lhs {
            *lmap.entry(c).or_insert(0) += 1;
        }
        for c in rhs {
            *rmap.entry(c).or_insert(0) += 1;
        }

        let mut lvec: Vec<u32> = lmap.into_values().collect();
        lvec.sort_by(|a, b| b.cmp(a)); /* reverse order */
        let mut rvec: Vec<u32> = rmap.into_values().collect();
        rvec.sort_by(|a, b| b.cmp(a)); /* reverse order */

        if lvec == rvec {
            for i in 0..lhs.len() {
                if lhs[i] != rhs[i] {
                    return lhs[i].partial_cmp(&rhs[i]).unwrap();
                }
            }
            return Ordering::Equal;
        }
        lvec.cmp(&rvec)
    });

    // println!("{:?}", card_pairs);
    print_total(&card_pairs);

    /* part 2 */
    card_pairs.sort_by(|(lhs, _), (rhs, _)| {
        let mut lmap = HashMap::new();
        let mut rmap = HashMap::new();
        let mut ljoker = 0;
        let mut rjoker = 0;

        for c in lhs {
            if *c == CardType::J {
                ljoker += 1;
            } else {
                *lmap.entry(c).or_insert(0) += 1;
            }
        }
        for c in rhs {
            if *c == CardType::J {
                rjoker += 1;
            } else {
                *rmap.entry(c).or_insert(0) += 1;
            }
        }

        let mut lvec: Vec<u32> = lmap.into_values().collect();
        lvec.sort_by(|a, b| b.cmp(a)); /* reverse order */
        if let Some(x) = lvec.first_mut() {
            *x += ljoker;
        } else {
            lvec.push(5);
        }

        let mut rvec: Vec<u32> = rmap.into_values().collect();
        rvec.sort_by(|a, b| b.cmp(a)); /* reverse order */
        if let Some(x) = rvec.first_mut() {
            *x += rjoker;
        } else {
            rvec.push(5);
        }

        if lvec == rvec {
            for i in 0..lhs.len() {
                if lhs[i] != rhs[i] {
                    return lhs[i].partial_cmp(&rhs[i]).unwrap();
                }
            }
            return Ordering::Equal;
        }

        // println!("{:?} {:?} {:?} {:?}", lhs, rhs, lvec, rvec);
        lvec.cmp(&rvec)
    });
    // println!("{:?}", card_pairs);
    print_total(&card_pairs);
}

fn print_total(card_pairs: &[(Vec<CardType>, u64)]) {
    let mut total = 0;
    let mut rank = 1;
    for (_, bid) in card_pairs {
        total += bid * rank;
        rank += 1;
    }
    println!("{total}");
}
