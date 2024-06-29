

 impl UnityVector3{

    pub fn new(x:f32,y:f32,z:f32)->UnityVector3{ UnityVector3{x,y,z}}
    pub fn new_zero()->UnityVector3{ UnityVector3{x:0.0,y:0.0,z:0.0}}
    
    pub fn set(&mut self,x:f32,y:f32,z:f32){
        self.x=x;
        self.y=y;
        self.z=z;
    }
    pub fn set_zero(&mut self){
        self.x=0.0;
        self.y=0.0;
        self.z=0.0;
    }
    pub fn add(&mut self,other:UnityVector3){
        self.x+=other.x;
        self.y+=other.y;
        self.z+=other.z;
    }
    pub fn sub(&mut self,other:UnityVector3){
        self.x-=other.x;
        self.y-=other.y;
        self.z-=other.z;
    }
    pub  fn mul(&mut self,other:UnityVector3){
        self.x*=other.x;
        self.y*=other.y;
        self.z*=other.z;
    }
    pub  fn div(&mut self,other:UnityVector3){
        self.x/=other.x;
        self.y/=other.y;
        self.z/=other.z;
    }
    
}



pub struct UnityVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub struct  UnityQuaternion {
   pub x: f32,
   pub y: f32,
   pub z: f32,
   pub w: f32,
}