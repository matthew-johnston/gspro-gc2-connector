#[derive(Debug)]
pub struct BallEvent {
    pub tm: u32,  // Another timestamp or count, using u32
    pub sp: f32,  // Speed, using f32 for floating-point numbers
    pub az: f32,  // Azimuth, using f32
    pub el: f32,  // Elevation, using f32
    pub ts: f32,  // Another speed or timestamp, using f32
    pub ss: f32,  // Another speed or score, using f32
    pub bs: f32,  // Another score or size, using f32
    pub cy: f32,  // Cycle or count, using f32
    pub tl: f32,  // Total length or time, using f32
    pub sm: f32,  // Sum or measurement, using f32
    pub hmt: u32, // Another count or measurement, using u32
}

impl BallEvent {
    pub fn from_data_line(data_line: &str) -> Option<BallEvent> {
        let mut ball_event = BallEvent {
            tm: 0,
            sp: 0.0,
            az: 0.0,
            el: 0.0,
            ts: 0.0,
            ss: 0.0,
            bs: 0.0,
            cy: 0.0,
            tl: 0.0,
            sm: 0.0,
            hmt: 0,
        };

        // Initialize flags for each key
        let mut tm_set = false;
        let mut sp_set = false;
        let mut az_set = false;
        let mut el_set = false;
        let mut ts_set = false;
        let mut ss_set = false;
        let mut bs_set = false;
        let mut cy_set = false;
        let mut tl_set = false;
        let mut sm_set = false;
        let mut hmt_set = false;

        for pair in data_line.split(',') {
            let mut parts = pair.split('=');
            match parts.next() {
                Some("TM") => {
                    ball_event.tm = parts.next()?.parse().ok()?;
                    tm_set = true;
                }
                Some("SP") => {
                    ball_event.sp = parts.next()?.parse().ok()?;
                    sp_set = true;
                }
                Some("AZ") => {
                    ball_event.az = parts.next()?.parse().ok()?;
                    az_set = true;
                }
                Some("EL") => {
                    ball_event.el = parts.next()?.parse().ok()?;
                    el_set = true;
                }
                Some("TS") => {
                    ball_event.ts = parts.next()?.parse().ok()?;
                    ts_set = true;
                }
                Some("SS") => {
                    ball_event.ss = parts.next()?.parse().ok()?;
                    ss_set = true;
                }
                Some("BS") => {
                    ball_event.bs = parts.next()?.parse().ok()?;
                    bs_set = true;
                }
                Some("CY") => {
                    ball_event.cy = parts.next()?.parse().ok()?;
                    cy_set = true;
                }
                Some("TL") => {
                    ball_event.tl = parts.next()?.parse().ok()?;
                    tl_set = true;
                }
                Some("SM") => {
                    ball_event.sm = parts.next()?.parse().ok()?;
                    sm_set = true;
                }
                Some("HMT") => {
                    ball_event.hmt = parts.next()?.parse().ok()?;
                    hmt_set = true;
                }
                _ => {}
            }
        }

        print!("{:?}", ball_event);

        // Check if all keys have been set
        if tm_set
            && sp_set
            && az_set
            && el_set
            && ts_set
            && ss_set
            && bs_set
            && cy_set
            && tl_set
            && sm_set
            && hmt_set
        {
            Some(ball_event)
        } else {
            None
        }
    }
}
