#[cfg(test)]
pub mod tests {
    use crate::Yak;
    #[test]
    fn test_yak_is_alive() {
        assert!(Yak::new("test".to_string(), 1, 'f').is_alive());
        assert!(Yak::new("test".to_string(), 10, 'f').is_alive());
        assert!(Yak::new("test".to_string(), 1, 'f').is_alive());
        assert!(!Yak::new("test".to_string(), 0, 'f').is_alive());
    }

    #[test]
    fn test_yak_years_to_days() {
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(1), 101);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(2), 102);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(3), 103);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(4), 104);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(5), 105);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(6), 106);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(7), 107);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(8), 108);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(9), 109);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(10), 110);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(11), 111);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(12), 112);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(13), 113);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(14), 114);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(15), 115);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(16), 116);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(17), 117);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(18), 118);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').years_to_days(19), 119);
    }
    #[test]
    fn test_yak_days_to_years() {
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(1), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(2), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(3), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(4), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(5), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(6), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(7), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(8), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(9), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(10), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(11), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(12), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(13), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(14), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(15), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(16), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(17), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(18), 1);
        assert_eq!(Yak::new("test".to_string(), 1, 'f').days_to_years(19), 1);
    }
    #[test]
    fn test_yak_calculate_milk_production() {
        assert_eq!(
            Yak::new("test".to_string(), 4, 'f').calculate_milk_production(4),
            37.88
        );
        assert_eq!(
            Yak::new("test".to_string(), 4, 'f').calculate_milk_production(1),
            37.97
        );
    }
    #[test]
    fn test_yak_can_be_shaved() {
        assert!(Yak::new("test".to_string(), 4, 'f').can_be_shaved(13));
        assert!(Yak::new("test".to_string(), 3, 'f').can_be_shaved(0));
        assert!(!Yak::new("test".to_string(), 3, 'f').can_be_shaved(5));
    }
}
