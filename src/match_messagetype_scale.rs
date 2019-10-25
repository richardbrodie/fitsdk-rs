type MatchScaleFn = dyn Fn(u8) -> Option<f32>;
fn match_scale_file_id(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_file_creator(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_timestamp_correlation(k: u8) -> Option<f32> {
    0 => Some(32768.0f32),
    2 => Some(32768.0f32),
    _ => None,
}
fn match_scale_software(k: u8) -> Option<f32> {
    3 => Some(100.0f32),
    _ => None,
}
fn match_scale_slave_device(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_capabilities(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_file_capabilities(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_mesg_capabilities(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_field_capabilities(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_device_settings(k: u8) -> Option<f32> {
    5 => Some(4.0f32),
    _ => None,
}
fn match_scale_user_profile(k: u8) -> Option<f32> {
    3 => Some(100.0f32),
    4 => Some(10.0f32),
    31 => Some(1000.0f32),
    32 => Some(1000.0f32),
    _ => None,
}
fn match_scale_hrm_profile(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_sdm_profile(k: u8) -> Option<f32> {
    2 => Some(10.0f32),
    3 => Some(100.0f32),
    _ => None,
}
fn match_scale_bike_profile(k: u8) -> Option<f32> {
    3 => Some(100.0f32),
    8 => Some(1000.0f32),
    9 => Some(1000.0f32),
    10 => Some(10.0f32),
    11 => Some(10.0f32),
    19 => Some(2.0f32),
    _ => None,
}
fn match_scale_connectivity(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_watchface_settings(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_ohr_settings(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_zones_target(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_sport(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_hr_zone(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_speed_zone(k: u8) -> Option<f32> {
    0 => Some(1000.0f32),
    _ => None,
}
fn match_scale_cadence_zone(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_power_zone(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_met_zone(k: u8) -> Option<f32> {
    2 => Some(10.0f32),
    3 => Some(10.0f32),
    _ => None,
}
fn match_scale_dive_settings(k: u8) -> Option<f32> {
    6 => Some(100.0f32),
    7 => Some(100.0f32),
    8 => Some(100.0f32),
    17 => Some(1.0f32),
    18 => Some(1.0f32),
    _ => None,
}
fn match_scale_dive_alarm(k: u8) -> Option<f32> {
    0 => Some(1000.0f32),
    1 => Some(1.0f32),
    _ => None,
}
fn match_scale_dive_gas(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_goal(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_activity(k: u8) -> Option<f32> {
    0 => Some(1000.0f32),
    _ => None,
}
fn match_scale_session(k: u8) -> Option<f32> {
    7 => Some(1000.0f32),
    8 => Some(1000.0f32),
    9 => Some(100.0f32),
    14 => Some(1000.0f32),
    15 => Some(1000.0f32),
    24 => Some(10.0f32),
    35 => Some(10.0f32),
    36 => Some(1000.0f32),
    41 => Some(10.0f32),
    42 => Some(100.0f32),
    44 => Some(100.0f32),
    49 => Some(5.0f32),
    50 => Some(5.0f32),
    52 => Some(100.0f32),
    53 => Some(100.0f32),
    54 => Some(100.0f32),
    55 => Some(100.0f32),
    56 => Some(100.0f32),
    59 => Some(1000.0f32),
    60 => Some(1000.0f32),
    61 => Some(1000.0f32),
    62 => Some(1000.0f32),
    63 => Some(1000.0f32),
    65 => Some(1000.0f32),
    66 => Some(1000.0f32),
    67 => Some(1000.0f32),
    68 => Some(1000.0f32),
    69 => Some(1000.0f32),
    71 => Some(5.0f32),
    87 => Some(100.0f32),
    88 => Some(100.0f32),
    89 => Some(10.0f32),
    90 => Some(100.0f32),
    91 => Some(10.0f32),
    92 => Some(128.0f32),
    93 => Some(128.0f32),
    94 => Some(128.0f32),
    95 => Some(100.0f32),
    96 => Some(100.0f32),
    97 => Some(100.0f32),
    98 => Some(10.0f32),
    99 => Some(10.0f32),
    100 => Some(10.0f32),
    101 => Some(2.0f32),
    102 => Some(2.0f32),
    103 => Some(2.0f32),
    104 => Some(2.0f32),
    105 => Some(2.0f32),
    112 => Some(1000.0f32),
    116 => Some(0.7111111f32),
    117 => Some(0.7111111f32),
    118 => Some(0.7111111f32),
    119 => Some(0.7111111f32),
    124 => Some(1000.0f32),
    125 => Some(1000.0f32),
    126 => Some(5.0f32),
    127 => Some(5.0f32),
    128 => Some(5.0f32),
    131 => Some(2.0f32),
    132 => Some(100.0f32),
    133 => Some(100.0f32),
    134 => Some(10.0f32),
    137 => Some(10.0f32),
    139 => Some(1000.0f32),
    _ => None,
}
fn match_scale_lap(k: u8) -> Option<f32> {
    7 => Some(1000.0f32),
    8 => Some(1000.0f32),
    9 => Some(100.0f32),
    13 => Some(1000.0f32),
    14 => Some(1000.0f32),
    37 => Some(100.0f32),
    42 => Some(5.0f32),
    43 => Some(5.0f32),
    45 => Some(100.0f32),
    46 => Some(100.0f32),
    47 => Some(100.0f32),
    48 => Some(100.0f32),
    49 => Some(100.0f32),
    52 => Some(1000.0f32),
    53 => Some(1000.0f32),
    54 => Some(1000.0f32),
    55 => Some(1000.0f32),
    56 => Some(1000.0f32),
    57 => Some(1000.0f32),
    58 => Some(1000.0f32),
    59 => Some(1000.0f32),
    60 => Some(1000.0f32),
    62 => Some(5.0f32),
    77 => Some(10.0f32),
    78 => Some(100.0f32),
    79 => Some(10.0f32),
    80 => Some(128.0f32),
    81 => Some(128.0f32),
    82 => Some(128.0f32),
    84 => Some(100.0f32),
    85 => Some(100.0f32),
    86 => Some(100.0f32),
    87 => Some(10.0f32),
    88 => Some(10.0f32),
    89 => Some(10.0f32),
    91 => Some(2.0f32),
    92 => Some(2.0f32),
    93 => Some(2.0f32),
    94 => Some(2.0f32),
    95 => Some(2.0f32),
    98 => Some(1000.0f32),
    102 => Some(0.7111111f32),
    103 => Some(0.7111111f32),
    104 => Some(0.7111111f32),
    105 => Some(0.7111111f32),
    110 => Some(1000.0f32),
    111 => Some(1000.0f32),
    112 => Some(5.0f32),
    113 => Some(5.0f32),
    114 => Some(5.0f32),
    117 => Some(2.0f32),
    118 => Some(100.0f32),
    119 => Some(100.0f32),
    120 => Some(10.0f32),
    121 => Some(1000.0f32),
    _ => None,
}
fn match_scale_length(k: u8) -> Option<f32> {
    3 => Some(1000.0f32),
    4 => Some(1000.0f32),
    6 => Some(1000.0f32),
    _ => None,
}
fn match_scale_record(k: u8) -> Option<f32> {
    2 => Some(5.0f32),
    5 => Some(100.0f32),
    6 => Some(1000.0f32),
    8 => Some(100.0f32),
    9 => Some(100.0f32),
    11 => Some(1000.0f32),
    12 => Some(100.0f32),
    17 => Some(16.0f32),
    32 => Some(1000.0f32),
    39 => Some(10.0f32),
    40 => Some(100.0f32),
    41 => Some(10.0f32),
    43 => Some(2.0f32),
    44 => Some(2.0f32),
    45 => Some(2.0f32),
    46 => Some(2.0f32),
    47 => Some(2.0f32),
    48 => Some(128.0f32),
    51 => Some(100.0f32),
    52 => Some(256.0f32),
    53 => Some(128.0f32),
    54 => Some(100.0f32),
    55 => Some(100.0f32),
    56 => Some(100.0f32),
    57 => Some(10.0f32),
    58 => Some(10.0f32),
    59 => Some(10.0f32),
    69 => Some(0.7111111f32),
    70 => Some(0.7111111f32),
    71 => Some(0.7111111f32),
    72 => Some(0.7111111f32),
    73 => Some(1000.0f32),
    78 => Some(5.0f32),
    81 => Some(2.0f32),
    83 => Some(100.0f32),
    84 => Some(100.0f32),
    85 => Some(10.0f32),
    92 => Some(1000.0f32),
    93 => Some(1000.0f32),
    94 => Some(1.0f32),
    95 => Some(1.0f32),
    96 => Some(1.0f32),
    98 => Some(1.0f32),
    _ => None,
}
fn match_scale_event(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_device_info(k: u8) -> Option<f32> {
    5 => Some(100.0f32),
    10 => Some(256.0f32),
    _ => None,
}
fn match_scale_training_file(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_hrv(k: u8) -> Option<f32> {
    0 => Some(1000.0f32),
    _ => None,
}
fn match_scale_weather_conditions(k: u8) -> Option<f32> {
    4 => Some(1000.0f32),
    _ => None,
}
fn match_scale_weather_alert(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_gps_metadata(k: u8) -> Option<f32> {
    3 => Some(5.0f32),
    4 => Some(1000.0f32),
    5 => Some(100.0f32),
    7 => Some(100.0f32),
    _ => None,
}
fn match_scale_camera_event(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_gyroscope_data(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_accelerometer_data(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_magnetometer_data(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_barometer_data(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_three_d_sensor_calibration(k: u8) -> Option<f32> {
    5 => Some(65535.0f32),
    _ => None,
}
fn match_scale_one_d_sensor_calibration(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_video_frame(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_obdii_data(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_nmea_sentence(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_aviation_attitude(k: u8) -> Option<f32> {
    2 => Some(10430.38f32),
    3 => Some(10430.38f32),
    4 => Some(100.0f32),
    5 => Some(100.0f32),
    6 => Some(1024.0f32),
    9 => Some(10430.38f32),
    _ => None,
}
fn match_scale_video(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_video_title(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_video_description(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_video_clip(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_set(k: u8) -> Option<f32> {
    0 => Some(1000.0f32),
    4 => Some(16.0f32),
    _ => None,
}
fn match_scale_jump(k: u8) -> Option<f32> {
    7 => Some(1000.0f32),
    8 => Some(1000.0f32),
    _ => None,
}
fn match_scale_course(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_course_point(k: u8) -> Option<f32> {
    4 => Some(100.0f32),
    _ => None,
}
fn match_scale_segment_id(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_segment_leaderboard_entry(k: u8) -> Option<f32> {
    4 => Some(1000.0f32),
    _ => None,
}
fn match_scale_segment_point(k: u8) -> Option<f32> {
    3 => Some(100.0f32),
    4 => Some(5.0f32),
    5 => Some(1000.0f32),
    _ => None,
}
fn match_scale_segment_lap(k: u8) -> Option<f32> {
    7 => Some(1000.0f32),
    8 => Some(1000.0f32),
    9 => Some(100.0f32),
    13 => Some(1000.0f32),
    14 => Some(1000.0f32),
    34 => Some(5.0f32),
    35 => Some(5.0f32),
    37 => Some(100.0f32),
    38 => Some(100.0f32),
    39 => Some(100.0f32),
    40 => Some(100.0f32),
    41 => Some(100.0f32),
    44 => Some(1000.0f32),
    45 => Some(1000.0f32),
    46 => Some(1000.0f32),
    47 => Some(1000.0f32),
    48 => Some(1000.0f32),
    49 => Some(1000.0f32),
    50 => Some(1000.0f32),
    51 => Some(1000.0f32),
    52 => Some(1000.0f32),
    54 => Some(5.0f32),
    56 => Some(1000.0f32),
    59 => Some(2.0f32),
    60 => Some(2.0f32),
    61 => Some(2.0f32),
    62 => Some(2.0f32),
    63 => Some(2.0f32),
    66 => Some(128.0f32),
    67 => Some(128.0f32),
    68 => Some(128.0f32),
    71 => Some(1000.0f32),
    75 => Some(0.7111111f32),
    76 => Some(0.7111111f32),
    77 => Some(0.7111111f32),
    78 => Some(0.7111111f32),
    _ => None,
}
fn match_scale_segment_file(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_workout(k: u8) -> Option<f32> {
    14 => Some(100.0f32),
    _ => None,
}
fn match_scale_workout_session(k: u8) -> Option<f32> {
    4 => Some(100.0f32),
    _ => None,
}
fn match_scale_workout_step(k: u8) -> Option<f32> {
    12 => Some(100.0f32),
    _ => None,
}
fn match_scale_exercise_title(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_schedule(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_totals(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_weight_scale(k: u8) -> Option<f32> {
    0 => Some(100.0f32),
    1 => Some(100.0f32),
    2 => Some(100.0f32),
    3 => Some(100.0f32),
    4 => Some(100.0f32),
    5 => Some(100.0f32),
    7 => Some(4.0f32),
    9 => Some(4.0f32),
    _ => None,
}
fn match_scale_blood_pressure(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_monitoring_info(k: u8) -> Option<f32> {
    3 => Some(5000.0f32),
    4 => Some(5000.0f32),
    _ => None,
}
fn match_scale_monitoring(k: u8) -> Option<f32> {
    2 => Some(100.0f32),
    3 => Some(2.0f32),
    4 => Some(1000.0f32),
    12 => Some(100.0f32),
    14 => Some(100.0f32),
    15 => Some(100.0f32),
    28 => Some(10.0f32),
    31 => Some(1000.0f32),
    32 => Some(1000.0f32),
    _ => None,
}
fn match_scale_hr(k: u8) -> Option<f32> {
    0 => Some(32768.0f32),
    1 => Some(256.0f32),
    9 => Some(1024.0f32),
    10 => Some(1024.0f32),
    _ => None,
}
fn match_scale_stress_level(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_memo_glob(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_ant_channel_id(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_ant_rx(k: u8) -> Option<f32> {
    0 => Some(32768.0f32),
    _ => None,
}
fn match_scale_ant_tx(k: u8) -> Option<f32> {
    0 => Some(32768.0f32),
    _ => None,
}
fn match_scale_exd_screen_configuration(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_exd_data_field_configuration(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_exd_data_concept_configuration(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_field_description(k: u8) -> Option<f32> {
    _ => None,
}
fn match_scale_developer_data_id(k: u8) -> Option<f32> {
    _ => None,
}
pub fn match_message_scale(m: MessageType) -> &'static MatchOffsetFn {
    match m {
        MessageType::FileId => &|x: u8| match_scale_file_id(x),
        MessageType::FileCreator => &|x: u8| match_scale_file_creator(x),
        MessageType::TimestampCorrelation => &|x: u8| match_scale_timestamp_correlation(x),
        MessageType::Software => &|x: u8| match_scale_software(x),
        MessageType::SlaveDevice => &|x: u8| match_scale_slave_device(x),
        MessageType::Capabilities => &|x: u8| match_scale_capabilities(x),
        MessageType::FileCapabilities => &|x: u8| match_scale_file_capabilities(x),
        MessageType::MesgCapabilities => &|x: u8| match_scale_mesg_capabilities(x),
        MessageType::FieldCapabilities => &|x: u8| match_scale_field_capabilities(x),
        MessageType::DeviceSettings => &|x: u8| match_scale_device_settings(x),
        MessageType::UserProfile => &|x: u8| match_scale_user_profile(x),
        MessageType::HrmProfile => &|x: u8| match_scale_hrm_profile(x),
        MessageType::SdmProfile => &|x: u8| match_scale_sdm_profile(x),
        MessageType::BikeProfile => &|x: u8| match_scale_bike_profile(x),
        MessageType::Connectivity => &|x: u8| match_scale_connectivity(x),
        MessageType::WatchfaceSettings => &|x: u8| match_scale_watchface_settings(x),
        MessageType::OhrSettings => &|x: u8| match_scale_ohr_settings(x),
        MessageType::ZonesTarget => &|x: u8| match_scale_zones_target(x),
        MessageType::Sport => &|x: u8| match_scale_sport(x),
        MessageType::HrZone => &|x: u8| match_scale_hr_zone(x),
        MessageType::SpeedZone => &|x: u8| match_scale_speed_zone(x),
        MessageType::CadenceZone => &|x: u8| match_scale_cadence_zone(x),
        MessageType::PowerZone => &|x: u8| match_scale_power_zone(x),
        MessageType::MetZone => &|x: u8| match_scale_met_zone(x),
        MessageType::DiveSettings => &|x: u8| match_scale_dive_settings(x),
        MessageType::DiveAlarm => &|x: u8| match_scale_dive_alarm(x),
        MessageType::DiveGas => &|x: u8| match_scale_dive_gas(x),
        MessageType::Goal => &|x: u8| match_scale_goal(x),
        MessageType::Activity => &|x: u8| match_scale_activity(x),
        MessageType::Session => &|x: u8| match_scale_session(x),
        MessageType::Lap => &|x: u8| match_scale_lap(x),
        MessageType::Length => &|x: u8| match_scale_length(x),
        MessageType::Record => &|x: u8| match_scale_record(x),
        MessageType::Event => &|x: u8| match_scale_event(x),
        MessageType::DeviceInfo => &|x: u8| match_scale_device_info(x),
        MessageType::TrainingFile => &|x: u8| match_scale_training_file(x),
        MessageType::Hrv => &|x: u8| match_scale_hrv(x),
        MessageType::WeatherConditions => &|x: u8| match_scale_weather_conditions(x),
        MessageType::WeatherAlert => &|x: u8| match_scale_weather_alert(x),
        MessageType::GpsMetadata => &|x: u8| match_scale_gps_metadata(x),
        MessageType::CameraEvent => &|x: u8| match_scale_camera_event(x),
        MessageType::GyroscopeData => &|x: u8| match_scale_gyroscope_data(x),
        MessageType::AccelerometerData => &|x: u8| match_scale_accelerometer_data(x),
        MessageType::MagnetometerData => &|x: u8| match_scale_magnetometer_data(x),
        MessageType::BarometerData => &|x: u8| match_scale_barometer_data(x),
        MessageType::ThreeDSensorCalibration => &|x: u8| match_scale_three_d_sensor_calibration(x),
        MessageType::OneDSensorCalibration => &|x: u8| match_scale_one_d_sensor_calibration(x),
        MessageType::VideoFrame => &|x: u8| match_scale_video_frame(x),
        MessageType::ObdiiData => &|x: u8| match_scale_obdii_data(x),
        MessageType::NmeaSentence => &|x: u8| match_scale_nmea_sentence(x),
        MessageType::AviationAttitude => &|x: u8| match_scale_aviation_attitude(x),
        MessageType::Video => &|x: u8| match_scale_video(x),
        MessageType::VideoTitle => &|x: u8| match_scale_video_title(x),
        MessageType::VideoDescription => &|x: u8| match_scale_video_description(x),
        MessageType::VideoClip => &|x: u8| match_scale_video_clip(x),
        MessageType::Set => &|x: u8| match_scale_set(x),
        MessageType::Jump => &|x: u8| match_scale_jump(x),
        MessageType::Course => &|x: u8| match_scale_course(x),
        MessageType::CoursePoint => &|x: u8| match_scale_course_point(x),
        MessageType::SegmentId => &|x: u8| match_scale_segment_id(x),
        MessageType::SegmentLeaderboardEntry => &|x: u8| match_scale_segment_leaderboard_entry(x),
        MessageType::SegmentPoint => &|x: u8| match_scale_segment_point(x),
        MessageType::SegmentLap => &|x: u8| match_scale_segment_lap(x),
        MessageType::SegmentFile => &|x: u8| match_scale_segment_file(x),
        MessageType::Workout => &|x: u8| match_scale_workout(x),
        MessageType::WorkoutSession => &|x: u8| match_scale_workout_session(x),
        MessageType::WorkoutStep => &|x: u8| match_scale_workout_step(x),
        MessageType::ExerciseTitle => &|x: u8| match_scale_exercise_title(x),
        MessageType::Schedule => &|x: u8| match_scale_schedule(x),
        MessageType::Totals => &|x: u8| match_scale_totals(x),
        MessageType::WeightScale => &|x: u8| match_scale_weight_scale(x),
        MessageType::BloodPressure => &|x: u8| match_scale_blood_pressure(x),
        MessageType::MonitoringInfo => &|x: u8| match_scale_monitoring_info(x),
        MessageType::Monitoring => &|x: u8| match_scale_monitoring(x),
        MessageType::Hr => &|x: u8| match_scale_hr(x),
        MessageType::StressLevel => &|x: u8| match_scale_stress_level(x),
        MessageType::MemoGlob => &|x: u8| match_scale_memo_glob(x),
        MessageType::AntChannelId => &|x: u8| match_scale_ant_channel_id(x),
        MessageType::AntRx => &|x: u8| match_scale_ant_rx(x),
        MessageType::AntTx => &|x: u8| match_scale_ant_tx(x),
        MessageType::ExdScreenConfiguration => &|x: u8| match_scale_exd_screen_configuration(x),
        MessageType::ExdDataFieldConfiguration => &|x: u8| match_scale_exd_data_field_configuration(x),
        MessageType::ExdDataConceptConfiguration => &|x: u8| match_scale_exd_data_concept_configuration(x),
        MessageType::FieldDescription => &|x: u8| match_scale_field_description(x),
        MessageType::DeveloperDataId => &|x: u8| match_scale_developer_data_id(x),
        MessageType::None => panic!("cannot call this with a None variant"),
        _ => &[]
    }
}
