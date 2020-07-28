use super::{Intersectable, HitRecord, Ray2};

pub struct IntersectableList<'a> { 
    pub objects: Vec<Box<dyn Intersectable +'a>>,
}

impl Intersectable for IntersectableList<'_> {
    fn intersect(&self, ray: &Ray2, t_min: f32, t_max:f32) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        //let mut hit_anything = false;
        let mut closest = t_max;
        for o in self.objects.iter() {
            let opt_hitrec = o.intersect(ray,t_min,closest);
            match opt_hitrec {
                Some(hitrec) => {
                    closest = hitrec.t;
                    result = Some(hitrec);
                },
                None => {}
            }
        }
        result
    }
}

impl<'a> IntersectableList<'a> {
    pub fn add(&mut self, obj: Box<dyn Intersectable + 'a>) {
        self.objects.push(obj);
    }
    
}