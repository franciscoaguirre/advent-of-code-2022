use super::CPU;

pub fn solve(cpu: &CPU) {
    let rows = cpu.get_pixels();

    for row in &rows {
        for pixel in row {
            print!("{pixel}");
        }
        println!();
    }
}
