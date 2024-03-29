# Procedure

Procedure allows to execute tasks with a visual indication to the end user in the stdout.

This is inteted to be used as a command line display.

![Sample Image](https://image.prntscr.com/image/uSUp9bGTS52VtzKNyta0zg.png)

## Features

- Padded action display (like the rust compiler / cargo).
- Colored output (Yello = Running, Green = Succeed, Red = Failed).
- Provide a description for the task.
- Percentage progress indication at your need (updates on the screen).

## Samples

```rust
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
```
