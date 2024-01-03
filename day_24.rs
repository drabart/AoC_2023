

#[derive(Copy, Clone, Debug)]
struct V3 {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Copy, Clone, Debug)]
struct Hailstone {
    pos: V3,
    vel: V3
}

impl FromIterator<f64> for V3 {
    fn from_iter<T: IntoIterator<Item=f64>>(iter: T) -> Self {
        let values: Vec<f64> = iter.into_iter().collect();
        let x = values[0];
        let y = values[1];
        let z = values[2];
        V3 {
            x,
            y,
            z
        }
    }
}

#[derive(Debug)]
struct LinearFunction {
    slope: f64,
    y_intercept: f64,
}

impl LinearFunction {
    fn intersect(&self, other: &LinearFunction) -> Option<(f64, f64)> {
        // Check if the functions are parallel
        if self.slope == other.slope {
            return None; // Parallel lines, no intersection
        }

        // Calculate intersection point
        let x = (other.y_intercept - self.y_intercept) / (self.slope - other.slope);
        let y = self.slope * x + self.y_intercept;

        Some((x, y))
    }
}
fn part1(lines: &Vec<&str>) {
    let begin = 200000000000000f64;
    let end = 400000000000000f64;
    // let delta = 0.000001f64;

    let hailstones = lines
        .iter()
        .map(|&line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            let pos = pos.split(", ").map(|val| val.trim().parse::<f64>().unwrap()).collect::<V3>();
            let vel = vel.split(", ").map(|val| val.trim().parse::<f64>().unwrap()).collect::<V3>();
            Hailstone {
                pos,
                vel
            }
        })
        .collect::<Vec<Hailstone>>();

    // dbg!(&hailstones);

    let mut result = 0;

    for i in 0..(hailstones.len()-1) {
        for j in (i+1)..hailstones.len() {
            // py + (vy / vx) (t - px)

            let hail1 = LinearFunction {
                slope: hailstones[i].vel.y / hailstones[i].vel.x,
                y_intercept: hailstones[i].pos.y - hailstones[i].pos.x *
                    (hailstones[i].vel.y / hailstones[i].vel.x)
            };

            let hail2 = LinearFunction {
                slope: hailstones[j].vel.y / hailstones[j].vel.x,
                y_intercept: hailstones[j].pos.y - hailstones[j].pos.x *
                    (hailstones[j].vel.y / hailstones[j].vel.x)
            };


            let intersection = hail1.intersect(&hail2);

            if intersection.is_none() {
                continue;
            }

            let (x, y) = intersection.unwrap();
            // println!("{i} {j} {x:.2} {y:.2}");

            if (x > hailstones[i].pos.x && hailstones[i].vel.x < 0.) ||
                (x < hailstones[i].pos.x && hailstones[i].vel.x > 0.) {
                continue;
            }
            if (y > hailstones[i].pos.y && hailstones[i].vel.y < 0.) ||
                (y < hailstones[i].pos.y && hailstones[i].vel.y > 0.) {
                continue;
            }
            if (x > hailstones[j].pos.x && hailstones[j].vel.x < 0.) ||
                (x < hailstones[j].pos.x && hailstones[j].vel.x > 0.) {
                continue;
            }
            if (y > hailstones[j].pos.y && hailstones[j].vel.y < 0.) ||
                (y < hailstones[j].pos.y && hailstones[j].vel.y > 0.) {
                continue;
            }

            if y > begin && y < end && x > begin && x < end {
                // println!("{i} {j} {x:.2} {y:.2}");
                result += 1;
            }
        }
    }

    println!("Result of part 1: {result}");
}

fn part2(lines: &Vec<&str>) {
    let hailstones = lines
        .iter()
        .map(|&line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            let pos = pos.split(", ").map(|val| val.trim().parse::<f64>().unwrap()).collect::<V3>();
            let vel = vel.split(", ").map(|val| val.trim().parse::<f64>().unwrap()).collect::<V3>();
            Hailstone {
                pos,
                vel
            }
        })
        .collect::<Vec<Hailstone>>();

    for vx in -300..300 {
        for vy in -300..300 {
            'vzf: for vz in -300..300 {
                let a = hailstones[0].pos.x;
                let b = hailstones[0].vel.x;
                let c = vx as f64;
                let d = hailstones[1].pos.x;
                let e = hailstones[1].vel.x;
                let f = hailstones[0].pos.y;
                let g = hailstones[0].vel.y;
                let h = vy as f64;
                let i = hailstones[1].pos.y;
                let j = hailstones[1].vel.y;

                if b * h + c * j + e * g == b * j + c * g + e * h {
                    continue;
                }

                let t1 = (-(a - d) * (h - j) + c * (f - i) - e * (f - i))/(b * (h - j) + c * (j - g) + e * (g - h));
                let t2 = ((a - d) * (g - h) - b * (f - i) + c * (f - i))/(b * (h - j) + c * (j - g) + e * (g - h));

                if t1 < 0.0 || t2 < 0.0 {
                    continue;
                }

                let px = hailstones[0].pos.x + hailstones[0].vel.x * t1 - vx as f64 * t1;
                let py = hailstones[0].pos.y + hailstones[0].vel.y * t1 - vy as f64 * t1;
                let pz = hailstones[0].pos.z + hailstones[0].vel.z * t1 - vz as f64 * t1;

                /*
                if vx == -3 && vy == 1 && vz == 2 {
                    println!("{px} {py} {pz}");
                }
                 */

                for x in 0..hailstones.len() {
                    let h = hailstones[x];
                    let t = (px - h.pos.x) / (h.vel.x - vx as f64);
                    if (py + vy as f64 * t - (h.pos.y + h.vel.y * t)).abs() > 0.1f64 {
                        continue 'vzf;
                    }
                    if (pz + vz as f64 * t - (h.pos.z + h.vel.z * t)).abs() > 0.1f64 {
                        continue 'vzf;
                    }
                }

                println!("Result of part 2: {}", px+py+pz);
                return;
            }
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("res/d24.in").unwrap();
    let lines = data.split("\n").filter(|&x| x != "").collect::<Vec<&str>>();

    part1(&lines);
    part2(&lines);
}
