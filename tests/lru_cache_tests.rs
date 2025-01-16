use crossfit_records::lru_cache::LruCache;

#[test]
fn test_lru_cache_put_and_get() {
    let mut cache = LruCache::new(3);

    cache.put("Snatch".to_string(), 65.0);
    cache.put("Clean and Jerk".to_string(), 75.0);
    cache.put("Deadlift".to_string(), 140.0);

    assert_eq!(cache.get(&"Snatch".to_string()), Some(&65.0));
    assert_eq!(cache.get(&"Clean and Jerk".to_string()), Some(&75.0));
    assert_eq!(cache.get(&"Deadlift".to_string()), Some(&140.0));
}

#[test]
fn test_lru_cache_eviction() {
    let mut cache = LruCache::new(2);

    cache.put("Squat".to_string(), 100.0);
    cache.put("Press".to_string(), 50.0);
    cache.put("Deadlift".to_string(), 150.0);

    assert_eq!(cache.get(&"Squat".to_string()), None); // Squat est éjecté
    assert_eq!(cache.get(&"Press".to_string()), Some(&50.0));
    assert_eq!(cache.get(&"Deadlift".to_string()), Some(&150.0));
}
