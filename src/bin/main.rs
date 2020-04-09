use tamil_font_converter_rs::*;

fn main() {
    // let (i, res) = parse_entity("அகர").unwrap();
    let mut i = "அக ரகர";
    // let (i, res) = parse_entity(i).unwrap();
    // println!("{:?}", res);
    // let (i, res) = parse_entity(i).unwrap();
    // println!("{:?}", res);
    // let (i, res) = parse_entity(i).unwrap();
    // println!("{:?}", res);
    // let (i, res) = parse_entity(i).unwrap();
    // println!("{:?}", res);
    // let (i, res) = parse_entity(i).unwrap();
    // println!("{:?}", res);
    // let (i, res) = parse_entity(i).unwrap();
    // println!("{:?}", res);
    //
    
    let mut data = "அகரகர";
    let it = std::iter::from_fn(move || {
        match parse_entity(data) {
            Ok((i, o)) => {
                data = i;
                Some(o)
            },
            _ => None
        }
    });

    for value in it {
        println!("{:?}", value);
    }


}
