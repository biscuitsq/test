use std::borrow::Cow;
use chrono::{DateTime, Local};

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
fn main() {

    kic_unixTime();
}