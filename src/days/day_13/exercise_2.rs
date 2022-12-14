use std::iter;

use super::Packet;

pub fn solve(pairs: &[(Packet, Packet)]) -> usize {
    let divider_packets: (Packet, Packet) = (
        vec![vec![2.into()].into()].into(),
        vec![vec![6.into()].into()].into(),
    );

    let mut packets: Vec<&Packet> = pairs
        .iter()
        .chain(iter::once(&divider_packets))
        .flat_map(|(left, right)| vec![left, right])
        .collect();

    packets.sort();

    let first_index = packets
        .iter()
        .position(|&packet| *packet == divider_packets.0)
        .unwrap()
        + 1;

    let second_index = packets
        .iter()
        .position(|&packet| *packet == divider_packets.1)
        .unwrap()
        + 1;

    first_index * second_index
}
