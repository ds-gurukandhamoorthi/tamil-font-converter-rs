// pub struct TamilEntities{
//     composedEntity(&str),
//     separateEntity(&str),
//     lineBreak(&str),
//     notTranslatable(&str),
// }

#[derive(Debug)]
pub enum TamilDetailedEntity<'a>{
    Vowel(char),
    Consonant(char),
    SpecialEntity(&'a str), //sri ...
    Mark((MarkType, char)),
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

use nom::{
    branch::alt,
    character::complete::one_of,
    bytes::complete::tag,
    IResult,
};

fn parse_sri(i: &str) -> IResult<&str, TamilDetailedEntity> {
    let (i, entity) = tag("ஸ்ரீ")(i)?;
    Ok((i, TamilDetailedEntity::SpecialEntity(entity)))
}

fn parse_vowel(i: &str) -> IResult<&str, TamilDetailedEntity> {
    let (i, entity) = one_of("அஆஇஈஉஊஎஏஐஒஓஔஃ")(i)?;
    Ok((i, TamilDetailedEntity::Vowel(entity)))
}

fn parse_consonant(i: &str) -> IResult<&str, TamilDetailedEntity> {
    let (i, entity) = one_of("கஙசஞடணதநபமயரலவழளறனஷஜஸஹ")(i)?;
    Ok((i, TamilDetailedEntity::Consonant(entity)))
}

//Aka Combining Mark in Unicode: pulli_ta (A kuril), I kuril, I nedil, U kuril, U nedil
fn parse_riding_mark(i: &str) -> IResult<&str, TamilDetailedEntity> {
    let (i, entity) = one_of("\u{0bcd}\u{0bbf}\u{0bc0}\u{0bc1}\u{0bc2}")(i)?;
    Ok((i, TamilDetailedEntity::Mark((MarkType::Riding, entity))))
}

// E kuil E nedil AI
fn parse_preceding_mark(i: &str) -> IResult<&str, TamilDetailedEntity> {
    let (i, entity) = one_of("\u{0bc6}\u{0bc7}\u{0bc8}")(i)?;
    Ok((i, TamilDetailedEntity::Mark((MarkType::Preceding, entity))))
}

// A nedil
fn parse_following_mark(i: &str) -> IResult<&str, TamilDetailedEntity> {
    let (i, entity) = one_of("\u{0bbe}")(i)?;
    Ok((i, TamilDetailedEntity::Mark((MarkType::Following, entity))))
}

//O kuril, O nedil, AU
fn parse_preceding_and_following_mark(i: &str) -> IResult<&str, TamilDetailedEntity> {
    let (i, entity) = one_of("\u{0bca}\u{0bcb}\u{0bcc}")(i)?;
    Ok((i, TamilDetailedEntity::Mark((MarkType::PrecedingAndFollowing, entity))))
}

//Any mark that precedes, follows or does both. (But doesn't modify the form of character it marks)
fn parse_non_riding_mark(i: &str) -> IResult<&str, TamilDetailedEntity> {
    alt((
        parse_preceding_and_following_mark,
        parse_preceding_mark,
        parse_following_mark,
    ))(i)
}

fn parse_mark(i: &str) -> IResult<&str, TamilDetailedEntity> {
    alt((
        parse_riding_mark,
        parse_non_riding_mark,
    ))(i)
}

fn parse_not_markable(i: &str) -> IResult<&str, TamilDetailedEntity> {
    alt((
        parse_sri,
        parse_vowel,
    ))(i)
}

pub fn parse_entity(i: &str) -> IResult<&str, TamilDetailedEntity> {
    alt((
        parse_not_markable,
        parse_consonant,
    ))(i)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
