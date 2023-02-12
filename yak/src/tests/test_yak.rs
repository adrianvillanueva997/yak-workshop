// def test_yak_alive():
//     assert yak.Yak(10).is_alive()
//     assert yak.Yak(-1).is_alive() == False
//     assert yak.Yak(15).is_alive() == False
//     assert yak.Yak(0).is_alive()

// def test_yak_can_be_shaved():
//     assert yak.Yak(4).can_be_shaved(13)
//     assert yak.Yak(4).can_be_shaved(0)
//     assert yak.Yak(4).can_be_shaved(5) == False

// def test_yak_milk_production():
//     assert yak.Yak(4).calculate_milk_production(4) == 37.88
//     assert yak.Yak(4).calculate_milk_production(1) == 37.97

// def test_years_to_days():
//     assert yak.Yak(4).years_to_days(4) == 404
//     assert yak.Yak(2).years_to_days(50) == 250
//     assert yak.Yak(3).years_to_days(4) == 304

// def test_days_to_years():
//     assert yak.Yak(4).days_to_years(4) == 4.04
//     assert yak.Yak(2).days_to_years(50) == 2.50
//     assert yak.Yak(3).days_to_years(4) == 3.04

pub mod tests {
    #[allow(unused_imports)]
    use crate::Yak;
    #[test]
    fn is_alive() {
        assert!(Yak::default(1.0).is_alive(), "Yak is alive");
        assert!(!Yak::default(-5.0).is_alive(), "Yak is not alive");
        assert!(!Yak::default(11.0).is_alive(), "Yak is not alive")
    }
    #[test]
    fn can_be_shaved() {
        assert!(Yak::default(4.0).can_be_shaved(13.0));
        assert!(Yak::default(4.0).can_be_shaved(0.0));
        assert!(!Yak::default(4.0).can_be_shaved(5.0));
    }
    #[test]
    fn yak_milk_production() {
        assert!(Yak::default(4.0).calculate_milk_production(4.0) == 37.88);
        assert!(Yak::default(4.0).calculate_milk_production(1.0) == 37.97)
    }
    #[test]
    fn years_to_days() {
        assert!(Yak::default(4.0).years_to_days(4.0) == 404.0);
        assert!(Yak::default(2.0).years_to_days(50.0) == 250.0);
    }
    #[test]
    fn days_to_years() {
        assert!(Yak::default(4.0).days_to_years(4.0) == 4.04);
        assert!(Yak::default(2.0).days_to_years(50.0) == 2.50);
    }
}
