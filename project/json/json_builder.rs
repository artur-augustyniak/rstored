use json::json_chunk::JsonChunk;

pub struct JsonBuilder<T: JsonChunk> {
    root_elem: T,
}

impl<T: JsonChunk> JsonBuilder<T> {
    pub fn new(root: T) -> JsonBuilder<T> {
        JsonBuilder { root_elem: root }
    }

    pub fn print(&self) -> () {
        println!("{}", self.root_elem);
    }
}


    //    fn push (&mut self, coordinate: f64) -> &mut CircleBuilder {
    //        self.x = coordinate;
    //        self
    //    }
    //
    //    fn finalize(&self) -> Circle {
    //        Circle { x: self.x, y: self.y, radius: self.radius }
    //    }
