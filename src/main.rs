use anyhow::{Context, Result};
use image::GenericImageView;
use tao::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::{Icon, WindowBuilder},
    event::{Event, WindowEvent},
};
use wry::{WebContext, WebViewBuilder};

use argh::FromArgs;
use reqwest::Url;

pub fn main() -> Result<()> {
    let conf: Conf = argh::from_env();
    let _host = conf.check_url_get_host().context(format!(
        "url {} parse error, url should start with http:// or https://",
        &conf.url
    ))?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(conf.get_name())
        .with_window_icon(fetch_icon(&conf.url).ok())
        .with_inner_size(LogicalSize::new(conf.width, conf.height))
        .build(&event_loop)?;
    // webContext
    let context_dir = dirs::config_dir()
        .unwrap()
        .join("ohy")
        .join(conf.get_webview_dir());

    let mut web_context = WebContext::new(Some(context_dir));

    let _builder = WebViewBuilder::new_with_web_context(&mut web_context)
        .with_clipboard(true)
        .with_url(&conf.url)
        .with_new_window_req_handler(|_url, _features| {
            dbg!("new window req: {_url} {_features:?}");
            wry::NewWindowResponse::Allow
        });

    let builder = if let Some(user_agent) = conf.user_agent {
        _builder.with_user_agent(user_agent)
    } else {
        _builder
    };

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let _webview = builder.build(&window)?;
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let _webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox)?
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}

#[derive(FromArgs)]
/// conf
struct Conf {
    /// url exemple https://www.github.com
    #[argh(option)]
    url: String,

    /// name
    #[argh(option, short = 'n')]
    name: Option<String>,

    /// width default 1200
    #[argh(option, default = "1200", short = 'w')]
    width: i32,

    /// height default 780
    #[argh(option, default = "800", short = 'h')]
    height: i32,

    /// user agent
    #[argh(option, short = 'a')]
    user_agent: Option<String>,
}

impl Conf {
    fn get_name(&self) -> String {
        if let Some(name) = &self.name {
            name.to_owned()
        } else {
            self.check_url_get_host().unwrap()
        }
    }

    fn check_url_get_host(&self) -> Option<String> {
        if let Ok(uri) = Url::parse(&self.url)
            && let Some(host) = uri.host_str()
        {
            let scheme = uri.scheme();
            if scheme.eq_ignore_ascii_case("https") || scheme.eq_ignore_ascii_case("http") {
                return Some(host.to_owned());
            }
        }
        None
    }

    fn get_webview_dir(&self) -> String {
        let name_default = match &self.name {
            Some(name) => &create_save_dir_str(name),
            _ => "default",
        };
        format!("{}_{}", self.check_url_get_host().unwrap(), name_default)
    }
}

fn fetch_icon(url: &str) -> Result<Icon> {
    // google favicon api
    let url = format!(
        "https://t{}.gstatic.com/faviconV2?client=SOCIAL&type=FAVICON&fallback_opts=TYPE,SIZE,URL&size=64&url={}",
        (url.len() % 3 + 1),
        url
    );
    let res = reqwest::blocking::get(url)?;
    let img_data = res.bytes()?;
    let img = image::load_from_memory(&img_data)?;
    let (width, height) = img.dimensions();
    if width < 17 || height < 17 {
        return Err(anyhow::anyhow!("fetch icon faile"));
    }
    let rgba = img.to_rgba8();
    let icon = Icon::from_rgba(rgba.into_raw(), width, height)?;

    Ok(icon)
}

fn create_save_dir_str(input: &str) -> String {
    // safe dir str [0-9a-zA-Z-_]
    let is_valid = input
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');

    if is_valid {
        input.to_string()
    } else {
        let hash = md5::compute(input.as_bytes());
        format!("{:x}", hash).chars().take(8).collect()
    }
}
