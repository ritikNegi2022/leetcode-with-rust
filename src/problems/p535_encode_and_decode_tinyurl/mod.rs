// 535. Encode and Decode TinyURL
struct Codec {
    urls: Vec<String>,
    base: String,
}

impl Codec {
    fn new() -> Self {
        Codec {
            urls: vec![],
            base: "https://tiny/".to_string(),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, long_url: String) -> String {
        self.urls.push(long_url);
        format!("{}{}", self.base, self.urls.len() - 1)
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, short_url: String) -> String {
        let index: usize = short_url.split("/").last().unwrap().parse().unwrap();
        self.urls[index].clone()
    }
}

pub fn run() {
    println!("Running problem 535: Encode and Decode TinyURL");
    let mut codec = Codec::new();
    let s: String = codec.encode("https://youtube.com".to_string());
    println!("{}", s);
    let d: String = codec.decode(s);
    println!("{}", d)
}
