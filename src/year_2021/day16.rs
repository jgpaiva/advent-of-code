#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    assert_eq!(parse_char('F'), [true, true, true, true]);
    assert_eq!(parse_char('a'), [true, false, true, false]);
    assert_eq!(parse_char('1'), [false, false, false, true]);
    assert_eq!(bits_as_usize(&[true, true, false]), 6);
    assert_eq!(
        parse_string("D2".to_string()),
        [true, true, false, true, false, false, true, false]
    );
    assert_eq!(
        parse_packet_string("D2FE28".to_string()),
        Packet {
            t: PacketType::Literal { value: 2021 },
            version: 6,
            len: 21,
        }
    );
    let input = utils::read_file("2021/test_day16-1");
    assert_eq!(part1(input.clone()), 16);
    let input = utils::read_file("2021/test_day16-2");
    assert_eq!(part1(input.clone()), 12);
    let input = utils::read_file("2021/test_day16-3");
    assert_eq!(part1(input.clone()), 23);
    let input = utils::read_file("2021/test_day16-4");
    assert_eq!(part1(input.clone()), 31);
    assert_eq!(part2("C200B40A82".to_string()), 3);
    assert_eq!(part2("04005AC33890".to_string()), 54);
    assert_eq!(part2("880086C3E88112".to_string()), 7);
    assert_eq!(part2("9C0141080250320F1802104A08".to_string()), 1);
}

fn parse_char(input: char) -> Vec<bool> {
    let parsed = u8::from_str_radix(input.to_string().as_ref(), 16).unwrap();
    [8, 4, 2, 1]
        .into_iter()
        .map(|i: u8| parsed & i != 0)
        .collect()
}

fn parse_string(input: String) -> Vec<bool> {
    input.chars().flat_map(parse_char).collect::<Vec<bool>>()
}

pub fn part2(input: String) -> usize {
    let parsed = parse_packet_string(input);
    parsed.apply()
}

pub fn part1(input: String) -> usize {
    let parsed = parse_packet_string(input);
    parsed.calculate_version_sum()
}

#[derive(Debug, PartialEq, Eq)]
enum PacketType {
    Literal { value: usize },
    Operation { op: usize, packets: Vec<Packet> },
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    len: usize,
    version: usize,
    t: PacketType,
}

impl Packet {
    fn calculate_version_sum(&self) -> usize {
        match &self.t {
            PacketType::Literal { .. } => self.version,
            PacketType::Operation { packets, .. } => {
                self.version
                    + packets
                        .iter()
                        .map(|p| p.calculate_version_sum())
                        .sum::<usize>()
            }
        }
    }
    fn apply(&self) -> usize {
        match &self.t {
            PacketType::Literal { value } => *value,
            PacketType::Operation { op, packets } => match op {
                0 => packets.iter().map(|p| p.apply()).sum(),
                1 => packets.iter().map(|p| p.apply()).product(),
                2 => packets.iter().map(|p| p.apply()).min().unwrap(),
                3 => packets.iter().map(|p| p.apply()).max().unwrap(),
                5 => {
                    if packets[0].apply() > packets[1].apply() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets[0].apply() < packets[1].apply() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets[0].apply() == packets[1].apply() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

fn parse_packet_string(input: String) -> Packet {
    let packet = parse_string(input);
    parse_packet(&packet)
}

fn parse_packet(packet: &[bool]) -> Packet {
    let mut reader = PacketReader { packet, cursor: 0 };
    let version = reader.read_bits_as_usize(3);
    let type_id = reader.read_bits_as_usize(3);
    match type_id {
        4 => parse_literal(version, reader),
        v => parse_operator(v, version, reader),
    }
}

fn parse_literal(version: usize, mut reader: PacketReader) -> Packet {
    let mut literal = vec![];
    loop {
        let continue_reading = reader.read_bits_as_usize(1) == 1;
        let bits = reader.read_bits(4);
        literal.append(&mut bits.to_vec());
        if !continue_reading {
            break;
        }
    }
    let value = bits_as_usize(&literal);
    Packet {
        t: PacketType::Literal { value },
        version,
        len: reader.len_read(),
    }
}

fn parse_operator(op: usize, version: usize, mut reader: PacketReader) -> Packet {
    let mut packets = vec![];
    if reader.read_bits(1)[0] {
        let sub_packet_n = reader.read_bits_as_usize(11);
        while packets.len() < sub_packet_n {
            let sub_packets = reader.get_buffer();
            let packet = parse_packet(sub_packets);
            reader.move_forward(packet.len);
            packets.push(packet);
        }
    } else {
        let bits_len = reader.read_bits_as_usize(15);
        let mut cursor = 0;
        while cursor < bits_len {
            let sub_packets = reader.get_buffer();
            let packet = parse_packet(sub_packets);
            reader.move_forward(packet.len);
            cursor += packet.len;
            packets.push(packet);
        }
    }
    Packet {
        t: PacketType::Operation { op, packets },
        version,
        len: reader.len_read(),
    }
}

fn bits_as_usize(bits: &[bool]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, b)| if *b { 1 << i } else { 0 })
        .sum()
}

struct PacketReader<'a> {
    packet: &'a [bool],
    cursor: usize,
}

impl PacketReader<'_> {
    fn read_bits_as_usize(&mut self, bits: usize) -> usize {
        let v = self.read_bits(bits);
        bits_as_usize(v)
    }

    fn read_bits(&mut self, bits: usize) -> &[bool] {
        let v = &self.packet[self.cursor..self.cursor + bits];
        self.cursor += bits;
        v
    }

    fn len_read(&self) -> usize {
        self.cursor
    }

    fn get_buffer(&self) -> &[bool] {
        &self.packet[self.cursor..self.packet.len()]
    }

    fn move_forward(&mut self, len: usize) {
        self.cursor += len;
    }
}
