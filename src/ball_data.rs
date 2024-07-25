use serde::{Deserialize, Serialize};

use crate::data_line::DataLine;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct BallData {
    DeviceID: String, // Hard coded to Foresight GC2
    Units: String,    // Hard coded to Yards
    ShotNumber: u32,
    APIversion: String, // Hard coded to 1
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
    ContainsBallData: bool, // Hard coded to true
    ContainsClubData: bool, // Hard coded to false
    LaunchMonitorIsReady: Option<bool>,
    LaunchMonitorBallDetected: Option<bool>,
    IsHeartBeat: Option<bool>,
}

impl ClubDetails {
    pub fn new() -> ClubDetails {
        ClubDetails {
            Speed: 0.0,
            AngleOfAttack: 0.0,
            FaceToTarget: 0.0,
            Lie: 0.0,
            Loft: 0.0,
            Path: 0.0,
            SpeedAtImpact: 0.0,
            VerticalFaceImpact: 0.0,
            HorizontalFaceImpact: 0.0,
            ClosureRate: 0.0,
        }
    }
}

impl ShotDataOptions {
    pub fn new() -> ShotDataOptions {
        ShotDataOptions {
            ContainsBallData: true,
            ContainsClubData: false,
            LaunchMonitorIsReady: None,
            LaunchMonitorBallDetected: None,
            IsHeartBeat: None,
        }
    }
}

// CT=1259299,SN=2638,HW=3,SW=4.0.0,ID=2,TM=1259299,SP=8.39,AZ=-6.08,EL=18.88,TS=800.00,SS=-125.00,BS=790.00,CY=0.95,TL=0.95,SM=0.00,HMT=0

impl From<DataLine> for BallData {
    fn from(data_line: DataLine) -> BallData {
        BallData {
            DeviceID: format!("Foresight GC2 ({})", data_line.sn),
            Units: "Yards".to_owned(),
            ShotNumber: data_line.id,
            APIversion: "1".to_owned(),
            BallData: BallDetails {
                Speed: data_line.sp,
                SpinAxis: data_line.AZ,
                TotalSpin: data_line.ts,
                BackSpin: Some(data_line.bs),
                SideSpin: Some(data_line.ss),
                HLA: data_line.az,
                VLA: data_line.el,
                CarryDistance: Some(data_line.cy),
            },
            ClubData: ClubDetails::new(),
            ShotDataOptions: ShotDataOptions::new(),
        }
    }
}
