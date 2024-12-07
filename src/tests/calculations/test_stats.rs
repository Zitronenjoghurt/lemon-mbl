use crate::calculations::stats::energy_from_potential_and_vigilance;

#[test]
fn test_energy_from_potential_and_vigilance() {
    let energy = energy_from_potential_and_vigilance(0, 0);
    assert_eq!(energy, 0);

    let energy = energy_from_potential_and_vigilance(5000, 0);
    assert_eq!(energy, 0);

    let energy = energy_from_potential_and_vigilance(5000, u16::MAX);
    assert_eq!(energy, 5000);

    let energy = energy_from_potential_and_vigilance(u16::MAX, u16::MAX);
    assert_eq!(energy, u16::MAX);

    let energy = energy_from_potential_and_vigilance(50, 6500);
    assert_eq!(energy, 4);
}