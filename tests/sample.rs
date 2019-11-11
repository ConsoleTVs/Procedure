
extern crate procedure;

#[test]
fn example() {
    let a = procedure::proceed("Download", "example_file.jpg", |progress| -> Result<&str, &str> {
        for _ in 0..100 {
            std::thread::sleep(std::time::Duration::from_millis(10));
            progress.increment(1);
        }
        Ok("example_file.jpg [256 KB]")
    });
    assert_eq!(a.unwrap(), "example_file.jpg [256 KB]");
    let b = procedure::proceed("Download", "some_other.zip", |progress| -> Result<&str, &str> {
        let min = 500;
        let max = 1000;
        for i in min..max {
            std::thread::sleep(std::time::Duration::from_millis(5));
            progress.set_from(min, max, i);
            if i == 975 {
                return Err("some_other.zip [Failed]");
            }
        }
        Ok("some_other.zip [1 MB]")
    });
    assert_eq!(b.unwrap_err(), "some_other.zip [Failed]");
}
