pub mod template;

pub mod rect {
    use crate::range::range_inclusive_overlap;
    use crate::vector::Vector2d;
    use std::ops::RangeInclusive;

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub struct Rectangle {
        pub x_from: i64,
        pub x_to: i64,
        pub y_from: i64,
        pub y_to: i64,
    }

    impl Rectangle {
        pub fn contains(&self, vector2d: &Vector2d) -> bool {
            self.x_from <= vector2d.x
                && vector2d.x <= self.x_to
                && self.y_from <= vector2d.y
                && vector2d.y <= self.y_to
        }

        pub fn width(&self) -> i64 {
            self.x_to - self.x_from
        }

        pub fn height(&self) -> i64 {
            self.y_to - self.y_from
        }

        pub fn x_range(&self) -> RangeInclusive<i64> {
            self.x_from..=self.x_to
        }

        pub fn y_range(&self) -> RangeInclusive<i64> {
            self.y_from..=self.y_to
        }

        pub fn overlaps(&self, other: &Rectangle) -> bool {
            range_inclusive_overlap(&self.x_range(), &other.x_range())
                && range_inclusive_overlap(&self.y_range(), &other.y_range())
        }

        pub fn move_dir(&self, dir: &Vector2d) -> Rectangle {
            Rectangle {
                x_from: self.x_from + dir.x,
                x_to: self.x_to + dir.x,
                y_from: self.y_from + dir.y,
                y_to: self.y_to + dir.y,
            }
        }
    }
}

pub mod range {
    use std::ops::{Range, RangeInclusive};

    pub fn range_inclusive_overlap(
        first: &RangeInclusive<i64>,
        second: &RangeInclusive<i64>,
    ) -> bool {
        first.start() <= second.end() && second.start() <= first.end()
    }

    pub fn range_overlap(first: &Range<i64>, second: &Range<i64>) -> bool {
        first.start < second.end && second.start < first.end
    }
}

pub mod vector {
    use std::ops::{Add, Mul, Sub};

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub struct Vector2d {
        pub x: i64,
        pub y: i64,
    }

    impl Add for Vector2d {
        type Output = Vector2d;

        fn add(self, other: Self) -> Vector2d {
            Vector2d {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Add for &Vector2d {
        type Output = Vector2d;

        fn add(self, other: Self) -> Self::Output {
            Vector2d {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Sub for Vector2d {
        type Output = Vector2d;

        fn sub(self, other: Self) -> Self::Output {
            Vector2d {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl Sub for &Vector2d {
        type Output = Vector2d;

        fn sub(self, other: Self) -> Self::Output {
            Vector2d {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl Mul<i64> for Vector2d {
        type Output = Vector2d;

        fn mul(self, other: i64) -> Self::Output {
            Vector2d {
                x: self.x * other,
                y: self.y * other,
            }
        }
    }

    impl Mul<i64> for &Vector2d {
        type Output = Vector2d;

        fn mul(self, other: i64) -> Self::Output {
            Vector2d {
                x: self.x * other,
                y: self.y * other,
            }
        }
    }

    pub const DIR_RIGHT: Vector2d = Vector2d { x: 1, y: 0 };
    pub const DIR_DOWN: Vector2d = Vector2d { x: 0, y: 1 };
    pub const DIR_LEFT: Vector2d = Vector2d { x: -1, y: 0 };
    pub const DIR_UP: Vector2d = Vector2d { x: 0, y: -1 };
    pub const DIRS_MAIN: [Vector2d; 4] = [DIR_RIGHT, DIR_UP, DIR_LEFT, DIR_DOWN];

    pub const DIR_RIGHT_DOWN: Vector2d = Vector2d { x: 1, y: 1 };
    pub const DIR_RIGHT_UP: Vector2d = Vector2d { x: 1, y: -1 };
    pub const DIR_LEFT_UP: Vector2d = Vector2d { x: -1, y: -1 };
    pub const DIR_LEFT_DOWN: Vector2d = Vector2d { x: -1, y: 1 };
    pub const DIRS_DIAGONALS: [Vector2d; 4] =
        [DIR_RIGHT_DOWN, DIR_RIGHT_UP, DIR_LEFT_UP, DIR_LEFT_DOWN];

    pub const DIRS_ALL: [Vector2d; 8] = [
        DIR_RIGHT,
        DIR_RIGHT_UP,
        DIR_UP,
        DIR_LEFT_UP,
        DIR_LEFT,
        DIR_LEFT_DOWN,
        DIR_DOWN,
        DIR_RIGHT_DOWN,
    ];

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub struct Vector3d {
        pub x: i64,
        pub y: i64,
        pub z: i64,
    }

    impl Add for Vector3d {
        type Output = Vector3d;

        fn add(self, other: Self) -> Vector3d {
            Vector3d {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl Add for &Vector3d {
        type Output = Vector3d;

        fn add(self, other: Self) -> Self::Output {
            Vector3d {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl Sub for Vector3d {
        type Output = Vector3d;

        fn sub(self, other: Self) -> Self::Output {
            Vector3d {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    impl Sub for &Vector3d {
        type Output = Vector3d;

        fn sub(self, other: Self) -> Self::Output {
            Vector3d {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    impl Mul<i64> for Vector3d {
        type Output = Vector3d;

        fn mul(self, other: i64) -> Self::Output {
            Vector3d {
                x: self.x * other,
                y: self.y * other,
                z: self.z * other
            }
        }
    }

    impl Mul<i64> for &Vector3d {
        type Output = Vector3d;

        fn mul(self, other: i64) -> Self::Output {
            Vector3d {
                x: self.x * other,
                y: self.y * other,
                z: self.z * other
            }
        }
    }

    impl Vector3d {
        pub fn from_str(s: &str) -> Vector3d {
            let coords: Vec<i64> = s.split(",").map(|x| x.parse().unwrap()).collect();
            Self {
                x: coords[0],
                y: coords[1],
                z: coords[2]
            }
        }
        pub fn distance_to(self, other: Vector3d) -> f64 {
            linear_distance(&self, &other)
        }

        pub fn distance_to_ref(self, other: &Vector3d) -> f64 {
            linear_distance(&self, other)
        }
    }

    pub fn linear_distance(first: &Vector3d, second: &Vector3d) -> f64 {
        (((first.x - second.x).pow(2) + (first.y - second.y).pow(2) + (first.z - second.z).pow(2)) as f64).sqrt()
    }

}
