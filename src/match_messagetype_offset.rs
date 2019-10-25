type MatchOffsetFn = dyn Fn(u8) -> Option<i16>;
fn match_offset_file_id(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_file_creator(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_timestamp_correlation(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_software(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_slave_device(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_capabilities(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_file_capabilities(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_mesg_capabilities(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_field_capabilities(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_device_settings(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_user_profile(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_hrm_profile(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_sdm_profile(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_bike_profile(k: u8) -> Option<i16> {
    19 => Some(-110i16),
    _ => None,
}
fn match_offset_connectivity(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_watchface_settings(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_ohr_settings(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_zones_target(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_sport(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_hr_zone(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_speed_zone(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_cadence_zone(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_power_zone(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_met_zone(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_dive_settings(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_dive_alarm(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_dive_gas(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_goal(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_activity(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_session(k: u8) -> Option<i16> {
    49 => Some(500i16),
    50 => Some(500i16),
    71 => Some(500i16),
    126 => Some(500i16),
    127 => Some(500i16),
    128 => Some(500i16),
    _ => None,
}
fn match_offset_lap(k: u8) -> Option<i16> {
    42 => Some(500i16),
    43 => Some(500i16),
    62 => Some(500i16),
    112 => Some(500i16),
    113 => Some(500i16),
    114 => Some(500i16),
    _ => None,
}
fn match_offset_length(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_record(k: u8) -> Option<i16> {
    2 => Some(500i16),
    78 => Some(500i16),
    _ => None,
}
fn match_offset_event(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_device_info(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_training_file(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_hrv(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_weather_conditions(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_weather_alert(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_gps_metadata(k: u8) -> Option<i16> {
    3 => Some(500i16),
    _ => None,
}
fn match_offset_camera_event(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_gyroscope_data(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_accelerometer_data(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_magnetometer_data(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_barometer_data(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_three_d_sensor_calibration(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_one_d_sensor_calibration(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_video_frame(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_obdii_data(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_nmea_sentence(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_aviation_attitude(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_video(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_video_title(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_video_description(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_video_clip(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_set(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_jump(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_course(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_course_point(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_segment_id(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_segment_leaderboard_entry(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_segment_point(k: u8) -> Option<i16> {
    4 => Some(500i16),
    _ => None,
}
fn match_offset_segment_lap(k: u8) -> Option<i16> {
    34 => Some(500i16),
    35 => Some(500i16),
    54 => Some(500i16),
    _ => None,
}
fn match_offset_segment_file(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_workout(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_workout_session(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_workout_step(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_exercise_title(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_schedule(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_totals(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_weight_scale(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_blood_pressure(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_monitoring_info(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_monitoring(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_hr(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_stress_level(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_memo_glob(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_ant_channel_id(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_ant_rx(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_ant_tx(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_exd_screen_configuration(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_exd_data_field_configuration(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_exd_data_concept_configuration(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_field_description(k: u8) -> Option<i16> {
    _ => None,
}
fn match_offset_developer_data_id(k: u8) -> Option<i16> {
    _ => None,
}
pub fn match_message_offset(m: MessageType) -> &'static MatchOffsetFn {
    match m {
        MessageType::FileId => &|x: u8| match_offset_file_id(x),
        MessageType::FileCreator => &|x: u8| match_offset_file_creator(x),
        MessageType::TimestampCorrelation => &|x: u8| match_offset_timestamp_correlation(x),
        MessageType::Software => &|x: u8| match_offset_software(x),
        MessageType::SlaveDevice => &|x: u8| match_offset_slave_device(x),
        MessageType::Capabilities => &|x: u8| match_offset_capabilities(x),
        MessageType::FileCapabilities => &|x: u8| match_offset_file_capabilities(x),
        MessageType::MesgCapabilities => &|x: u8| match_offset_mesg_capabilities(x),
        MessageType::FieldCapabilities => &|x: u8| match_offset_field_capabilities(x),
        MessageType::DeviceSettings => &|x: u8| match_offset_device_settings(x),
        MessageType::UserProfile => &|x: u8| match_offset_user_profile(x),
        MessageType::HrmProfile => &|x: u8| match_offset_hrm_profile(x),
        MessageType::SdmProfile => &|x: u8| match_offset_sdm_profile(x),
        MessageType::BikeProfile => &|x: u8| match_offset_bike_profile(x),
        MessageType::Connectivity => &|x: u8| match_offset_connectivity(x),
        MessageType::WatchfaceSettings => &|x: u8| match_offset_watchface_settings(x),
        MessageType::OhrSettings => &|x: u8| match_offset_ohr_settings(x),
        MessageType::ZonesTarget => &|x: u8| match_offset_zones_target(x),
        MessageType::Sport => &|x: u8| match_offset_sport(x),
        MessageType::HrZone => &|x: u8| match_offset_hr_zone(x),
        MessageType::SpeedZone => &|x: u8| match_offset_speed_zone(x),
        MessageType::CadenceZone => &|x: u8| match_offset_cadence_zone(x),
        MessageType::PowerZone => &|x: u8| match_offset_power_zone(x),
        MessageType::MetZone => &|x: u8| match_offset_met_zone(x),
        MessageType::DiveSettings => &|x: u8| match_offset_dive_settings(x),
        MessageType::DiveAlarm => &|x: u8| match_offset_dive_alarm(x),
        MessageType::DiveGas => &|x: u8| match_offset_dive_gas(x),
        MessageType::Goal => &|x: u8| match_offset_goal(x),
        MessageType::Activity => &|x: u8| match_offset_activity(x),
        MessageType::Session => &|x: u8| match_offset_session(x),
        MessageType::Lap => &|x: u8| match_offset_lap(x),
        MessageType::Length => &|x: u8| match_offset_length(x),
        MessageType::Record => &|x: u8| match_offset_record(x),
        MessageType::Event => &|x: u8| match_offset_event(x),
        MessageType::DeviceInfo => &|x: u8| match_offset_device_info(x),
        MessageType::TrainingFile => &|x: u8| match_offset_training_file(x),
        MessageType::Hrv => &|x: u8| match_offset_hrv(x),
        MessageType::WeatherConditions => &|x: u8| match_offset_weather_conditions(x),
        MessageType::WeatherAlert => &|x: u8| match_offset_weather_alert(x),
        MessageType::GpsMetadata => &|x: u8| match_offset_gps_metadata(x),
        MessageType::CameraEvent => &|x: u8| match_offset_camera_event(x),
        MessageType::GyroscopeData => &|x: u8| match_offset_gyroscope_data(x),
        MessageType::AccelerometerData => &|x: u8| match_offset_accelerometer_data(x),
        MessageType::MagnetometerData => &|x: u8| match_offset_magnetometer_data(x),
        MessageType::BarometerData => &|x: u8| match_offset_barometer_data(x),
        MessageType::ThreeDSensorCalibration => &|x: u8| match_offset_three_d_sensor_calibration(x),
        MessageType::OneDSensorCalibration => &|x: u8| match_offset_one_d_sensor_calibration(x),
        MessageType::VideoFrame => &|x: u8| match_offset_video_frame(x),
        MessageType::ObdiiData => &|x: u8| match_offset_obdii_data(x),
        MessageType::NmeaSentence => &|x: u8| match_offset_nmea_sentence(x),
        MessageType::AviationAttitude => &|x: u8| match_offset_aviation_attitude(x),
        MessageType::Video => &|x: u8| match_offset_video(x),
        MessageType::VideoTitle => &|x: u8| match_offset_video_title(x),
        MessageType::VideoDescription => &|x: u8| match_offset_video_description(x),
        MessageType::VideoClip => &|x: u8| match_offset_video_clip(x),
        MessageType::Set => &|x: u8| match_offset_set(x),
        MessageType::Jump => &|x: u8| match_offset_jump(x),
        MessageType::Course => &|x: u8| match_offset_course(x),
        MessageType::CoursePoint => &|x: u8| match_offset_course_point(x),
        MessageType::SegmentId => &|x: u8| match_offset_segment_id(x),
        MessageType::SegmentLeaderboardEntry => &|x: u8| match_offset_segment_leaderboard_entry(x),
        MessageType::SegmentPoint => &|x: u8| match_offset_segment_point(x),
        MessageType::SegmentLap => &|x: u8| match_offset_segment_lap(x),
        MessageType::SegmentFile => &|x: u8| match_offset_segment_file(x),
        MessageType::Workout => &|x: u8| match_offset_workout(x),
        MessageType::WorkoutSession => &|x: u8| match_offset_workout_session(x),
        MessageType::WorkoutStep => &|x: u8| match_offset_workout_step(x),
        MessageType::ExerciseTitle => &|x: u8| match_offset_exercise_title(x),
        MessageType::Schedule => &|x: u8| match_offset_schedule(x),
        MessageType::Totals => &|x: u8| match_offset_totals(x),
        MessageType::WeightScale => &|x: u8| match_offset_weight_scale(x),
        MessageType::BloodPressure => &|x: u8| match_offset_blood_pressure(x),
        MessageType::MonitoringInfo => &|x: u8| match_offset_monitoring_info(x),
        MessageType::Monitoring => &|x: u8| match_offset_monitoring(x),
        MessageType::Hr => &|x: u8| match_offset_hr(x),
        MessageType::StressLevel => &|x: u8| match_offset_stress_level(x),
        MessageType::MemoGlob => &|x: u8| match_offset_memo_glob(x),
        MessageType::AntChannelId => &|x: u8| match_offset_ant_channel_id(x),
        MessageType::AntRx => &|x: u8| match_offset_ant_rx(x),
        MessageType::AntTx => &|x: u8| match_offset_ant_tx(x),
        MessageType::ExdScreenConfiguration => &|x: u8| match_offset_exd_screen_configuration(x),
        MessageType::ExdDataFieldConfiguration => &|x: u8| match_offset_exd_data_field_configuration(x),
        MessageType::ExdDataConceptConfiguration => &|x: u8| match_offset_exd_data_concept_configuration(x),
        MessageType::FieldDescription => &|x: u8| match_offset_field_description(x),
        MessageType::DeveloperDataId => &|x: u8| match_offset_developer_data_id(x),
        MessageType::None => panic!("cannot call this with a None variant"),
        _ => &[]
    }
}
