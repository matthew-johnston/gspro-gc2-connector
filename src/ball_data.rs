use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct BallData {
    DeviceID: String,
    Units: String,
    ShotNumber: u32,
    APIversion: String,
    BallData: BallDetails,
    ClubData: ClubDetails,
    ShotDataOptions: ShotDataOptions,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct BallDetails {
    Speed: f64,
    SpinAxis: f64,
    TotalSpin: f64,
    BackSpin: Option<f64>,
    SideSpin: Option<f64>,
    HLA: f64,
    VLA: f64,
    CarryDistance: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ClubDetails {
    Speed: f64,
    AngleOfAttack: f64,
    FaceToTarget: f64,
    Lie: f64,
    Loft: f64,
    Path: f64,
    SpeedAtImpact: f64,
    VerticalFaceImpact: f64,
    HorizontalFaceImpact: f64,
    ClosureRate: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ShotDataOptions {
    ContainsBallData: bool,
    ContainsClubData: bool,
    LaunchMonitorIsReady: Option<bool>,
    LaunchMonitorBallDetected: Option<bool>,
    IsHeartBeat: Option<bool>,
}
