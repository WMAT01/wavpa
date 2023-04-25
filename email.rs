// Email module

use lettre::{Message, SmtpTransport, Transport};
use lettre_email::EmailBuilder;
use crate::views::base;

pub fn render_email(confirmation_url: String) -> String {
    format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>WAVPA - Email Confirmation</title>
                <link rel="stylesheet" href="https://your_domain.com/static/css/main.css" />
            </head>
            <body>
                <div class="content">
                    <h1>WAVPA</h1>
                    <h2>Email Confirmation</h2>
                    <p>Thank you for signing up for WAVPA. Please click the link below to confirm your email address:</p>
                    <a href="{confirmation_url}">Confirm Email Address</a>
                </div>
            </body>
        </html>
        "#,
        confirmation_url = confirmation_url
    )
}

/**
// Send confirmation email to the user
pub fn send_confirmation_email(user_email: &str, confirmation_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Construct confirmation URL
    let confirmation_url = format!("https://your_domain.com/confirm/{}", confirmation_token);

    // Use base.html as the basis for the email
    let email_body = base::render_email(confirmation_url);

    // Build the email
    let email = EmailBuilder::new()
        .to(user_email)
        .from("noreply@your_domain.com")
        .subject("Email confirmation - WAVPA")
        .html(email_body)
        .build()?;

    // Send the email using an SMTP server
    let mut mailer = SmtpTransport::builder("smtp.example.com")
        .port(587)
        .credentials("your_username", "your_password")
        .build();
    mailer.send(&email.into())?;

    Ok(())
}
**/
/**
// Email module
use lettre::{Message, SmtpTransport, Transport};
use lettre_email::EmailBuilder;
use crate::views::base;
**/
// Send confirmation email to the user
pub fn send_confirmation_email(user_email: &str, confirmation_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Construct confirmation URL
    let confirmation_url = format!("https://your_domain.com/confirm/{}", confirmation_token);

    // Use base::render_email to create the email body with the confirmation URL
    let email_body = base::render_email(confirmation_url);

    // Build the email
    let email = EmailBuilder::new()
        .to(user_email)
        .from("noreply@your_domain.com")
        .subject("Email confirmation - WAVPA")
        .html(email_body)
        .build()?;

    // Send the email using an SMTP server
    let mut mailer = SmtpTransport::builder("smtp.example.com")
        .port(587)
        .credentials("your_username", "your_password")
        .build();
    mailer.send(&email.into())?;

    Ok(())
}
