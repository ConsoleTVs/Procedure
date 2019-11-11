
extern crate procedure;

use procedure::{ proceed, Progress };

#[test]
fn example() {
    let a = proceed("Download", "example_file.jpg", |progress: &mut Progress| -> Result<(&str, &str), &str> {
        for _ in 0..100 {
            std::thread::sleep(std::time::Duration::from_millis(10));
            progress.increment(1);
        }
        Ok(("256KB", "example_file.jpg [256 KB]"))
    });
    assert_eq!(a.unwrap(), "256KB");
    let b = proceed("Download", "some_other.zip", |progress: &mut Progress| -> Result<(&str, &str), &str> {
        let min = 500;
        let max = 1000;
        for i in min..max {
            std::thread::sleep(std::time::Duration::from_millis(5));
            progress.set_from(min, max, i);
            if i == 975 {
                return Err("some_other.zip [Failed]");
            }
        }
        Ok(("1MB", "some_other.zip [1 MB]"))
    });
    assert_eq!(b.unwrap_err(), "some_other.zip [Failed]");
}
