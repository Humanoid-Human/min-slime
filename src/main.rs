use rayon::iter::{IntoParallelIterator, ParallelIterator};

const TN_SEED: i64 = -8080661144804004702;
const TARGET: i32 = 1300;
const RANGE: i32 = 30000;
const STEP: i32 = 5;

fn main() {
    let steps = RANGE / STEP;
    let mins: Vec<(i32, i32, i32)> = (-steps..steps).into_par_iter().map(
        |x| {
            let mut min = TARGET;
            let mut minz = 0;
            for z in -steps..steps {
                let s = slime_spawnable_nearby(x * STEP, z * STEP, min);
                if s < min {
                    min = s;
                    minz = z;
                }
            }
            (min, x, minz)
        }
    ).filter(|x| x.0 < TARGET ).collect();

    for (val, x, z) in mins {
        println!("min: {} at {}, {}", val, (x as i32 - steps) * STEP, z * STEP);
    }
}

fn slime_spawnable_nearby(x: i32, z: i32, min: i32) -> i32 {
    let mut out = 0;
    let cx = x >> 3;
    let cz = z >> 3;

    for dcx in -8..9 {
        for dcz in -8..9 {
            if is_slime_chunk(cx + dcx, cz + dcz) {
                let cd = dcx * dcx + dcz * dcz;
                if cd <= 45 && cd > 5 {
                    out += 16 * 16;
                    if out >= min {
                        return min + 1;
                    }
                } else {
                    for dx in 0..16 {
                        for dz in 0..16 {
                            let dist_x = (cx + dcx << 3) + dx - x;
                            let dist_z = (cz + dcz << 3) + dz - z;
                            let d2 = (dist_x * dist_x) + (dist_z * dist_z);
                            if d2 <= 128 * 128 && d2 >= 24 * 24 {
                                out += 1;
                            }
                        }
                        if out >= min {
                            return min + 1;
                        }
                    }
                }
            }
        }
    }

    return out;
}

fn is_slime_chunk(cx: i32, cz: i32) -> bool {
    let mut seed = (
        TN_SEED + 
        (cx * cx * 4987142) as i64 +
        (cx * 5947611) as i64 +
        (cz * cz) as i64 * 4392871 +
        (cz * 389711) as i64
    ) ^ 25303508018;

    loop {
        seed = (seed * 25214903917 + 11) & ((1 << 48) - 1);
        let raw = (seed >> 17) as i32;
        let val = raw % 10;
        if raw - val + 9 >= 0 {
            return val == 0;
        }
    }
}