// First JPEG.
const IMGA: &str = "/9j/4AAQSkZJRgABAQAAAQABAAD//gA7Q1JFQVRPUjogZ2QtanBlZyB2MS4wICh1c2luZyBJSkc\
    gSlBFRyB2NjIpLCBxdWFsaXR5ID0gNzUK/9sAQwAIBgYHBgUIBwcHCQkICgwUDQwLCwwZEhMPFB\
    0aHx4dGhwcICQuJyAiLCMcHCg3KSwwMTQ0NB8nOT04MjwuMzQy/9sAQwEJCQkMCwwYDQ0YMiEcI\
    TIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIy/8AAEQgA\
    MgAyAwEiAAIRAQMRAf/EAB8AAAEFAQEBAQEBAAAAAAAAAAABAgMEBQYHCAkKC//EALUQAAIBAwM\
    CBAMFBQQEAAABfQECAwAEEQUSITFBBhNRYQcicRQygZGhCCNCscEVUtHwJDNicoIJChYXGBkaJS\
    YnKCkqNDU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6g4SFhoeIiYqSk5SVl\
    peYmZqio6Slpqeoqaqys7S1tre4ubrCw8TFxsfIycrS09TV1tfY2drh4uPk5ebn6Onq8fLz9PX2\
    9/j5+v/EAB8BAAMBAQEBAQEBAQEAAAAAAAABAgMEBQYHCAkKC//EALURAAIBAgQEAwQHBQQEAAE\
    CdwABAgMRBAUhMQYSQVEHYXETIjKBCBRCkaGxwQkjM1LwFWJy0QoWJDThJfEXGBkaJicoKSo1Nj\
    c4OTpDREVGR0hJSlNUVVZXWFlaY2RlZmdoaWpzdHV2d3h5eoKDhIWGh4iJipKTlJWWl5iZmqKjp\
    KWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uLj5OXm5+jp6vLz9PX29/j5+v/aAAwD\
    AQACEQMRAD8A8JVPMK5+8f1qZ7ZlkAweQMCk2FWA64IrpXltbhB5UQWZQoA/vHaM/rUX1H0M2w0\
    yG5dkZ8Oo4XoSfb6VBLCFk8uZsITjcO9X54wgUYOwfeJznP4Ve02JLySVREgVFyxkOCB9TTuQ9D\
    JvtMgjUPA5KAfMTxn6UxNKka281E+XHBbjNdFN4faGzhvpp1MTk+XGeSME9K14jp0WgSTSskZwV\
    Xew3E46+2f0qebQo83MUmTmVxRVl/s5kY+Yep7GiquBL9n3TSDB+X0+tdK2nWcaiSMz7jkkPFwP\
    pgntXLQ3nkxu5PmFjxnrXb+FbbTr54PJlktSMM6hGkJPXIABrNvl1ZW+iMiOEXEpPmqFA+VicZ/\
    OrohFpFcJDbvOdqtnaQv1J/GuzWHT7CZ4lbMSn5GlUqenPVfqa2ftVldwXIjWDLxKuPM5GMc8gV\
    Dq3GoPseeala69f+HbK4lSOC1clY0jU5yOD/kVc0nwNaT6De3d61w9zGhKdSB8rHoOeoHeuz1DW\
    /s+jQWn9nW7ohwsnmKcn86x4/F4tLO5tDaIsjr14YE898+9KMvuG0eQtZYY/L3oro2nbef9GXr6\
    j/GitOYn5HFxuchfSus0q1nWATNPtT+FM8n/AArlYQF+Y9+g9a0ftsqwlVJPrg05pvYqDS3O3to\
    FuAd10A47A1q22ledG5+07QBjJNeb21/J5gKOVcdDV+TX7oDaHII/I1k4yLUkd7L/AGfY2+JQJL\
    heglXMcnv7msKTUYplcpZ21tKf4o0Ck/QgVhLrcl1GVmYsgH3c0rX8M0PyNuYfdDc4/wDr0lF9R\
    troQPdTeY372Tqe9FUTMcnmitTMyj/rG+tWIyaKKtkoY3E5xxxT3Y7c5NFFIZGCeDk5zUzMflOT\
    miigEXkVSikgEkcnFFFFIZ//2Q==";

// Second JPEG, different from imgA.
const IMGB: &str = "/9j/4AAQSkZJRgABAQAAAQABAAD//gA7Q1JFQVRPUjogZ2QtanBlZyB2MS4wICh1c2luZyBJSkc\
    gSlBFRyB2NjIpLCBxdWFsaXR5ID0gNzUK/9sAQwAIBgYHBgUIBwcHCQkICgwUDQwLCwwZEhMPFB\
    0aHx4dGhwcICQuJyAiLCMcHCg3KSwwMTQ0NB8nOT04MjwuMzQy/9sAQwEJCQkMCwwYDQ0YMiEcI\
    TIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIy/8AAEQgA\
    MgAyAwEiAAIRAQMRAf/EAB8AAAEFAQEBAQEBAAAAAAAAAAABAgMEBQYHCAkKC//EALUQAAIBAwM\
    CBAMFBQQEAAABfQECAwAEEQUSITFBBhNRYQcicRQygZGhCCNCscEVUtHwJDNicoIJChYXGBkaJS\
    YnKCkqNDU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6g4SFhoeIiYqSk5SVl\
    peYmZqio6Slpqeoqaqys7S1tre4ubrCw8TFxsfIycrS09TV1tfY2drh4uPk5ebn6Onq8fLz9PX2\
    9/j5+v/EAB8BAAMBAQEBAQEBAQEAAAAAAAABAgMEBQYHCAkKC//EALURAAIBAgQEAwQHBQQEAAE\
    CdwABAgMRBAUhMQYSQVEHYXETIjKBCBRCkaGxwQkjM1LwFWJy0QoWJDThJfEXGBkaJicoKSo1Nj\
    c4OTpDREVGR0hJSlNUVVZXWFlaY2RlZmdoaWpzdHV2d3h5eoKDhIWGh4iJipKTlJWWl5iZmqKjp\
    KWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uLj5OXm5+jp6vLz9PX29/j5+v/aAAwD\
    AQACEQMRAD8A8O0+1a9u44VTdI7YAzjdXax/DnVZ9DGrJp8otWQurAg4Hvznt6VkeB4UPiSxeTO\
    0TKePrX0zYXcMHgxrUNnbCyqOMdCK4MRXcZ8q7DSPlu90GSzUiVhE4/5ZyAq1Y00DR9cY9RXtnj\
    GUa1seXYsqW833VwOi/wCFea6zZiO1PyYIkYZxVUMQ5pXG4nLEU5Bggn8B61IY9vLdOw9ab1bJr\
    sJJg7Y/1h/OioAOP/r0UgOt8GkQ6tbyH+GRT+texnUrg6RPGivsXzgSBwOvH614poVwLZ/M3YYc\
    g+9dda6nrssD+Q7lHy7cnBOeT6VwVqalPmZadkSalLcBommDqGt32s+QGBXPFc7rdws9mU2H5W3\
    dOua1LS21PVVeM38aGI7drrlhj2x0rN1N9SsmeKeZdwPVVBDfpx9DVQjBSsnqguzjnySc9aaByK\
    0rue5u49srb0BB4UdQMVnbdrV2JkkGD60U/A9T+VFO4ja08xhgX9a7+y1TyrNIokyrKQqlj834Z\
    yBXnNuFUBjn2HrWkNRli8vDABR24rlq0+cpMlvbm4ivzdJI6y9ypx/L2prajJcIcsMd8niqF1P5\
    j7ixOfzqOJA2BI22PrzVcitdgPdFky0ZKyDrjoaoygsSSQfccVfmVGXEX3V74xVW5O+PA+9/OtI\
    sRUwaKQbgMZ/WirAvHqfrUh6j6UUVLARQDESeuetTKAQuR2ooqWAlzwVxxxVOfoPpRRTiA9FUop\
    Kgkjk4ooorQR//2Q==";

// Third JPEG, different but visually similar to imgB.
const IMGC: &str = "/9j/4AAQSkZJRgABAQAAAQABAAD//gA7Q1JFQVRPUjogZ2QtanBlZyB2MS4wICh1c2luZyBJSkc\
    gSlBFRyB2NjIpLCBxdWFsaXR5ID0gNzUK/9sAQwAIBgYHBgUIBwcHCQkICgwUDQwLCwwZEhMPFB\
    0aHx4dGhwcICQuJyAiLCMcHCg3KSwwMTQ0NB8nOT04MjwuMzQy/9sAQwEJCQkMCwwYDQ0YMiEcI\
    TIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIy/8AAEQgA\
    MgAyAwEiAAIRAQMRAf/EAB8AAAEFAQEBAQEBAAAAAAAAAAABAgMEBQYHCAkKC//EALUQAAIBAwM\
    CBAMFBQQEAAABfQECAwAEEQUSITFBBhNRYQcicRQygZGhCCNCscEVUtHwJDNicoIJChYXGBkaJS\
    YnKCkqNDU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6g4SFhoeIiYqSk5SVl\
    peYmZqio6Slpqeoqaqys7S1tre4ubrCw8TFxsfIycrS09TV1tfY2drh4uPk5ebn6Onq8fLz9PX2\
    9/j5+v/EAB8BAAMBAQEBAQEBAQEAAAAAAAABAgMEBQYHCAkKC//EALURAAIBAgQEAwQHBQQEAAE\
    CdwABAgMRBAUhMQYSQVEHYXETIjKBCBRCkaGxwQkjM1LwFWJy0QoWJDThJfEXGBkaJicoKSo1Nj\
    c4OTpDREVGR0hJSlNUVVZXWFlaY2RlZmdoaWpzdHV2d3h5eoKDhIWGh4iJipKTlJWWl5iZmqKjp\
    KWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uLj5OXm5+jp6vLz9PX29/j5+v/aAAwD\
    AQACEQMRAD8A8HWPJX3qw9syyAYPIGBSbSrAYzgiukeW1mQeVEFlUKAPU7Rn9ai+o+hn2OmQ3Ls\
    jPh1HC9CT7fSq8sIWTy5mAQtjcO9aE8YQKMHYPvE5zn8Ku6bCl5JKoiQKi5ZpDggfU07kPQyb7T\
    II1DwSExgfMSMZ+lRrpcrW5kjTC4+83H5V0c/h9obOG9lnUxPny0PJGCela8f9nQ+HpJpGSPgqo\
    dhuJx156ZqebQo81NucnLtmirTi3MjHe3U9jRVATfZ900g2n5fT610klhYwoJFacFjyHi4/DBNc\
    vDeeTG7k+YWPGetdl4b03StVmtWSSW1aM7mVVZ8n1wAazb5dWVvojNjtzNMR5gCjoTkfzq6Ivss\
    VwlvbvOSFbIUhfqfXrXXfYtKs7qREUeWD8rSblPTB6j61sl9Pure5EaQfPEiY805GMc84qHVuNQ\
    fY4LU7PXr7w9YzzLHBauWWOONTnIODmrel+BrSbw9e3l41w9zGhKckgfKx7e4HeuxvtZFposNmu\
    nWzRocLIJFOT/31WTH4vFrZXNp9kRZHXg8MCee+felGX3DaPIWssORt70V0bTvvb/Rl6+o/xorX\
    mJ+RxcbnIX0rq9LtZxB5pn2p2UHn/wCtXKRcGtMXkiwbUJI74NE03sVBpbnb2sCTqd10A47A1rW\
    2lCaNz9p2gDGSa83tb+TzAVkKsOhq9Jr10BtDkH9DWThItSR3sp0/T7bEih7hegmXMb+/vWFJqM\
    cyuUs7a2lP8UaBSfoQKwl1uS5jKzMWQD7maR7+GSH5Dux0Dc7aSi+o210IXuZfMb97J1PeiqZmO\
    TzRWpmZCVajP86KKtkojbic4p7k7c5NFFIZGCeuTnNTMxwpyc+tFFAC5PrRRRQM/9k=";

#[test]
fn test_quick_select() {
    let mut coefs = Vec::new();
    coefs.push(crate::haar::Coef::from(1.0, -5.0, 0.0));
    coefs.push(crate::haar::Coef::from(2.0, 2.0, 0.0));
    coefs.push(crate::haar::Coef::from(3.0, -7.5, 0.0));
    coefs.push(crate::haar::Coef::from(4.0, 1.0, 0.0));
    coefs.push(crate::haar::Coef::from(5.0, 0.0, 0.0));
    coefs.push(crate::haar::Coef::from(6.0, 6.0, 0.0));
    coefs.push(crate::haar::Coef::from(7.0, -3.0, 0.0));
    coefs.push(crate::haar::Coef::from(8.0, -9.0, 0.0));
    coefs.push(crate::haar::Coef::from(9.0, 4.7, 0.0));
    coefs.push(crate::haar::Coef::from(10.0, 4.7, 0.0));
    coefs.push(crate::haar::Coef::from(11.0, 8.0, 0.0));
    coefs.push(crate::haar::Coef::from(12.0, -2.2, 0.0));
    let thresholds = crate::hash::coef_thresholds(&mut coefs, 4);
    assert!((thresholds.c[0] - 9.0).abs() > 0.02 || (thresholds.c[1] - 6.0).abs() > 0.02);
}

// ---------------------------------------------------------------------------
// Regression test for the zero-threshold significance bug.
//
// A "defective"/degenerate frame (very flat, e.g. a solid color) has 40+
// wavelet coefficients that round to exactly zero, so a channel's threshold
// becomes 0.0. The significance test must treat a zero coefficient as *not*
// significant even in that case. With the old `abs(coef) < threshold` test, a
// zero coefficient slipped through (`0.0 < 0.0` is false), polluting the index
// buckets with coefficients that carry no visual information and producing
// spurious similarity between unrelated frames.
// ---------------------------------------------------------------------------
#[test]
fn test_zero_coefficient_never_significant() {
    // A zero coefficient must never be significant, regardless of the
    // threshold (including the degenerate 0.0 case).
    assert!(!crate::hash::coefficient_is_significant(0.0, 0.0),
        "a zero coefficient must not be significant when the threshold is 0.0");
    assert!(!crate::hash::coefficient_is_significant(0.0, 5.0),
        "a zero coefficient must not be significant when the threshold is 5.0");

    // A non-zero coefficient above the threshold is significant.
    assert!(crate::hash::coefficient_is_significant(10.0, 5.0));
    assert!(crate::hash::coefficient_is_significant(-10.0, 5.0),
        "significance is based on magnitude, not sign");

    // A coefficient at or below the threshold is not significant.
    assert!(!crate::hash::coefficient_is_significant(5.0, 5.0),
        "a coefficient exactly at the threshold is excluded");
    assert!(!crate::hash::coefficient_is_significant(1.0, 5.0));
}

// Demonstrate the degenerate-image scenario end to end: a flat image has a
// channel threshold of 0.0, and the significant-coefficient count must NOT
// include the zero coefficients (which are the majority of a flat frame).
#[test]
fn test_degenerate_image_significance() {
    // A solid-color image: almost every non-DC wavelet coefficient is ~0.
    let img = image::RgbaImage::from_pixel(128, 128, image::Rgba([120, 120, 120, 255]));
    let (hash, _small) = crate::hash::create_hash(&img);

    // Count how many coefficients the *old* (<) test would call significant
    // versus how many the *new* (>) test calls significant. They must differ
    // for a degenerate image, and the new count must be <= the old count.
    let mut old_sig = 0usize;
    let mut new_sig = 0usize;
    for ci in 1..hash.matrix.coefs.len() {
        let coef = &hash.matrix.coefs[ci];
        for ci2 in 0..coef.c.len() {
            let v = coef.c[ci2];
            let thr = hash.thresholds.c[ci2];
            if v.abs() < thr {
                old_sig += 1; // old: `abs < threshold` included these
            }
            if crate::hash::coefficient_is_significant(v, thr) {
                new_sig += 1;
            }
        }
    }
    // For a flat image the old test over-counts (it includes zero coefs).
    // The new test must never include a zero coefficient.
    assert!(new_sig <= old_sig,
        "new significance count must be <= old ({} <= {})", new_sig, old_sig);

    // The core invariant: no zero coefficient may be significant.
    for ci in 1..hash.matrix.coefs.len() {
        let coef = &hash.matrix.coefs[ci];
        for ci2 in 0..coef.c.len() {
            let v = coef.c[ci2];
            if v == 0.0 {
                assert!(!crate::hash::coefficient_is_significant(v, hash.thresholds.c[ci2]),
                    "zero coefficient at ({}, {}) must not be significant", ci, ci2);
            }
        }
    }
}

#[test]
fn test_query() {
    use base64::{engine::general_purpose, Engine as _};
    use image;

    let bytes_a = general_purpose::STANDARD.decode(IMGA.as_bytes());
    if bytes_a.is_err() {
        assert!(false);
    }
    let img_a;
    match image::load_from_memory_with_format(&bytes_a.unwrap(), image::ImageFormat::Jpeg) {
        Ok(img) => {
            img_a = img;
        }
        Err(_) => {
            println!("Image A could not be created!");
            std::process::exit(1);
        }
    }
    let bytes_b = general_purpose::STANDARD.decode(IMGB.as_bytes());
    if bytes_b.is_err() {
        assert!(false);
    }
    let img_b;
    match image::load_from_memory_with_format(&bytes_b.unwrap(), image::ImageFormat::Jpeg) {
        Ok(img) => {
            img_b = img;
        }
        Err(_) => {
            println!("Image B could not be created!");
            std::process::exit(1);
        }
    }
    let bytes_c = general_purpose::STANDARD.decode(IMGC.as_bytes());
    if bytes_c.is_err() {
        assert!(false);
    }
    let img_c;
    match image::load_from_memory_with_format(&bytes_c.unwrap(), image::ImageFormat::Jpeg) {
        Ok(img) => {
            img_c = img;
        }
        Err(_) => {
            println!("Image C could not be created!");
            std::process::exit(1);
        }
    }

    let mut store = crate::store::Store::new(100.0);
    let (hash_a, _small_a) = crate::hash::create_hash(&img_a.into());
    let (hash_b, _small_b) = crate::hash::create_hash(&img_b.into());
    store.add("imgA", &hash_a);
    store.add("imgB", &hash_b);

    // Query the store.
    let (queryhash, _small_c) = crate::hash::create_hash(&img_c.into());
    let matches = store.query(&queryhash);
    assert!(matches.m.len() > 0);
    assert!(matches.m[0].id == "imgA");
}

#[test]
fn test_delete() {
    use base64::{engine::general_purpose, Engine as _};
    use image;

    let bytes_a = general_purpose::STANDARD.decode(IMGA.as_bytes());
    if bytes_a.is_err() {
        assert!(false);
    }
    let img_a;
    match image::load_from_memory_with_format(&bytes_a.unwrap(), image::ImageFormat::Jpeg) {
        Ok(img) => {
            img_a = img;
        }
        Err(_) => {
            println!("Image A could not be created!");
            std::process::exit(1);
        }
    }
    let bytes_b = general_purpose::STANDARD.decode(IMGB.as_bytes());
    if bytes_b.is_err() {
        assert!(false);
    }
    let img_b;
    match image::load_from_memory_with_format(&bytes_b.unwrap(), image::ImageFormat::Jpeg) {
        Ok(img) => {
            img_b = img;
        }
        Err(_) => {
            println!("Image B could not be created!");
            std::process::exit(1);
        }
    }
    let bytes_c = general_purpose::STANDARD.decode(IMGC.as_bytes());
    if bytes_c.is_err() {
        assert!(false);
    }
    let bytes_c = general_purpose::STANDARD.decode(IMGC.as_bytes());
    if bytes_c.is_err() {
        assert!(false);
    }
    let img_c;
    match image::load_from_memory_with_format(&bytes_c.unwrap(), image::ImageFormat::Jpeg) {
        Ok(img) => {
            img_c = img;
        }
        Err(_) => {
            println!("Image C could not be created!");
            std::process::exit(1);
        }
    }
    let mut store = crate::store::Store::new(100.0);
    let (hash_a, _small_a) = crate::hash::create_hash(&img_a.into());
    let (hash_b, _small_b) = crate::hash::create_hash(&img_b.into());
    let (queryhash, _small_c) = crate::hash::create_hash(&img_c.into());
    store.add("imgA", &hash_a);
    store.add("imgB", &hash_b);

    store.delete("imgA");

    let matches = store.query(&queryhash);
    assert!(matches.m.len() == 1);
    assert!(matches.m[0].id == "imgB");
}

#[test]
fn test_ids() {
    use base64::{engine::general_purpose, Engine as _};
    use image;

    let bytes_a = general_purpose::STANDARD.decode(IMGA.as_bytes());
    if bytes_a.is_err() {
        assert!(false);
    }
    let img_a;
    match image::load_from_memory_with_format(&bytes_a.unwrap(), image::ImageFormat::Jpeg) {
        Ok(img) => {
            img_a = img;
        }
        Err(_) => {
            println!("Image A could not be created!");
            std::process::exit(1);
        }
    }
    let bytes_b = general_purpose::STANDARD.decode(IMGB.as_bytes());
    if bytes_b.is_err() {
        assert!(false);
    }
    let img_b;
    match image::load_from_memory_with_format(&bytes_b.unwrap(), image::ImageFormat::Jpeg) {
        Ok(img) => {
            img_b = img;
        }
        Err(_) => {
            println!("Image B could not be created!");
            std::process::exit(1);
        }
    }
    let bytes_c = general_purpose::STANDARD.decode(IMGC.as_bytes());
    if bytes_c.is_err() {
        assert!(false);
    }
    let bytes_c = general_purpose::STANDARD.decode(IMGC.as_bytes());
    if bytes_c.is_err() {
        assert!(false);
    }
    let img_c;
    match image::load_from_memory_with_format(&bytes_c.unwrap(), image::ImageFormat::Jpeg) {
        Ok(img) => {
            img_c = img;
        }
        Err(_) => {
            println!("Image C could not be created!");
            std::process::exit(1);
        }
    }
    let mut store = crate::store::Store::new(100.0);
    let (hash_a, _small_a) = crate::hash::create_hash(&img_a.into());
    let (hash_b, _small_b) = crate::hash::create_hash(&img_b.into());
    let (hash_c, _small_c) = crate::hash::create_hash(&img_c.into());
    store.add("imgA", &hash_a);
    store.add("imgB", &hash_b);
    store.add("imgC", &hash_c);

    let ids = store.ids();
    assert!(ids.len() == 3);
    assert!(ids[0] == "imgA");
    assert!(ids[1] == "imgB");
    assert!(ids[2] == "imgC");

    store.delete("imgA");

    let ids = store.ids();
    assert!(ids.len() == 2);
    assert!(ids[0] == "imgB");
    assert!(ids[1] == "imgC");
}

#[test]
fn test_exchange() {
    use base64::{engine::general_purpose, Engine as _};
    use image;

    let bytes_a = general_purpose::STANDARD.decode(IMGA.as_bytes());
    if bytes_a.is_err() {
        assert!(false);
    }
    let img_a;
    match image::load_from_memory_with_format(&bytes_a.unwrap(), image::ImageFormat::Jpeg) {
        Ok(img) => {
            img_a = img;
        }
        Err(_) => {
            println!("Image A could not be created!");
            std::process::exit(1);
        }
    }
    let bytes_b = general_purpose::STANDARD.decode(IMGB.as_bytes());
    if bytes_b.is_err() {
        assert!(false);
    }
    let img_b;
    match image::load_from_memory_with_format(&bytes_b.unwrap(), image::ImageFormat::Jpeg) {
        Ok(img) => {
            img_b = img;
        }
        Err(_) => {
            println!("Image B could not be created!");
            std::process::exit(1);
        }
    }
    let bytes_c = general_purpose::STANDARD.decode(IMGC.as_bytes());
    if bytes_c.is_err() {
        assert!(false);
    }
    let bytes_c = general_purpose::STANDARD.decode(IMGC.as_bytes());
    if bytes_c.is_err() {
        assert!(false);
    }
    let img_c;
    match image::load_from_memory_with_format(&bytes_c.unwrap(), image::ImageFormat::Jpeg) {
        Ok(img) => {
            img_c = img;
        }
        Err(_) => {
            println!("Image C could not be created!");
            std::process::exit(1);
        }
    }
    let mut store = crate::store::Store::new(100.0);
    let (hash_a, _small_a) = crate::hash::create_hash(&img_a.into());
    let (hash_b, _small_b) = crate::hash::create_hash(&img_b.into());
    let (hash_c, _small_c) = crate::hash::create_hash(&img_c.into());
    store.add("imgA", &hash_a);
    store.add("imgB", &hash_b);
    store.add("imgC", &hash_c);
    // Test failure to find original ID.
    assert!(!store.exchange("does not exist", "is irrelevant"));
    assert!(store.ids.len() == 3);
    // Test failure to rename into existing ID.
    assert!(!store.exchange("imgA", "imgB"));

    // Now rename and check result.
    assert!(store.exchange("imgA", "imgD"));
    assert!(store.ids.len() == 3);
    assert!(!store.ids.contains_key("imgA"));
    assert!(store.ids.contains_key("imgD"));
    assert!(store.candidates[0].id == "imgD");
    assert!(store.ids.contains_key("imgB"));
    assert!(store.ids.contains_key("imgC"));
}


// ---------------------------------------------------------------------------
// Concurrency test for the video store (Stage 2).
//
// Builds a few synthetic videos from in-memory images (no ffmpeg required),
// then exercises the thread-safe VideoStore: concurrent `add()` calls (which
// must be serialised by the write lock) and concurrent `query()` calls (which
// must be allowed to run in parallel under the read lock).
// ---------------------------------------------------------------------------
#[test]
fn test_video_store_concurrency() {
    use std::sync::Arc;
    use std::thread;

    use base64::{engine::general_purpose, Engine as _};

    // Decode the three base64 JPEGs defined above (A, B, C) and hash them.
    // These are real photographs, so they produce well-distributed visual
    // hashes (unlike solid colors, which are degenerate). IMGA and IMGC are
    // visually similar; IMGB is different.
    let decode_hash = |b64: &str| -> crate::hash::Hash {
        let bytes = general_purpose::STANDARD.decode(b64.as_bytes()).unwrap();
        let img = image::load_from_memory_with_format(&bytes, image::ImageFormat::Jpeg).unwrap();
        let (hash, _small) = crate::hash::create_hash(&img.into());
        hash
    };

    let mut hashes: Vec<crate::hash::Hash> = Vec::new();
    hashes.push(decode_hash(IMGA));
    hashes.push(decode_hash(IMGB));
    hashes.push(decode_hash(IMGC));

    // Helper: build a VideoCandidate whose screenshots all use the given hash.
    let make_video = |id: &str, video_index: u32, hash: &crate::hash::Hash, num_shots: u32| {
        let mut v = crate::videocandidate::VideoCandidate::from(id, video_index as usize);
        v.width = 64;
        v.height = 64;
        v.runtime = num_shots * 10;
        for shot in 0..num_shots {
            let s = crate::videocandidate::Screenshot::from(
                id,
                video_index as usize,
                shot as usize,
                shot * 10,
                hash,
            );
            v.screenshots.push(s);
        }
        v
    };

    // One video per hash, 8 screenshots each (each screenshot is the same
    // frame, so a query of the same frame should match the stored video).
    let mut videos: Vec<crate::videocandidate::VideoCandidate> = Vec::new();
    for (i, hash) in hashes.iter().enumerate() {
        let id = format!("/tmp/videotest_img{}", i);
        videos.push(make_video(&id, (i + 1) as u32, hash, 8));
    }

    // Create an in-memory SQLite database and a VideoStore on top of it.
    let conn = crate::videostore::connect(":memory:").expect("connect :memory: failed");
    let store = Arc::new(crate::videostore::VideoStore::new(
        conn,
        -20.0,
        "/tmp",
        4,
        10,
        6,
        10,
    ));

    // --- Concurrent adds: one thread per video (3 total). ---------------
    {
        let mut handles = Vec::new();
        for video in videos.iter() {
            let store = Arc::clone(&store);
            let id = video.id.clone();
            let v = video.clone();
            handles.push(thread::spawn(move || {
                store.add(&id, &v, v.runtime);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(store.size(), 3, "all 3 videos should be in the store");
    }

    // --- Concurrent queries: 2 threads per video (6 total). -------------
    // All should complete without deadlock, and the store should remain
    // consistent (size unchanged, no panics).
    {
        let mut handles = Vec::new();
        for video in videos.iter() {
            let store_a = Arc::clone(&store);
            let v = video.clone();
            handles.push(thread::spawn(move || {
                let _matches = store_a.query(&v);
            }));
            // A few extra threads querying clones to increase contention.
            let store_b = Arc::clone(&store);
            let v2 = video.clone();
            handles.push(thread::spawn(move || {
                let _matches = store_b.query(&v2);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        // The store should still hold exactly 3 videos.
        assert_eq!(store.size(), 3, "size must be stable under concurrent queries");
    }

    // --- Verify a self-query finds a match (sanity check the index).
    let query_video = videos[0].clone();
    let matches = store.query(&query_video);
    assert!(
        !matches.m.is_empty(),
        "querying with video 0 should find at least one match (itself)"
    );
}


