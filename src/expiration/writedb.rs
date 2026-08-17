

pub fn writedb() -> Result<()> {
    let db = ProjectDirs::from("com", "PromoChecker", "PromoChecker").context("Unable to locate db")?;
    let folder = db.data_local_dir();
    fs::create_dir_all(folder)?
    let path = folder.join("database.db");
    let db = Connection::open(path)
    
}