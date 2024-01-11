use std::time::Instant;

#[derive(Debug)]
struct Hail {
    pos: (i128, i128, i128),
    velocity: (i128, i128, i128),
}

impl Hail {
    #[allow(unused_doc_comments)]
    fn find_xy_collision(&self, other: &Hail) -> Option<(f64, f64)> {
        /**
         * this problem made me realise how rusty my math is :"
         *
         * given (x, y) and (vx, vy)
         * vx -> rate of change of x, vy -> rate of change of y
         *
         * y = mx + c -> y = (dy/dx)x + c
         *
         * ax + by + c = 0
         * by = -(ax + c)
         * y = -(ax + c) / b
         * -a/b <==> m, which we can find using vy/vx
         *
         * after finding a and b,
         * we can just substitute x and y to find c
         * c = -ax -by
         *
         * finally, https://www.geeksforgeeks.org/point-of-intersection-of-two-lines-formula/
         * (did the math myself, ended up with the same formula)
         */
        let a0 = -self.velocity.1; // dy
        let b0 = self.velocity.0; // dx
        let c0 = -a0 * self.pos.0 - b0 * self.pos.1;

        let a1 = -other.velocity.1; // dy
        let b1 = other.velocity.0; // dx
        let c1 = -a1 * other.pos.0 - b1 * other.pos.1;

        // println!("{:?} = {a0}x + {b0}y + {c0}", self);
        // println!("{:?} = {a1}x + {b1}y + {c1}", other);

        if a0 as f64 / b0 as f64 == a1 as f64 / b1 as f64 {
            return None;
        }
        let x = (b0 * c1 - b1 * c0) as f64 / (a0 * b1 - a1 * b0) as f64;
        let y = (c0 * a1 - c1 * a0) as f64 / (a0 * b1 - a1 * b0) as f64;

        let t0 = (x - self.pos.0 as f64) / self.velocity.0 as f64;
        let t1 = (x - other.pos.0 as f64) / other.velocity.0 as f64;
        if t0 <= 0.0 || t1 <= 0.0 {
            return None;
        }
        Some((x, y))
    }
}
fn main() {
    let start = Instant::now();

    let input = include_str!("./input");
    let parser = |line: &str| {
        let (p, v) = line.split_once('@').unwrap();
        let p = p
            .split(',')
            .map(|c| c.trim().parse::<i128>().unwrap())
            .collect::<Vec<_>>();
        let v = v
            .split(',')
            .map(|c| c.trim().parse::<i128>().unwrap())
            .collect::<Vec<_>>();
        Hail {
            pos: (p[0], p[1], p[2]),
            velocity: (v[0], v[1], v[2]),
        }
    };
    let hails = input.lines().map(parser).collect::<Vec<_>>();

    const TEST_AREA_START: f64 = 200000000000000.0;
    const TEST_AREA_END: f64 = 400000000000000.0;
    // const TEST_AREA_START: f64 = 7.0;
    // const TEST_AREA_END: f64 = 27.0;

    let n = hails.len();
    let mut num_collisions = 0;
    for i in 0..n {
        for j in i + 1..n {
            if let Some((x, y)) = hails[i].find_xy_collision(&hails[j]) {
                // println!("intersect: ({x},{y})");
                if (TEST_AREA_START..=TEST_AREA_END).contains(&x)
                    && (TEST_AREA_START..=TEST_AREA_END).contains(&y)
                {
                    num_collisions += 1;
                }
            } else {
                // println!("does not intersect");
            }
        }
    }
    println!("{}", num_collisions);
    println!("{:?}", start.elapsed());
}
