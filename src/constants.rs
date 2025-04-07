pub const HELP_MESSAGE: &str = "
Hello there, Human!

You have summoned me. Let's see about getting you what you need.

❓ Need to chat with people?
➡️ Head over to <#1098683584279220236> channel and other humans will converse with you.

❓ Looking for the Code of Conduct?
➡️ Here it is: <https://opensource.facebook.com/code-of-conduct>

❓ Something wrong?
➡️ You can flag an admin with @admin

I hope that resolves your issue!

— HelpBot 🤖
";

// 1356235876996218890
pub const ROLE_CHANNEL_ID: u64 = 903508810688241675;
pub const ROLE_MESSAGE_ID: u64 = 1356364509996126369;

// Role IDs
pub const RED_ROLE_ID: u64 = 1356241153900417146;
pub const GREEN_ROLE_ID: u64 = 1356241255537053920;
pub const BLUE_ROLE_ID: u64 = 1356241300625817610;

pub mod commands {
    pub const HELP: &str = "(0help";
    pub const PING: &str = "(0ping";
    pub const TIME: &str = "(0time";
    pub const CONFIRM_TERMS: &str = "(0terms";
    pub const SETUP_REACTION_ROLES: &str = "(0setuproles";
}
