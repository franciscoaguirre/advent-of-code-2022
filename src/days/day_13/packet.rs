use std::cmp::{Ord, Ordering};
use std::convert::From;

#[derive(Debug, PartialEq, Eq)]
pub enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl From<u32> for Packet {
    fn from(item: u32) -> Self {
        Self::Number(item)
    }
}

impl From<Vec<Packet>> for Packet {
    fn from(item: Vec<Packet>) -> Self {
        Self::List(item)
    }
}

impl Packet {
    pub fn from(line: &str) -> Self {
        if line.starts_with("[") {
            let mut line_without_brackets = line[1..line.len() - 1].to_string();
            line_without_brackets += ",";
            let subpackets_raw: Vec<String> = line_without_brackets
                .chars()
                .fold(
                    (Vec::new(), String::new(), String::new(), String::new()),
                    |(
                        mut subpackets,
                        mut current_element,
                        mut brackets_helper,
                        mut numbers_helper,
                    ),
                     char| {
                        if char == '[' {
                            current_element.push(char);
                            brackets_helper.push(char);
                        } else if char == ']' {
                            current_element.push(char);
                            brackets_helper.push(char);

                            if numbers_helper.len() > 0 {
                                subpackets.push(numbers_helper.clone());
                                numbers_helper.clear();
                            }

                            let starting_square_bracket_index = brackets_helper.rfind('[').unwrap();
                            let previous_starting_square_bracket =
                                brackets_helper[..starting_square_bracket_index].rfind('[');

                            if let Some(_) = previous_starting_square_bracket {
                                brackets_helper.truncate(starting_square_bracket_index);
                            } else {
                                subpackets.push(current_element.clone());
                                current_element.clear();
                                brackets_helper.clear();
                            }
                        } else if char == ',' {
                            if current_element.len() > 0 {
                                // I'm inside a list
                                current_element.push(char);
                            } else {
                                if numbers_helper.len() > 0 {
                                    subpackets.push(numbers_helper.clone());
                                    numbers_helper.clear();
                                }
                            }
                        } else {
                            // It's a number
                            if current_element.len() > 0 {
                                // I'm inside a list
                                current_element.push(char);
                            } else {
                                numbers_helper.push(char);
                            }
                        }

                        (subpackets, current_element, brackets_helper, numbers_helper)
                    },
                )
                .0;
            let subpackets: Vec<Packet> = subpackets_raw
                .iter()
                .map(|subpacket_raw| Self::from(subpacket_raw))
                .collect();
            Self::List(subpackets)
        } else {
            Self::Number(line.parse().unwrap())
        }
    }
}

fn compare_packets(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (Packet::Number(left_value), Packet::Number(right_value)) => left_value.cmp(&right_value),
        (Packet::List(left_subpackets), Packet::List(right_subpackets)) => {
            let mut left_subpackets_iter = left_subpackets.iter();
            let mut right_subpackets_iter = right_subpackets.iter();

            loop {
                match (left_subpackets_iter.next(), right_subpackets_iter.next()) {
                    (Some(left_subpacket), Some(right_subpacket)) => {
                        let values_ordering = compare_packets(left_subpacket, right_subpacket);

                        if values_ordering == Ordering::Less || values_ordering == Ordering::Greater
                        {
                            return values_ordering;
                        }
                    }
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (None, None) => return Ordering::Equal,
                }
            }
        }
        (Packet::Number(left_value), right_packet) => compare_packets(
            &Packet::List(vec![Packet::Number(*left_value)]),
            right_packet,
        ),
        (left_packet, Packet::Number(right_value)) => compare_packets(
            left_packet,
            &Packet::List(vec![Packet::Number(*right_value)]),
        ),
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_packets(self, other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_packets(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering_works() {
        let first: Packet = vec![1.into(), 1.into(), 3.into(), 1.into(), 1.into()].into();
        let second: Packet = vec![1.into(), 1.into(), 5.into(), 1.into(), 1.into()].into();
        assert!(first < second);

        let first: Packet = vec![
            vec![1.into()].into(),
            vec![2.into(), 3.into(), 4.into()].into(),
        ]
        .into();
        let second: Packet = vec![vec![1.into()].into(), 4.into()].into();
        assert!(first < second);

        let first: Packet = vec![9.into()].into();
        let second: Packet = vec![vec![8.into(), 7.into(), 6.into()].into()].into();
        assert!(first > second);

        let first: Packet = vec![vec![4.into(), 4.into()].into(), 4.into(), 4.into()].into();
        let second: Packet = vec![
            vec![4.into(), 4.into()].into(),
            4.into(),
            4.into(),
            4.into(),
        ]
        .into();
        assert!(first < second);

        let first: Packet = vec![7.into(), 7.into(), 7.into(), 7.into()].into();
        let second: Packet = vec![7.into(), 7.into(), 7.into()].into();
        assert!(first > second);

        let first: Packet = vec![].into();
        let second: Packet = vec![3.into()].into();
        assert!(first < second);

        let first: Packet = vec![vec![vec![].into()].into()].into();
        let second: Packet = vec![vec![].into()].into();
        assert!(first > second);

        let first: Packet = vec![
            1.into(),
            vec![
                2.into(),
                vec![
                    3.into(),
                    vec![4.into(), vec![5.into(), 6.into(), 7.into()].into()].into(),
                ]
                .into(),
            ]
            .into(),
            8.into(),
            9.into(),
        ]
        .into();
        let second: Packet = vec![
            1.into(),
            vec![
                2.into(),
                vec![
                    3.into(),
                    vec![4.into(), vec![5.into(), 6.into(), 0.into()].into()].into(),
                ]
                .into(),
            ]
            .into(),
            8.into(),
            9.into(),
        ]
        .into();
        assert!(first > second);
    }
}
