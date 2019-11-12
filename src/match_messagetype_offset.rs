use super::MessageType;
type MatchOffsetFn = dyn Fn(usize) -> Option<i16>;
fn match_offset_accelerometer_data(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_activity(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_ant_channel_id(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_ant_rx(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_ant_tx(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_aviation_attitude(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_barometer_data(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_bike_profile(k: usize) -> Option<i16> {
    match k {
        19 => Some(-110i16),
        _ => None,
    }
}
fn match_offset_blood_pressure(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_cadence_zone(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_camera_event(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_capabilities(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_connectivity(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_course(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_course_point(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_developer_data_id(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_device_info(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_device_settings(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_dive_alarm(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_dive_gas(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_dive_settings(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_dive_summary(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_event(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_exd_data_concept_configuration(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_exd_data_field_configuration(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_exd_screen_configuration(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_exercise_title(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_field_capabilities(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_field_description(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_file_capabilities(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_file_creator(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_file_id(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_goal(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_gps_metadata(k: usize) -> Option<i16> {
    match k {
        3 => Some(500i16),
        _ => None,
    }
}
fn match_offset_gyroscope_data(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_hr(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_hr_zone(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_hrm_profile(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_hrv(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_jump(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_lap(k: usize) -> Option<i16> {
    match k {
        42 => Some(500i16),
        43 => Some(500i16),
        62 => Some(500i16),
        112 => Some(500i16),
        113 => Some(500i16),
        114 => Some(500i16),
        _ => None,
    }
}
fn match_offset_length(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_magnetometer_data(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_memo_glob(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_mesg_capabilities(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_met_zone(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_monitoring(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_monitoring_info(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_nmea_sentence(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_obdii_data(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_ohr_settings(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_one_d_sensor_calibration(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_power_zone(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_record(k: usize) -> Option<i16> {
    match k {
        2 => Some(500i16),
        78 => Some(500i16),
        _ => None,
    }
}
fn match_offset_schedule(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_sdm_profile(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_segment_file(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_segment_id(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_segment_lap(k: usize) -> Option<i16> {
    match k {
        34 => Some(500i16),
        35 => Some(500i16),
        54 => Some(500i16),
        _ => None,
    }
}
fn match_offset_segment_leaderboard_entry(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_segment_point(k: usize) -> Option<i16> {
    match k {
        4 => Some(500i16),
        _ => None,
    }
}
fn match_offset_session(k: usize) -> Option<i16> {
    match k {
        49 => Some(500i16),
        50 => Some(500i16),
        71 => Some(500i16),
        126 => Some(500i16),
        127 => Some(500i16),
        128 => Some(500i16),
        _ => None,
    }
}
fn match_offset_set(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_slave_device(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_software(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_speed_zone(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_sport(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_stress_level(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_three_d_sensor_calibration(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_timestamp_correlation(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_totals(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_training_file(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_user_profile(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_video(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_video_clip(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_video_description(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_video_frame(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_video_title(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_watchface_settings(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_weather_alert(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_weather_conditions(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_weight_scale(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_workout(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_workout_session(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_workout_step(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_zones_target(k: usize) -> Option<i16> {
    match k {
        _ => None,
    }
}
fn match_offset_none(_: usize) -> Option<i16> {
    return None;
}
pub fn match_message_offset(m: MessageType) -> &'static MatchOffsetFn {
    match m {
        MessageType::FileId => &|x: usize| match_offset_file_id(x),
        MessageType::FileCreator => &|x: usize| match_offset_file_creator(x),
        MessageType::TimestampCorrelation => &|x: usize| match_offset_timestamp_correlation(x),
        MessageType::Software => &|x: usize| match_offset_software(x),
        MessageType::SlaveDevice => &|x: usize| match_offset_slave_device(x),
        MessageType::Capabilities => &|x: usize| match_offset_capabilities(x),
        MessageType::FileCapabilities => &|x: usize| match_offset_file_capabilities(x),
        MessageType::MesgCapabilities => &|x: usize| match_offset_mesg_capabilities(x),
        MessageType::FieldCapabilities => &|x: usize| match_offset_field_capabilities(x),
        MessageType::DeviceSettings => &|x: usize| match_offset_device_settings(x),
        MessageType::UserProfile => &|x: usize| match_offset_user_profile(x),
        MessageType::HrmProfile => &|x: usize| match_offset_hrm_profile(x),
        MessageType::SdmProfile => &|x: usize| match_offset_sdm_profile(x),
        MessageType::BikeProfile => &|x: usize| match_offset_bike_profile(x),
        MessageType::Connectivity => &|x: usize| match_offset_connectivity(x),
        MessageType::WatchfaceSettings => &|x: usize| match_offset_watchface_settings(x),
        MessageType::OhrSettings => &|x: usize| match_offset_ohr_settings(x),
        MessageType::ZonesTarget => &|x: usize| match_offset_zones_target(x),
        MessageType::Sport => &|x: usize| match_offset_sport(x),
        MessageType::HrZone => &|x: usize| match_offset_hr_zone(x),
        MessageType::SpeedZone => &|x: usize| match_offset_speed_zone(x),
        MessageType::CadenceZone => &|x: usize| match_offset_cadence_zone(x),
        MessageType::PowerZone => &|x: usize| match_offset_power_zone(x),
        MessageType::MetZone => &|x: usize| match_offset_met_zone(x),
        MessageType::DiveSettings => &|x: usize| match_offset_dive_settings(x),
        MessageType::DiveAlarm => &|x: usize| match_offset_dive_alarm(x),
        MessageType::DiveGas => &|x: usize| match_offset_dive_gas(x),
        MessageType::Goal => &|x: usize| match_offset_goal(x),
        MessageType::Activity => &|x: usize| match_offset_activity(x),
        MessageType::Session => &|x: usize| match_offset_session(x),
        MessageType::Lap => &|x: usize| match_offset_lap(x),
        MessageType::Length => &|x: usize| match_offset_length(x),
        MessageType::Record => &|x: usize| match_offset_record(x),
        MessageType::Event => &|x: usize| match_offset_event(x),
        MessageType::DeviceInfo => &|x: usize| match_offset_device_info(x),
        MessageType::TrainingFile => &|x: usize| match_offset_training_file(x),
        MessageType::Hrv => &|x: usize| match_offset_hrv(x),
        MessageType::WeatherConditions => &|x: usize| match_offset_weather_conditions(x),
        MessageType::WeatherAlert => &|x: usize| match_offset_weather_alert(x),
        MessageType::GpsMetadata => &|x: usize| match_offset_gps_metadata(x),
        MessageType::CameraEvent => &|x: usize| match_offset_camera_event(x),
        MessageType::GyroscopeData => &|x: usize| match_offset_gyroscope_data(x),
        MessageType::AccelerometerData => &|x: usize| match_offset_accelerometer_data(x),
        MessageType::MagnetometerData => &|x: usize| match_offset_magnetometer_data(x),
        MessageType::BarometerData => &|x: usize| match_offset_barometer_data(x),
        MessageType::ThreeDSensorCalibration => &|x: usize| match_offset_three_d_sensor_calibration(x),
        MessageType::OneDSensorCalibration => &|x: usize| match_offset_one_d_sensor_calibration(x),
        MessageType::VideoFrame => &|x: usize| match_offset_video_frame(x),
        MessageType::ObdiiData => &|x: usize| match_offset_obdii_data(x),
        MessageType::NmeaSentence => &|x: usize| match_offset_nmea_sentence(x),
        MessageType::AviationAttitude => &|x: usize| match_offset_aviation_attitude(x),
        MessageType::Video => &|x: usize| match_offset_video(x),
        MessageType::VideoTitle => &|x: usize| match_offset_video_title(x),
        MessageType::VideoDescription => &|x: usize| match_offset_video_description(x),
        MessageType::VideoClip => &|x: usize| match_offset_video_clip(x),
        MessageType::Set => &|x: usize| match_offset_set(x),
        MessageType::Jump => &|x: usize| match_offset_jump(x),
        MessageType::Course => &|x: usize| match_offset_course(x),
        MessageType::CoursePoint => &|x: usize| match_offset_course_point(x),
        MessageType::SegmentId => &|x: usize| match_offset_segment_id(x),
        MessageType::SegmentLeaderboardEntry => &|x: usize| match_offset_segment_leaderboard_entry(x),
        MessageType::SegmentPoint => &|x: usize| match_offset_segment_point(x),
        MessageType::SegmentLap => &|x: usize| match_offset_segment_lap(x),
        MessageType::SegmentFile => &|x: usize| match_offset_segment_file(x),
        MessageType::Workout => &|x: usize| match_offset_workout(x),
        MessageType::WorkoutSession => &|x: usize| match_offset_workout_session(x),
        MessageType::WorkoutStep => &|x: usize| match_offset_workout_step(x),
        MessageType::ExerciseTitle => &|x: usize| match_offset_exercise_title(x),
        MessageType::Schedule => &|x: usize| match_offset_schedule(x),
        MessageType::Totals => &|x: usize| match_offset_totals(x),
        MessageType::WeightScale => &|x: usize| match_offset_weight_scale(x),
        MessageType::BloodPressure => &|x: usize| match_offset_blood_pressure(x),
        MessageType::MonitoringInfo => &|x: usize| match_offset_monitoring_info(x),
        MessageType::Monitoring => &|x: usize| match_offset_monitoring(x),
        MessageType::Hr => &|x: usize| match_offset_hr(x),
        MessageType::StressLevel => &|x: usize| match_offset_stress_level(x),
        MessageType::MemoGlob => &|x: usize| match_offset_memo_glob(x),
        MessageType::AntChannelId => &|x: usize| match_offset_ant_channel_id(x),
        MessageType::AntRx => &|x: usize| match_offset_ant_rx(x),
        MessageType::AntTx => &|x: usize| match_offset_ant_tx(x),
        MessageType::ExdScreenConfiguration => &|x: usize| match_offset_exd_screen_configuration(x),
        MessageType::ExdDataFieldConfiguration => &|x: usize| match_offset_exd_data_field_configuration(x),
        MessageType::ExdDataConceptConfiguration => &|x: usize| match_offset_exd_data_concept_configuration(x),
        MessageType::FieldDescription => &|x: usize| match_offset_field_description(x),
        MessageType::DeveloperDataId => &|x: usize| match_offset_developer_data_id(x),
        MessageType::DiveSummary => &|x: usize| match_offset_dive_summary(x),
        _ => &|x: usize| match_offset_none(x)
    }
}
