
pub struct MarginOffset {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    #[allow(dead_code)]
    private: (),
}

impl From<f32> for MarginOffset {
    fn from(margin: f32) -> Self {
        MarginOffset {
            left: margin,
            right: margin,
            bottom: margin,
            top: margin,
            private: (),
        }
    }
}

impl From<(f32, f32)> for MarginOffset {
    fn from(hv: (f32, f32)) -> Self {
        let (h, v) = hv;
        MarginOffset {
            left: h,
            right: h,
            bottom: v,
            top: v,
            private: (),
        }
    }
}

impl From<(f32, f32, f32, f32)> for MarginOffset {
    //noinspection SpellCheckingInspection
    fn from(trbl: (f32, f32, f32, f32)) -> Self {
        let (top, right, bottom, left) = trbl;
        MarginOffset {
            left,
            right,
            bottom,
            top,
            private: (),
        }
    }
}