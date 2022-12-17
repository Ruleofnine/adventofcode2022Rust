#![allow(unused_variables)]
#![allow(unused_imports)]
const EXAMPLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, none_of},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    *,
};
use std::cmp::Ordering;
#[derive(Debug, Eq)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}
#[derive(Debug, PartialEq)]
struct Pair {
    left: Packet,
    right: Packet,
}
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        use Packet::*;
        match (&self, other) {
            (List(l_packets), List(r_packets)) => l_packets == r_packets,
            (Number(l_num), Number(r_num)) => l_num == r_num,
            (List(l_packets), Number(r_num)) => l_packets == &vec![Number(*r_num)],
            (Number(l_num), List(r_packets)) => r_packets == &vec![Number(*l_num)],
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (&self, other) {
            (List(l_packets), List(r_packets)) => l_packets.cmp(r_packets),
            (Number(l_num), Number(r_num)) => l_num.cmp(r_num),
            (List(l_packets), Number(r_num)) => l_packets.cmp(&vec![Number(*r_num)]),
            (Number(l_num), List(r_packets)) => vec![Number(*l_num)].cmp(&r_packets),
        }
    }
}
fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]"))
            .map(|vec| Packet::List(vec)),
        nom::character::complete::u32.map(|num| Packet::Number(num)),
    ))(input)
}
fn parse_pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(parse_packet, newline, parse_packet).map(|(p1, p2)| Pair {
            right: p1,
            left: p2,
        }),
    )(input)
}
fn part_one(input:&str)->usize{
    let (_, pairs) = (parse_pairs(input)).unwrap();
    let mut sum_of_indexes = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if pair.left > pair.right{
            sum_of_indexes+=i+1;
        }
    }
    sum_of_indexes


}
fn part_two(input:&str) ->usize{
    let (_, pairs) = (parse_pairs(input)).unwrap();
    let div1 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    let mut packets:Vec<&Packet> = pairs.iter().flat_map(|Pair {left, right}| [left,right]).chain([&div1,&div2]).collect();
    let mut answer = 1;
    packets.sort();
    for (i,p) in packets.iter().enumerate(){
        if p == &&div1 || p==&&div2{
            answer*=i+1;
        }
    } 
    answer
}
fn main(){
    let input = std::fs::read_to_string("inputs/day13.txt").unwrap();
    dbg!(part_two(&input));

}
#[test]
fn test_part_one(){
    assert_eq!(13,part_one(EXAMPLE_INPUT));
    assert_eq!(5506,part_one(std::fs::read_to_string("inputs/day13.txt").unwrap().as_str()));
}
#[test]
fn test_part_two(){
    assert_eq!(140,part_two(EXAMPLE_INPUT));
    assert_eq!(21756,part_two(std::fs::read_to_string("inputs/day13.txt").unwrap().as_str()));
}
