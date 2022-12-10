use super::CPU;

const SKIP_CYCLES: usize = 20;
const STEP_BY_CYCLES: usize = 40;

pub fn solve(cpu: &CPU) -> i32 {
    let register_x_log = cpu.get_log();

    // Cycles in log are 0-indexed
    register_x_log
        .iter()
        .enumerate()
        .skip(SKIP_CYCLES - 1)
        .step_by(STEP_BY_CYCLES)
        .map(|(cycle_index, register_x_value)| {
            let cycle_number = cycle_index + 1;

            register_x_value * cycle_number as i32
        })
        .sum()
}
