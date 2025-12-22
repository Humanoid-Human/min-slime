use std::{io::stdin, time::Instant};
use rayon::prelude::*;

const TN_SEED: i64 = -8080661144804004702;

fn main() {
    let seed = get_i64("Seed", TN_SEED);
    let step = get_i64("Step size", 10) as i32;
    let range = get_i64("Range", 30000) as i32;
    let target = get_i64("Maximum slime-spawnable blocks", 700) as i32;

    println!("Calculating");

    let tic = Instant::now();
    print_mins(seed, step, range, target);
    let toc = tic.elapsed();

    println!("Finished in {}ms", toc.as_millis());
}

fn get_i64(print: &str, default: i64) -> i64 {
    println!("{} (leave blank for {}): ", print, default);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read from stdin");
    if input.len() == 1 {
        default
    } else {
        input.strip_suffix("\n").unwrap().parse().expect("failed to parse input.")
    }
}

fn print_mins(seed: i64, step: i32, range: i32, target: i32) {
    let steps = range / step;
    let mut mins: Vec<(i32, i32, i32)> = Vec::with_capacity(steps as usize);

    for x in -steps..steps {
        let mut zmins = (-steps..steps).into_par_iter().map (
                |z| (slime_spawnable_nearby(seed, x * step, z * step, target), x, z)
            ).filter(|x| x.0 < target).collect();
        mins.append(&mut zmins);
    }

    mins.par_sort_by(|a, b| a.0.cmp(&b.0));

    for (val, x, z) in mins {
        println!("{} at {}, {}", val, x * step, z * step);
    }
}

fn slime_spawnable_nearby(seed: i64, x: i32, z: i32, max: i32) -> i32 {
    let mut out = 0;
    let cx = x >> 4;
    let cz = z >> 4;

    for dcx in -9..=9 {
        for dcz in -9..=9 {
            let cd = dcx * dcx + dcz * dcz;
            if cd > 85 || !is_slime_chunk(seed, cx + dcx, cz + dcz) {
                continue;
            }

            if cd <= 45 && cd > 5 {
                out += 256;
                if out > max { return out; }
                continue;
            }

            for dx in 0..16 {
                for dz in 0..16 {
                    let dist_x = (dcx << 4) + dx;
                    let dist_z = (dcz << 4) + dz;
                    let d2 = (dist_x * dist_x) + (dist_z * dist_z);
                    if d2 <= 128 * 128 && d2 >= 24 * 24 {
                        out += 1;
                    }
                }
                if out > max { return out; }
            }
        }
    }

    return out;
}

fn is_slime_chunk(seed: i64, cx: i32, cz: i32) -> bool {
    let mut seed = (
        seed + 
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