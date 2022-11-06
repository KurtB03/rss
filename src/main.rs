
use std::error::Error;
use rss::Channel;
use tokio;
use iced::{Settings, Length, Scrollable, scrollable, Row, Button, button};
use iced::widget::{Column, Container, Text};
use iced::Sandbox;


#[derive(Debug, Clone)]
struct Post {
    title : String,
    description : String,
    url : String,

    style : PostStyle,

    btn_state: button::State
}

#[derive(Debug, Clone, Copy)]
struct PostStyle {
    text_size_title : u16,
    text_size_description : u16,
    text_size_url : u16,

    spacing : u16
}

#[derive(Debug)]
struct Reader {
    url : String,
    posts : Vec<Post>,

    post_style : PostStyle,

    scrollable_state: scrollable::State,
    btn_state: button::State
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Messages {
    Refresh,
    Open(String)
}


fn main() {
    println!("{:?}",Reader::run(Settings::default()));
}

impl Reader {
    /*pub fn new(url : String) -> Result<Reader, Box<dyn Error>> {
        Ok(Reader{
            url,
            posts : Vec::new(),

            scrollable_state: scrollable::State::new(),
            btn_state: button::State::new(),

            post_style: PostStyle { text_size_title: 20, text_size_description: 14, text_size_url: 12, spacing : 8}
        })
    } */
    pub fn fetch(&mut self){
        let rt = tokio::runtime::Runtime::new().unwrap();
        let feed = rt.block_on(example_feed(self.url.to_string())).unwrap();
        //let feed = futures::executor::block_on(example_feed(self.url.to_string())).unwrap();
        self.posts = feed.items().iter().take(20).map(|p| 
            Post{
                title : p.title().unwrap_or("No Title Provided").to_string(),
                description : p.description().unwrap_or("No Description Provided").to_string(),
                url : p.link().unwrap_or("No Link Provided").to_string(),
                style : self.post_style,
                btn_state : button::State::new()
            }).collect();
        self.log();
    }
    pub fn log(&self){
        for p in self.posts.iter(){
            println!("{}", p.title);
            println!("{}", p.description);
            println!("{}", p.url);
            println!("------------------------------------------------");
        }
    }


}

impl Sandbox for Reader {
    type Message = Messages;

    fn title(&self) -> String {
        String::from("BMG Feed")
    }


    fn update(&mut self, message: Self::Message) {
        println!("Update called on reader");
        match message {
            Messages::Refresh => self.fetch(),
            Messages::Open(url) => open(url),
        }
    }

    fn new() -> Self {
        Reader{
            url : "https://www.bundesgesundheitsministerium.de/meldungen.xml".to_string(),
            posts : Vec::new(),

            scrollable_state : scrollable::State::new(),
            btn_state: button::State::new(),

            post_style: PostStyle { text_size_title: 20, text_size_description: 14, text_size_url: 12, spacing : 8}
        }
        }
    

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        //let post = Post{title : "Title".to_string(), description: "Description".to_string(), url : "www.url.com".to_string()};

        let mut news = Scrollable::new(&mut self.scrollable_state)
            .spacing(20)
            .padding(25);

        for post in &mut self.posts {
            news = news.push(post.view());
        }
        
        let refresh = Button::new(&mut self.btn_state, Text::new("Reload")).on_press(Messages::Refresh);
        let head = Row::new().push(Text::new("BMG Feed").size(30)).push(refresh);
        let reader = Column::new().push(head).push(news);

        Container::new(reader)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .into()
    }
}

impl Post {
    pub fn view(&mut self) -> iced::Element<Messages> {
        Column::new()
        .push(Text::new(&self.title).size(self.style.text_size_title))
        .push(Text::new(&self.description).size(self.style.text_size_description))
        .push(Button::new(&mut self.btn_state, Text::new(&self.url).size(self.style.text_size_url)).on_press(Messages::Open(self.url.to_string())))
        .spacing(self.style.spacing)
        .into()
    }



}

fn open(url : String) {
    match open::that(url.as_str()) {
        Ok(()) => println!("Opened '{}' successfully.", url),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", url, err),
    }
}

async fn example_feed(url : String) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
    .await?
    .bytes()
    .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}