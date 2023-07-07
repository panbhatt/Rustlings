// This will show how the video 11 in the Let's get rusty series depicts traits.

pub trait Summary {
    fn summary(&self) -> String; // This has to be implemented by ALl the Trait implementation classes.

    fn author(&self) -> String {
        // This is the default implementation for the function, it can be implemented or not implemented by the Struct.
        String::from("Author .. Not Implemented")
    }
}

pub struct Tweet {
    pub text: String,
    pub enabled: bool,
    pub author: String,
}

impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("Tweet : text : {}", self.text)
    }

    fn author(&self) -> String {
        self.author.clone()
    }
}

struct NewsArticle {
    text: String,
    author: String,
    news_paper_name: String,
    date_of_print: String,
}

impl Summary for NewsArticle {
    fn summary(&self) -> String {
        format!(
            "NewsPaper : {}, Article -> {}",
            self.news_paper_name, self.text
        )
    }

    fn author(&self) -> String {
        self.author.clone()
    }
}

struct Blog {
    text: String,
    author: String,
    website: String,
}

impl Summary for Blog {
    fn summary(&self) -> String {
        format!("Website : {}, text -> {}", self.website, self.text)
    }
}

fn main() {
    let tweet = Tweet {
        author: "Pankaj Bhatt".to_string(),
        text: "First Tweet".to_string(),
        enabled: false,
    };
    println!(" TWEET   \n \t -> {}", tweet.summary());

    let news_article = NewsArticle {
        text: "GOlden days of my life".to_string(),
        author: "Rahul CHUTIYA".to_string(),
        news_paper_name: "TOI".to_string(),
        date_of_print: "2023-07-07".to_string(),
    };

    println!(
        " NEWS   \n \t -> {}, AUTHOR -> {}",
        news_article.summary(),
        news_article.author()
    );

    notify(&news_article);
}

fn notify(item: &impl Summary) {
    println!(
        "\n**************** BREAKING NEWS ************** \n-> \t{}",
        item.summary()
    )
}
