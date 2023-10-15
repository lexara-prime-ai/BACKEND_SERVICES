use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::response::Response;
use tera::Context;

pub struct HtmlMailer {
    pub credentials: Credentials,
    pub smtp_host: String,
    pub template_engine: tera::Tera,
}

impl HtmlMailer {
    pub fn send_email(self, to: &String, template_name: &str, context: &Context) -> Result<Response, Box<dyn std::error::Error>> {
        let html_body = self.template_engine.render(template_name, &context)?;

        let message = Message::builder()
            .subject("Cr8s digest")
            .from("Cr8s irfanghat@hotmail.com".parse()?)
            .to(to.parse()?)
            .header(ContentType::TEXT_HTML)
            .body(html_body)?;

        let mailer = SmtpTransport::relay(&self.smtp_host)
            .unwrap()
            .credentials(self.credentials)
            .build();

        mailer.send(&message).map_err(|e| e.into())
    }
}