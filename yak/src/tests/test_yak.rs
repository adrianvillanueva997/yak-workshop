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
        let yak = Yak { age: 1.0 };
        assert!(yak.is_alive(), "Yak is not alive");
    }
}
