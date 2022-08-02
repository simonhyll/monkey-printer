use std::thread;
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};


#[tokio::main]
async fn main() -> Result<(), ()> {
    if std::env::args().len() < 2 {
        println!("You didn't the monkeys anything to print 😱");
        return Ok(());
    }

    let m = MultiProgress::new();
    let message: Vec<u8> = std::env::args()
        .into_iter()
        .skip(1)
        .collect::<Vec<String>>()
        .join(" ")
        .as_bytes()
        .into();

    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("🐒🌴🍌");

    let mut found_characters: Vec<u8> = vec![];

    let mut handles = vec![];
    for character in message {
        let pb = m.add(ProgressBar::new(254));
        pb.set_style(sty.clone());

        let h1 = tokio::spawn(async move {
            for i in 0..254 {
                thread::sleep(Duration::from_millis(25));
                if i == (character as i32) - 1 {
                    pb.set_message(format!(
                        "The monkeys think they found it, they gotta think real hard... #{}",
                        i + 1
                    ));
                    thread::sleep(Duration::from_millis(5000));
                } else if i == character as i32 {
                    pb.set_message(format!("They were wrong, this is actually it! #{}", i + 1));
                    thread::sleep(Duration::from_millis(250));
                    pb.finish_and_clear();
                    break;
                } else {
                    pb.set_message(format!("Need more monkeys to crunch the numbers... #{}", i + 1));
                    pb.inc(1);
                }
            }
        });
        handles.push(h1);
        found_characters.push(character);
    }
    futures::future::join_all(handles).await;
    m.clear().unwrap();
    println!("{}", found_characters.iter().map(|x| *x as char).collect::<String>());
    Ok(())
}
