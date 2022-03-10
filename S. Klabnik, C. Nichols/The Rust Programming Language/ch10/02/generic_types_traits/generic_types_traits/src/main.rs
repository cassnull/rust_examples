use aggregator::{NewsArticle, Summary, Tweet};

fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        };
        Box::new(article)
    } else {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        Box::new(tweet)
    }
}

fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest2<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list {
        if *item > largest {
            largest = item.clone();
        }
    }

    largest
}

fn largest3<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let tweet = returns_summarizable(false);

    println!("1 new tweet: {}", tweet.summarize());

    let article = returns_summarizable(true);

    println!("New article available! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    aggregator::notify(&tweet);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let result = largest2(&number_list);
    println!("The largest number is {}", result);
    let result = largest3(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
    let result = largest2(&char_list);
    println!("The largest char is {}", result);
    let result = largest3(&char_list);
    println!("The largest char is {}", result);
}
