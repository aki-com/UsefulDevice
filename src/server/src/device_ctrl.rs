
use windows_volume_control::AudioController;

pub fn vol(){
    unsafe {
                
        let mut controller = AudioController::init(None);
        controller.GetSessions();
        controller.GetDefaultAudioEnpointVolumeControl();
        controller.GetAllProcessSessions();
        let test = controller.get_all_session_names();

        println!("{:?}",test);
        let discord_session = controller.get_session_by_name("Discord".to_string());
        println!("{:?}",discord_session.unwrap().getVolume());
        discord_session.unwrap().setVolume(0.5);

        let master_volume = controller.get_session_by_name("master".to_string());
        println!("{:?}",master_volume.unwrap().getVolume());
        master_volume.unwrap().setVolume(0.5);
    }
}