use std::{
    time::Duration,
    thread::sleep
};

const SYMBOLS: &str = ".,-~:;=!*#$@";

fn main() {
    let mut angle_a: f32 = 1.;
    let mut angle_b: f32 = 1.;

    loop {

        angle_a += 0.07;
        angle_b += 0.03;

        let mut depth_buffer = [0.; 1760];
        let mut output_buffer = [' '; 1760];

        let (sin_a, cos_a) = angle_a.sin_cos();
        let (sin_b, cos_b) = angle_b.sin_cos();

        // originaly it was 0..44 and now division
        for j in 0..220{
            let new_j = j as f32 / 5.;
            let (sin_j, cos_j) = new_j.sin_cos();

            for i in 0..220 {
                let new_i = i as f32 / 5.;
                let (sin_i, cos_i) = new_i.sin_cos();
                let h = cos_j + 2.0;
                let d = 1.0 / (sin_i * h * sin_a + sin_j * cos_a + 5.0);
                let t = sin_i * h * cos_a - sin_j * sin_a;
                let x = (40.0 + 30.0 * d * (cos_i * h * cos_b - t * sin_b)) as usize;
                let y = ((12.0 + 15.0 * d * (cos_i * h * sin_b + t * cos_b))) as usize;
                let o = x + 80 * y;
                let n = 8.0 * ((sin_j * sin_a - sin_i * cos_j * cos_a) * cos_b - sin_i * cos_j * sin_a - sin_j * cos_a - cos_i * cos_j * sin_b);

                if y < 22 && x < 79 && d > depth_buffer[o] {
                    depth_buffer[o] = d;
                    output_buffer[o] = SYMBOLS.chars().nth(n as usize).or(Some(' ')).unwrap();
                }
            }
        }

        let mut output = String::new();
        for chunk in output_buffer.chunks_exact(80) {
            output.push_str(&String::from_iter(chunk));
            output.push('\n');
        }
        print!("\x1B[H{}", output);

        sleep(Duration::from_millis(30));
    }
}
