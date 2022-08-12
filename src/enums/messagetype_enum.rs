/// an enum of all defined messages in the Fit SDK
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MessageType {
    AccelerometerData,
    Activity,
    AntChannelId,
    AntRx,
    AntTx,
    AviationAttitude,
    BarometerData,
    BikeProfile,
    BloodPressure,
    CadenceZone,
    CameraEvent,
    Capabilities,
    ClimbPro,
    Connectivity,
    Course,
    CoursePoint,
    DeveloperDataId,
    DeviceAuxBatteryInfo,
    DeviceInfo,
    DeviceSettings,
    DiveAlarm,
    DiveGas,
    DiveSettings,
    DiveSummary,
    Event,
    ExdDataConceptConfiguration,
    ExdDataFieldConfiguration,
    ExdScreenConfiguration,
    ExerciseTitle,
    FieldCapabilities,
    FieldDescription,
    FileCapabilities,
    FileCreator,
    FileId,
    Goal,
    GpsMetadata,
    GyroscopeData,
    Hr,
    HrZone,
    HrmProfile,
    Hrv,
    Jump,
    Lap,
    Length,
    MagnetometerData,
    MemoGlob,
    MesgCapabilities,
    MetZone,
    Monitoring,
    MonitoringInfo,
    NmeaSentence,
    ObdiiData,
    OhrSettings,
    OneDSensorCalibration,
    PowerZone,
    Record,
    Schedule,
    SdmProfile,
    SegmentFile,
    SegmentId,
    SegmentLap,
    SegmentLeaderboardEntry,
    SegmentPoint,
    Session,
    Set,
    SlaveDevice,
    Software,
    SpeedZone,
    Sport,
    StressLevel,
    ThreeDSensorCalibration,
    TimestampCorrelation,
    Totals,
    TrainingFile,
    UserProfile,
    Video,
    VideoClip,
    VideoDescription,
    VideoFrame,
    VideoTitle,
    WatchfaceSettings,
    WeatherAlert,
    WeatherConditions,
    WeightScale,
    Workout,
    WorkoutSession,
    WorkoutStep,
    ZonesTarget,
    Pad,
    MfgRangeMax,
    MfgRangeMin,
    None,
}
