use crate::raytracing::{Intersectable, Ray2, Point32, HitRecord};

pub struct Triangle {
    vertex_1: Point32,
    vertex_2: Point32,
    vertex_3: Point32,
}

impl Triangle {
    //Creates a Triangle given its three vertices.
    pub fn new(vertex1: Point32, vertex2: Point32, vertex3: Point32) -> Triangle {
        Triangle {
            vertex_1: vertex1,
            vertex_2: vertex2,
            vertex_3: vertex3,
        }
    }
    
    //Creates an equilateral Triangle of side sqrt(2)
    pub fn new_default() -> Triangle {
        Triangle::new(
            Point32::new(1.0,0.0,0.0), 
            Point32::new(0.0,1.0,0.0), 
            Point32::new(0.0,0.0,1.0)
        )
    }
}

impl Intersectable for Triangle {

    //Computes the intersection of this figure with a given Ray, if any.
    fn intersect(&self, ray: &Ray2, t_min: f32, t_max:f32) -> Option<HitRecord> {
        //TODO
        Option::None
    }
}