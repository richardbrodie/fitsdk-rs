import pandas as pd

FIELDTYPE_ENUM_FILE = "fieldtype_enum.rs"
MESSAGETYPE_ENUM_FILE = "messagetype_enum.rs"
MATCH_MESSAGETYPE_FIELD_FILE = "match_messagetype_field.rs"
MATCH_MESSAGETYPE_OFFSET_FILE = "match_messagetype_offset.rs"
MATCH_MESSAGETYPE_SCALE_FILE = "match_messagetype_scale.rs"
MATCH_MESSAGETYPE_FILE = "match_messagetype.rs"
MATCH_CUSTOM_FIELD_VALUE_FILE = "match_custom_field_value.rs"


def camel_case(s):
    return ''.join(x.title() for x in s.split('_'))


def snake_case(s):
    return s.lower()


def shouty_snake_case(s):
    return snake_case(s).upper()


def write_fieldtype_enum(set_ft):
    f = open("src/" + FIELDTYPE_ENUM_FILE, "w+")
    f.writelines([
        "#[derive(Debug, Copy, Clone, PartialEq)]\n",
        "pub enum FieldType {\n"
    ])
    f.writelines(f"    {camel_case(v)},\n" for v in set_ft)
    f.writelines([
        "    Coordinates,\n"
        "    None,\n",
        "}\n"
    ])
    f.close()


def write_messagetype_enum(set_mt):
    f = open("src/" + MESSAGETYPE_ENUM_FILE, "w+")
    f.writelines([
        "#[derive(Debug, Copy, Clone, PartialEq)]\n",
        "pub enum MessageType {\n"
    ])
    f.writelines(f"    {camel_case(v)},\n" for v in set_mt)
    f.writelines([
        "    Pad,\n"
        "    MfgRangeMax,\n",
        "    MfgRangeMin,\n",
        "    None,\n",
        "}\n",
    ])
    f.close()


def write_match_message_field(set_mt, msgs_store):
    f = open("src/" + MATCH_MESSAGETYPE_FIELD_FILE, "w+")
    f.writelines([
        "use super::{FieldType, MessageType};\n",
        "type MatchFieldFn = dyn Fn(usize) -> FieldType;\n"
    ])
    for message_type in set_mt:
        values = next(x['values'] for x in iter(msgs_store) if x['name'] == message_type)
        values = sorted(values, key=lambda k: k['id'])
        f.writelines([
            f"fn match_field_{message_type.lower()}(k: usize) -> FieldType {{\n"
            "    match k {\n"
        ])
        f.writelines(f"        {m['id']} => FieldType::{camel_case(m['field_type'])},\n" for m in values)
        f.writelines([
            "        _ => FieldType::None\n",
            "    }"
            "}\n"
        ])
    f.writelines([
        "fn match_field_none(_: usize) -> FieldType {\n",
        "    return FieldType::None;\n",
        "}\n"
    ])
    f.writelines([
        "pub fn match_message_field(m: MessageType) -> &'static MatchFieldFn {\n",
        "    match m {\n"
    ])

    f.writelines(f"        MessageType::{camel_case(v)} => &|x: usize| match_field_{v.lower()}(x),\n" for v in set_mt)
    f.writelines([
        "        _ => &|x: usize| match_field_none(x)\n",
        "    }\n",
        "}\n"
    ])
    f.close()


def write_match_message_offset(set_mt, msgs_store):
    f = open("src/" + MATCH_MESSAGETYPE_OFFSET_FILE, "w+")
    f.writelines([
        "use super::MessageType;\n",
        "type MatchOffsetFn = dyn Fn(usize) -> Option<i16>;\n"
    ])
    for message_type in set_mt:
        values = next(x['values'] for x in iter(msgs_store) if x['name'] == message_type)
        values = sorted(values, key=lambda k: k['id'])
        f.writelines([
            f"fn match_offset_{message_type.lower()}(k: usize) -> Option<i16> {{\n",
            "    match k {\n"
        ])
        f.writelines(f"        {m['id']} => Some({int(m['offset'])}i16),\n" for m in values if pd.notna(m['offset']))
        f.writelines([
            "        _ => None,\n",
            "    }\n"
            "}\n"
        ])
    f.writelines([
        "fn match_offset_none(_: usize) -> Option<i16> {\n",
        "    return None;\n",
        "}\n"
    ])
    f.writelines([
        "pub fn match_message_offset(m: MessageType) -> &'static MatchOffsetFn {\n",
        "    match m {\n"
    ])

    f.writelines(f"        MessageType::{camel_case(v)} => &|x: usize| match_offset_{v.lower()}(x),\n" for v in set_mt)
    f.writelines([
        "        _ => &|x: usize| match_offset_none(x)\n",
        "    }\n",
        "}\n"
    ])
    f.close()


def write_match_message_scale(set_mt, msgs_store):
    f = open("src/" + MATCH_MESSAGETYPE_SCALE_FILE, "w+")
    f.writelines([
        "use super::MessageType;\n",
        "type MatchScaleFn = dyn Fn(usize) -> Option<f32>;\n"
    ])
    for message_type in set_mt:
        values = next(x['values'] for x in iter(msgs_store) if x['name'] == message_type)
        values = sorted(values, key=lambda k: k['id'])
        f.writelines([
            f"fn match_scale_{message_type.lower()}(k: usize) -> Option<f32> {{\n",
            "    match k {\n"
        ])
        for m in values:
            if pd.notna(m['scale']):
                v = m['scale']
                try:
                    v = m['scale'].split(',')[0]
                except AttributeError:
                    pass
                f.write(f"        {m['id']} => Some({float(v)}f32),\n")
        f.writelines([
            "        _ => None,\n",
            "    }\n"
            "}\n"
        ])
    f.writelines([
        "fn match_scale_none(_: usize) -> Option<f32> {\n",
        "    return None;\n",
        "}\n"
    ])
    f.writelines([
        "pub fn match_message_scale(m: MessageType) -> &'static MatchScaleFn {\n",
        "    match m {\n"
    ])
    f.writelines(f"        MessageType::{camel_case(v)} => &|x: usize| match_scale_{v.lower()}(x),\n" for v in set_mt)
    f.writelines([
        "        _ => &|x: usize| match_scale_none(x)\n",
        "    }\n",
        "}\n"
    ])
    f.close()


def write_match_messagetype(map):
    f = open("src/" + MATCH_MESSAGETYPE_FILE, "w+")
    f.writelines([
        "use super::MessageType;\n",
        "pub fn match_messagetype(k: u16) -> MessageType {\n",
        "    match k {\n"
    ])
    f.writelines(f"        {k} => MessageType::{camel_case(v)},\n" for k, v in map.items())
    f.writelines([
        "        _ => MessageType::None\n",
        "    }\n",
        "}\n"
    ])
    f.close()


def write_match_custom_field_value(set_f, types_store):
    f = open("src/" + MATCH_CUSTOM_FIELD_VALUE_FILE, "w+")
    f.writelines([
        "use super::FieldType;\n",
        "pub fn match_custom_field_value(f: FieldType, k: usize) -> Option<&'static str> {\n",
        "    match f {\n"
    ])
    for ft in set_f:
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
        else:
            val = row[4]
        r = {'id': int(row[2]), 'field_name': row[3], 'field_type': val, 'scale': row[7], 'offset': row[8]}
        msg['values'].append(r)
msgs_list.append(msg)

msg_types = next(x for x in iter(types_list) if x['name'] == 'mesg_num')
msgs = [x['name'] for x in msgs_list]

write_fieldtype_enum(fields_set)
write_messagetype_enum(msgs)
write_match_message_field(msgs, msgs_list)
write_match_message_offset(msgs, msgs_list)
write_match_message_scale(msgs, msgs_list)
write_match_messagetype(msg_types['values'])
write_match_custom_field_value(fields_set, types_list)
