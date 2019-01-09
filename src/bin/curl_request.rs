extern crate curl;
extern crate uuid;

use curl::easy::Easy;
use std::io::prelude::*;
use std::io::stdin;
use uuid::Uuid;

fn main() {
    let my_uuid = Uuid::new_v4();
    let url = format!(
        "http://update.mxj360.com/rest/devices/mtool/config?uuid={}",
        my_uuid
    );

    println!(
        "你的帮助代码是: \n{}, \n请复制它，并发送给协助你解决问题的人.\n",
        my_uuid
    );

    let mut easy = Easy::new();
    easy.url(url.as_str()).unwrap();
    easy.post(true).unwrap();
    /*easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();*/

    easy.perform().unwrap();

    println!("返回码：{}", easy.response_code().unwrap());
    println!("按任意键退出...");

    let _ = stdin().read(&mut [0u8]).unwrap();
}
