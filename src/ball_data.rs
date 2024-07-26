use serde::{Deserialize, Serialize};

use crate::data_line::DataLine;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct BallData {
    DeviceID: String, // Hard coded to Foresight GC2
    Units: String,    // Hard coded to Yards
    pub ShotNumber: u32,
    APIversion: String, // Hard coded to 1
    BallData: BallDetails,
    ClubData: ClubDetails,
    ShotDataOptions: ShotDataOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct BallDetails {
    Speed: f32,
    SpinAxis: f32,
    TotalSpin: f32,
    BackSpin: Option<f32>,
    SideSpin: Option<f32>,
    HLA: f32,
    VLA: f32,
    CarryDistance: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct ClubDetails {
    Speed: f32,
    AngleOfAttack: f32,
    FaceToTarget: f32,
    Lie: f32,
    Loft: f32,
    Path: f32,
    SpeedAtImpact: f32,
    VerticalFaceImpact: f32,
    HorizontalFaceImpact: f32,
    ClosureRate: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct ShotDataOptions {
    ContainsBallData: bool, // Hard coded to true
    ContainsClubData: bool, // Hard coded to false
    LaunchMonitorIsReady: bool,
    LaunchMonitorBallDetected: bool,
    IsHeartBeat: bool,
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
            LaunchMonitorIsReady: true,
            LaunchMonitorBallDetected: true,
            IsHeartBeat: false,
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
                SpinAxis: calculate_spin_axis(data_line.ss, data_line.bs),
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

fn calculate_spin_axis(side_spin: f32, back_spin: f32) -> f32 {
    if back_spin == 0.0 {
        // Handle the case where back_spin is 0 to avoid division by zero.
        // Returning 90.0 or -90.0 degrees based on the side_spin sign,
        // indicating a purely horizontal spin axis.
        return if side_spin >= 0.0 { 90.0 } else { -90.0 };
    }

    // Calculate the angle in radians
    let angle_radians = (side_spin / back_spin).atan();

    // Convert the angle to degrees
    let angle_degrees = angle_radians.to_degrees();

    // Return the calculated angle
    angle_degrees
}
