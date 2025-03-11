mod server;
mod shortcut_cmd;

fn main() {
    server::start_server();

    shortcut_cmd::adjust_volume(30);
    shortcut_cmd::parse_volume_command("Vad(75)");
}