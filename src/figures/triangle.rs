use raytracing::{Intersectable, Ray2, Hit, Point32};

pub struct Triangle {
    vertex_1: Point32,
    vertex_2: Point32,
    vertex_3: Point32,
}

impl Intersectable for Triangle {
    
    //Creates an equilateral Triangle of side sqrt(2)
    fn new() -> Triangle {
        Triangle{
            vertex_1: Point32(1.0,0.0,0.0),
            vertex_2: Point32(0.0,1.0,0.0),
            vertex_3: Point32(0.0,0.0,1.0),
        }
    }

    //Creates a Triangle given its three vertices.
    fn new(vertex1: Point32, vertex2: Point32, vertex3: Point32) -> Triangle {
        Triangle {
            vertex_1: vertex1,
            vertex_2: vertex2,
            vertex_3: vertex3,
        }
    }
    
    //Computes the intersection of this figure with a given Ray, if any.
    fn intersect(&self, ray: Ray2) -> Option<Hit> {
        //TODO
        Option::None
    }
}