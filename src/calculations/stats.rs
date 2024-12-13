pub fn energy_from_potential_and_vigilance(potential: u32, vigilance: u32) -> u32 {
    let ratio = vigilance as f32 / u16::MAX as f32;
    (potential as f32 * ratio).floor() as u32
}