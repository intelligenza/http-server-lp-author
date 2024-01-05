#[derive(Debug, PartialEq)]
enum Lang {
  English,
  Spanish,
  Chinese,
  Texan,
  Japanese,
  Italian,
}

struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
  let mut v :Vec<Greeting> = Vec::new();

  let g : Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Japanese, message: String::from("こんにちは WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Italian, message: String::from("Ciao WasmEdge!") };
  v.push(g);

  for e in &v {
    println!("{:?} {}", e.lang, e.message);
  }

  if let Some(Greeting {message, ..}) = v.iter().find(|g| g.lang == Lang::Japanese) {
    println!("Japanese message: {message}");
  }
}
