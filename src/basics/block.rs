struct Block {
    w : u32,
    l : u32,
    h : u32,
}

impl Block {
    pub(crate) fn get_surface_area(&self) -> u32 {
          2* (self.w *  self.l+ self.w *  self.h+ self.l *  self.h)
    }
}

impl Block {
    pub(crate) fn get_volume(&self) -> u32 {
        self.w * self.h * self.l
    }
}

impl Block {
    pub(crate) fn get_height(&self) -> u32 {
        self.h
    }
}

impl Block {
    pub(crate) fn get_length(&self) -> u32 {
        self.l
    }
}

impl Block {
    pub(crate) fn get_width(&self) -> u32 {
        self.w
    }
}

impl Block {
    pub(crate) fn new(p0: &[u32; 3]) -> Self {
        Block   {w : p0[0] , l : p0[1] , h: p0[2]}
    }
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::Block;

    #[test]
    fn example_test() {
        let block = Block::new(&[2, 4, 6]);
        assert_eq!(block.get_width(), 2, "Incorrect width\nExpected 2 but got {}", block.get_width());
        assert_eq!(block.get_length(), 4, "Incorrect length\nExpected 4 but got {}", block.get_length());
        assert_eq!(block.get_height(), 6, "Incorrect height\nExpected 6 but got {}", block.get_height());
        assert_eq!(block.get_volume(), 48, "Incorrect volume\nExpected 48 but got {}", block.get_volume());
        assert_eq!(block.get_surface_area(), 88, "Incorrect surface area\nExpected 88 but got {}", block.get_surface_area());
    }
}
