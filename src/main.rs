use std::sync::{atomic::{AtomicU64, AtomicU8, Ordering}, Arc, Mutex};
use fltk::{app, button::Button, frame::Frame, group::{Pack, PackType}, prelude::*, window::Window};
use fltk_theme::{WidgetTheme, ThemeType};

//static POM: AtomicU64 = AtomicU64::new(25*60);
struct Pomodoro {
    time: AtomicU64,
    session: AtomicU64,
    session_type: AtomicU8,
}

static POM: Pomodoro = Pomodoro {time: AtomicU64::new(0),
                                 session: AtomicU64::new(0),
                                 session_type: AtomicU8::new(0)};

static WORK_DURATION: u64 = 25*60;
static BREAK_DURATION: u64 = 5*60;

static SESSION_NONE: u8 = 0;
static SESSION_WORK: u8 = 1;
static SESSION_BREAK: u8 = 2;

fn update_timer(frame: &mut Frame, but_work: &mut Button, but_break: &mut Button, wind: &mut Window, handle: *mut ()) {
    let pom = POM.time.fetch_sub(1, Ordering::SeqCst) - 1;
    let mins = pom / 60;
    let secs = pom % 60;
    let time = format!("{}:{:02}", mins, secs);
    frame.set_label(&time);
    if pom == 0 {
        but_work.show();
        but_break.show();
        wind.set_size(120, 100);
        if POM.session_type.load(Ordering::SeqCst) == SESSION_WORK {
            wind.set_label(&format!("Session {}", POM.session.fetch_add(1, Ordering::SeqCst) + 1));
        }
        POM.session_type.store(SESSION_NONE, Ordering::SeqCst)
    } else {
        app::repeat_timeout3(1.0, handle);
    }
}

fn main() {
    let app = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::Metro);
    widget_theme.apply();
    let mut wind = Window::default().with_size(120, 100).with_label("Session 0");
    let mut pack = Pack::default().with_size(120, 100); //.center_of(&wind);
    pack.set_spacing(10);
    let mut frame = Frame::default().with_size(0, 60).with_label("0:00");
    frame.set_label_size(30);
    let mut pack_buttons = Pack::default().with_size(120, 30);
    pack_buttons.set_type(PackType::Horizontal);
    let but_work = Button::default().with_size(60, 30).with_label("Work");
    let but_break = Button::default().with_size(60, 30).with_label("Break");
    pack_buttons.end();
    wind.set_xclass("pomodoro-rs-fltk");
    pack.end();
    //wind.resizable(&wind);
    wind.end();
    wind.show();
    let frame = Arc::new(Mutex::new(frame));
    let frame2 = Arc::clone(&frame);
    let but_break = Arc::new(Mutex::new(but_break));
    let but_work = Arc::new(Mutex::new(but_work));
    let but_break_inside = Arc::clone(&but_break);
    let but_work_inside = Arc::clone(&but_work);
    let wind_inside = Arc::new(Mutex::new(wind));
    let wind_inside2 = Arc::clone(&wind_inside);
    but_work.lock().unwrap().set_callback(move |_| {
        let frame = Arc::clone(&frame);
        let but_work_pass = Arc::clone(&but_work_inside);
        let but_break_pass = Arc::clone(&but_break_inside);
        let wind_pass = Arc::clone(&wind_inside);
        but_work_inside.lock().unwrap().hide();
        but_break_inside.lock().unwrap().hide();
        wind_inside.lock().unwrap().set_size(120, 60);
        POM.time.store(WORK_DURATION, Ordering::SeqCst);
        POM.session_type.store(SESSION_WORK, Ordering::SeqCst);
        app::add_timeout3(1.0, move |handle| update_timer(&mut frame.lock().unwrap(), &mut but_work_pass.lock().unwrap(), &mut but_break_pass.lock().unwrap(),&mut wind_pass.lock().unwrap(), handle) );
    });

    let but_work_inside = Arc::clone(&but_work);
    let but_break_inside = Arc::clone(&but_break);
    but_break.lock().unwrap().set_callback(move |_| {
        let frame = Arc::clone(&frame2);
        let but_work_pass = Arc::clone(&but_work_inside);
        let but_break_pass = Arc::clone(&but_break_inside);
        let wind_pass = Arc::clone(&wind_inside2);
        but_work_inside.lock().unwrap().hide();
        but_break_inside.lock().unwrap().hide();
        wind_inside2.lock().unwrap().set_size(120, 60);
        POM.time.store(BREAK_DURATION, Ordering::SeqCst);
        POM.session_type.store(SESSION_BREAK, Ordering::SeqCst);
        app::add_timeout3(1.0, move |handle| update_timer(&mut frame.lock().unwrap(), &mut but_work_pass.lock().unwrap(), &mut but_break_pass.lock().unwrap(), &mut wind_pass.lock().unwrap(), handle) );
    });   
    app.run().unwrap();
}

