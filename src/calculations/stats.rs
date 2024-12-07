pub fn energy_from_potential_and_vigilance(potential: u16, vigilance: u16) -> u16 {
    let ratio = vigilance as f32 / 65535f32;
    (potential as f32 * ratio).floor() as u16
}