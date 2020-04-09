// pub struct TamilEntities{
//     composedEntity(&str),
//     separateEntity(&str),
//     lineBreak(&str),
//     notTranslatable(&str),
// }
use nom::{
    branch::alt,
    character::complete::one_of,
    bytes::complete::tag,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
pub enum TamilDetailedEntity<'a>{
    Vowel(char),
    Consonant(char),
    SeparateEntity((char, char)),
    ComposedEntity((char, MarkType, char)),
    SpecialEntity(&'a str), //sri ...
    Mark((MarkType, char)),
    Other(char),
    // RidingMark(char),
    // PrecedingMark(char),
    // FollowingMark(char),
    // PrecedingAndFollowingMark(char),
}

#[derive(Debug)]
pub enum MarkType {
    Riding,
    Preceding,
    Following,
    PrecedingAndFollowing,
}


fn parse_sri(i: &str) -> IResult<&str, TamilDetailedEntity> {
    let (i, entity) = tag("ஸ்ரீ")(i)?;
    Ok((i, TamilDetailedEntity::SpecialEntity(entity)))
}

fn parse_vowel(i: &str) -> IResult<&str, TamilDetailedEntity> {
    let (i, entity) = one_of("அஆஇஈஉஊஎஏஐஒஓஔஃ")(i)?;
    Ok((i, TamilDetailedEntity::Vowel(entity)))
}

fn parse_consonant_helper(i: &str) -> IResult<&str, char> {
    one_of("கஙசஞடணதநபமயரலவழளறனஷஜஸஹ")(i)
}

fn parse_consonant(i: &str) -> IResult<&str, TamilDetailedEntity> {
    let (i, entity) = parse_consonant_helper(i)?;
    Ok((i, TamilDetailedEntity::Consonant(entity)))
}

//Aka Combining Mark in Unicode: pulli_ta (A kuril), I kuril, I nedil, U kuril, U nedil
fn parse_riding_mark(i: &str) -> IResult<&str, (MarkType, char)> {
    let (i, entity) = one_of("\u{0bcd}\u{0bbf}\u{0bc0}\u{0bc1}\u{0bc2}")(i)?;
    Ok((i, (MarkType::Riding, entity)))
}

// E kuil E nedil AI
fn parse_preceding_mark(i: &str) -> IResult<&str, (MarkType, char)> {
    let (i, entity) = one_of("\u{0bc6}\u{0bc7}\u{0bc8}")(i)?;
    Ok((i, (MarkType::Preceding, entity)))
}

// A nedil
fn parse_following_mark(i: &str) -> IResult<&str, (MarkType, char)> {
    let (i, entity) = one_of("\u{0bbe}")(i)?;
    Ok((i, (MarkType::Following, entity)))
}

//O kuril, O nedil, AU
fn parse_preceding_and_following_mark(i: &str) -> IResult<&str, (MarkType, char)> {
    let (i, entity) = one_of("\u{0bca}\u{0bcb}\u{0bcc}")(i)?;
    Ok((i, (MarkType::PrecedingAndFollowing, entity)))
}

//Any mark that precedes, follows or does both. (But doesn't modify the form of character it marks)
fn parse_non_riding_mark(i: &str) -> IResult<&str, (MarkType, char)> {
    alt((
        parse_preceding_and_following_mark,
        parse_preceding_mark,
        parse_following_mark,
    ))(i)
}

fn parse_mark(i: &str) -> IResult<&str, TamilDetailedEntity> {
    let (i, o) = alt((
        parse_riding_mark,
        parse_non_riding_mark,
    ))(i)?;
    Ok((i, TamilDetailedEntity::Mark(o)))
}

fn parse_other(i: &str) -> IResult<&str, TamilDetailedEntity> {
    let (i, entity) = nom::character::complete::anychar(i)?;
    Ok((i, TamilDetailedEntity::Other(entity)))
    
}

fn parse_not_markable(i: &str) -> IResult<&str, TamilDetailedEntity> {
    alt((
        parse_sri,
        parse_vowel,
    ))(i)
}

pub fn parse_separate_entity(i: &str) -> IResult<&str, TamilDetailedEntity> {
    let (i, consonant) = parse_consonant_helper(i)?;
    let (i, (_, riding_mark)) = parse_riding_mark(i)?;
    Ok((i, TamilDetailedEntity::SeparateEntity((consonant, riding_mark))))
}

pub fn parse_composed_entity(i: &str) -> IResult<&str, TamilDetailedEntity> {
    let (i, consonant) = parse_consonant_helper(i)?;
    let (i, (mark_type, non_riding_mark)) = parse_non_riding_mark(i)?;
    Ok((i, TamilDetailedEntity::ComposedEntity((consonant, mark_type, non_riding_mark))))

}

pub fn parse_entity(i: &str) -> IResult<&str, TamilDetailedEntity> {
    alt((
        parse_separate_entity,
        parse_composed_entity,
        parse_not_markable,
        parse_consonant,
        parse_other,
    ))(i)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



pub fn convert_unic_stmzh(source: &str) -> String {
    let mut UNIC_STMZH_MAP_CHAR_CHAR = HashMap::new();
    //Vowels
    UNIC_STMZH_MAP_CHAR_CHAR.insert('அ', '\u{f0b6}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ஆ','\u{f067}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('இ','\u{f0d6}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ஈ','\u{f07e}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('உ','\u{f063}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ஊ','\u{f0bb}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('எ','\u{f0a8}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ஏ','\u{f0b0}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ஐ','\u{f06e}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ஒ','\u{f0ce}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ஓ','\u{f07b}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ஔ','\u{f0c1}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ஃ','\u{f0e0}');


    //Consonants
    UNIC_STMZH_MAP_CHAR_CHAR.insert('க', '\u{f0ef}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ங', '\u{f0f4}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ச', '\u{f0c4}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ஞ', '\u{f051}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ட', '\u{f0a6}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ண', '\u{f0f0}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('த', '\u{f03e}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ந', '\u{f0e5}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ப', '\u{f0c3}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ம', '\u{f05c}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ய', '\u{f042}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ர', '\u{f0ab}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ல', '\u{f0e9}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('வ', '\u{f06b}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ழ', '\u{f077}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ள', '\u{f065}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ற', '\u{f0c5}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ன', '\u{f0aa}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ஷ', '\u{f0ad}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ஜ', '\u{f0db}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ஸ', '\u{f076}');
    UNIC_STMZH_MAP_CHAR_CHAR.insert('ஹ', '\u{f0c7}');

    let mut UNIC_STMZH_MAP_TUPLE_CHAR = HashMap::new();
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('க','\u{0bcd}'), '\u{f0c2}');

    //pulli
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('க','\u{0bcd}'), '\u{f0c2}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ங','\u{0bcd}'), '\u{f0ba}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ச','\u{0bcd}'), '\u{f0df}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ஞ','\u{0bcd}'), '\u{f0de}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ட','\u{0bcd}'), '\u{f0e2}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ண','\u{0bcd}'), '\u{f0f5}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('த','\u{0bcd}'), '\u{f0dd}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ந','\u{0bcd}'), '\u{f0cd}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ப','\u{0bcd}'), '\u{f0a9}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ம','\u{0bcd}'), '\u{f044}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ய','\u{0bcd}'), '\u{f046}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ர','\u{0bcd}'), '\u{f0ec}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ல','\u{0bcd}'), '\u{f05f}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('வ','\u{0bcd}'), '\u{f0cb}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ழ','\u{0bcd}'), '\u{f0b5}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ள','\u{0bcd}'), '\u{f05e}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ற','\u{0bcd}'), '\u{f075}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ன','\u{0bcd}'), '\u{f05b}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ஷ','\u{0bcd}'), '\u{f069}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ஜ','\u{0bcd}'), '\u{f0eb}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ஸ','\u{0bcd}'), '\u{f0fc}');
    UNIC_STMZH_MAP_TUPLE_CHAR.insert(('ஹ','\u{0bcd}'), '\u{f0e3}');

    let mut data = source;
    let it = std::iter::from_fn(move || {
        match parse_entity(data) {
            Ok((i, o)) => {
                data = i;
                Some(o)
            }
            _ => None
        }
    });

    let mut output: String= "".to_string();

    for entity in it {
        match entity {
            TamilDetailedEntity::Vowel(v) => output.push(*UNIC_STMZH_MAP_CHAR_CHAR.get(&v).unwrap()),
            TamilDetailedEntity::Consonant(v) => output.push(*UNIC_STMZH_MAP_CHAR_CHAR.get(&v).unwrap()),
            TamilDetailedEntity::SeparateEntity((c,m)) => output.push(*UNIC_STMZH_MAP_TUPLE_CHAR.get(&(c,m)).unwrap()),
            _ => ()
        }
    }
    output.to_string()
}
