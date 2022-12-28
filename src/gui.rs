// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{}", &buffer);
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        self.label
            .lines()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(0)
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        writeln!(buffer, "{}", self.label).unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + 8
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.width();
        write!(buffer, "+").unwrap();
        for _ in 0..self.width() {
            write!(buffer, "-").unwrap();
        }
        writeln!(buffer, "+").unwrap();
        writeln!(buffer, "|{:^width$}|", &self.label.label).unwrap();
        write!(buffer, "+").unwrap();
        for _ in 0..self.width() {
            write!(buffer, "-").unwrap();
        }
        write!(buffer, "+").unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        std::cmp::max(
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
            self.title.chars().count(),
        )
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.width();
        let mut inner = String::new();
        for widget in &self.widgets {
            widget.draw_into(&mut inner);
        }
        // title
        writeln!(buffer, "+-{:-<width$}-+", "").unwrap();
        writeln!(buffer, "| {:^width$} |", &self.title).unwrap();
        writeln!(buffer, "+={:=<width$}=+", "").unwrap();
        for l in inner.lines() {
            writeln!(buffer, "| {:<width$} |", l).unwrap();
        }
        writeln!(buffer, "+-{:-<width$}-+", "").unwrap();
    }
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}
