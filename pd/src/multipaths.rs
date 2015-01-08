use glob::Paths;

pub struct MultiPaths {
    paths: Vec<Paths>,
    idx: uint,
}

impl MultiPaths {
    pub fn from(paths: Vec<Paths>) -> MultiPaths {
        MultiPaths {
            paths: paths,
            idx: 0
        }
    }
}

impl Iterator for MultiPaths {
    type Item = Path;

    fn next(&mut self) -> Option<Path> {
        loop {
            if let Some(p) = self.paths.get_mut(self.idx) {
                match p.next() {
                    Some(x) => return Some(x),
                    None => { self.idx += 1; continue; }
                }
            }
            
            return None;
        }
    }
}
