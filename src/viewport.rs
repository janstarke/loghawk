use getset::Getters;
use ratatui::layout::Rect;


#[derive(Clone, Getters)]
#[getset(get="pub")]
pub struct ViewPort {
    hoffset: usize,
    voffset: usize,
    hsize: usize,
    vsize: usize,
}

impl ViewPort {
    pub fn new(hoffset: usize, voffset: usize, hsize: usize, vsize: usize) -> Self {
        Self {
            hoffset,
            voffset,
            hsize,
            vsize,
        }
    }

    pub fn from_rect(hoffset: usize, voffset: usize, area: &Rect) -> Self {
        Self {
            hoffset,
            voffset,
            hsize: area.width.into(),
            vsize: area.height.into(),
        }
    }
    
    pub fn hbegin(&self) -> usize {
        *self.hoffset()
    }

    pub fn hend(&self) -> usize {
        self.hoffset() + self.hsize()
    }

    pub fn vbegin(&self) -> usize {
        *self.voffset()
    }

    pub fn vend(&self) -> usize {
        self.voffset() + self.vsize()
    }
}
