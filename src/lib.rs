mod messagetype_enum;
mod fieldtype_enum;
mod match_predefined_field_value;
mod match_messagetype;
mod match_messagetype_field;
mod match_messagetype_offset;
mod match_messagetype_scale;
mod match_message_timestamp_field;

pub use messagetype_enum::MessageType;
pub use fieldtype_enum::FieldType;
pub use match_predefined_field_value::match_predefined_field_value;
pub use match_messagetype::match_messagetype;
pub use match_messagetype_field::match_message_field;
pub use match_messagetype_offset::match_message_offset;
pub use match_messagetype_scale::match_message_scale;
pub use match_message_timestamp_field::match_message_timestamp_field;

pub type MatchScaleFn = dyn std::ops::Fn(usize) -> std::option::Option<f32>;
pub type MatchOffsetFn = dyn std::ops::Fn(usize) -> std::option::Option<i16>;
pub type MatchFieldTypeFn = dyn std::ops::Fn(usize) -> FieldType;