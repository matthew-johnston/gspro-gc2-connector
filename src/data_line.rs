#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_line_from_line() {
        let line = "CT=1259299,SN=2638,HW=3,SW=4.0.0,ID=2,TM=1259299,SP=8.39,AZ=-6.08,EL=18.88,TS=800.00,SS=-125.00,BS=790.00,CY=0.95,TL=0.95,SM=0.00,HMT=0";
        let data_line = DataLine::from_line(line).unwrap();

        assert_eq!(data_line.ct, 1259299);
        assert_eq!(data_line.sn, 2638);
        assert_eq!(data_line.hw, 3);
        assert_eq!(data_line.sw, "4.0.0");
        assert_eq!(data_line.id, 2);
        assert_eq!(data_line.tm, 1259299);
        assert_eq!(data_line.sp, 8.39);
        assert_eq!(data_line.az, -6.08);
        assert_eq!(data_line.el, 18.88);
        assert_eq!(data_line.ts, 800.00);
        assert_eq!(data_line.ss, -125.00);
        assert_eq!(data_line.bs, 790.00);
        assert_eq!(data_line.cy, 0.95);
        assert_eq!(data_line.tl, 0.95);
        assert_eq!(data_line.sm, 0.00);
        assert_eq!(data_line.hmt, 0.00);
    }
}
