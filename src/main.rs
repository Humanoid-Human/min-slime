use std::env::args;
use rayon::prelude::*;
use itertools::Itertools;

const TN_SEED: i64 = -8080661144804004702;

fn main() {
    let mut seed = TN_SEED;
    let mut step64 = 8;
    let mut range64 = 30000;
    let mut target64 = 700;
    let mut limit = 10;

    let args: Vec<String> = args().collect();

    let mut p: Option<&mut i64> = None;
    for arg in &args[1..] {
        if let Some(x) = p {
            *x = arg.parse().unwrap();
            p = None;
            continue;
        }
        match arg.as_str() {
            "-S" | "--seed" => p = Some(&mut seed),
            "-s" | "--step" => p = Some(&mut step64),
            "-r" | "--range" => p = Some(&mut range64),
            "-m" | "--max" => p = Some(&mut target64),
            "-l" | "--limit" => p = Some(&mut limit),
            "-h" | "--help" => { print_help(); return; },
            _ => ()
        }
    }

    println!("Blocks, X, Z");

    for min in mins(seed, step64 as i32, range64 as i32, target64 as i32, limit as usize) {
        println!("{}, {}, {}", min.num, min.x, min.z);
    }
}

fn print_help() {
    println!("Usage:");
    println!("  -S <n> | --seed <n>   The seed to check. Default is Technut SMP seed ({})", TN_SEED);
    println!("  -s <n> | --step <n>   Check locations every n blocks. Default is 8");
    println!("  -r <n> | --range <n>  Check locations within n blocks in all directions. Default is 30000");
    println!("  -m <n> | --max <n>    Maximum number of slime-spawnable points for a location to be listed. Default is 700");
    println!("  -l <n> | --limit <n>  Maximum number of locations to list. Default is 10");
    println!("  -h     | --help       Show this command");
}

struct Loc {
    x: i32,
    z: i32,
    num: i32
}

fn mins(seed: i64, step: i32, range: i32, target: i32, limit: usize) -> Vec<Loc> {
    let r = (-range..range).step_by(step as usize);
    let check: Vec<(i32, i32)> = r.clone().cartesian_product(r).collect();

    let mut mins: Vec<Loc> = check.into_par_iter().filter_map(
        |(x, z)| {
            let num = slime_spawnable_nearby(seed, x, z, target);
            if num < target {
                Some(Loc { x, z, num })
            } else {
                None
            }
        }
    ).collect();
    
    mins.par_sort_by(|a, b| a.num.cmp(&b.num));
    mins.truncate(limit);
    mins
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
                    if (24 * 24..=128 * 128).contains(&d2) {
                        out += 1;
                    }
                }
                if out > max { return out; }
            }
        }
    }

    out
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
