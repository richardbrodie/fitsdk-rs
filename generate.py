import pandas as pd

FIELDTYPE_ENUM_FILE = "fieldtype_enum.rs"
MESSAGETYPE_ENUM_FILE = "messagetype_enum.rs"

GET_MESSAGE_TYPE_FILE = "get_message_type.rs"
GET_MESSAGE_TIMESTAMP_FIELD_FILE = "get_message_timestamp_field.rs"

GET_FIELD_TYPE_FILE = "get_field_type.rs"
GET_FIELD_OFFSET_FILE = "get_field_offset.rs"
GET_FIELD_SCALE_FILE = "get_field_scale.rs"
GET_FIELD_STRING_VALUE_FILE = "get_field_string_value.rs"
GET_FIELD_NAME_FILE = "get_field_name_value.rs"


def camel_case(s):
    return ''.join(x.title() for x in s.split('_'))


def snake_case(s):
    return s.lower()


def shouty_snake_case(s):
    return snake_case(s).upper()


def write_fieldtype_enum(set_ft):
    f = open("src/enums/" + FIELDTYPE_ENUM_FILE, "w+")
    f.writelines([
        "/// An enum of all possible data types a `Message` field may be\n",
        "#[derive(Debug, Copy, Clone, PartialEq)]\n",
        "pub enum FieldType {\n"
    ])
    f.writelines(f"    {camel_case(v)},\n" for v in sorted(set_ft))
    f.writelines([
        "    Coordinates,\n"
        "    Timestamp,\n",
        "    None,\n",
        "}\n"
    ])
    f.close()


def write_messagetype_enum(set_mt):
    f = open("src/enums/" + MESSAGETYPE_ENUM_FILE, "w+")
    f.writelines([
        "/// an enum of all defined messages in the Fit SDK\n",
        "#[derive(Debug, Copy, Clone, PartialEq)]\n",
        "pub enum MessageType {\n"
    ])
    f.writelines(f"    {camel_case(v)},\n" for v in sorted(set_mt))
    f.writelines([
        "    Pad,\n"
        "    MfgRangeMax,\n",
        "    MfgRangeMin,\n",
        "    None,\n",
        "}\n",
    ])
    f.close()


def write_get_message_type(map):
    f = open("src/message_matchers/" + GET_MESSAGE_TYPE_FILE, "w+")
    f.writelines([
        "use crate::MessageType;\n",
        "\n",
        "/// Convert a global_message_id into a `MessageType` enum\n",
        "pub fn get_message_type(k: u16) -> MessageType {\n",
        "    match k {\n"
    ])
    f.writelines(f"        {k} => MessageType::{camel_case(v)},\n" for k, v in map.items())
    f.writelines([
        "        _ => MessageType::None\n",
        "    }\n",
        "}\n"
    ])
    f.close()


def write_get_message_timestamp_field(set_mt, msgs_store):
    f = open("src/message_matchers/" + GET_MESSAGE_TIMESTAMP_FIELD_FILE, "w+")
    f.writelines([
        "use crate::MessageType;\n",
        "\n",
        "/// A method for obtaining the field ID a specified `MessageType` uses for its timestamp. Usually it's 253 but unfortunately not always.\n",
        "pub fn get_message_timestamp_field(mt: MessageType) -> Option<usize> {\n",
        "    match mt {\n"
    ])
    for mt in sorted(set_mt):
        try:
            values = next(x['values'] for x in iter(msgs_store) if x['name'] == mt)
            ts = next(x['id'] for x in iter(values) if x['field_type'] == "Timestamp")
            f.write(f"        MessageType::{camel_case(mt)} => Some({ts}),\n")
        except StopIteration:
            pass
    f.writelines([
        "        _ => None\n",
        "    }\n",
        "}\n"
    ])
    f.close()


def write_get_field_type(set_mt, msgs_store):
    f = open("src/field_matchers/" + GET_FIELD_TYPE_FILE, "w+")
    f.writelines([
        "use crate::{FieldType, MessageType, MatchFieldTypeFn};\n",
        "\n"
    ])
    for message_type in sorted(set_mt):
        values = next(x['values'] for x in iter(msgs_store) if x['name'] == message_type)
        values = sorted(values, key=lambda k: k['id'])
        f.writelines([
            f"fn match_field_{message_type.lower()}(k: usize) -> FieldType {{\n"
            "    match k {\n"
        ])
        f.writelines(f"        {m['id']} => FieldType::{camel_case(m['field_type'])},\n" for m in values)
        f.writelines([
            "        _ => FieldType::None,\n",
            "    }\n"
            "}\n"
        ])
    f.writelines([
        "fn match_field_none(_: usize) -> FieldType {\n",
        "    return FieldType::None;\n",
        "}\n",
    ])
    f.write("""
/// Determines a specific `FieldType` of any `MessageType`.
///
/// The method is called with a `MessageType` argument and returns a static closure
/// which is then called with a field_id `usize` and yields a `FieldType`.
/// Any field that is not defined will return a `FieldType::None` variant.
///
/// # Example
///
/// ```
/// let message_type = MessageType::WorkoutSession;
/// let parsed_value = 3;
/// let field_fn = match_message_field(message_type);
/// let field = field_fn(parsed_value);
/// assert_eq!(field, FieldType::Uint16);
/// ```
""")
    f.writelines([
        "pub fn get_field_type_fn(m: MessageType) -> MatchFieldTypeFn {\n",
        "    match m {\n"
    ])

    f.writelines(f"        MessageType::{camel_case(v)} => match_field_{v.lower()},\n" for v in set_mt)
    f.writelines([
        "        _ => match_field_none,\n",
        "    }\n",
        "}\n"
    ])
    f.close()


def write_get_field_offset(set_mt, msgs_store):
    f = open("src/field_matchers/" + GET_FIELD_OFFSET_FILE, "w+")
    f.writelines([
        "use crate::{MessageType, MatchOffsetFn};\n",
        "\n"
    ])
    for message_type in sorted(set_mt):
        values = next(x['values'] for x in iter(msgs_store) if x['name'] == message_type)
        values = sorted(values, key=lambda k: k['id'])
        v = [x for x in values if pd.notna(x['offset'])]
        if v:
            f.writelines([
                f"fn match_offset_{message_type.lower()}(k: usize) -> Option<i16> {{\n",
                "    match k {\n"
            ])
            f.writelines(f"        {m['id']} => Some({int(m['offset'])}i16),\n" for m in v)
            f.writelines([
                "        _ => None,\n",
                "    }\n"
            ])
        else:
            f.writelines([
                f"fn match_offset_{message_type.lower()}(_: usize) -> Option<i16> {{\n",
                "    None\n"
            ])
        f.writelines(
                "}\n"
        )
    f.writelines([
        "fn match_offset_none(_: usize) -> Option<i16> {\n",
        "    None\n",
        "}\n",
    ])
    f.write("""
/// Determines whether any SDK-defined `Message` defines an offset for any of its fields.
///
/// The method is called with a `MessageType` argument and returns a static closure which is called with a
/// field_id `usize` which yields an `Option<i16>`.
///
/// # Example
///
/// ```
/// let message_type = MessageType::Session;
/// let parsed_value = 71;
/// let offset_fn = match_message_offset(message_type);
/// let offset = offset_fn(parsed_value);
/// assert_eq!(offset, Some(500.0));
/// ```
""")
    f.writelines([
        "pub fn get_field_offset_fn(m: MessageType) -> MatchOffsetFn {\n",
        "    match m {\n"
    ])

    f.writelines(f"        MessageType::{camel_case(v)} => match_offset_{v.lower()},\n" for v in set_mt)
    f.writelines([
        "        _ => match_offset_none\n",
        "    }\n",
        "}\n"
    ])
    f.close()


def write_get_field_scale(set_mt, msgs_store):
    f = open("src/field_matchers/" + GET_FIELD_SCALE_FILE, "w+")
    f.writelines([
        "use crate::{MessageType, MatchScaleFn};\n",
        "\n"
    ])
    for message_type in sorted(set_mt):
        values = next(x['values'] for x in iter(msgs_store) if x['name'] == message_type)
        values = sorted(values, key=lambda k: k['id'])
        v = [x for x in values if pd.notna(x['scale'])]
        if v:
            f.writelines([
                f"fn match_scale_{message_type.lower()}(k: usize) -> Option<f32> {{\n",
                "    match k {\n"
            ])
            for m in v:
                vv = m['scale']
                try:
                    vv = m['scale'].split(',')[0]
                except AttributeError:
                    pass
                f.write(f"        {m['id']} => Some({float(vv)}f32),\n")
            f.writelines([
                "        _ => None,\n",
                "    }\n"
            ])
        else:
            f.writelines([
                f"fn match_scale_{message_type.lower()}(_: usize) -> Option<f32> {{\n",
                "    None\n"
            ])
        f.writelines(
                "}\n"
        )
    f.writelines([
        "fn match_scale_none(_: usize) -> Option<f32> {\n",
        "    return None;\n",
        "}\n",
    ])
    f.write("""
/// Determines whether any SDK-defined `Message` defines a scale for any of its fields.
///
/// The method is called with a `MessageType` argument and returns a static closure which is called with a field_id `usize`
/// and yields an `Option<f32>`.
///
/// # Example
///
/// ```
/// let message_type = MessageType::Workout;
/// let parsed_value = 14;
/// let scale_fn = match_message_scale(message_type);
/// let scale = scale_fn(parsed_value);
/// assert_eq!(scale, Some(100.0));
/// ```
""")
    f.writelines([
        "pub fn get_field_scale_fn(m: MessageType) -> MatchScaleFn {\n",
        "    match m {\n"
    ])
    f.writelines(f"        MessageType::{camel_case(v)} => match_scale_{v.lower()},\n" for v in set_mt)
    f.writelines([
        "        _ => match_scale_none\n",
        "    }\n",
        "}\n"
    ])
    f.close()


def write_get_field_string_value(set_f, types_store):
    f = open("src/field_matchers/" + GET_FIELD_STRING_VALUE_FILE, "w+")
    f.writelines([
        "use crate::FieldType;\n",
        "\n",
    ])
    f.write("""
/// Certain `FieldType` values refer to predefined text strings in the SDK.
///
/// # Example
///
/// ```
/// let field_type = FieldType::BodyLocation;
/// let parsed_value = 27;
/// let predefined_text = match_predefined_field_value(field_type, parsed_value);
/// assert_eq!(predefined_text, Some("left_forearm_extensors"));
/// ```
""")
    f.writelines([
        "pub fn get_field_string_value_fn(f: FieldType, k: usize) -> Option<&'static str> {\n",
        "    match f {\n"
    ])
    for ft in sorted(set_f):
        try:
            map = next(x['values'] for x in iter(types_store) if x['name'] == ft)
            f.write(f"        FieldType::{camel_case(ft)} => match k {{\n")
            f.writelines(f"            {int(k, 0)} => Some(\"{v}\"),\n" for k, v in map.items())
            f.writelines([
                "            _ => None,\n",
                "        }\n"
            ])
        except StopIteration:
            pass
    f.writelines([
        "        FieldType::None => None,\n",
        "        _ => None\n",
        "    }\n",
        "}\n"
    ])
    f.close()


df = pd.read_excel('Profile.xlsx', sheet_name='Types')
types_list = []
elem = {'values': {}}

for row in df.itertuples():
    if pd.notna(row[1]) and pd.notna(row[2]):
        if 'name' in elem:
            types_list.append(elem)
            elem = {'values': {}}
        elem['name'] = row[1]
        elem['base_type'] = row[2]
    elif pd.notna(row[3]) and pd.notna(row[4]):
        elem['values'][str(row[4])] = row[3]
types_list.append(elem)

# read messages
df = pd.read_excel('Profile.xlsx', sheet_name='Messages')
msgs_list = []
msg = {'values': []}
fields_set = set([])

for row in df.itertuples():
    if pd.notna(row[1]):
        if 'name' in msg:
            msgs_list.append(msg)
            msg = {'values': []}
        msg['name'] = row[1]
    elif pd.notna(row[2]) and pd.notna(row[3]):
        fields_set.add(row[4])
        if row[3].endswith("_lat") or row[3].endswith("_long"):
            val = "Coordinates"
        elif row[3].endswith("timestamp"):
            val = "Timestamp"
        else:
            val = row[4]
        r = {'id': int(row[2]), 'field_name': row[3], 'field_type': val, 'scale': row[7], 'offset': row[8]}
        msg['values'].append(r)
msgs_list.append(msg)

msg_types = next(x for x in iter(types_list) if x['name'] == 'mesg_num')
msgs = [x['name'] for x in msgs_list]

write_fieldtype_enum(fields_set)
write_messagetype_enum(msgs)

write_get_message_type(msg_types['values'])
write_get_message_timestamp_field(msgs, msgs_list)

write_get_field_type(msgs, msgs_list)
write_get_field_offset(msgs, msgs_list)
write_get_field_scale(msgs, msgs_list)
write_get_field_string_value(fields_set, types_list)
