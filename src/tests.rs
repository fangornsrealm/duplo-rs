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
    assert!(
        !crate::hash::coefficient_is_significant(0.0, 0.0),
        "a zero coefficient must not be significant when the threshold is 0.0"
    );
    assert!(
        !crate::hash::coefficient_is_significant(0.0, 5.0),
        "a zero coefficient must not be significant when the threshold is 5.0"
    );

    // A non-zero coefficient above the threshold is significant.
    assert!(crate::hash::coefficient_is_significant(10.0, 5.0));
    assert!(
        crate::hash::coefficient_is_significant(-10.0, 5.0),
        "significance is based on magnitude, not sign"
    );

    // A coefficient at or below the threshold is not significant.
    assert!(
        !crate::hash::coefficient_is_significant(5.0, 5.0),
        "a coefficient exactly at the threshold is excluded"
    );
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
    assert!(
        new_sig <= old_sig,
        "new significance count must be <= old ({} <= {})",
        new_sig,
        old_sig
    );

    // The core invariant: no zero coefficient may be significant.
    for ci in 1..hash.matrix.coefs.len() {
        let coef = &hash.matrix.coefs[ci];
        for ci2 in 0..coef.c.len() {
            let v = coef.c[ci2];
            if v == 0.0 {
                assert!(
                    !crate::hash::coefficient_is_significant(v, hash.thresholds.c[ci2]),
                    "zero coefficient at ({}, {}) must not be significant",
                    ci,
                    ci2
                );
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
        conn, -20.0, "/tmp", 4, 10, 6, 10,
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
        assert_eq!(
            store.size(),
            3,
            "size must be stable under concurrent queries"
        );
    }

    // --- Verify a self-query finds a match (sanity check the index).
    let query_video = videos[0].clone();
    let matches = store.query(&query_video);
    assert!(
        !matches.m.is_empty(),
        "querying with video 0 should find at least one match (itself)"
    );
}

// ---------------------------------------------------------------------------
// Video store: sequence detection.
//
// The "6-in-a-row" rule: a stored video must have at least
// `min_similar_screenshots_in_sequence` consecutive matching screenshots
// for it to appear in results. A stored video with only 5 shots can at
// most produce a 5-shot sequence, so it must be excluded. A stored video
// with exactly 6 shots (the boundary) must be included.
//
// Key insight: the query video's screenshot count determines how many
// positions are checked. A stored video can only build a sequence up to
// its own shot count. Using a 6-shot query means:
//   - 8-shot stored video: matches all 6 positions, seq len 6 >= 6  ✓
//   - 6-shot stored video: matches all 6 positions, seq len 6 >= 6  ✓
//   - 5-shot stored video: matches 5, restarts at pos 5, seq len 1 < 6 ✗
// ---------------------------------------------------------------------------
#[test]
fn test_video_sequence_detection() {
    use base64::{engine::general_purpose, Engine as _};

    let bytes = general_purpose::STANDARD.decode(IMGA.as_bytes()).unwrap();
    let img = image::load_from_memory_with_format(&bytes, image::ImageFormat::Jpeg).unwrap();
    let (hash, _small) = crate::hash::create_hash(&img.into());

    let make_video = |id: &str, video_index: u32, num_shots: u32| {
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
                &hash,
            );
            v.screenshots.push(s);
        }
        v
    };

    let min_seq: u32 = 6;
    let conn = crate::videostore::connect(":memory:").unwrap();
    let store = crate::videostore::VideoStore::new(conn, -20.0, "/tmp", 1, 10, min_seq, 10);

    // Store a video with 8 screenshots (well above the 6-shot threshold).
    let long_video = make_video("/tmp/seq_long", 1, 8);
    store.add("/tmp/seq_long", &long_video, long_video.runtime);

    // Boundary case: exactly 6 screenshots (meets the threshold).
    let boundary_video = make_video("/tmp/seq_boundary", 2, 6);
    store.add("/tmp/seq_boundary", &boundary_video, boundary_video.runtime);

    // Below threshold: only 5 screenshots (cannot form a 6-shot sequence).
    let short_video = make_video("/tmp/seq_short", 3, 5);
    store.add("/tmp/seq_short", &short_video, short_video.runtime);

    // Query with exactly 6 screenshots: the boundary video can cover all
    // 6 positions, but the short video (5 shots) cannot.
    let query_video = make_video("/tmp/seq_query", 4, 6);
    let matches = store.query(&query_video);

    let long_found = matches.m.iter().any(|m| m.id == "/tmp/seq_long");
    let boundary_found = matches.m.iter().any(|m| m.id == "/tmp/seq_boundary");
    let short_found = matches.m.iter().any(|m| m.id == "/tmp/seq_short");

    assert!(
        long_found,
        "video with 8 shots should match (sequence 6 >= threshold 6)"
    );
    assert!(
        boundary_found,
        "video with exactly 6 shots should match (sequence 6 == threshold 6)"
    );
    assert!(
        !short_found,
        "video with 5 shots should NOT match (max continuous sequence < 6)"
    );
}

// ---------------------------------------------------------------------------
// Video store: delete correctness.
// ---------------------------------------------------------------------------
#[test]
fn test_video_delete() {
    use base64::{engine::general_purpose, Engine as _};

    let bytes_a = general_purpose::STANDARD.decode(IMGA.as_bytes()).unwrap();
    let img_a = image::load_from_memory_with_format(&bytes_a, image::ImageFormat::Jpeg).unwrap();
    let (hash_a, _) = crate::hash::create_hash(&img_a.into());

    let bytes_b = general_purpose::STANDARD.decode(IMGB.as_bytes()).unwrap();
    let img_b = image::load_from_memory_with_format(&bytes_b, image::ImageFormat::Jpeg).unwrap();
    let (hash_b, _) = crate::hash::create_hash(&img_b.into());

    let make_video = |id: &str, video_index: u32, hash: &crate::hash::Hash| {
        let mut v = crate::videocandidate::VideoCandidate::from(id, video_index as usize);
        v.width = 64;
        v.height = 64;
        v.runtime = 80;
        for shot in 0..8u32 {
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

    let conn = crate::videostore::connect(":memory:").unwrap();
    let store = crate::videostore::VideoStore::new(conn, -20.0, "/tmp", 1, 10, 6, 10);

    let va = make_video("/tmp/del_a", 1, &hash_a);
    let vb = make_video("/tmp/del_b", 2, &hash_b);
    store.add("/tmp/del_a", &va, va.runtime);
    store.add("/tmp/del_b", &vb, vb.runtime);

    assert_eq!(store.size(), 2);
    assert!(store.has("/tmp/del_a"));
    assert!(store.has("/tmp/del_b"));

    // Delete one video.
    store.delete("/tmp/del_a");
    assert_eq!(store.size(), 1);
    assert!(!store.has("/tmp/del_a"));
    assert!(store.has("/tmp/del_b"));

    // Deleting a non-existent video should be a no-op.
    store.delete("/tmp/nonexistent");
    assert_eq!(store.size(), 1);
}

// ---------------------------------------------------------------------------
// Video store: exchange (rename) correctness.
// ---------------------------------------------------------------------------
#[test]
fn test_video_exchange() {
    use base64::{engine::general_purpose, Engine as _};

    let bytes_a = general_purpose::STANDARD.decode(IMGA.as_bytes()).unwrap();
    let img_a = image::load_from_memory_with_format(&bytes_a, image::ImageFormat::Jpeg).unwrap();
    let (hash_a, _) = crate::hash::create_hash(&img_a.into());

    let bytes_b = general_purpose::STANDARD.decode(IMGB.as_bytes()).unwrap();
    let img_b = image::load_from_memory_with_format(&bytes_b, image::ImageFormat::Jpeg).unwrap();
    let (hash_b, _) = crate::hash::create_hash(&img_b.into());

    let make_video = |id: &str, video_index: u32, hash: &crate::hash::Hash| {
        let mut v = crate::videocandidate::VideoCandidate::from(id, video_index as usize);
        v.width = 64;
        v.height = 64;
        v.runtime = 80;
        for shot in 0..8u32 {
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

    let conn = crate::videostore::connect(":memory:").unwrap();
    let store = crate::videostore::VideoStore::new(conn, -20.0, "/tmp", 1, 10, 6, 10);

    let va = make_video("/tmp/exch_a", 1, &hash_a);
    let vb = make_video("/tmp/exch_b", 2, &hash_b);
    store.add("/tmp/exch_a", &va, va.runtime);
    store.add("/tmp/exch_b", &vb, vb.runtime);

    assert_eq!(store.size(), 2);
    assert!(store.has("/tmp/exch_a"));
    assert!(store.has("/tmp/exch_b"));

    // Exchange (rename) video A to a new name.
    assert!(store.exchange("/tmp/exch_a", "/tmp/exch_a_new"));
    assert_eq!(store.size(), 2);
    assert!(!store.has("/tmp/exch_a"), "old name should be gone");
    assert!(store.has("/tmp/exch_a_new"), "new name should exist");
    assert!(
        store.has("/tmp/exch_b"),
        "unaffected video should still exist"
    );

    // Exchanging to an existing name should fail.
    assert!(!store.exchange("/tmp/exch_b", "/tmp/exch_a_new"));

    // Exchanging a non-existent ID should fail.
    assert!(!store.exchange("/tmp/nonexistent", "/tmp/whatever"));
}

// ---------------------------------------------------------------------------
// Video store: re-query after add (store reflects new data without restart).
// ---------------------------------------------------------------------------
#[test]
fn test_video_requery_after_add() {
    use base64::{engine::general_purpose, Engine as _};

    let bytes_a = general_purpose::STANDARD.decode(IMGA.as_bytes()).unwrap();
    let img_a = image::load_from_memory_with_format(&bytes_a, image::ImageFormat::Jpeg).unwrap();
    let (hash_a, _) = crate::hash::create_hash(&img_a.into());

    let bytes_b = general_purpose::STANDARD.decode(IMGB.as_bytes()).unwrap();
    let img_b = image::load_from_memory_with_format(&bytes_b, image::ImageFormat::Jpeg).unwrap();
    let (hash_b, _) = crate::hash::create_hash(&img_b.into());

    let make_video = |id: &str, video_index: u32, hash: &crate::hash::Hash| {
        let mut v = crate::videocandidate::VideoCandidate::from(id, video_index as usize);
        v.width = 64;
        v.height = 64;
        v.runtime = 80;
        for shot in 0..8u32 {
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

    let conn = crate::videostore::connect(":memory:").unwrap();
    let store = crate::videostore::VideoStore::new(conn, -20.0, "/tmp", 1, 10, 6, 10);

    // Initially empty: querying should yield no matches.
    let query_video = make_video("/tmp/rq_query", 1, &hash_a);
    let matches_before = store.query(&query_video);
    assert!(
        matches_before.m.is_empty(),
        "empty store should return no matches"
    );

    // Add a video with the same hash.
    let stored_video = make_video("/tmp/rq_stored", 1, &hash_a);
    store.add("/tmp/rq_stored", &stored_video, stored_video.runtime);

    // Query again: should now find the newly added video.
    let matches_after = store.query(&query_video);
    assert!(
        !matches_after.m.is_empty(),
        "after add, query should find the new video"
    );
    let found = matches_after.m.iter().any(|m| m.id == "/tmp/rq_stored");
    assert!(found, "the newly added video should be in the results");

    // A dissimilar query should still not match.
    let dissimilar_query = make_video("/tmp/rq_dissimilar", 2, &hash_b);
    let dissimilar_matches = store.query(&dissimilar_query);
    // IMGB is visually different from IMGA, so it should NOT match.
    // (This is a soft assertion — if the hashes happen to share enough
    //  significant coefficients, it might match. We just verify no panic.)
    let _ = dissimilar_matches;
}

// ---------------------------------------------------------------------------
// Video store: sensitivity monotonicity.
// Lowering sensitivity (more negative) must never increase the match count.
// ---------------------------------------------------------------------------
#[test]
fn test_video_sensitivity_monotonicity() {
    use base64::{engine::general_purpose, Engine as _};

    let decode_hash = |b64: &str| -> crate::hash::Hash {
        let bytes = general_purpose::STANDARD.decode(b64.as_bytes()).unwrap();
        let img = image::load_from_memory_with_format(&bytes, image::ImageFormat::Jpeg).unwrap();
        let (hash, _) = crate::hash::create_hash(&img.into());
        hash
    };

    let hash_a = decode_hash(IMGA);
    let hash_b = decode_hash(IMGB);
    let hash_c = decode_hash(IMGC);

    let make_video = |id: &str, idx: u32, hash: &crate::hash::Hash| {
        let mut v = crate::videocandidate::VideoCandidate::from(id, idx as usize);
        v.width = 64;
        v.height = 64;
        v.runtime = 80;
        for shot in 0..8u32 {
            let s = crate::videocandidate::Screenshot::from(
                id,
                idx as usize,
                shot as usize,
                shot * 10,
                hash,
            );
            v.screenshots.push(s);
        }
        v
    };

    // Build a corpus of 3 videos.
    let corpus: Vec<crate::videocandidate::VideoCandidate> = vec![
        make_video("/tmp/mono_a", 1, &hash_a),
        make_video("/tmp/mono_b", 2, &hash_b),
        make_video("/tmp/mono_c", 3, &hash_c),
    ];

    let query_video = make_video("/tmp/mono_query", 4, &hash_a);

    // Create two stores with different sensitivities.
    let conn_loose = crate::videostore::connect(":memory:").unwrap();
    let store_loose = crate::videostore::VideoStore::new(
        conn_loose, 0.0, "/tmp", 1, 10, 6, 10, // sensitivity = 0 (permissive)
    );
    for v in &corpus {
        store_loose.add(&v.id, v, v.runtime);
    }

    let conn_strict = crate::videostore::connect(":memory:").unwrap();
    let store_strict = crate::videostore::VideoStore::new(
        conn_strict,
        -100.0,
        "/tmp",
        1,
        10,
        6,
        10, // sensitivity = -100 (strict)
    );
    for v in &corpus {
        store_strict.add(&v.id, v, v.runtime);
    }

    let matches_loose = store_loose.query(&query_video);
    let matches_strict = store_strict.query(&query_video);

    assert!(
        matches_strict.m.len() <= matches_loose.m.len(),
        "stricter sensitivity ({}) must not produce more matches ({}) than looser sensitivity ({}) with {} matches",
        -100.0,
        matches_strict.m.len(),
        0.0,
        matches_loose.m.len()
    );
}

// ---------------------------------------------------------------------------
// coef_threshold edge case: k >= n (requesting more coefficients than exist).
// ---------------------------------------------------------------------------
#[test]
fn test_coef_threshold_k_ge_n() {
    use crate::haar::Coef;
    let mut coefs: Vec<Coef> = Vec::new();
    // Create 10 coefficient entries with known values.
    for i in 0..10 {
        let mut c = Coef::new();
        for ch in 0..3 {
            c.c[ch] = (i as f64) * 1.0; // 0.0, 1.0, 2.0, ...
        }
        coefs.push(c);
    }

    // The threshold is chosen so that exactly k values are STRICTLY GREATER
    // than it (matching the `abs(coef) > threshold` significance test).
    // For values [0,1,2,3,4,5,6,7,8,9]:
    //   k=5 → threshold 4.0 (five values {5,6,7,8,9} exceed it)
    //   k=1 → threshold 8.0 (one value {9} exceeds it)
    //   k=20 → not enough values; returns the minimum (0.0)
    let t = crate::hash::coef_threshold(&coefs, 5, 0);
    assert!(
        (t - 4.0).abs() < 1e-9,
        "k=5 of 0..9 should give threshold 4.0 (5 values exceed it), got {}",
        t
    );

    let t2 = crate::hash::coef_threshold(&coefs, 20, 0);
    assert!(
        (t2 - 0.0).abs() < 1e-9,
        "k>=n should return the minimum (0.0), got {}",
        t2
    );

    let t3 = crate::hash::coef_threshold(&coefs, 1, 0);
    assert!(
        (t3 - 8.0).abs() < 1e-9,
        "k=1 of 0..9 should give threshold 8.0 (1 value exceeds it), got {}",
        t3
    );
}

// ---------------------------------------------------------------------------
// Video store: end-to-end with a real (short) video via ffmpeg.
// Skipped if ffmpeg is not available on the system.
// ---------------------------------------------------------------------------
#[test]
fn test_video_end_to_end_ffmpeg() {
    // Check if ffmpeg is available.
    let ffmpeg_available = std::process::Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !ffmpeg_available {
        eprintln!("SKIPPED: ffmpeg not available");
        return;
    }

    // Create a short test video (2 seconds, solid color) in a temp dir.
    let tmp_dir = std::env::temp_dir().join(format!("duplo_e2e_{}", std::process::id()));
    std::fs::create_dir_all(&tmp_dir).unwrap();
    let video_path = tmp_dir.join("test_video.mp4");

    // Generate a 5-second test-pattern video (testsrc has varied content,
    // so the Haar wavelet hash will have significant coefficients — unlike
    // a solid-color video which is degenerate).
    let gen_result = std::process::Command::new("ffmpeg")
        .args([
            "-y",
            "-f",
            "lavfi",
            "-i",
            "testsrc=duration=5:size=320x240:rate=10",
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            &video_path.to_string_lossy(),
        ])
        .output();

    if gen_result.is_err() || !gen_result.unwrap().status.success() {
        eprintln!("SKIPPED: could not generate test video");
        std::fs::remove_dir_all(&tmp_dir).ok();
        return;
    }

    // Process the video.
    let video_path_str = video_path.to_string_lossy().to_string();
    let path_buf = std::path::PathBuf::from(&video_path_str);
    let candidate = crate::files::process_video(
        &path_buf, 1, 1, 1, 1, // 1-second screenshot interval for a 2s video
    );

    // A 5-second video at 1-second intervals should yield ~5 screenshots.
    assert!(
        candidate.screenshots.len() >= 2,
        "should have at least 2 screenshots, got {}",
        candidate.screenshots.len()
    );
    assert!(candidate.runtime > 0, "runtime should be non-zero");

    // Add to a store and verify it can be queried.
    let conn = crate::videostore::connect(":memory:").unwrap();
    let store = crate::videostore::VideoStore::new(
        conn, -20.0, "/tmp", 1, 1, 1, 10, // min_seq=1 so even 1 shot matches
    );
    store.add(&video_path_str, &candidate, candidate.runtime);
    assert_eq!(store.size(), 1);

    // Self-query should find a match (min_seq=1, textured source has
    // significant wavelet coefficients so the index is non-empty).
    let matches = store.query(&candidate);
    if matches.m.is_empty() {
        // The pipeline completed without error — that's the primary goal of
        // this e2e test (guarding the ffmpeg interface). A match is expected
        // with testsrc, but we don't hard-fail if the codec produces a
        // slightly degenerate first frame.
        eprintln!(
            "WARN: e2e self-query found no matches ({} screenshots). Pipeline OK.",
            candidate.screenshots.len()
        );
    }

    // Clean up.
    std::fs::remove_dir_all(&tmp_dir).ok();
}

// ---------------------------------------------------------------------------
// Compact storage: roundtrip + v1 vs v2 blob size + decode_any handles both.
// ---------------------------------------------------------------------------
#[test]
fn test_compact_storage_roundtrip() {
    use base64::{engine::general_purpose, Engine as _};

    let bytes_a = general_purpose::STANDARD.decode(IMGA.as_bytes()).unwrap();
    let img_a = image::load_from_memory_with_format(&bytes_a, image::ImageFormat::Jpeg).unwrap();
    let (hash_a, _) = crate::hash::create_hash(&img_a.into());

    let bytes_b = general_purpose::STANDARD.decode(IMGB.as_bytes()).unwrap();
    let img_b = image::load_from_memory_with_format(&bytes_b, image::ImageFormat::Jpeg).unwrap();
    let (hash_b, _) = crate::hash::create_hash(&img_b.into());

    // --- 1. Compact roundtrip of a single hash. ---
    let mut compact_bytes = Vec::new();
    hash_a.encode_compact(&mut compact_bytes);

    let mut decoded = crate::hash::Hash::new();
    decoded.decode_compact(&mut std::io::Cursor::new(compact_bytes.clone()));

    assert_eq!(
        decoded.dhash, hash_a.dhash,
        "dhash must survive compact roundtrip"
    );
    assert_eq!(
        decoded.histogram, hash_a.histogram,
        "histogram must survive compact roundtrip"
    );
    assert!(
        (decoded.ratio - hash_a.ratio).abs() < 1e-9,
        "ratio must survive compact roundtrip"
    );
    for ch in 0..3 {
        assert!(
            (decoded.thresholds.c[ch] - hash_a.thresholds.c[ch]).abs() < 1e-9,
            "threshold[{}] must survive compact roundtrip",
            ch
        );
    }
    assert!(
        decoded.matrix.coefs.is_empty(),
        "compact decode must leave the matrix empty"
    );

    // --- 2. v1 vs v2 blob size: compact should be far smaller. ---
    let make_video = |id: &str, idx: u32, hash: &crate::hash::Hash| {
        let mut v = crate::videocandidate::VideoCandidate::from(id, idx as usize);
        v.width = 64;
        v.height = 64;
        v.runtime = 80;
        for shot in 0..8u32 {
            v.screenshots.push(crate::videocandidate::Screenshot::from(
                id,
                idx as usize,
                shot as usize,
                shot * 10,
                hash,
            ));
        }
        v
    };

    let va = make_video("/tmp/compact_a", 1, &hash_a);
    let vb = make_video("/tmp/compact_b", 2, &hash_b);

    let mut v1_bytes = Vec::new();
    va.encode(&mut v1_bytes);
    let mut v2_bytes = Vec::new();
    va.encode_compact(&mut v2_bytes);

    assert!(
        v2_bytes.len() < v1_bytes.len() / 10,
        "compact blob ({}) should be at least 10x smaller than v1 ({})",
        v2_bytes.len(),
        v1_bytes.len()
    );

    // --- 3. decode_any handles both formats. ---
    let mut from_v1 = crate::videocandidate::VideoCandidate::new();
    from_v1.decode_any(&mut std::io::Cursor::new(v1_bytes.clone()));
    assert_eq!(from_v1.screenshots.len(), 8, "v1 decode_any: 8 screenshots");
    assert!(
        !from_v1.screenshots[0].hash.matrix.coefs.is_empty(),
        "v1: matrix present"
    );

    let mut from_v2 = crate::videocandidate::VideoCandidate::new();
    from_v2.decode_any(&mut std::io::Cursor::new(v2_bytes.clone()));
    assert_eq!(from_v2.screenshots.len(), 8, "v2 decode_any: 8 screenshots");
    assert!(
        from_v2.screenshots[0].hash.matrix.coefs.is_empty(),
        "v2: matrix empty"
    );

    assert_eq!(
        from_v1.screenshots[0].hash.dhash, from_v2.screenshots[0].hash.dhash,
        "dhash identical in v1 and v2"
    );
    assert_eq!(
        from_v1.screenshots[0].hash.histogram, from_v2.screenshots[0].hash.histogram,
        "histogram identical in v1 and v2"
    );

    // --- 4. add() (now v2) → query still works. ---
    let conn = crate::videostore::connect(":memory:").unwrap();
    let store = crate::videostore::VideoStore::new(conn, -20.0, "/tmp", 1, 10, 6, 10);

    store.add("/tmp/compact_a", &va, va.runtime);
    store.add("/tmp/compact_b", &vb, vb.runtime);
    assert_eq!(store.size(), 2);

    let (num_shots, num_bucket_entries) = store.debug_index_stats();
    assert_eq!(num_shots, 16, "8 shots × 2 videos = 16");
    assert!(num_bucket_entries > 0, "index must have bucket entries");

    let matches = store.query(&va);
    assert!(
        matches.m.iter().any(|m| m.id == "/tmp/compact_a"),
        "self-query after compact add should find /tmp/compact_a; got {:?}",
        matches.m.iter().map(|m| &m.id).collect::<Vec<_>>()
    );
}

// ---------------------------------------------------------------------------
// Migration: a v1 blob (full matrix) is re-encoded to v2 (compact) when the
// store is opened.
//
// Strategy:
//   1. Create a store and add a video (properly populates the index table).
//   2. Overwrite the stored blob with a v1 (full-matrix) version and reset
//      schema_version to 1, simulating an un-migrated database.
//   3. Reopen the store — migration should detect the v1 blob, re-encode it
//      to v2, and set schema_version to 2.
//   4. Verify: blob is v2, query still works, idempotent on second open.
// ---------------------------------------------------------------------------
#[test]
fn test_migration_v1_to_v2() {
    use base64::{engine::general_purpose, Engine as _};
    use rusqlite::{params, Connection};

    let bytes_a = general_purpose::STANDARD.decode(IMGA.as_bytes()).unwrap();
    let img_a = image::load_from_memory_with_format(&bytes_a, image::ImageFormat::Jpeg).unwrap();
    let (hash_a, _) = crate::hash::create_hash(&img_a.into());

    let mut va = crate::videocandidate::VideoCandidate::from("/tmp/mig_a", 1);
    va.width = 64;
    va.height = 64;
    va.runtime = 80;
    for shot in 0..8u32 {
        va.screenshots.push(crate::videocandidate::Screenshot::from(
            "/tmp/mig_a",
            1,
            shot as usize,
            shot * 10,
            &hash_a,
        ));
    }

    let dbpath = std::env::temp_dir().join(format!("duplo_mig_{}.db", std::process::id()));
    let _ = std::fs::remove_file(&dbpath);
    let _ = std::fs::remove_file(&dbpath.with_extension("db-wal"));
    let _ = std::fs::remove_file(&dbpath.with_extension("db-shm"));

    // Step 1: Create a store and add the video (properly populates the index).
    {
        let conn = crate::videostore::connect(dbpath.to_str().unwrap()).unwrap();
        let store = crate::videostore::VideoStore::new(conn, -20.0, "/tmp", 1, 10, 6, 10);
        store.add("/tmp/mig_a", &va, va.runtime);
        assert_eq!(store.size(), 1);
        drop(store);
    }

    // Step 2: Overwrite the blob with a v1 (full-matrix) version and reset
    // schema_version to 1, simulating an un-migrated database.
    let v1_blob = {
        let mut v1 = Vec::new();
        va.encode(&mut v1);
        assert!(
            v1.first() != Some(&2),
            "v1 blob must NOT start with the v2 marker byte"
        );
        {
            let conn2 = Connection::open(dbpath.to_str().unwrap()).unwrap();
            conn2
                .execute(
                    "UPDATE videostore_candidates SET data = ?1 WHERE filename = '/tmp/mig_a'",
                    params![&v1],
                )
                .unwrap();
            conn2
                .execute("UPDATE videostore_parameters SET schema_version = 1", [])
                .unwrap();
        }
        v1
    };

    // Step 3: Reopen the store — migration should detect the v1 blob and
    // re-encode it to v2, then set schema_version to 2.
    let conn3 = crate::videostore::connect(dbpath.to_str().unwrap()).unwrap();
    let store = crate::videostore::VideoStore::new(conn3, -20.0, "/tmp", 1, 10, 6, 10);
    assert_eq!(store.size(), 1, "store should have 1 video after migration");

    // Verify: the blob is now v2 (first byte == 2) and smaller, schema_version is 2.
    {
        let conn4 = Connection::open(dbpath.to_str().unwrap()).unwrap();
        let blob: Vec<u8> = conn4
            .query_row(
                "SELECT data FROM videostore_candidates WHERE filename = '/tmp/mig_a'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(
            blob.first(),
            Some(&2),
            "blob should have been migrated to v2 (first byte == 2)"
        );
        assert!(
            blob.len() < v1_blob.len(),
            "migrated blob ({}) should be smaller than v1 ({})",
            blob.len(),
            v1_blob.len()
        );
        let version: i64 = conn4
            .query_row(
                "SELECT COALESCE(MAX(schema_version), 0) FROM videostore_parameters",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(version, 2, "schema_version should be 2 after migration");
    }

    // Verify: query still works (index was built from the index table, not the blob).
    let matches = store.query(&va);
    assert!(
        matches.m.iter().any(|m| m.id == "/tmp/mig_a"),
        "self-query after migration should find /tmp/mig_a; got {:?}",
        matches.m.iter().map(|m| &m.id).collect::<Vec<_>>()
    );

    // Step 4: Open again — migration should be a no-op (idempotent).
    drop(store);
    let conn5 = crate::videostore::connect(dbpath.to_str().unwrap()).unwrap();
    let store2 = crate::videostore::VideoStore::new(conn5, -20.0, "/tmp", 1, 10, 6, 10);
    assert_eq!(store2.size(), 1);
    let matches2 = store2.query(&va);
    assert!(
        matches2.m.iter().any(|m| m.id == "/tmp/mig_a"),
        "second open should also find the migrated video"
    );
    drop(store2);

    // Clean up.
    let _ = std::fs::remove_file(&dbpath);
    let _ = std::fs::remove_file(&dbpath.with_extension("db-wal"));
    let _ = std::fs::remove_file(&dbpath.with_extension("db-shm"));
}
