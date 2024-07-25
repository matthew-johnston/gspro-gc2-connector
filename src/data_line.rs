pub struct DataLine {
    pub ct: u32,
    pub sn: u32,
    pub hw: u8,
    pub sw: String,
    pub id: u32,
    pub tm: u32,
    pub sp: f32,
    pub az: f32,
    pub el: f32,
    pub ts: f32,
    pub ss: f32,
    pub bs: f32,
    pub cy: f32,
    pub tl: f32,
    pub sm: f32,
    pub hmt: f32,
}

impl DataLine {
    pub fn from_line(line: &str) -> Option<Self> {
        let mut data = DataLine {
            ct: 0,
            sn: 0,
            hw: 0,
            sw: String::new(),
            id: 0,
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
            hmt: 0.0,
        };

        for part in line.split(',') {
            let mut kv = part.split('=');
            match kv.next() {
                Some("CT") => data.ct = kv.next()?.parse().ok()?,
                Some("SN") => data.sn = kv.next()?.parse().ok()?,
                Some("HW") => data.hw = kv.next()?.parse().ok()?,
                Some("SW") => data.sw = kv.next()?.to_string(),
                Some("ID") => data.id = kv.next()?.parse().ok()?,
                Some("TM") => data.tm = kv.next()?.parse().ok()?,
                Some("SP") => data.sp = kv.next()?.parse().ok()?,
                Some("AZ") => data.az = kv.next()?.parse().ok()?,
                Some("EL") => data.el = kv.next()?.parse().ok()?,
                Some("TS") => data.ts = kv.next()?.parse().ok()?,
                Some("SS") => data.ss = kv.next()?.parse().ok()?,
                Some("BS") => data.bs = kv.next()?.parse().ok()?,
                Some("CY") => data.cy = kv.next()?.parse().ok()?,
                Some("TL") => data.tl = kv.next()?.parse().ok()?,
                Some("SM") => data.sm = kv.next()?.parse().ok()?,
                Some("HMT") => data.hmt = kv.next()?.parse().ok()?,
                _ => {}
            }
        }

        Some(data)
    }
}
