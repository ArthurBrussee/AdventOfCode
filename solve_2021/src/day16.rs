struct BitStream {
    bits: Vec<bool>,
}

struct BitStreamCursor<'a> {
    stream: &'a BitStream,
    idx: usize,
}

impl From<&str> for BitStream {
    fn from(input: &str) -> Self {
        fn to_bits(hex: char) -> [bool; 4] {
            let d = hex.to_digit(16).expect("Not hex");
            (0..4)
                .map(|i| (d & (1 << (3 - i))) > 0)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        }
        let bits = input.chars().flat_map(to_bits).collect();
        BitStream { bits }
    }
}

pub trait BitToNum {
    fn bit_to_num(self) -> usize;
}

impl<I> BitToNum for I
where
    I: DoubleEndedIterator<Item = bool>,
{
    fn bit_to_num(self) -> usize {
        self.rev()
            .enumerate()
            .fold(0usize, |acc, (i, x)| acc + (if x { 1 } else { 0 } << (i)))
    }
}

impl BitStream {
    fn read_bits<const N: usize>(&self, cursor: usize) -> [bool; N] {
        self.bits[cursor..cursor + N].try_into().unwrap()
    }
}

impl From<&BitStream> for Packet {
    fn from(stream: &BitStream) -> Self {
        let mut cursor = BitStreamCursor { stream, idx: 0 };
        cursor.read_recurs()
    }
}

enum Operator {
    Sum,
    Prod,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

enum Packet {
    Operator {
        version: usize,
        op: Operator,
        sub_packets: Vec<Packet>,
    },
    Literal {
        version: usize,
        literal: usize,
    },
}

impl Packet {
    fn value(&self) -> usize {
        match self {
            Packet::Operator {
                version: _,
                op,
                sub_packets,
            } => {
                let sub_values = sub_packets.iter().map(|p| p.value());

                match op {
                    Operator::Sum => sub_values.sum(),
                    Operator::Prod => sub_values.product(),
                    Operator::Min => sub_values.min().unwrap(),
                    Operator::Max => sub_values.max().unwrap(),
                    Operator::Gt => {
                        if sub_packets[0].value() > sub_packets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                    Operator::Lt => {
                        if sub_packets[0].value() < sub_packets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                    Operator::Eq => {
                        if sub_packets[0].value() == sub_packets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
            Packet::Literal {
                version: _,
                literal,
            } => *literal,
        }
    }
}

impl<'a> BitStreamCursor<'a> {
    fn advance_bits<const N: usize>(&mut self) -> [bool; N] {
        let res = self.stream.read_bits::<N>(self.idx);
        self.idx += N;
        res
    }

    fn advance<const N: usize>(&mut self) -> usize {
        self.advance_bits::<N>().into_iter().bit_to_num()
    }

    fn read_recurs(&mut self) -> Packet {
        let version = self.advance::<3>();
        let id = self.advance::<3>();

        match id {
            // Literal
            4 => {
                let mut cont = true;
                let mut v = Vec::new();

                while cont {
                    cont = self.advance::<1>() > 0;
                    v.extend(self.advance_bits::<4>());
                }

                Packet::Literal {
                    version,
                    literal: v.into_iter().bit_to_num(),
                }
            }
            // Operator
            _ => {
                let mut sub_packets = Vec::new();

                let length_type_id = self.advance::<1>();

                if length_type_id == 0 {
                    let read_length_bits = self.advance::<15>();

                    let start_idx = self.idx;
                    while self.idx - start_idx < read_length_bits {
                        sub_packets.push(self.read_recurs());
                    }
                } else {
                    let num_packets = self.advance::<11>();
                    for _ in 0..num_packets {
                        sub_packets.push(self.read_recurs());
                    }
                }

                let op = match id {
                    0 => Operator::Sum,
                    1 => Operator::Prod,
                    2 => Operator::Min,
                    3 => Operator::Max,
                    5 => Operator::Gt,
                    6 => Operator::Lt,
                    7 => Operator::Eq,
                    _ => panic!("Unknown op {}", id),
                };

                Packet::Operator {
                    version,
                    op,
                    sub_packets,
                }
            }
        }
    }
}

fn version_sum(packet: &Packet) -> usize {
    match packet {
        Packet::Operator {
            version,
            op: _,
            sub_packets,
        } => *version + sub_packets.iter().map(version_sum).sum::<usize>(),
        Packet::Literal {
            version,
            literal: _,
        } => *version,
    }
}

pub fn calc(input: &str) -> (usize, usize) {
    let stream = BitStream::from(input);
    let parent_packet = Packet::from(&stream);

    let p1 = version_sum(&parent_packet);
    let p2 = parent_packet.value();

    (p1, p2)
}

#[test]
fn test() {
    let (p1, _) = calc("8A004A801A8002F478");
    assert_eq!(p1, 16);

    assert_eq!(calc("C200B40A82").1, 3);
    assert_eq!(calc("04005AC33890").1, 54);
    assert_eq!(calc("880086C3E88112").1, 7);
    assert_eq!(calc("CE00C43D881120").1, 9);
    assert_eq!(calc("D8005AC2A8F0").1, 1);
    assert_eq!(calc("F600BC2D8F").1, 0);
    assert_eq!(calc("9C005AC2F8F0").1, 0);
    assert_eq!(calc("9C0141080250320F1802104A08").1, 1);
}
