mod enums;
pub use enums::fieldtype_enum::FieldType;
pub use enums::messagetype_enum::MessageType;

mod field_matchers;
pub use field_matchers::get_field_offset::get_field_offset_fn;
pub use field_matchers::get_field_scale::get_field_scale_fn;
pub use field_matchers::get_field_string_value::get_field_string_value_fn;
pub use field_matchers::get_field_type::get_field_type_fn;

mod message_matchers;
pub use message_matchers::get_message_timestamp_field::get_message_timestamp_field;
pub use message_matchers::get_message_type::get_message_type;

pub type MatchScaleFn = fn(usize) -> std::option::Option<f32>;
pub type MatchOffsetFn = fn(usize) -> std::option::Option<i16>;
pub type MatchFieldTypeFn = fn(usize) -> FieldType;
