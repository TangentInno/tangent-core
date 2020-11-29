mod hashing;
mod inbound;
mod dispatcher;
mod fs;
mod log;

fn print_logo() {
    println!(r"
    _________  ________  ________   ________  _______   ________   _________   
   |\___   ___\\   __  \|\   ___  \|\   ____\|\  ___ \ |\   ___  \|\___   ___\ 
   \|___ \  \_\ \  \|\  \ \  \\ \  \ \  \___|\ \   __/|\ \  \\ \  \|___ \  \_| 
        \ \  \ \ \   __  \ \  \\ \  \ \  \  __\ \  \_|/_\ \  \\ \  \   \ \  \  
         \ \  \ \ \  \ \  \ \  \\ \  \ \  \|\  \ \  \_|\ \ \  \\ \  \   \ \  \ 
          \ \__\ \ \__\ \__\ \__\\ \__\ \_______\ \_______\ \__\\ \__\   \ \__\
           \|__|  \|__|\|__|\|__| \|__|\|_______|\|_______|\|__| \|__|    \|__| 
    ");
}

#[tokio::main]
async fn main() {
    /* Display the tangent logo on run. */
    print_logo();

    /* Create eh folders we will be using */
    match fs::initilize_fs() {
        Ok(_) => {},
        Err(e) => {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                log::print_error("FileSystem", &format!{"There was an issue creating the folders for the file system: {:?}", e})
            } else { log::print_normal("FileSystem", "There already exists folders for the file system.")}
        }
    };

    /* When we get a job system going, we will have no need for this. */
    tokio::spawn(async move {
        let _ = match inbound::reciever::start_inbound_server().await {
            Ok(n) => n,
            Err(e) => log::print_error("Reciever", &format!("The inbound reciever failed to intilize: {:?}", e))
        };
    });

    loop {
        // if ALL JOBS FINISHED
        std::thread::sleep(std::time::Duration::from_millis(1))
    }
}
