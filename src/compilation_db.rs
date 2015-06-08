use rustc_serialize::{json, Decodable, Decoder};

#[derive(Debug)]
pub struct ClangCompUnit {
    directory: String,
    command: String,
    file: String
}

// Since I can't derive it currently, it looks like
impl Decodable for ClangCompUnit {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<ClangCompUnit, D::Error> {
        decoder.read_struct("root", 0, |decoder| {
            Ok(ClangCompUnit{
                directory: try!(decoder.read_struct_field("directory", 0, |decoder| Decodable::decode(decoder))),
                command: try!(decoder.read_struct_field("command", 0, |decoder| Decodable::decode(decoder))),
                file: try!(decoder.read_struct_field("file", 0, |decoder| Decodable::decode(decoder)))
            })
        })
    }
}


pub fn parse(input: String) -> Vec<ClangCompUnit> {
    let value: Vec<ClangCompUnit> = json::decode(&input).unwrap();
    value
}
