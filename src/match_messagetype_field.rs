use super::{FieldType, MessageType};
type MatchFieldFn = dyn Fn(usize) -> FieldType;
fn match_field_file_id(k: usize) -> FieldType {
    match k {
        0 => FieldType::File,
        1 => FieldType::Manufacturer,
        2 => FieldType::Uint16,
        3 => FieldType::Uint32Z,
        4 => FieldType::DateTime,
        5 => FieldType::Uint16,
        8 => FieldType::String,
        _ => FieldType::None
    }}
fn match_field_file_creator(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::Uint8,
        _ => FieldType::None
    }}
fn match_field_timestamp_correlation(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::DateTime,
        2 => FieldType::Uint16,
        3 => FieldType::LocalDateTime,
        4 => FieldType::Uint16,
        5 => FieldType::Uint16,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_software(k: usize) -> FieldType {
    match k {
        3 => FieldType::Uint16,
        5 => FieldType::String,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_slave_device(k: usize) -> FieldType {
    match k {
        0 => FieldType::Manufacturer,
        1 => FieldType::Uint16,
        _ => FieldType::None
    }}
fn match_field_capabilities(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint8Z,
        1 => FieldType::SportBits0,
        21 => FieldType::WorkoutCapabilities,
        23 => FieldType::ConnectivityCapabilities,
        _ => FieldType::None
    }}
fn match_field_file_capabilities(k: usize) -> FieldType {
    match k {
        0 => FieldType::File,
        1 => FieldType::FileFlags,
        2 => FieldType::String,
        3 => FieldType::Uint16,
        4 => FieldType::Uint32,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_mesg_capabilities(k: usize) -> FieldType {
    match k {
        0 => FieldType::File,
        1 => FieldType::MesgNum,
        2 => FieldType::MesgCount,
        3 => FieldType::Uint16,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_field_capabilities(k: usize) -> FieldType {
    match k {
        0 => FieldType::File,
        1 => FieldType::MesgNum,
        2 => FieldType::Uint8,
        3 => FieldType::Uint16,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_device_settings(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint8,
        1 => FieldType::Uint32,
        2 => FieldType::Uint32,
        4 => FieldType::TimeMode,
        5 => FieldType::Sint8,
        12 => FieldType::BacklightMode,
        36 => FieldType::Bool,
        39 => FieldType::DateTime,
        40 => FieldType::Uint16,
        46 => FieldType::Bool,
        47 => FieldType::DateMode,
        55 => FieldType::DisplayOrientation,
        56 => FieldType::Side,
        57 => FieldType::Uint16,
        58 => FieldType::Uint16,
        59 => FieldType::Uint16,
        80 => FieldType::Bool,
        86 => FieldType::Bool,
        89 => FieldType::AutoSyncFrequency,
        90 => FieldType::AutoActivityDetect,
        94 => FieldType::Uint8,
        95 => FieldType::DisplayOrientation,
        134 => FieldType::Switch,
        _ => FieldType::None
    }}
fn match_field_user_profile(k: usize) -> FieldType {
    match k {
        0 => FieldType::String,
        1 => FieldType::Gender,
        2 => FieldType::Uint8,
        3 => FieldType::Uint8,
        4 => FieldType::Uint16,
        5 => FieldType::Language,
        6 => FieldType::DisplayMeasure,
        7 => FieldType::DisplayMeasure,
        8 => FieldType::Uint8,
        9 => FieldType::Uint8,
        10 => FieldType::Uint8,
        11 => FieldType::Uint8,
        12 => FieldType::DisplayHeart,
        13 => FieldType::DisplayMeasure,
        14 => FieldType::DisplayMeasure,
        16 => FieldType::DisplayPower,
        17 => FieldType::ActivityClass,
        18 => FieldType::DisplayPosition,
        21 => FieldType::DisplayMeasure,
        22 => FieldType::UserLocalId,
        23 => FieldType::Byte,
        28 => FieldType::LocaltimeIntoDay,
        29 => FieldType::LocaltimeIntoDay,
        30 => FieldType::DisplayMeasure,
        31 => FieldType::Uint16,
        32 => FieldType::Uint16,
        47 => FieldType::DisplayMeasure,
        49 => FieldType::Uint32,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_hrm_profile(k: usize) -> FieldType {
    match k {
        0 => FieldType::Bool,
        1 => FieldType::Uint16Z,
        2 => FieldType::Bool,
        3 => FieldType::Uint8Z,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_sdm_profile(k: usize) -> FieldType {
    match k {
        0 => FieldType::Bool,
        1 => FieldType::Uint16Z,
        2 => FieldType::Uint16,
        3 => FieldType::Uint32,
        4 => FieldType::Bool,
        5 => FieldType::Uint8Z,
        7 => FieldType::Uint8,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_bike_profile(k: usize) -> FieldType {
    match k {
        0 => FieldType::String,
        1 => FieldType::Sport,
        2 => FieldType::SubSport,
        3 => FieldType::Uint32,
        4 => FieldType::Uint16Z,
        5 => FieldType::Uint16Z,
        6 => FieldType::Uint16Z,
        7 => FieldType::Uint16Z,
        8 => FieldType::Uint16,
        9 => FieldType::Uint16,
        10 => FieldType::Uint16,
        11 => FieldType::Uint16,
        12 => FieldType::Bool,
        13 => FieldType::Bool,
        14 => FieldType::Uint8,
        15 => FieldType::Bool,
        16 => FieldType::Bool,
        17 => FieldType::Bool,
        18 => FieldType::Bool,
        19 => FieldType::Uint8,
        20 => FieldType::Bool,
        21 => FieldType::Uint8Z,
        22 => FieldType::Uint8Z,
        23 => FieldType::Uint8Z,
        24 => FieldType::Uint8Z,
        37 => FieldType::Uint8,
        38 => FieldType::Uint8Z,
        39 => FieldType::Uint8Z,
        40 => FieldType::Uint8Z,
        41 => FieldType::Uint8Z,
        44 => FieldType::Bool,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_connectivity(k: usize) -> FieldType {
    match k {
        0 => FieldType::Bool,
        1 => FieldType::Bool,
        2 => FieldType::Bool,
        3 => FieldType::String,
        4 => FieldType::Bool,
        5 => FieldType::Bool,
        6 => FieldType::Bool,
        7 => FieldType::Bool,
        8 => FieldType::Bool,
        9 => FieldType::Bool,
        10 => FieldType::Bool,
        11 => FieldType::Bool,
        12 => FieldType::Bool,
        _ => FieldType::None
    }}
fn match_field_watchface_settings(k: usize) -> FieldType {
    match k {
        0 => FieldType::WatchfaceMode,
        1 => FieldType::Byte,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_ohr_settings(k: usize) -> FieldType {
    match k {
        0 => FieldType::Switch,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_zones_target(k: usize) -> FieldType {
    match k {
        1 => FieldType::Uint8,
        2 => FieldType::Uint8,
        3 => FieldType::Uint16,
        5 => FieldType::HrZoneCalc,
        7 => FieldType::PwrZoneCalc,
        _ => FieldType::None
    }}
fn match_field_sport(k: usize) -> FieldType {
    match k {
        0 => FieldType::Sport,
        1 => FieldType::SubSport,
        3 => FieldType::String,
        _ => FieldType::None
    }}
fn match_field_hr_zone(k: usize) -> FieldType {
    match k {
        1 => FieldType::Uint8,
        2 => FieldType::String,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_speed_zone(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::String,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_cadence_zone(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint8,
        1 => FieldType::String,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_power_zone(k: usize) -> FieldType {
    match k {
        1 => FieldType::Uint16,
        2 => FieldType::String,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_met_zone(k: usize) -> FieldType {
    match k {
        1 => FieldType::Uint8,
        2 => FieldType::Uint16,
        3 => FieldType::Uint8,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_dive_settings(k: usize) -> FieldType {
    match k {
        0 => FieldType::String,
        1 => FieldType::TissueModelType,
        2 => FieldType::Uint8,
        3 => FieldType::Uint8,
        4 => FieldType::WaterType,
        5 => FieldType::Float32,
        6 => FieldType::Uint8,
        7 => FieldType::Uint8,
        8 => FieldType::Uint8,
        9 => FieldType::Bool,
        10 => FieldType::Float32,
        11 => FieldType::Uint32,
        12 => FieldType::Bool,
        13 => FieldType::Uint32,
        14 => FieldType::DiveBacklightMode,
        15 => FieldType::Uint8,
        16 => FieldType::BacklightTimeout,
        17 => FieldType::Uint16,
        18 => FieldType::Uint16,
        19 => FieldType::SourceType,
        20 => FieldType::Uint8,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_dive_alarm(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint32,
        1 => FieldType::Sint32,
        2 => FieldType::Bool,
        3 => FieldType::DiveAlarmType,
        4 => FieldType::Tone,
        5 => FieldType::SubSport,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_dive_gas(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint8,
        1 => FieldType::Uint8,
        2 => FieldType::DiveGasStatus,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_goal(k: usize) -> FieldType {
    match k {
        0 => FieldType::Sport,
        1 => FieldType::SubSport,
        2 => FieldType::DateTime,
        3 => FieldType::DateTime,
        4 => FieldType::Goal,
        5 => FieldType::Uint32,
        6 => FieldType::Bool,
        7 => FieldType::Uint32,
        8 => FieldType::GoalRecurrence,
        9 => FieldType::Uint16,
        10 => FieldType::Bool,
        11 => FieldType::GoalSource,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_activity(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint32,
        1 => FieldType::Uint16,
        2 => FieldType::Activity,
        3 => FieldType::Event,
        4 => FieldType::EventType,
        5 => FieldType::LocalDateTime,
        6 => FieldType::Uint8,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_session(k: usize) -> FieldType {
    match k {
        0 => FieldType::Event,
        1 => FieldType::EventType,
        2 => FieldType::DateTime,
        3 => FieldType::Coordinates,
        4 => FieldType::Coordinates,
        5 => FieldType::Sport,
        6 => FieldType::SubSport,
        7 => FieldType::Uint32,
        8 => FieldType::Uint32,
        9 => FieldType::Uint32,
        10 => FieldType::Uint32,
        11 => FieldType::Uint16,
        13 => FieldType::Uint16,
        14 => FieldType::Uint16,
        15 => FieldType::Uint16,
        16 => FieldType::Uint8,
        17 => FieldType::Uint8,
        18 => FieldType::Uint8,
        19 => FieldType::Uint8,
        20 => FieldType::Uint16,
        21 => FieldType::Uint16,
        22 => FieldType::Uint16,
        23 => FieldType::Uint16,
        24 => FieldType::Uint8,
        25 => FieldType::Uint16,
        26 => FieldType::Uint16,
        27 => FieldType::Uint8,
        28 => FieldType::SessionTrigger,
        29 => FieldType::Coordinates,
        30 => FieldType::Coordinates,
        31 => FieldType::Coordinates,
        32 => FieldType::Coordinates,
        34 => FieldType::Uint16,
        35 => FieldType::Uint16,
        36 => FieldType::Uint16,
        37 => FieldType::LeftRightBalance100,
        41 => FieldType::Uint32,
        42 => FieldType::Uint16,
        43 => FieldType::SwimStroke,
        44 => FieldType::Uint16,
        45 => FieldType::Uint16,
        46 => FieldType::DisplayMeasure,
        47 => FieldType::Uint16,
        48 => FieldType::Uint32,
        49 => FieldType::Uint16,
        50 => FieldType::Uint16,
        51 => FieldType::Uint8,
        52 => FieldType::Sint16,
        53 => FieldType::Sint16,
        54 => FieldType::Sint16,
        55 => FieldType::Sint16,
        56 => FieldType::Sint16,
        57 => FieldType::Sint8,
        58 => FieldType::Sint8,
        59 => FieldType::Uint32,
        60 => FieldType::Sint16,
        61 => FieldType::Sint16,
        62 => FieldType::Sint16,
        63 => FieldType::Sint16,
        64 => FieldType::Uint8,
        65 => FieldType::Uint32,
        66 => FieldType::Uint32,
        67 => FieldType::Uint32,
        68 => FieldType::Uint32,
        69 => FieldType::Uint32,
        70 => FieldType::Uint16,
        71 => FieldType::Uint16,
        82 => FieldType::Uint16,
        83 => FieldType::Uint16,
        84 => FieldType::String,
        85 => FieldType::Uint16,
        86 => FieldType::Uint16,
        87 => FieldType::Uint16,
        88 => FieldType::Uint16,
        89 => FieldType::Uint16,
        90 => FieldType::Uint16,
        91 => FieldType::Uint16,
        92 => FieldType::Uint8,
        93 => FieldType::Uint8,
        94 => FieldType::Uint8,
        95 => FieldType::Uint16,
        96 => FieldType::Uint16,
        97 => FieldType::Uint16,
        98 => FieldType::Uint16,
        99 => FieldType::Uint16,
        100 => FieldType::Uint16,
        101 => FieldType::Uint8,
        102 => FieldType::Uint8,
        103 => FieldType::Uint8,
        104 => FieldType::Uint8,
        105 => FieldType::Uint8,
        111 => FieldType::Uint8,
        112 => FieldType::Uint32,
        113 => FieldType::Uint16,
        114 => FieldType::Sint8,
        115 => FieldType::Sint8,
        116 => FieldType::Uint8,
        117 => FieldType::Uint8,
        118 => FieldType::Uint8,
        119 => FieldType::Uint8,
        120 => FieldType::Uint16,
        121 => FieldType::Uint16,
        122 => FieldType::Uint8,
        123 => FieldType::Uint8,
        124 => FieldType::Uint32,
        125 => FieldType::Uint32,
        126 => FieldType::Uint32,
        127 => FieldType::Uint32,
        128 => FieldType::Uint32,
        129 => FieldType::Uint16,
        130 => FieldType::Uint16,
        131 => FieldType::Uint8,
        132 => FieldType::Uint16,
        133 => FieldType::Uint16,
        134 => FieldType::Uint16,
        137 => FieldType::Uint8,
        139 => FieldType::Uint16,
        181 => FieldType::Float32,
        182 => FieldType::Float32,
        183 => FieldType::Uint16,
        186 => FieldType::Float32,
        187 => FieldType::Float32,
        253 => FieldType::DateTime,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_lap(k: usize) -> FieldType {
    match k {
        0 => FieldType::Event,
        1 => FieldType::EventType,
        2 => FieldType::DateTime,
        3 => FieldType::Coordinates,
        4 => FieldType::Coordinates,
        5 => FieldType::Coordinates,
        6 => FieldType::Coordinates,
        7 => FieldType::Uint32,
        8 => FieldType::Uint32,
        9 => FieldType::Uint32,
        10 => FieldType::Uint32,
        11 => FieldType::Uint16,
        12 => FieldType::Uint16,
        13 => FieldType::Uint16,
        14 => FieldType::Uint16,
        15 => FieldType::Uint8,
        16 => FieldType::Uint8,
        17 => FieldType::Uint8,
        18 => FieldType::Uint8,
        19 => FieldType::Uint16,
        20 => FieldType::Uint16,
        21 => FieldType::Uint16,
        22 => FieldType::Uint16,
        23 => FieldType::Intensity,
        24 => FieldType::LapTrigger,
        25 => FieldType::Sport,
        26 => FieldType::Uint8,
        32 => FieldType::Uint16,
        33 => FieldType::Uint16,
        34 => FieldType::LeftRightBalance100,
        35 => FieldType::Uint16,
        37 => FieldType::Uint16,
        38 => FieldType::SwimStroke,
        39 => FieldType::SubSport,
        40 => FieldType::Uint16,
        41 => FieldType::Uint32,
        42 => FieldType::Uint16,
        43 => FieldType::Uint16,
        44 => FieldType::Uint8,
        45 => FieldType::Sint16,
        46 => FieldType::Sint16,
        47 => FieldType::Sint16,
        48 => FieldType::Sint16,
        49 => FieldType::Sint16,
        50 => FieldType::Sint8,
        51 => FieldType::Sint8,
        52 => FieldType::Uint32,
        53 => FieldType::Sint16,
        54 => FieldType::Sint16,
        55 => FieldType::Sint16,
        56 => FieldType::Sint16,
        57 => FieldType::Uint32,
        58 => FieldType::Uint32,
        59 => FieldType::Uint32,
        60 => FieldType::Uint32,
        61 => FieldType::Uint16,
        62 => FieldType::Uint16,
        63 => FieldType::Uint8,
        71 => FieldType::MessageIndex,
        74 => FieldType::Uint16,
        75 => FieldType::Uint16,
        76 => FieldType::Uint16,
        77 => FieldType::Uint16,
        78 => FieldType::Uint16,
        79 => FieldType::Uint16,
        80 => FieldType::Uint8,
        81 => FieldType::Uint8,
        82 => FieldType::Uint8,
        83 => FieldType::Uint16,
        84 => FieldType::Uint16,
        85 => FieldType::Uint16,
        86 => FieldType::Uint16,
        87 => FieldType::Uint16,
        88 => FieldType::Uint16,
        89 => FieldType::Uint16,
        91 => FieldType::Uint8,
        92 => FieldType::Uint8,
        93 => FieldType::Uint8,
        94 => FieldType::Uint8,
        95 => FieldType::Uint8,
        98 => FieldType::Uint32,
        99 => FieldType::Uint16,
        100 => FieldType::Sint8,
        101 => FieldType::Sint8,
        102 => FieldType::Uint8,
        103 => FieldType::Uint8,
        104 => FieldType::Uint8,
        105 => FieldType::Uint8,
        106 => FieldType::Uint16,
        107 => FieldType::Uint16,
        108 => FieldType::Uint8,
        109 => FieldType::Uint8,
        110 => FieldType::Uint32,
        111 => FieldType::Uint32,
        112 => FieldType::Uint32,
        113 => FieldType::Uint32,
        114 => FieldType::Uint32,
        115 => FieldType::Uint16,
        116 => FieldType::Uint16,
        117 => FieldType::Uint8,
        118 => FieldType::Uint16,
        119 => FieldType::Uint16,
        120 => FieldType::Uint16,
        121 => FieldType::Uint16,
        149 => FieldType::Float32,
        150 => FieldType::Float32,
        151 => FieldType::Uint16,
        153 => FieldType::Float32,
        154 => FieldType::Float32,
        253 => FieldType::DateTime,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_length(k: usize) -> FieldType {
    match k {
        0 => FieldType::Event,
        1 => FieldType::EventType,
        2 => FieldType::DateTime,
        3 => FieldType::Uint32,
        4 => FieldType::Uint32,
        5 => FieldType::Uint16,
        6 => FieldType::Uint16,
        7 => FieldType::SwimStroke,
        9 => FieldType::Uint8,
        10 => FieldType::Uint8,
        11 => FieldType::Uint16,
        12 => FieldType::LengthType,
        18 => FieldType::Uint16,
        19 => FieldType::Uint16,
        20 => FieldType::Uint16,
        21 => FieldType::Uint16,
        253 => FieldType::DateTime,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_record(k: usize) -> FieldType {
    match k {
        0 => FieldType::Coordinates,
        1 => FieldType::Coordinates,
        2 => FieldType::Uint16,
        3 => FieldType::Uint8,
        4 => FieldType::Uint8,
        5 => FieldType::Uint32,
        6 => FieldType::Uint16,
        7 => FieldType::Uint16,
        8 => FieldType::Byte,
        9 => FieldType::Sint16,
        10 => FieldType::Uint8,
        11 => FieldType::Sint32,
        12 => FieldType::Uint8,
        13 => FieldType::Sint8,
        17 => FieldType::Uint8,
        18 => FieldType::Uint8,
        19 => FieldType::Uint32,
        28 => FieldType::Uint16,
        29 => FieldType::Uint32,
        30 => FieldType::LeftRightBalance,
        31 => FieldType::Uint8,
        32 => FieldType::Sint16,
        33 => FieldType::Uint16,
        39 => FieldType::Uint16,
        40 => FieldType::Uint16,
        41 => FieldType::Uint16,
        42 => FieldType::ActivityType,
        43 => FieldType::Uint8,
        44 => FieldType::Uint8,
        45 => FieldType::Uint8,
        46 => FieldType::Uint8,
        47 => FieldType::Uint8,
        48 => FieldType::Uint8,
        49 => FieldType::StrokeType,
        50 => FieldType::Uint8,
        51 => FieldType::Uint16,
        52 => FieldType::Uint16,
        53 => FieldType::Uint8,
        54 => FieldType::Uint16,
        55 => FieldType::Uint16,
        56 => FieldType::Uint16,
        57 => FieldType::Uint16,
        58 => FieldType::Uint16,
        59 => FieldType::Uint16,
        62 => FieldType::DeviceIndex,
        67 => FieldType::Sint8,
        68 => FieldType::Sint8,
        69 => FieldType::Uint8,
        70 => FieldType::Uint8,
        71 => FieldType::Uint8,
        72 => FieldType::Uint8,
        73 => FieldType::Uint32,
        78 => FieldType::Uint32,
        81 => FieldType::Uint8,
        82 => FieldType::Uint16,
        83 => FieldType::Uint16,
        84 => FieldType::Uint16,
        85 => FieldType::Uint16,
        91 => FieldType::Uint32,
        92 => FieldType::Uint32,
        93 => FieldType::Uint32,
        94 => FieldType::Uint32,
        95 => FieldType::Uint32,
        96 => FieldType::Uint32,
        97 => FieldType::Uint8,
        98 => FieldType::Uint16,
        114 => FieldType::Float32,
        115 => FieldType::Float32,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_event(k: usize) -> FieldType {
    match k {
        0 => FieldType::Event,
        1 => FieldType::EventType,
        2 => FieldType::Uint16,
        3 => FieldType::Uint32,
        4 => FieldType::Uint8,
        7 => FieldType::Uint16,
        8 => FieldType::Uint16,
        9 => FieldType::Uint8Z,
        10 => FieldType::Uint8Z,
        11 => FieldType::Uint8Z,
        12 => FieldType::Uint8Z,
        13 => FieldType::DeviceIndex,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_device_info(k: usize) -> FieldType {
    match k {
        0 => FieldType::DeviceIndex,
        1 => FieldType::Uint8,
        2 => FieldType::Manufacturer,
        3 => FieldType::Uint32Z,
        4 => FieldType::Uint16,
        5 => FieldType::Uint16,
        6 => FieldType::Uint8,
        7 => FieldType::Uint32,
        10 => FieldType::Uint16,
        11 => FieldType::BatteryStatus,
        18 => FieldType::BodyLocation,
        19 => FieldType::String,
        20 => FieldType::Uint8Z,
        21 => FieldType::Uint16Z,
        22 => FieldType::AntNetwork,
        25 => FieldType::SourceType,
        27 => FieldType::String,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_training_file(k: usize) -> FieldType {
    match k {
        0 => FieldType::File,
        1 => FieldType::Manufacturer,
        2 => FieldType::Uint16,
        3 => FieldType::Uint32Z,
        4 => FieldType::DateTime,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_hrv(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        _ => FieldType::None
    }}
fn match_field_weather_conditions(k: usize) -> FieldType {
    match k {
        0 => FieldType::WeatherReport,
        1 => FieldType::Sint8,
        2 => FieldType::WeatherStatus,
        3 => FieldType::Uint16,
        4 => FieldType::Uint16,
        5 => FieldType::Uint8,
        6 => FieldType::Sint8,
        7 => FieldType::Uint8,
        8 => FieldType::String,
        9 => FieldType::DateTime,
        10 => FieldType::Coordinates,
        11 => FieldType::Coordinates,
        12 => FieldType::DayOfWeek,
        13 => FieldType::Sint8,
        14 => FieldType::Sint8,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_weather_alert(k: usize) -> FieldType {
    match k {
        0 => FieldType::String,
        1 => FieldType::DateTime,
        2 => FieldType::DateTime,
        3 => FieldType::WeatherSeverity,
        4 => FieldType::WeatherSevereType,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_gps_metadata(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::Coordinates,
        2 => FieldType::Coordinates,
        3 => FieldType::Uint32,
        4 => FieldType::Uint32,
        5 => FieldType::Uint16,
        6 => FieldType::DateTime,
        7 => FieldType::Sint16,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_camera_event(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::CameraEventType,
        2 => FieldType::String,
        3 => FieldType::CameraOrientationType,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_gyroscope_data(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::Uint16,
        2 => FieldType::Uint16,
        3 => FieldType::Uint16,
        4 => FieldType::Uint16,
        5 => FieldType::Float32,
        6 => FieldType::Float32,
        7 => FieldType::Float32,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_accelerometer_data(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::Uint16,
        2 => FieldType::Uint16,
        3 => FieldType::Uint16,
        4 => FieldType::Uint16,
        5 => FieldType::Float32,
        6 => FieldType::Float32,
        7 => FieldType::Float32,
        8 => FieldType::Sint16,
        9 => FieldType::Sint16,
        10 => FieldType::Sint16,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_magnetometer_data(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::Uint16,
        2 => FieldType::Uint16,
        3 => FieldType::Uint16,
        4 => FieldType::Uint16,
        5 => FieldType::Float32,
        6 => FieldType::Float32,
        7 => FieldType::Float32,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_barometer_data(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::Uint16,
        2 => FieldType::Uint32,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_three_d_sensor_calibration(k: usize) -> FieldType {
    match k {
        0 => FieldType::SensorType,
        1 => FieldType::Uint32,
        2 => FieldType::Uint32,
        3 => FieldType::Uint32,
        4 => FieldType::Sint32,
        5 => FieldType::Sint32,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_one_d_sensor_calibration(k: usize) -> FieldType {
    match k {
        0 => FieldType::SensorType,
        1 => FieldType::Uint32,
        2 => FieldType::Uint32,
        3 => FieldType::Uint32,
        4 => FieldType::Sint32,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_video_frame(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::Uint32,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_obdii_data(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::Uint16,
        2 => FieldType::Byte,
        3 => FieldType::Byte,
        4 => FieldType::Uint8,
        5 => FieldType::Uint32,
        6 => FieldType::DateTime,
        7 => FieldType::Uint16,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_nmea_sentence(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::String,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_aviation_attitude(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::Uint32,
        2 => FieldType::Sint16,
        3 => FieldType::Sint16,
        4 => FieldType::Sint16,
        5 => FieldType::Sint16,
        6 => FieldType::Sint16,
        7 => FieldType::AttitudeStage,
        8 => FieldType::Uint8,
        9 => FieldType::Uint16,
        10 => FieldType::AttitudeValidity,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_video(k: usize) -> FieldType {
    match k {
        0 => FieldType::String,
        1 => FieldType::String,
        2 => FieldType::Uint32,
        _ => FieldType::None
    }}
fn match_field_video_title(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::String,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_video_description(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::String,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_video_clip(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::DateTime,
        2 => FieldType::Uint16,
        3 => FieldType::DateTime,
        4 => FieldType::Uint16,
        6 => FieldType::Uint32,
        7 => FieldType::Uint32,
        _ => FieldType::None
    }}
fn match_field_set(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint32,
        3 => FieldType::Uint16,
        4 => FieldType::Uint16,
        5 => FieldType::SetType,
        6 => FieldType::DateTime,
        7 => FieldType::ExerciseCategory,
        8 => FieldType::Uint16,
        9 => FieldType::FitBaseUnit,
        10 => FieldType::MessageIndex,
        11 => FieldType::MessageIndex,
        254 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_jump(k: usize) -> FieldType {
    match k {
        0 => FieldType::Float32,
        1 => FieldType::Float32,
        2 => FieldType::Uint8,
        3 => FieldType::Float32,
        4 => FieldType::Float32,
        5 => FieldType::Coordinates,
        6 => FieldType::Coordinates,
        7 => FieldType::Uint16,
        8 => FieldType::Uint32,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_course(k: usize) -> FieldType {
    match k {
        4 => FieldType::Sport,
        5 => FieldType::String,
        6 => FieldType::CourseCapabilities,
        7 => FieldType::SubSport,
        _ => FieldType::None
    }}
fn match_field_course_point(k: usize) -> FieldType {
    match k {
        1 => FieldType::DateTime,
        2 => FieldType::Coordinates,
        3 => FieldType::Coordinates,
        4 => FieldType::Uint32,
        5 => FieldType::CoursePoint,
        6 => FieldType::String,
        8 => FieldType::Bool,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_segment_id(k: usize) -> FieldType {
    match k {
        0 => FieldType::String,
        1 => FieldType::String,
        2 => FieldType::Sport,
        3 => FieldType::Bool,
        4 => FieldType::Uint32,
        5 => FieldType::Uint32,
        6 => FieldType::Uint8,
        7 => FieldType::SegmentDeleteStatus,
        8 => FieldType::SegmentSelectionType,
        _ => FieldType::None
    }}
fn match_field_segment_leaderboard_entry(k: usize) -> FieldType {
    match k {
        0 => FieldType::String,
        1 => FieldType::SegmentLeaderboardType,
        2 => FieldType::Uint32,
        3 => FieldType::Uint32,
        4 => FieldType::Uint32,
        5 => FieldType::String,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_segment_point(k: usize) -> FieldType {
    match k {
        1 => FieldType::Coordinates,
        2 => FieldType::Coordinates,
        3 => FieldType::Uint32,
        4 => FieldType::Uint16,
        5 => FieldType::Uint32,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_segment_lap(k: usize) -> FieldType {
    match k {
        0 => FieldType::Event,
        1 => FieldType::EventType,
        2 => FieldType::DateTime,
        3 => FieldType::Coordinates,
        4 => FieldType::Coordinates,
        5 => FieldType::Coordinates,
        6 => FieldType::Coordinates,
        7 => FieldType::Uint32,
        8 => FieldType::Uint32,
        9 => FieldType::Uint32,
        10 => FieldType::Uint32,
        11 => FieldType::Uint16,
        12 => FieldType::Uint16,
        13 => FieldType::Uint16,
        14 => FieldType::Uint16,
        15 => FieldType::Uint8,
        16 => FieldType::Uint8,
        17 => FieldType::Uint8,
        18 => FieldType::Uint8,
        19 => FieldType::Uint16,
        20 => FieldType::Uint16,
        21 => FieldType::Uint16,
        22 => FieldType::Uint16,
        23 => FieldType::Sport,
        24 => FieldType::Uint8,
        25 => FieldType::Coordinates,
        26 => FieldType::Coordinates,
        27 => FieldType::Coordinates,
        28 => FieldType::Coordinates,
        29 => FieldType::String,
        30 => FieldType::Uint16,
        31 => FieldType::LeftRightBalance100,
        32 => FieldType::SubSport,
        33 => FieldType::Uint32,
        34 => FieldType::Uint16,
        35 => FieldType::Uint16,
        36 => FieldType::Uint8,
        37 => FieldType::Sint16,
        38 => FieldType::Sint16,
        39 => FieldType::Sint16,
        40 => FieldType::Sint16,
        41 => FieldType::Sint16,
        42 => FieldType::Sint8,
        43 => FieldType::Sint8,
        44 => FieldType::Uint32,
        45 => FieldType::Sint16,
        46 => FieldType::Sint16,
        47 => FieldType::Sint16,
        48 => FieldType::Sint16,
        49 => FieldType::Uint32,
        50 => FieldType::Uint32,
        51 => FieldType::Uint32,
        52 => FieldType::Uint32,
        53 => FieldType::Uint16,
        54 => FieldType::Uint16,
        55 => FieldType::Uint8,
        56 => FieldType::Uint32,
        57 => FieldType::MessageIndex,
        58 => FieldType::SportEvent,
        59 => FieldType::Uint8,
        60 => FieldType::Uint8,
        61 => FieldType::Uint8,
        62 => FieldType::Uint8,
        63 => FieldType::Uint8,
        64 => FieldType::SegmentLapStatus,
        65 => FieldType::String,
        66 => FieldType::Uint8,
        67 => FieldType::Uint8,
        68 => FieldType::Uint8,
        69 => FieldType::Uint16,
        70 => FieldType::Uint16,
        71 => FieldType::Uint32,
        72 => FieldType::Uint16,
        73 => FieldType::Sint8,
        74 => FieldType::Sint8,
        75 => FieldType::Uint8,
        76 => FieldType::Uint8,
        77 => FieldType::Uint8,
        78 => FieldType::Uint8,
        79 => FieldType::Uint16,
        80 => FieldType::Uint16,
        81 => FieldType::Uint8,
        82 => FieldType::Uint8,
        83 => FieldType::Manufacturer,
        84 => FieldType::Float32,
        85 => FieldType::Float32,
        86 => FieldType::Float32,
        87 => FieldType::Float32,
        253 => FieldType::DateTime,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_segment_file(k: usize) -> FieldType {
    match k {
        1 => FieldType::String,
        3 => FieldType::Bool,
        4 => FieldType::Uint32,
        7 => FieldType::SegmentLeaderboardType,
        8 => FieldType::Uint32,
        9 => FieldType::Uint32,
        10 => FieldType::String,
        11 => FieldType::Uint8,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_workout(k: usize) -> FieldType {
    match k {
        4 => FieldType::Sport,
        5 => FieldType::WorkoutCapabilities,
        6 => FieldType::Uint16,
        8 => FieldType::String,
        11 => FieldType::SubSport,
        14 => FieldType::Uint16,
        15 => FieldType::DisplayMeasure,
        _ => FieldType::None
    }}
fn match_field_workout_session(k: usize) -> FieldType {
    match k {
        0 => FieldType::Sport,
        1 => FieldType::SubSport,
        2 => FieldType::Uint16,
        3 => FieldType::Uint16,
        4 => FieldType::Uint16,
        5 => FieldType::DisplayMeasure,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_workout_step(k: usize) -> FieldType {
    match k {
        0 => FieldType::String,
        1 => FieldType::WktStepDuration,
        2 => FieldType::Uint32,
        3 => FieldType::WktStepTarget,
        4 => FieldType::Uint32,
        5 => FieldType::Uint32,
        6 => FieldType::Uint32,
        7 => FieldType::Intensity,
        8 => FieldType::String,
        9 => FieldType::WorkoutEquipment,
        10 => FieldType::ExerciseCategory,
        11 => FieldType::Uint16,
        12 => FieldType::Uint16,
        13 => FieldType::FitBaseUnit,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_exercise_title(k: usize) -> FieldType {
    match k {
        0 => FieldType::ExerciseCategory,
        1 => FieldType::Uint16,
        2 => FieldType::String,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_schedule(k: usize) -> FieldType {
    match k {
        0 => FieldType::Manufacturer,
        1 => FieldType::Uint16,
        2 => FieldType::Uint32Z,
        3 => FieldType::DateTime,
        4 => FieldType::Bool,
        5 => FieldType::Schedule,
        6 => FieldType::LocalDateTime,
        _ => FieldType::None
    }}
fn match_field_totals(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint32,
        1 => FieldType::Uint32,
        2 => FieldType::Uint32,
        3 => FieldType::Sport,
        4 => FieldType::Uint32,
        5 => FieldType::Uint16,
        6 => FieldType::Uint32,
        9 => FieldType::Uint8,
        253 => FieldType::DateTime,
        254 => FieldType::MessageIndex,
        _ => FieldType::None
    }}
fn match_field_weight_scale(k: usize) -> FieldType {
    match k {
        0 => FieldType::Weight,
        1 => FieldType::Uint16,
        2 => FieldType::Uint16,
        3 => FieldType::Uint16,
        4 => FieldType::Uint16,
        5 => FieldType::Uint16,
        7 => FieldType::Uint16,
        8 => FieldType::Uint8,
        9 => FieldType::Uint16,
        10 => FieldType::Uint8,
        11 => FieldType::Uint8,
        12 => FieldType::MessageIndex,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_blood_pressure(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::Uint16,
        2 => FieldType::Uint16,
        3 => FieldType::Uint16,
        4 => FieldType::Uint16,
        5 => FieldType::Uint16,
        6 => FieldType::Uint8,
        7 => FieldType::HrType,
        8 => FieldType::BpStatus,
        9 => FieldType::MessageIndex,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_monitoring_info(k: usize) -> FieldType {
    match k {
        0 => FieldType::LocalDateTime,
        1 => FieldType::ActivityType,
        3 => FieldType::Uint16,
        4 => FieldType::Uint16,
        5 => FieldType::Uint16,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_monitoring(k: usize) -> FieldType {
    match k {
        0 => FieldType::DeviceIndex,
        1 => FieldType::Uint16,
        2 => FieldType::Uint32,
        3 => FieldType::Uint32,
        4 => FieldType::Uint32,
        5 => FieldType::ActivityType,
        6 => FieldType::ActivitySubtype,
        7 => FieldType::ActivityLevel,
        8 => FieldType::Uint16,
        9 => FieldType::Uint16,
        10 => FieldType::Uint16,
        11 => FieldType::LocalDateTime,
        12 => FieldType::Sint16,
        14 => FieldType::Sint16,
        15 => FieldType::Sint16,
        16 => FieldType::Uint16,
        19 => FieldType::Uint16,
        24 => FieldType::Byte,
        25 => FieldType::Uint8,
        26 => FieldType::Uint16,
        27 => FieldType::Uint8,
        28 => FieldType::Uint8,
        29 => FieldType::Uint16,
        30 => FieldType::Uint32,
        31 => FieldType::Uint32,
        32 => FieldType::Uint32,
        33 => FieldType::Uint16,
        34 => FieldType::Uint16,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_hr(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::Uint8,
        6 => FieldType::Uint8,
        9 => FieldType::Uint32,
        10 => FieldType::Byte,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_stress_level(k: usize) -> FieldType {
    match k {
        0 => FieldType::Sint16,
        1 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_memo_glob(k: usize) -> FieldType {
    match k {
        0 => FieldType::Byte,
        1 => FieldType::Uint16,
        2 => FieldType::MessageIndex,
        250 => FieldType::Uint32,
        _ => FieldType::None
    }}
fn match_field_ant_channel_id(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint8,
        1 => FieldType::Uint8Z,
        2 => FieldType::Uint16Z,
        3 => FieldType::Uint8Z,
        4 => FieldType::DeviceIndex,
        _ => FieldType::None
    }}
fn match_field_ant_rx(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::Byte,
        2 => FieldType::Byte,
        3 => FieldType::Uint8,
        4 => FieldType::Byte,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_ant_tx(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint16,
        1 => FieldType::Byte,
        2 => FieldType::Byte,
        3 => FieldType::Uint8,
        4 => FieldType::Byte,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_exd_screen_configuration(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint8,
        1 => FieldType::Uint8,
        2 => FieldType::ExdLayout,
        3 => FieldType::Bool,
        _ => FieldType::None
    }}
fn match_field_exd_data_field_configuration(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint8,
        1 => FieldType::Byte,
        2 => FieldType::Uint8,
        3 => FieldType::Uint8,
        4 => FieldType::ExdDisplayType,
        5 => FieldType::String,
        _ => FieldType::None
    }}
fn match_field_exd_data_concept_configuration(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint8,
        1 => FieldType::Byte,
        2 => FieldType::Uint8,
        3 => FieldType::Uint8,
        4 => FieldType::Uint8,
        5 => FieldType::Uint8,
        6 => FieldType::Uint8,
        8 => FieldType::ExdDataUnits,
        9 => FieldType::ExdQualifiers,
        10 => FieldType::ExdDescriptors,
        11 => FieldType::Bool,
        _ => FieldType::None
    }}
fn match_field_field_description(k: usize) -> FieldType {
    match k {
        0 => FieldType::Uint8,
        1 => FieldType::Uint8,
        2 => FieldType::FitBaseType,
        3 => FieldType::String,
        4 => FieldType::Uint8,
        5 => FieldType::String,
        6 => FieldType::Uint8,
        7 => FieldType::Sint8,
        8 => FieldType::String,
        9 => FieldType::String,
        10 => FieldType::String,
        13 => FieldType::FitBaseUnit,
        14 => FieldType::MesgNum,
        15 => FieldType::Uint8,
        _ => FieldType::None
    }}
fn match_field_developer_data_id(k: usize) -> FieldType {
    match k {
        0 => FieldType::Byte,
        1 => FieldType::Byte,
        2 => FieldType::Manufacturer,
        3 => FieldType::Uint8,
        4 => FieldType::Uint32,
        _ => FieldType::None
    }}
fn match_field_dive_summary(k: usize) -> FieldType {
    match k {
        0 => FieldType::MesgNum,
        1 => FieldType::MessageIndex,
        2 => FieldType::Uint32,
        3 => FieldType::Uint32,
        4 => FieldType::Uint32,
        5 => FieldType::Uint8,
        6 => FieldType::Uint8,
        7 => FieldType::Uint16,
        8 => FieldType::Uint16,
        9 => FieldType::Uint16,
        10 => FieldType::Uint32,
        11 => FieldType::Uint32,
        253 => FieldType::DateTime,
        _ => FieldType::None
    }}
fn match_field_none(_: usize) -> FieldType {
    return FieldType::None;
}
pub fn match_message_field(m: MessageType) -> &'static MatchFieldFn {
    match m {
        MessageType::FileId => &|x: usize| match_field_file_id(x),
        MessageType::FileCreator => &|x: usize| match_field_file_creator(x),
        MessageType::TimestampCorrelation => &|x: usize| match_field_timestamp_correlation(x),
        MessageType::Software => &|x: usize| match_field_software(x),
        MessageType::SlaveDevice => &|x: usize| match_field_slave_device(x),
        MessageType::Capabilities => &|x: usize| match_field_capabilities(x),
        MessageType::FileCapabilities => &|x: usize| match_field_file_capabilities(x),
        MessageType::MesgCapabilities => &|x: usize| match_field_mesg_capabilities(x),
        MessageType::FieldCapabilities => &|x: usize| match_field_field_capabilities(x),
        MessageType::DeviceSettings => &|x: usize| match_field_device_settings(x),
        MessageType::UserProfile => &|x: usize| match_field_user_profile(x),
        MessageType::HrmProfile => &|x: usize| match_field_hrm_profile(x),
        MessageType::SdmProfile => &|x: usize| match_field_sdm_profile(x),
        MessageType::BikeProfile => &|x: usize| match_field_bike_profile(x),
        MessageType::Connectivity => &|x: usize| match_field_connectivity(x),
        MessageType::WatchfaceSettings => &|x: usize| match_field_watchface_settings(x),
        MessageType::OhrSettings => &|x: usize| match_field_ohr_settings(x),
        MessageType::ZonesTarget => &|x: usize| match_field_zones_target(x),
        MessageType::Sport => &|x: usize| match_field_sport(x),
        MessageType::HrZone => &|x: usize| match_field_hr_zone(x),
        MessageType::SpeedZone => &|x: usize| match_field_speed_zone(x),
        MessageType::CadenceZone => &|x: usize| match_field_cadence_zone(x),
        MessageType::PowerZone => &|x: usize| match_field_power_zone(x),
        MessageType::MetZone => &|x: usize| match_field_met_zone(x),
        MessageType::DiveSettings => &|x: usize| match_field_dive_settings(x),
        MessageType::DiveAlarm => &|x: usize| match_field_dive_alarm(x),
        MessageType::DiveGas => &|x: usize| match_field_dive_gas(x),
        MessageType::Goal => &|x: usize| match_field_goal(x),
        MessageType::Activity => &|x: usize| match_field_activity(x),
        MessageType::Session => &|x: usize| match_field_session(x),
        MessageType::Lap => &|x: usize| match_field_lap(x),
        MessageType::Length => &|x: usize| match_field_length(x),
        MessageType::Record => &|x: usize| match_field_record(x),
        MessageType::Event => &|x: usize| match_field_event(x),
        MessageType::DeviceInfo => &|x: usize| match_field_device_info(x),
        MessageType::TrainingFile => &|x: usize| match_field_training_file(x),
        MessageType::Hrv => &|x: usize| match_field_hrv(x),
        MessageType::WeatherConditions => &|x: usize| match_field_weather_conditions(x),
        MessageType::WeatherAlert => &|x: usize| match_field_weather_alert(x),
        MessageType::GpsMetadata => &|x: usize| match_field_gps_metadata(x),
        MessageType::CameraEvent => &|x: usize| match_field_camera_event(x),
        MessageType::GyroscopeData => &|x: usize| match_field_gyroscope_data(x),
        MessageType::AccelerometerData => &|x: usize| match_field_accelerometer_data(x),
        MessageType::MagnetometerData => &|x: usize| match_field_magnetometer_data(x),
        MessageType::BarometerData => &|x: usize| match_field_barometer_data(x),
        MessageType::ThreeDSensorCalibration => &|x: usize| match_field_three_d_sensor_calibration(x),
        MessageType::OneDSensorCalibration => &|x: usize| match_field_one_d_sensor_calibration(x),
        MessageType::VideoFrame => &|x: usize| match_field_video_frame(x),
        MessageType::ObdiiData => &|x: usize| match_field_obdii_data(x),
        MessageType::NmeaSentence => &|x: usize| match_field_nmea_sentence(x),
        MessageType::AviationAttitude => &|x: usize| match_field_aviation_attitude(x),
        MessageType::Video => &|x: usize| match_field_video(x),
        MessageType::VideoTitle => &|x: usize| match_field_video_title(x),
        MessageType::VideoDescription => &|x: usize| match_field_video_description(x),
        MessageType::VideoClip => &|x: usize| match_field_video_clip(x),
        MessageType::Set => &|x: usize| match_field_set(x),
        MessageType::Jump => &|x: usize| match_field_jump(x),
        MessageType::Course => &|x: usize| match_field_course(x),
        MessageType::CoursePoint => &|x: usize| match_field_course_point(x),
        MessageType::SegmentId => &|x: usize| match_field_segment_id(x),
        MessageType::SegmentLeaderboardEntry => &|x: usize| match_field_segment_leaderboard_entry(x),
        MessageType::SegmentPoint => &|x: usize| match_field_segment_point(x),
        MessageType::SegmentLap => &|x: usize| match_field_segment_lap(x),
        MessageType::SegmentFile => &|x: usize| match_field_segment_file(x),
        MessageType::Workout => &|x: usize| match_field_workout(x),
        MessageType::WorkoutSession => &|x: usize| match_field_workout_session(x),
        MessageType::WorkoutStep => &|x: usize| match_field_workout_step(x),
        MessageType::ExerciseTitle => &|x: usize| match_field_exercise_title(x),
        MessageType::Schedule => &|x: usize| match_field_schedule(x),
        MessageType::Totals => &|x: usize| match_field_totals(x),
        MessageType::WeightScale => &|x: usize| match_field_weight_scale(x),
        MessageType::BloodPressure => &|x: usize| match_field_blood_pressure(x),
        MessageType::MonitoringInfo => &|x: usize| match_field_monitoring_info(x),
        MessageType::Monitoring => &|x: usize| match_field_monitoring(x),
        MessageType::Hr => &|x: usize| match_field_hr(x),
        MessageType::StressLevel => &|x: usize| match_field_stress_level(x),
        MessageType::MemoGlob => &|x: usize| match_field_memo_glob(x),
        MessageType::AntChannelId => &|x: usize| match_field_ant_channel_id(x),
        MessageType::AntRx => &|x: usize| match_field_ant_rx(x),
        MessageType::AntTx => &|x: usize| match_field_ant_tx(x),
        MessageType::ExdScreenConfiguration => &|x: usize| match_field_exd_screen_configuration(x),
        MessageType::ExdDataFieldConfiguration => &|x: usize| match_field_exd_data_field_configuration(x),
        MessageType::ExdDataConceptConfiguration => &|x: usize| match_field_exd_data_concept_configuration(x),
        MessageType::FieldDescription => &|x: usize| match_field_field_description(x),
        MessageType::DeveloperDataId => &|x: usize| match_field_developer_data_id(x),
        MessageType::DiveSummary => &|x: usize| match_field_dive_summary(x),
        _ => &|x: usize| match_field_none(x)
    }
}
