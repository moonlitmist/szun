use crate::runtime::{Reference, type_key};
use crate::tag;

mod builder; pub use builder::*;
mod util; pub use util::*;

mod varying; pub use varying::Varying;
mod boolean; pub use boolean::Boolean;
mod natural; pub use natural::Natural;
mod integer; pub use integer::Integer;
mod signfiicant; pub use signfiicant::Significant;
mod block; pub use block::Block;
mod sequence; pub use sequence::Sequence;
mod array; pub use array::Array;
mod list; pub use list::List;
mod schema; pub use schema::Schema;
mod record; pub use record::Record;

pub enum Type {
    Null,
    Varying(Varying),
    Boolean(Boolean),
    Natural(Natural),
    Integer(Integer),
    Block(Block),
    Sequence(Sequence),
    Array(Array),
    List(List),
    Record(Record),
    Schema(Schema),
}
impl Type {
    pub fn from(addr:Reference) -> Self
    {
        match unsafe {type_key(addr.class)} {
            tag::NULL => Type::Null,
            tag::VARYING => Type::Varying(Varying::from(addr).unwrap()),
            tag::BOOLEAN => Type::Boolean(Boolean::from(addr).unwrap()),
            tag::NATURAL => Type::Natural(Natural::from(addr).unwrap()),
            tag::INTEGER => Type::Integer(Integer::from(addr).unwrap()),
            tag::BLOCK => Type::Block(Block::from(addr).unwrap()),
            tag::SEQUENCE => Type::Sequence(Sequence::from(addr).unwrap()),
            tag::ARRAY => Type::Array(Array::from(addr).unwrap()),
            tag::LIST => Type::List(List::from(addr).unwrap()),
            tag::RECORD => Type::Record(Record::from(addr).unwrap()),
            _ => Type::Null,
        }
    }

    pub fn get(&self) -> Reference
    {
        match &self {
            Type::Varying(obj) => **obj,
            Type::Boolean(obj) => **obj,
            Type::Natural(obj) => **obj,
            Type::Integer(obj) => **obj,
            Type::Block(obj) => **obj,
            Type::Sequence(obj) => **obj,
            Type::Array(obj) => **obj,
            Type::List(obj) => **obj,
            Type::Record(obj) => **obj,
            _ => Reference::null(),
        }
    }
}
