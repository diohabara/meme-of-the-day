extern crate chrono;
use chrono::prelude::*;
use chrono::Duration;
use std::collections::HashMap;
use std::string::String;
use unindent::unindent;

fn create_day_to_meme() -> HashMap<String, (&'static str, &'static str)> {
  let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(); // 閏年も考慮する
  let end_date = NaiveDate::from_ymd_opt(2020, 12, 31).unwrap();
  let memes:Vec<(&str, &str)> = vec![
        (r###"正直感情が無くて全てが滑稽に思えるが"###, "http://blog.livedoor.jp/goldennews/archives/51766045.html"),
        (r###"キリトかなーやっぱw
        一応オタクだけど彼女いるし、俺って退けない性格だしそこら辺とかめっちゃ似てるって言われる()
        握力も31キロあってクラスの女子にたかられる←彼女いるからやめろ！笑
        俺、これでも中1ですよ？
        ps
        彼女はアスナ似です(聞いてねえ
        "###, "https://dic.nicovideo.jp/a/%E3%82%A4%E3%82%AD%E3%83%AA%E3%83%88"),
        (r###"　　　　　　　　　　　　　　　　　　,,,,,,,,,,,,,,,,,,,,,,,,,,,,
        　　　　　　　　　　　　,,--―'''""`ヽ'　　　　　　　　￣`ヽ､
        　　　　　　　　　　 ／　　　　　　　　ヾ　　/　　　　　　　~`ヽ
        　　　　　　　　　／　　　　　　　　　　　ヽ;:　　／"""ヾ　　　ヽ
        　　　　　　　 ／　　　　　　　　;:;;:::''''　　　l　/;:;;:::'''　 ＼　　　i
        　　　　　　／　　　　　　　　／;:;;:::'''　　　　　　　　　　　ヽ　 ヽ
        　　　　　　|　　　　　　　　　|　　　　　　　　　　　　　　　ヽ　　|
        　　　　　 /　　　　　　　　;/　　　　　　　　　　　　　　　　ヽ　ヽ
        　　　　　/　　　　　　　　;:;:ヽ　　　　　　　　　　　　,,,,;;::'''''ヽ　　|
        　　　　　i　　　　　　　　　　/　　,,,,;;:::::::::::::::　　　　　　　__ ヽ　ヽ
        　　　　　|　　　　　　　　　　|　　"　　　＿＿　::::　　'"ゞ'-'　|　　|
        　　　　　|　　　　　　　　　　|.　　　 - '"-ゞ'-'　::::::..　　　　　 |.　|
        　　　　　|　　　　　　　　　;:|　　　　　　　　　　 :::::::　　　　 　 |　:|
        　　　　　 |　　　　　　　　　ヽ.　　　　　　　　　(　,-､ ,:‐､　　　|　|　
        　　　　　　|　　　　　　　／ヾ..　　　　　　　　　　　　　　　　　 |　|
        　　　　　　|　　　　　 　　 　|　　　　　　　　　__,-'ﾆニニヽ .　 |　 |
        ..　　　　　　 |　　　　　　　　`､ヽ　　　　　　　　ヾニ二ン" 　/　 |
        　　　　　　　 |　　　　　　　　　ヽ＼　　　　　　　　　　　　　/　　|
        　　　　　　　　|　　　　　　　　 　l　　`ｰ-::､_　　　　　　　,,..'|ヽ./　
        　　　　　　　　ヽ.　　　　　 　　:人　　　　　　`ー――''''' 　/　ヽ
        　　　　　　　　/;:;:;:;;:;:;:　＿／　　`ｰ-､　　　　　　　　　 ,.-'"　　 ＼ー-､
        　　　　　　　　 　　,.-'"　　＼:　　　　　 ＼　　　　　 .,.-''"　　　　 |
        　　　　　　　　　／.　　　　　＼　　　　　　　 ~＞､,.-''"　　　　　　|
        　　　　,,..-‐'''""　　　　　　　　ヾ　　　　,.-''"| 　　　/――――､/

        　 　　　　　　　　　　うそはうそであると見抜ける人でないと
        　　　　　　　　　　　（掲示板を使うのは）難しい"###, "https://dic.nicovideo.jp/a/%E3%81%86%E3%81%9D%E3%81%AF%E3%81%86%E3%81%9D%E3%81%A7%E3%81%82%E3%82%8B%E3%81%A8%E8%A6%8B%E6%8A%9C%E3%81%91%E3%82%8B%E4%BA%BA%E3%81%A7%E3%81%AA%E3%81%84%E3%81%A8%28%E6%8E%B2%E7%A4%BA%E6%9D%BF%E3%82%92%E4%BD%BF%E3%81%86%E3%81%AE%E3%81%AF%29%E9%9B%A3%E3%81%97%E3%81%84"),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###"席替えでクラスのイケメンと隣になっちゃって「俺の隣で光栄だろ」って言われて思わず「ハァ？」って返したら、後日「君の俺に対する汚物を見るかのような目が快感になってしまった…付き合ってくれない？」って言われて新しい扉を開けてしまってなんだか申し訳ないと思ったけどさすがに断った。"###, "https://dic.nicovideo.jp/a/%E5%98%98%E6%9D%BE"),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###"電源コードを変えると音が変わるのはピュア界では常識です。
        私は発電所から専用線で我が家まで電力を引っ張り込んでいます。
        電線の材質は無酸素銅が最高ですよ。
        おかげで、ウチはミニコンですが、ハイエンドよりいい音がしますよ。

        ちなみに電力会社の違いでも味付けにサがでるよ。

        電力会社　 　　 長所　　　　　　短所　　 お奨め度
        ------------------------------------------------------------------
        東京電力　 　　 バランス　　　ﾓｯｻﾘ遅い　 　　C
        中部電力　　　　低域量感　　 低域強すぎ　　 A+
        関西電力　　　　高域ヌケ　　　特徴薄い　　　 B
        中国電力　　　 透明感　　　　 低域薄い　　　 B+
        北陸電力　 ｳｪｯﾄな艶　　 低域薄い　　 　 A-
        東北電力　　　　密度とSN　 　低域薄い　 　 A+
        四国電力　　色彩感と温度 　 低域薄い　　　　A
        九州電力　　 　 バランス　　　距離感　　 　 　C
        北海道電力 　 低域品質　　　音場狭い 　　 　B-
        沖縄電力　　　　中高域艶　　 ﾓｯｻﾘ遅い 　　 　A"###, "https://lavender.5ch.net/test/read.cgi/pav/1676116020/l50"),
        (r###"この歌詞私のことだ..."###, "https://dic.nicovideo.jp/a/%E3%81%93%E3%81%AE%E6%AD%8C%E8%A9%9E%E7%A7%81%E3%81%AE%E3%81%93%E3%81%A8%E3%81%A0.."),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
        (r###""###, ""),
    ];
  assert_eq!(memes.len(), (end_date - start_date).num_days() as usize + 1);
  let mut date_to_meme: HashMap<String, (&str, &str)> = HashMap::new();
  let date_range = (0..)
    .map(|x| start_date + Duration::days(x))
    .take_while(|&date| date <= end_date);
  for (date, meme) in date_range.zip(memes.iter()) {
    date_to_meme.insert(date.format("%m-%d").to_string(), *meme);
  }
  date_to_meme
}

fn main() {
  let today = Utc::now();
  let date_to_meme = create_day_to_meme();
  let (meme, reference) = date_to_meme[&today.format("%m-%d").to_string()];
  if meme.is_empty() {
    println!(
      "今日({}月{}日)のミームはまだありません。コミットしてみませんか？",
      today.month(),
      today.day()
    );
    return;
  }
  println!("【今日({}月{}日)のミーム】", today.month(), today.day());
  println!("{}", unindent(meme));
  println!("\n出典: {}", reference);
}
