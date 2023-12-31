use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn strength(&self) -> usize {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

fn get_card_strength(card: char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as usize,
    }
}

fn get_card_strength_with_joker(card: char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as usize,
    }
}

fn compare_hands(hand1: &str, hand2: &str) -> Ordering {
    let type1 = determine_hand_type(hand1);
    let type2 = determine_hand_type(hand2);

    let type1_strength = type1.strength();
    let type2_strength = type2.strength();

    if type1_strength != type2_strength {
        return type1_strength.cmp(&type2_strength);
    }

    // Both hands have the same type; compare cards one by one
    let cards1: Vec<_> = hand1.chars().collect();
    let cards2: Vec<_> = hand2.chars().collect();

    for (card1, card2) in cards1.iter().zip(cards2.iter()) {
        let strength1 = get_card_strength(*card1);
        let strength2 = get_card_strength(*card2);

        match strength1.cmp(&strength2) {
            Ordering::Equal => continue,
            other => return other,
        }
    }

    cards1.len().cmp(&cards2.len()).reverse() // Compare lengths in reverse order
}

fn generate_map_to_joker(hands: &Vec<String>) -> HashMap<String, String> {
    let mut hand_map = HashMap::new();

    for original_hand in hands {
        let j_indices: HashSet<usize> = original_hand
            .char_indices()
            .filter(|&(_, c)| c == 'J')
            .map(|(i, _)| i)
            .collect();

        if j_indices.is_empty() {
            // If there are no 'J' in the original hand, just return the original hand
            hand_map.insert(original_hand.clone(), original_hand.clone());
            continue;
        }

        let mut all_hands = vec![original_hand.clone()];

        for replacement in original_hand.chars().filter(|&c| c != 'J') {
            let mut modified_hand_chars: Vec<char> =
                original_hand.chars().collect();
            for i in &j_indices {
                modified_hand_chars[*i] = replacement;
            }
            let modified_hand: String = modified_hand_chars.iter().collect();
            all_hands.push(modified_hand);
        }

        // Find the hand with maximum strength among all hands
        if let Some(max_strength_hand) = find_max_strength_hand(&all_hands) {
            hand_map.insert(original_hand.clone(), max_strength_hand.clone());
        }
    }

    hand_map
}

fn compare_hands_with_joker(
    hand1: &str,
    joker_hand1: &str,
    hand2: &str,
    joker_hand2: &str,
) -> Ordering {
    let type1 = determine_hand_type(joker_hand1);
    let type2 = determine_hand_type(joker_hand2);

    let type1_strength = type1.strength();
    let type2_strength = type2.strength();

    if type1_strength != type2_strength {
        return type1_strength.cmp(&type2_strength);
    }

    // Both hands have the same type; compare cards one by one
    let cards1: Vec<_> = hand1.chars().collect();
    let cards2: Vec<_> = hand2.chars().collect();

    for (card1, card2) in cards1.iter().zip(cards2.iter()) {
        let strength1 = get_card_strength_with_joker(*card1);
        let strength2 = get_card_strength_with_joker(*card2);

        match strength1.cmp(&strength2) {
            Ordering::Equal => continue,
            other => return other,
        }
    }

    cards1.len().cmp(&cards2.len()).reverse() // Compare lengths in reverse order
}

fn determine_hand_type(hand: &str) -> HandType {
    let card_counts = count_cards(hand);

    let distinct_labels = card_counts.len();

    if card_counts.values().any(|&count| count == 5) {
        HandType::FiveOfAKind
    } else if card_counts.values().any(|&count| count == 4) {
        HandType::FourOfAKind
    } else if card_counts.values().any(|&count| count == 3)
        && distinct_labels == 2
    {
        HandType::FullHouse
    } else if card_counts.values().any(|&count| count == 3) {
        HandType::ThreeOfAKind
    } else if card_counts.values().filter(|&&count| count == 2).count() == 2
        && distinct_labels == 3
    {
        HandType::TwoPair
    } else if card_counts.values().any(|&count| count == 2)
        && distinct_labels == 4
    {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn count_cards(hand: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();

    for card in hand.chars() {
        *counts.entry(card).or_insert(0) += 1;
    }

    counts
}

pub fn calculate_total(result_map: &HashMap<String, u32>) -> (u32, u32) {
    let mut hands: Vec<_> = result_map.keys().cloned().collect();
    hands.sort_by(|a, b| compare_hands(a, b));

    let mut total = 0;
    for (index, hand) in hands.iter().enumerate() {
        let bet = result_map.get(hand).unwrap();
        total += (index as u32 + 1) * bet;
    }

    let joker_map = generate_map_to_joker(&hands);
    hands.sort_by(|a, b| {
        compare_hands_with_joker(
            a,
            joker_map.get(a).unwrap(),
            b,
            joker_map.get(b).unwrap(),
        )
    });

    let mut joker_total = 0;
    for (index, hand) in hands.iter().enumerate() {
        let bet = result_map.get(hand).unwrap();
        joker_total += (index as u32 + 1) * bet;
    }

    (total, joker_total)
}

fn find_max_strength_hand(hands: &Vec<String>) -> Option<String> {
    hands.iter().max_by(|&a, &b| compare_hands(a, b)).cloned()
}

pub fn process_file(file_path: &str) -> HashMap<String, u32> {
    // Create a HashMap to store key-value pairs
    let mut map = HashMap::new();

    // Open the file and create a BufReader to efficiently read lines
    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);

        // Iterate over each line in the file
        for line in reader.lines() {
            if let Ok(line_content) = line {
                // Split the line into key and value
                let mut parts = line_content.split_whitespace();

                if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                    // Parse the value into u32
                    if let Ok(parsed_value) = value.parse::<u32>() {
                        // Insert key-value pair into the HashMap
                        map.insert(key.to_string(), parsed_value);
                    } else {
                        eprintln!("Error parsing value as u32: {}", value);
                    }
                } else {
                    eprintln!("Error parsing line: {}", line_content);
                }
            }
        }
    } else {
        eprintln!("Error opening file: {}", file_path);
    }

    map
}
