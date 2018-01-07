extern crate time;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use time::PreciseTime;

#[derive(Debug)]
struct Particle {
    p: Vec<i64>,
    v: Vec<i64>,
    a: Vec<i64>,
}

fn data_to_vec(data: &str) -> Vec<i64> {
    let data: &str = data.split("<").collect::<Vec<&str>>()[1];
    let data: &str = data.split(">").collect::<Vec<&str>>()[0];
    data.split(",").map(|str| str.replace(' ', "").parse::<i64>().unwrap()).collect()
}

fn f_to_vec_i(x: f64) -> Vec<i64> {
    let remains = x % 1.;
    if remains == 0.0 {
        return vec![x as i64];
    }
    let x = (x - remains) as i64;
    if remains > 0. {
        return vec![x, x + 1];
    } else {
        return vec![x, x - 1];
    }
}

fn quadratic_ts(a: f64, b: f64, c: f64) -> Vec<f64> {
    println!("a:{} b:{} c:{}", a, b, c);
    if a == 0. {
        if b == 0. {
            return vec![];
        } else {
            return vec![-c / b];
        }
    }
    println!("b: {}, b*b: {}", b, b*b);
    let before_sqrt_val = b*b - 4.*a*c;
    println!("before sqrt val {}", before_sqrt_val);
    if before_sqrt_val < 0. {
        return vec![-b / (2. * a)];
    }
    let sqrt_val = before_sqrt_val.sqrt();
    // println!("sqrt val {}", sqrt_val);
    let x1: f64 = (-b + sqrt_val) / (2. * a);
    let x2: f64 = (-b - sqrt_val) / (2. * a);
    // println!("x1: {}, x2: {}", x1, x2);
    vec![x1, x2]
}

fn shortest_ts(p: i64, v: i64, a: i64) -> Vec<i64> {
    let p = p as f64;
    let v = v as f64;
    let a = a as f64;
    let mut result_vec: Vec<i64> = vec![];
    for xf in quadratic_ts(a/2., v+a/2., p) {
        result_vec.extend(f_to_vec_i(xf));
    }
    result_vec
}

fn p_at_t(p: i64, v: i64, a:i64, t: i64) -> i64 {
    // p = sigma(t) * a + v * t + p
    // sigma(t) = t * (t + 1) / 2
    p + v * t + a * t * (t + 1) / 2
}

impl Particle {
    fn load_particles(file_name: &str) -> Vec<Particle> {
        let file = BufReader::new(File::open(file_name).unwrap());
        let mut particles: Vec<Particle> = vec![];
        for line in file.lines() {
            let line = line.unwrap();
            particles.push(Particle::new(line));
        }
        particles
    }
    fn new(line: String) -> Particle {
        let vecs_data: Vec<&str> = line.split(", ").collect();
        Particle {
            p: data_to_vec(vecs_data[0]),
            v: data_to_vec(vecs_data[1]),
            a: data_to_vec(vecs_data[2]),
        }
    }
    fn shortest_distance_in(&self, from: i64, to: i64) -> f64 {
        let mut shortest_distance = self.distance_at(from);
        // println!("distance({}) = {}", from, shortest_distance);
        for t in from + 1 .. to + 1 {
            let distance = self.distance_at(t);
            // println!("distance({}) = {}", t, distance);
            if distance < shortest_distance {
                shortest_distance = distance;
            }
        }
        shortest_distance
    }
    fn shortest_distance(&self) -> f64 {
        // let mut ts: Vec<i64> = vec![];
        // for i in 0..3 {
        //     ts.extend(shortest_ts(self.p[i], self.v[i], self.a[i]));
        // }
        // // println!("ts: {:?}", ts);
        // let min_ts = *ts.iter().min().unwrap();
        // let max_ts = *ts.iter().max().unwrap();
        // if max_ts < 0 {
        //     return self.distance_at(0);
        // }
        // if min_ts < 0 {
        //     return self.shortest_distance_in(0, max_ts);
        // }
        // self.shortest_distance_in(min_ts, max_ts)
        self.distance_at(10000)
    }
    fn distance_at(&self, t: i64) -> f64 {
        let xp = p_at_t(self.p[0], self.v[0], self.a[0], t) as f64;
        let yp = p_at_t(self.p[1], self.v[1], self.a[1], t) as f64;
        let zp = p_at_t(self.p[2], self.v[2], self.a[2], t) as f64;
        // println!("{}: {} {} {}", t, xp, yp, zp);
        (xp * xp + yp * yp + zp * zp).sqrt()
    }
    // fn sum_a(&self) -> f64 {
    //     let sum = self.a.iter().fold(0, |mut sum, a| {sum = sum + *a * *a; sum});
    //     println!("{} {}", sum, (sum as f64).sqrt());
    //     (sum as f64).sqrt()
    // }
}

fn main() {
    let start = PreciseTime::now();

    let file_name = "./data/input.txt";
    // let file_name = "./data/example.txt";
    let particles = Particle::load_particles(file_name);
    let mut shortest_distance: f64 = particles[0].shortest_distance();
    let mut shortest_particle: usize = 0;
    // let mut lowest_accel: f64 = particles[0].sum_a();
    // let mut lowest_accel_particle: usize = 0;
    // println!("{:?}", particles);

    for (i, particle) in particles.iter().enumerate() {
        // let sum_a = particle.sum_a();
        // if sum_a < lowest_accel {
        //     lowest_accel = sum_a;
        //     lowest_accel_particle = i;
        // }
        println!("index {}", i);
        let distance = particle.shortest_distance();
        if distance < shortest_distance {
            shortest_particle = i;
            shortest_distance = distance;
        }
    }

    let end = PreciseTime::now();
    println!("shortest distance: {:?}\nindex: {}", shortest_distance, shortest_particle);
    // println!("lowest accel: {:?}\nindex: {}", lowest_accel, lowest_accel_particle);
    println!("{} seconds", start.to(end));
}
