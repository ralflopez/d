use askama::Template;

pub enum ToastSeverity {
    Succes,
    Failure,
}

#[derive(Template)]
#[template(path = "toasts/success.html")]
pub struct ToastSuccessTemplate<'a> {
    message: &'a str,
}

#[derive(Template)]
#[template(path = "toasts/failure.html")]
pub struct ToastFailureTemplate<'a> {
    message: &'a str,
}

pub fn with_toast_response(reply_html: String, severity: ToastSeverity, message: &str) -> String {
    let toast_reply = match severity {
        ToastSeverity::Failure => {
            let fail_template = ToastFailureTemplate { message };
            fail_template.render().unwrap()
        }
        ToastSeverity::Succes => {
            let success_template = ToastSuccessTemplate { message };
            success_template.render().unwrap()
        }
    };

    [reply_html, toast_reply].join("\n\n")
}
