use rand::seq::SliceRandom;

const START_TEXT: &str = r#"Hello there, thank you for visiting me.
I am Whatlang, a Rust library for natural language detection. You give me a text, I tell what language it is written in.
"#;

const MID_TEXT_RUS: &str = r#"Ну, как дела чувак? Ты не скучай, зайди на гитхаб, посмотри сорцы да звёздочку поставить не забудь!"#;

const MID_TEXT_UKR: &str = r#"Привіт, чуваче! Дуже радий тебе бачити на просторах Інету!
Якщо ти сьогодні вже поїв трохи борщу з сальцем, почуваєшся у доброму гуморі, то може зайдеш на гітхаб та ткнеш зірочку?
         Я тобі трохи заздрю, бо вже дуже давно не куштував борщ!           ."#;

const MID_TEXT_DEU: &str = r#"Wenn Du verstehst, was ich schreibe, dann bist Du wahrscheinlich deutsch.
Für Dich habe ich eine kleine Gedankenstütze: Kein Bier vor Vier. Ansonsten geht es..... immer los!!!"#;

const MID_TEXT_EPO: &str = r#"Se vi tion komprenas, verŝajne vi estas esperantisto aŭ esperantistino.
Do mi salutegas vin kaj deziras al vi, ke vi neniam perdu la Esperon!
                Ĝis revido!"#;

const MID_TEXT_ESP: &str = r#"¡Hola Amigo!
¿Dónde está la marcha?
¿Vas a tomar una cervezita conmigo cuando acabe la cuarentena?
           ¡Acordado! "#;

const MID_TEXTS: &[&str; 5] = &[
    MID_TEXT_UKR,
    MID_TEXT_RUS,
    MID_TEXT_DEU,
    MID_TEXT_EPO,
    MID_TEXT_ESP,
];

const END_TEXT: &str = r#"You can try to type something yourself.
Try also to click "Debug" to see what is happening inside of me.          ... GO! GO! GO!
"#;

struct Slide(&'static str);

pub struct Show {
    slide_index: usize,
    slides: Vec<Slide>,
    char_iter: std::str::Chars<'static>,
    text: String,
}

impl Show {
    pub fn gen_random() -> Self {
        let start = Slide(START_TEXT);
        let end = Slide(END_TEXT);

        let slides = match MID_TEXTS.choose(&mut rand::thread_rng()) {
            Some(mid_text) => vec![start, Slide(mid_text), end],
            None => vec![start, end],
        };
        Self::new(slides)
    }

    fn new(slides: Vec<Slide>) -> Self {
        let char_iter = slides[0].0.chars();
        Self {
            slide_index: 0,
            text: String::new(),
            slides,
            char_iter,
        }
    }

    pub fn next(&mut self) -> Option<String> {
        match self.char_iter.next() {
            Some(ch) => {
                self.text.push(ch);
                Some(self.text.clone())
            }
            None => {
                self.slide_index += 1;
                if self.slide_index < self.slides.len() {
                    self.char_iter = self.slides[self.slide_index].0.chars();
                    self.text = String::new();
                    Some(String::new())
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show() {
        let mut show = Show::new(vec![Slide("Sal"), Slide("Мир")]);

        assert_eq!(show.next().unwrap(), "S");
        assert_eq!(show.next().unwrap(), "Sa");
        assert_eq!(show.next().unwrap(), "Sal");
        assert_eq!(show.next().unwrap(), "");
        assert_eq!(show.next().unwrap(), "М");
        assert_eq!(show.next().unwrap(), "Ми");
        assert_eq!(show.next().unwrap(), "Мир");
        assert_eq!(show.next(), None);
    }
}
