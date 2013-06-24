/*
* Rust-SFML - Copyright (c) Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
* 
* 3. This notice may not be removed or altered from any source distribution.
*/

/*!
* Utility Class providing 3 dimensional vectors for float.
* 
* Create your own by implementing the Trait Vector3
*
*/

pub use std::libc::{c_float};

pub struct Vector3f {
    x : f32,
    y : f32,
    z : f32
}

trait Vector3fOp {
    fn add_to_Vector3f(&self, lhs: &Vector3f) -> Vector3f;
    fn div_to_Vector3f(&self, lhs: &Vector3f) -> Vector3f;
    fn mul_to_Vector3f(&self, lhs: &Vector3f) -> Vector3f;
    fn sub_to_Vector3f(&self, lhs: &Vector3f) -> Vector3f;
}

impl Vector3f {
    pub fn new(x : f32, y : f32, z : f32) -> Vector3f {
        Vector3f{x : x, y : y, z : z}
    }
}

impl<R: Vector3fOp> Add<R, Vector3f> for Vector3f {
    fn add(&self, rhs: &R) -> Vector3f {
         rhs.add_to_Vector3f(self)
    }
}

impl<R: Vector3fOp> Sub<R, Vector3f> for Vector3f {
    fn sub(&self, rhs: &R) -> Vector3f {
         rhs.sub_to_Vector3f(self)
    }
}

impl<R: Vector3fOp> Mul<R, Vector3f> for Vector3f {
    fn mul(&self, rhs: &R) -> Vector3f {
         rhs.mul_to_Vector3f(self)
    }
}

impl<R: Vector3fOp> Div<R, Vector3f> for Vector3f {
    fn div(&self, rhs: &R) -> Vector3f {
         rhs.div_to_Vector3f(self)
    }
}

impl Vector3fOp for Vector3f {
    fn add_to_Vector3f(&self, lhs: &Vector3f) -> Vector3f {
        Vector3f { x : lhs.x + self.x, y : lhs.y + self.y , z : lhs.z + self.z}
    }

    fn sub_to_Vector3f(&self, lhs: &Vector3f) -> Vector3f {
        Vector3f { x : lhs.x - self.x, y : lhs.y - self.y, z : lhs.z - self.z }
    }

    fn div_to_Vector3f(&self, lhs: &Vector3f) -> Vector3f {
        Vector3f { x : lhs.x / self.x, y : lhs.y / self.y, z : lhs.z / self.z }
    }

    fn mul_to_Vector3f(&self, lhs: &Vector3f) -> Vector3f {
        Vector3f { x : lhs.x * self.x, y : lhs.y * self.y, z : lhs.z * self.z }
    }
}

impl Vector3fOp for float {
    fn add_to_Vector3f(&self, lhs: &Vector3f) -> Vector3f {
        Vector3f { x : lhs.x + *self as f32, y : lhs.y + *self as f32, z : lhs.z + *self as f32}
    }

    fn sub_to_Vector3f(&self, lhs: &Vector3f) -> Vector3f {
        Vector3f { x : lhs.x - *self as f32, y : lhs.y - *self as f32, z : lhs.z - *self as f32}
    }

    fn mul_to_Vector3f(&self, lhs: &Vector3f) -> Vector3f {
        Vector3f { x : lhs.x * (*self as f32), y : lhs.y * (*self as f32), z : lhs.z * (*self as f32)}
    }

    fn div_to_Vector3f(&self, lhs: &Vector3f) -> Vector3f {
        Vector3f { x : lhs.x / (*self as f32), y : lhs.y / (*self as f32), z : lhs.z / (*self as f32)}
    }
}

impl Eq for Vector3f {
    fn eq(&self, rhs : &Vector3f) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
    fn ne(&self, rhs : &Vector3f) -> bool {
        self.x != rhs.x && self.y != rhs.y && self.z != rhs.z
    }
}
