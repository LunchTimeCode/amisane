use comfy_table::{Attribute, Cell, Color, Row, Table};

#[derive(Debug, Clone, Default)]
pub enum State {
    Success,
    Failure,
    Warning,
    #[default]
    Info,
    Intemediate,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            State::Success => "Success",
            State::Failure => "Failure",
            State::Warning => "Warning",
            State::Info => "Info",
            State::Intemediate => "Intemediate",
        };
        write!(f, "{}", s)
    }
}

impl State {
    pub fn color(&self) -> Color {
        match self {
            State::Success => Color::Green,
            State::Failure => Color::Red,
            State::Warning => Color::Yellow,
            State::Info => Color::DarkCyan,
            State::Intemediate => Color::Magenta,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Out {
    text: String,
    state: State,
}

impl Out {
    pub fn to_row(&self) -> Row {
        let mut row = Row::new();
        row.add_cell(Cell::new(self.state.to_string()).bg(self.state.color()));
        row.add_cell(Cell::new(&self.text).bg(self.state.color()));
        row
    }
}

impl From<(String, State)> for Out {
    fn from(x: (String, State)) -> Self {
        Out {
            text: x.0,
            state: x.1,
        }
    }
}

#[derive(Debug, Default)]
pub struct Print {
    outs: Vec<Out>,
}

#[allow(dead_code)]
impl Print {
    pub fn flush(&mut self) {
        let outs: Vec<Out> = self.outs.clone();
        let rows: Vec<Row> = outs.iter().map(|x| x.to_row()).collect();

        let mut table = Table::new();
        table.set_header(vec![
            Cell::new("State").add_attribute(Attribute::Bold),
            Cell::new("Message").add_attribute(Attribute::Bold),
        ]);
        table.add_rows(rows);

        self.outs.clear();
        println!("{table}");
    }

    pub fn add(&mut self, text: impl Into<String>, state: State) {
        self.outs.push(Out::from((text.into(), state)));
    }

    pub fn infos(&mut self, texts: Vec<impl Into<String>>) {
        for text in texts {
            self.add(text, State::Info);
        }
    }

    pub fn info(&mut self, text: &str) {
        self.infos(vec![text]);
    }

    pub fn warnings(&mut self, texts: Vec<impl Into<String>>) {
        for text in texts {
            self.add(text, State::Warning);
        }
    }

    pub fn warning(&mut self, text: &str) {
        self.warnings(vec![text]);
    }

    pub fn successes(&mut self, texts: Vec<impl Into<String>>) {
        for text in texts {
            self.add(text, State::Success);
        }
    }

    pub fn success(&mut self, text: &str) {
        self.successes(vec![text]);
    }

    pub fn failures(&mut self, texts: Vec<impl Into<String>>) {
        for text in texts {
            self.add(text.into(), State::Failure);
        }
    }

    pub fn failure(&mut self, text: impl Into<String>) {
        self.failures(vec![text.into()]);
    }

    pub fn intermediates(&mut self, texts: Vec<&str>) {
        for text in texts {
            self.add(text, State::Intemediate);
        }
    }

    pub fn intermediate(&mut self, text: &str) {
        self.intermediates(vec![text]);
    }
}
