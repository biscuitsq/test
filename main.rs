use std::borrow::Cow;
use chrono::Weekday;
use chrono::{DateTime, Local};//unixtime
use chrono::prelude::*;
use std::time::Duration;
use std::time::{Instant};//stopwatch
use std::thread;//thread
use websocket::sync::Server;//ws_server
use websocket::{Message, OwnedMessage};//ws_server

fn myfunc<'a, S: Into<Cow<'a, str>>>(val: S,mac: S, offset: u8, count: u8) -> String{
    let starPos = 0;
    let mut s2: String = String::from("");
    s2
}
//getLenght
fn kic_getLenght(val: String,text: String,mac: String, offset: usize, count: usize) -> String{
    
    let mut val1 = &val;
    let mac2 : String = String::from(text);
    let pos : i8 = kic_indexOf(val1,mac2);
    let starPos = pos as usize;
    
    let mut result: String = String::from("");
    let lens :usize = val1.chars().count();
    let mut cc : usize = 0;

    for i in 0..lens{
        let mut ns : usize = i as usize;
        ns = starPos + ns;
        let c :char = val1.chars().nth(ns).unwrap();
        let x : char= mac.chars().nth(0).unwrap();

        if c == x {
            if cc >= count {
                break;
            }
            result = "".to_string();
            cc += 1;
            continue;
        }
        result.push(c);
    }
    result
}
//indexOf
fn kic_indexOf(val : &String, text : String) -> i8{

    let mut res : i8 = -1;
    let lens : usize = val.chars().count();

    let array : Vec<char> = text.chars().collect();
    let tlens : usize = text.chars().count();
    let mut count : usize = 0;

    for i in 0..lens{
        let ns : usize = i as usize;
        let c :char = val.chars().nth(ns).unwrap();

        if array[0] == c{
            count = 0;
            res = i as i8;
            for k in 0..tlens{
                let mut ns2 : usize = i as usize;
                let mut ns3 : usize = k as usize;
                ns3 = ns2 + ns3;
                let c2 :char = val.chars().nth(ns2).unwrap();
                if array[k] == c2{
                    count += 1;
                }   
            }
        }
        if count == tlens{
            break;
        }
    }
    println!("count= {}",count);
    if count != tlens{
        res = -1;
    }
    println!("res= {}",res);
    res
}
fn test_getLenght(){
    let val : String = String::from("\"aaaa\"bbbb\"");
    let text : String = String::from("a");
    let matches : String = String::from("\"");
    let offset = 0 as usize;
    let count = 1 as usize;

    let s3 = kic_getLenght(val,text,matches,offset,count);

    println!("s3 = {}",&s3);

    let val2 : String = String::from("\"aaaa\"bbbb\"");
    let matches2 : String = String::from("babb");
    let mut starPos : i8 = kic_indexOf(&val2,matches2);
}
//unxTime
fn kic_unixTime() -> i64 
{
    let dt : DateTime<Local> = Local::now();
    //現在のローカル時間
    let text = dt.format("%Y年%m月%d日 %H時%M分%S秒 %Z").to_string();
    println!("{}", text);
    let timestamp : i64 = dt.timestamp_millis();
    println!("{}",timestamp);
    timestamp
}
//曜日確認
fn format_japan_weekday(weekday: &Weekday) -> &str {
    match weekday {
        Weekday::Mon => "月",
        Weekday::Tue => "火",
        Weekday::Wed => "水",
        Weekday::Thu => "木",
        Weekday::Fri => "金",
        Weekday::Sat => "土",
        Weekday::Sun => "日",
    }
}
fn dayofweek() -> String{
    let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
    let val = local.weekday();
    let dayofweek = format_japan_weekday(&val);
    dayofweek.to_string()
}
//例外処理1
fn Exception1(){
    let a : Result<i32,_>= "4693".parse();
    let b : Result<i32,_>= "hao123".parse();
    match a{
        Ok(x)=> println!("{}",x),
        Err(_)=> println!("変換に失敗")
    };
    match b{
        Ok(x) => println!("{}",x),
        Err(_) => println!("変換に失敗")
    };
}
//stopwatch//millis
fn stopwatch() -> u32{
    let start = Instant::now();
    for elem in 0..100000 {
        
    }
    let end = start.elapsed();
    let delay_millis = end.subsec_millis();
    let delay_nanos = end.subsec_nanos();
    println!("{}", delay_millis);
    delay_millis
}
//thread_test
fn do_thread(x: i32) -> i32{
    let time = x as u32;
    thread::sleep_ms(time);
    x * 2
}
fn thread_test1(){
    let delay : i32 = 1000;
    thread::spawn(move || {
        let result = do_thread(delay);
        println!("{:?}",result);
    });
    thread::sleep_ms(500);
}
fn thread_test2(){
    let delay : i32 = 10;
    thread::spawn(move || {
        let result = do_thread(delay);
        println!("{:?}",result);
    });
    thread::sleep_ms(500);
}
fn ws_server(){
    let server = Server::bind("localhost:2001").unwrap();

    for request in server.filter_map(Result::ok) {
        thread::spawn(|| {
            let mut client = request.accept().unwrap();

            let ip = client.peer_addr().unwrap();

            println!("Connection from {}", ip);

            for _ in 0..1000 {
                let message = OwnedMessage::Text("Hello".to_string());
                client.send_message(&message).unwrap();
                std::thread::sleep(std::time::Duration::from_millis(50)); // 少し待つ
            }
        });
    }
}
fn main() {
    thread_test1();
    thread_test2();
    let dayofweek = dayofweek();
    println!("{}",dayofweek);
}


//cargo.toml
//[dependencies]
//websocket = "0.23"
//rand = "0.7"
//chrono = { version = "0.4", features = ["serde"] }