// Copyright 2013 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cgmath::BaseNum;

/// An intersection test with a result.
///
/// An example would be a Ray vs AABB intersection test that returns a Point in space.
///
pub trait Continuous<RHS> {
    /// Result returned by the intersection test
    type Result;

    /// Intersection test
    fn intersection(&self, &RHS) -> Option<Self::Result>;
}

/// A boolean intersection test.
///
pub trait Discrete<RHS> {
    /// Intersection test
    fn intersects(&self, &RHS) -> bool;
}

/// Boolean containment test.
///
pub trait Contains<RHS> {
    /// Containment test
    #[inline]
    fn contains(&self, &RHS) -> bool;
}

/// Shape surface area
///
pub trait SurfaceArea {
    /// Result type returned from surface area computation
    type Scalar: BaseNum;

    /// Compute surface area
    fn surface_area(&self) -> Self::Scalar;
}

/// Build the union of two shapes.
///
pub trait Union<RHS = Self> {
    /// Union shape created
    type Output;

    /// Build the union shape of self and the given shape.
    fn union(&self, &RHS) -> Self::Output;
}
